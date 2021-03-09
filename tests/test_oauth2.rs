use chrono::prelude::*;
use chrono::Duration;
use maybe_async::maybe_async;
use rspotify::client::SpotifyBuilder;
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder, Token, TokenBuilder};
use rspotify::scopes;
use std::{collections::HashMap, fs, io::Read, path::PathBuf, thread::sleep};
use url::Url;
mod common;

use common::maybe_async_test;

#[test]
fn test_get_authorize_url() {
    let oauth = OAuthBuilder::default()
        .state("fdsafdsfa")
        .redirect_uri("localhost")
        .scope(scopes!("playlist-read-private"))
        .build()
        .unwrap();

    let creds = CredentialsBuilder::default()
        .id("this-is-my-client-id")
        .secret("this-is-my-client-secret")
        .build()
        .unwrap();

    let spotify = SpotifyBuilder::default()
        .credentials(creds)
        .oauth(oauth)
        .build()
        .unwrap();

    let authorize_url = spotify.get_authorize_url(false).unwrap();
    let hash_query: HashMap<_, _> = Url::parse(&authorize_url)
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect();

    assert_eq!(hash_query.get("client_id").unwrap(), "this-is-my-client-id");
    assert_eq!(hash_query.get("response_type").unwrap(), "code");
    assert_eq!(hash_query.get("redirect_uri").unwrap(), "localhost");
    assert_eq!(hash_query.get("scope").unwrap(), "playlist-read-private");
    assert_eq!(hash_query.get("state").unwrap(), "fdsafdsfa");
}

#[maybe_async]
#[maybe_async_test]
async fn test_read_token_cache() {
    let now: DateTime<Utc> = Utc::now();
    let scope = scopes!("playlist-read-private", "playlist-read-collaborative");

    let tok = TokenBuilder::default()
        .access_token("test-access_token")
        .expires_in(Duration::seconds(3600))
        .expires_at(now)
        .scope(scope.clone())
        .refresh_token("...")
        .build()
        .unwrap();

    let predefined_spotify = SpotifyBuilder::default()
        .token(tok.clone())
        .cache_path(PathBuf::from(".test_read_token_cache.json"))
        .build()
        .unwrap();

    // write token data to cache_path
    predefined_spotify.write_token_cache().unwrap();
    assert!(predefined_spotify.cache_path.exists());

    let oauth_scope = scopes!("playlist-read-private");
    let oauth = OAuthBuilder::default()
        .state("fdasfasfdasd")
        .redirect_uri("http://localhost:8000")
        .scope(oauth_scope)
        .build()
        .unwrap();

    let mut spotify = SpotifyBuilder::default()
        .oauth(oauth)
        .cache_path(PathBuf::from(".test_read_token_cache.json"))
        .build()
        .unwrap();
    // read token from cache file
    let tok_from_file = spotify.read_token_cache().await.unwrap();
    assert_eq!(tok_from_file.scope, scope);
    assert_eq!(tok_from_file.refresh_token.unwrap(), "...");
    assert_eq!(tok_from_file.expires_in, Duration::seconds(3600));
    assert_eq!(tok_from_file.expires_at.unwrap(), now);

    // delete cache file in the end
    fs::remove_file(&spotify.cache_path).unwrap();
}
#[test]
fn test_write_token() {
    let now: DateTime<Utc> = Utc::now();
    let scope = scopes!("playlist-read-private", "playlist-read-collaborative");

    let tok = TokenBuilder::default()
        .access_token("test-access_token")
        .expires_in(Duration::seconds(3600))
        .expires_at(now)
        .scope(scope.clone())
        .refresh_token("...")
        .build()
        .unwrap();

    let spotify = SpotifyBuilder::default()
        .token(tok.clone())
        .cache_path(PathBuf::from(".test_write_token_cache.json"))
        .build()
        .unwrap();

    let tok_str = serde_json::to_string(&tok).unwrap();
    spotify.write_token_cache().unwrap();

    let mut file = fs::File::open(&spotify.cache_path).unwrap();
    let mut tok_str_file = String::new();
    file.read_to_string(&mut tok_str_file).unwrap();

    assert_eq!(tok_str, tok_str_file);
    let tok_from_file: Token = serde_json::from_str(&tok_str_file).unwrap();
    assert_eq!(tok_from_file.scope, scope);
    assert_eq!(tok_from_file.expires_in, Duration::seconds(3600));
    assert_eq!(tok_from_file.expires_at.unwrap(), now);

    // delete cache file in the end
    fs::remove_file(&spotify.cache_path).unwrap();
}

#[test]
fn test_token_is_expired() {
    let scope = scopes!("playlist-read-private", "playlist-read-collaborative");

    let tok = TokenBuilder::default()
        .access_token("test-access_token")
        .expires_in(Duration::seconds(1))
        .expires_at(Utc::now())
        .scope(scope)
        .refresh_token("...")
        .build()
        .unwrap();
    assert!(!tok.is_expired());
    sleep(std::time::Duration::from_secs(2));
    assert!(tok.is_expired());
}

#[test]
fn test_parse_response_code() {
    let spotify = SpotifyBuilder::default().build().unwrap();

    let url = "http://localhost:8888/callback";
    let code = spotify.parse_response_code(url);
    assert_eq!(code, None);

    let url = "http://localhost:8888/callback?code=AQD0yXvFEOvw";
    let code = spotify.parse_response_code(url);
    assert_eq!(code, Some("AQD0yXvFEOvw".to_string()));

    let url = "http://localhost:8888/callback?code=AQD0yXvFEOvw&state=sN#_=_";
    let code = spotify.parse_response_code(url);
    assert_eq!(code, Some("AQD0yXvFEOvw".to_string()));

    let url = "http://localhost:8888/callback?state=sN&code=AQD0yXvFEOvw#_=_";
    let code = spotify.parse_response_code(url);
    assert_eq!(code, Some("AQD0yXvFEOvw".to_string()));
}
