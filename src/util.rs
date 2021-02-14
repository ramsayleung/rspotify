use getrandom::getrandom;

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
