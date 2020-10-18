use std::io;

use chrono::prelude::*;
use getrandom::getrandom;

use super::client::ClientResult;

/// Convert datetime to unix timestampe
pub(in crate) fn datetime_to_timestamp(elapsed: u32) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() + i64::from(elapsed)
}

/// Generate `length` random characters.
pub(in crate) fn generate_random_string(length: usize) -> ClientResult<String> {
    let alphanum: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let mut buf = vec![0u8; length];
    getrandom(&mut buf).map_err(|e| io::Error::from(e))?;
    let range = alphanum.len();

    let rand = buf
        .iter()
        .map(|byte| alphanum[*byte as usize % range] as char)
        .collect();
    Ok(rand)
}
