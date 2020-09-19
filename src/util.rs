//! utils function
use chrono::prelude::*;
use getrandom::getrandom;

use super::oauth2::{SpotifyOAuth, TokenInfo};

/// convert datetime to unix timestampe
pub fn datetime_to_timestamp(elapsed: u32) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() + i64::from(elapsed)
}
/// generate `length` random chars
pub fn generate_random_string(length: usize) -> String {
    let alphanum: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let mut buf = vec![0u8; length];
    getrandom(&mut buf).unwrap();
    let range = alphanum.len();

    buf.iter()
        .map(|byte| alphanum[*byte as usize % range] as char)
        .collect()
}

#[cfg(feature = "browser")]
pub fn request_token(spotify_oauth: &mut SpotifyOAuth) {
    let state = generate_random_string(16);
    let auth_url = spotify_oauth.get_authorize_url(Some(&state), None);
    match webbrowser::open(&auth_url) {
        Ok(_) => println!("Opened {} in your browser", auth_url),
        Err(why) => eprintln!("Error {:?};Please navigate here [{:?}] ", why, auth_url),
    }
}

pub async fn process_token(
    spotify_oauth: &mut SpotifyOAuth,
    input: &mut String,
) -> Option<TokenInfo> {
    match spotify_oauth.parse_response_code(input) {
        Some(code) => spotify_oauth.get_access_token(&code).await,
        None => None,
    }
}

pub async fn process_token_without_cache(
    spotify_oauth: &mut SpotifyOAuth,
    input: &mut String,
) -> Option<TokenInfo> {
    match spotify_oauth.parse_response_code(input) {
        Some(code) => spotify_oauth.get_access_token_without_cache(&code).await,
        None => None,
    }
}

/// get tokenInfo by Authorization
#[cfg(feature = "browser")]
pub async fn get_token(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    use std::io;
    match spotify_oauth.get_cached_token().await {
        Some(token_info) => Some(token_info),
        None => {
            request_token(spotify_oauth);
            println!("Enter the URL you were redirected to: ");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => process_token(spotify_oauth, &mut input).await,
                Err(_) => None,
            }
        }
    }
}

/// get tokenInfo by Authorization without cache
#[cfg(feature = "browser")]
pub async fn get_token_without_cache(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    use std::io;
    request_token(spotify_oauth);
    println!("Enter the URL you were redirected to: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => process_token_without_cache(spotify_oauth, &mut input).await,
        Err(_) => None,
    }
}

/// get tokenInfo by authorization and code
pub async fn get_token_by_code(spotify_oauth: &mut SpotifyOAuth, code: &str) -> Option<TokenInfo> {
    spotify_oauth.get_access_token(&code).await
}
