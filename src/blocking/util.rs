//! Utils function

use super::oauth2::{SpotifyOAuth, TokenInfo};
use crate::util::generate_random_string;
#[cfg(feature = "browser")]
pub fn request_token(spotify_oauth: &mut SpotifyOAuth) {
    let state = generate_random_string(16);
    let auth_url = spotify_oauth.get_authorize_url(Some(&state), None);
    match webbrowser::open(&auth_url) {
        Ok(_) => println!("Opened {} in your browser", auth_url),
        Err(why) => eprintln!("Error {:?};Please navigate here [{:?}] ", why, auth_url),
    }
}

pub fn process_token(spotify_oauth: &mut SpotifyOAuth, input: &mut String) -> Option<TokenInfo> {
    match spotify_oauth.parse_response_code(input) {
        Some(code) => spotify_oauth.get_access_token(&code),
        None => None,
    }
}

pub fn process_token_without_cache(
    spotify_oauth: &mut SpotifyOAuth,
    input: &mut String,
) -> Option<TokenInfo> {
    match spotify_oauth.parse_response_code(input) {
        Some(code) => spotify_oauth.get_access_token_without_cache(&code),
        None => None,
    }
}

/// Get tokenInfo by Authorization
#[cfg(feature = "browser")]
pub fn get_token(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    use std::io;
    match spotify_oauth.get_cached_token() {
        Some(token_info) => Some(token_info),
        None => {
            request_token(spotify_oauth);
            println!("Enter the URL you were redirected to: ");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => process_token(spotify_oauth, &mut input),
                Err(_) => None,
            }
        }
    }
}

/// Get tokenInfo by Authorization without cache.
#[cfg(feature = "browser")]
pub fn get_token_without_cache(spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    use std::io;
    request_token(spotify_oauth);
    println!("Enter the URL you were redirected to: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => process_token_without_cache(spotify_oauth, &mut input),
        Err(_) => None,
    }
}

/// Get tokenInfo by authorization and code
pub fn get_token_by_code(spotify_oauth: &mut SpotifyOAuth, code: &str) -> Option<TokenInfo> {
    spotify_oauth.get_access_token(&code)
}
