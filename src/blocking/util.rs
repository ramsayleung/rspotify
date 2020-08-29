//! Utils function
use chrono::prelude::*;
use getrandom::getrandom;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::string::ToString;

use crate::blocking::oauth2::{SpotifyOAuth, TokenInfo};
use crate::blocking::RT;

pub fn datetime_to_timestamp(elapsed: u32) -> i64 {
    crate::util::datetime_to_timestamp(elapsed)
}

pub fn generate_random_string(length: usize) -> String {
    crate::util::generate_random_string(length)
}

pub fn convert_map_to_string<
    K: Debug + Eq + Hash + ToString,
    V: Debug + ToString,
    S: ::std::hash::BuildHasher,
>(
    map: &HashMap<K, V, S>,
) -> String {
    crate::util::convert_map_to_string(map)
}

pub fn convert_str_to_map(query_str: &mut str) -> HashMap<&str, &str> {
    crate::util::convert_str_to_map(query_str)
}

#[cfg(feature = "browser")]
pub fn request_token(spotify_oauth: &mut SpotifyOAuth) {
    crate::util::request_token(spotify_oauth)
}

pub fn process_token(
    mut spotify_oauth: &mut SpotifyOAuth,
    mut input: &mut String,
) -> Option<TokenInfo> {
    RT.handle()
        .block_on(async move { crate::util::process_token(&mut spotify_oauth, &mut input).await })
}

pub fn process_token_without_cache(
    mut spotify_oauth: &mut SpotifyOAuth,
    mut input: &mut String,
) -> Option<TokenInfo> {
    RT.handle().block_on(async move {
        crate::util::process_token_without_cache(&mut spotify_oauth, &mut input).await
    })
}

/// Get tokenInfo by Authorization
#[cfg(feature = "browser")]
pub fn get_token(mut spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    RT.handle()
        .block_on(async move { crate::util::get_token(&mut spotify_oauth).await })
}

/// Get tokenInfo by Authorization without cache.
#[cfg(feature = "browser")]
pub fn get_token_without_cache(mut spotify_oauth: &mut SpotifyOAuth) -> Option<TokenInfo> {
    RT.handle()
        .block_on(async move { crate::util::get_token_without_cache(&mut spotify_oauth).await })
}

/// Get tokenInfo by authorization and code
pub fn get_token_by_code(mut spotify_oauth: &mut SpotifyOAuth, code: &str) -> Option<TokenInfo> {
    RT.handle()
        .block_on(async move { crate::util::get_token_by_code(&mut spotify_oauth, code).await })
}
