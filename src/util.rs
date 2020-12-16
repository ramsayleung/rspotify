use crate::model::Page;
use chrono::prelude::*;
use getrandom::getrandom;
use std::error::Error;

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

#[cfg(feature = "__async")]
pub(in crate) fn page_stream<'a, T, E, Fut, Function>(
    f: Function,
) -> impl futures_util::stream::Stream<Item = Result<T, E>> + 'a
where
    T: Unpin + 'static,
    E: Error + Unpin + 'static,
    Fut: futures_util::future::Future<Output = Result<Page<T>, E>>,
    Function: 'a + Fn(u32, u32) -> Fut,
{
    use async_stream::stream;
    let mut offset = 0;
    stream! {
        loop {
            let page = f(50, offset).await?;
            for item in page.items {
                yield Ok(item);
            }
            offset += 50;
            if page.next.is_none() {
                break;
            }
        }
    }
}
