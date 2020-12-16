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

#[cfg(feature = "__sync")]
pub(in crate) fn page_iterator<'a, T, E, Function>(
    f: Function,
) -> impl Iterator<Item = Result<T, E>> + 'a
where
    T: Unpin + 'static,
    E: Error + Unpin + 'static,
    Function: 'a + Fn(u32, u32) -> Result<Page<T>, E>,
{
    use itertools::Either;
    use std::iter::once;

    let pages = PageIterator {
        f,
        offset: 0,
        done: false,
    };
    pages.flat_map(|result| match result {
        Ok(page) => Either::Left(page.items.into_iter().map(Ok)),
        Err(e) => Either::Right(once(Err(e))),
    })
}

struct PageIterator<T, E, Function>
where
    T: Unpin + 'static,
    E: Error + Unpin + 'static,
    Function: Fn(u32, u32) -> Result<Page<T>, E>,
{
    f: Function,
    offset: u32,
    done: bool,
}

impl<T, E, Function> Iterator for PageIterator<T, E, Function>
where
    T: Unpin + 'static,
    E: Error + Unpin + 'static,
    Function: Fn(u32, u32) -> Result<Page<T>, E>,
{
    type Item = Result<Page<T>, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = match (self.f)(50, self.offset) {
                Ok(page) if page.items.is_empty() => {
                    self.done = true;
                    None
                }
                Ok(page) => Some(Ok(page)),
                Err(e) => Some(Err(e)),
            };
            self.offset += 50;
            result
        }
    }
}
