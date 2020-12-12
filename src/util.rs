use chrono::prelude::*;
use getrandom::getrandom;

/// Convert datetime to unix timestamp
pub(in crate) fn datetime_to_timestamp(elapsed: u32) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() + i64::from(elapsed)
}

/// Generate `length` random chars
pub(in crate) fn generate_random_string(length: usize) -> String {
    let alphanum: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let mut buf = vec![0u8; length];
    getrandom(&mut buf).unwrap();
    let range = alphanum.len();

    buf.iter()
        .map(|byte| alphanum[*byte as usize % range] as char)
        .collect()
}
