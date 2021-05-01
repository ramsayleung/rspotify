mod common;

use chrono::prelude::*;
use chrono::Duration;
use maybe_async::maybe_async;
use rspotify::{
    prelude::*, scopes, ClientCredentialsSpotify, CodeAuthSpotify, Config, Credentials, OAuth,
    Token,
};
use std::{collections::HashMap, fs, io::Read, path::PathBuf, thread::sleep};
use url::Url;

use common::maybe_async_test;

#[test]
fn test_get_authorize_url() {
    let oauth = OAuth {
        state: "fdsafdsfa".to_owned(),
        redirect_uri: "localhost".to_owned(),
        scope: scopes!("playlist-read-private"),
        ..Default::default()
    };
    let creds = Credentials::new("this-is-my-client-id", "this-is-my-client-secret");

    let spotify = CodeAuthSpotify::new(creds, oauth);

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
    let now = Utc::now();
    let scope = scopes!("playlist-read-private", "playlist-read-collaborative");

    let tok = Token {
        access_token: "test-access_token".to_owned(),
        expires_in: Duration::seconds(3600),
        expires_at: Some(now),
        scope: scope.clone(),
        refresh_token: Some("...".to_owned()),
    };

    let config = Config {
        cache_path: PathBuf::from(".test_read_token_cache.json"),
        ..Default::default()
    };
    let mut predefined_spotify = ClientCredentialsSpotify::default();
    predefined_spotify.config = config.clone();
    predefined_spotify.token = Some(tok.clone());

    // write token data to cache_path
    predefined_spotify.write_token_cache().unwrap();
    assert!(predefined_spotify.config.cache_path.exists());

    let mut spotify = ClientCredentialsSpotify::default();
    spotify.config = config;

    // read token from cache file
    let tok_from_file = spotify.read_token_cache().await.unwrap();
    assert_eq!(tok_from_file.scope, scope);
    assert_eq!(tok_from_file.refresh_token.unwrap(), "...");
    assert_eq!(tok_from_file.expires_in, Duration::seconds(3600));
    assert_eq!(tok_from_file.expires_at.unwrap(), now);

    // delete cache file in the end
    fs::remove_file(&spotify.config.cache_path).unwrap();
}

#[test]
fn test_write_token() {
    let now = Utc::now();
    let scope = scopes!("playlist-read-private", "playlist-read-collaborative");

    let tok = Token {
        access_token: "test-access_token".to_owned(),
        expires_in: Duration::seconds(3600),
        expires_at: Some(now),
        scope: scope.clone(),
        refresh_token: Some("...".to_owned()),
    };

    let config = Config {
        cache_path: PathBuf::from(".test_write_token_cache.json"),
        ..Default::default()
    };
    let mut spotify = ClientCredentialsSpotify::default();
    spotify.token = Some(tok.clone());
    spotify.config = config;

    let tok_str = serde_json::to_string(&tok).unwrap();
    spotify.write_token_cache().unwrap();

    let mut file = fs::File::open(&spotify.config.cache_path).unwrap();
    let mut tok_str_file = String::new();
    file.read_to_string(&mut tok_str_file).unwrap();

    assert_eq!(tok_str, tok_str_file);
    let tok_from_file: Token = serde_json::from_str(&tok_str_file).unwrap();
    assert_eq!(tok_from_file.scope, scope);
    assert_eq!(tok_from_file.expires_in, Duration::seconds(3600));
    assert_eq!(tok_from_file.expires_at.unwrap(), now);

    // delete cache file in the end
    fs::remove_file(&spotify.config.cache_path).unwrap();
}

#[test]
fn test_token_is_expired() {
    let scope = scopes!("playlist-read-private", "playlist-read-collaborative");

    let tok = Token {
        scope,
        access_token: "test-access_token".to_owned(),
        expires_in: Duration::seconds(1),
        expires_at: Some(Utc::now()),
        refresh_token: Some("...".to_owned()),
    };
    assert!(!tok.is_expired());
    sleep(std::time::Duration::from_secs(2));
    assert!(tok.is_expired());
}

#[test]
fn test_parse_response_code() {
    let spotify = CodeAuthSpotify::default();

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
