//! Asynchronous implementation of automatic pagination requests.

use crate::model::Page;
use std::error::Error;

/// Alias for `futures::stream::Stream<Item = T>`, since async mode is enabled.
pub trait StreamOrIterator<T>: futures::stream::Stream<Item = T> {}
impl<T, I: futures::stream::Stream<Item = T>> StreamOrIterator<T> for I {}

pub fn page_stream<'a, T, E, Fut, Function>(
    f: Function,
    page_size: u32,
) -> impl futures::stream::Stream<Item = Result<T, E>> + 'a
where
    T: Unpin + 'static,
    E: Error + Unpin + 'static,
    Fut: futures::future::Future<Output = Result<Page<T>, E>>,
    Function: 'a + Fn(u32, u32) -> Fut,
{
    use async_stream::stream;
    let mut offset = 0;
    stream! {
        loop {
            let page = f(page_size, offset).await?;
            offset += page.items.len() as u32;
            for item in page.items {
                yield Ok(item);
            }
            if page.next.is_none() {
                break;
            }
        }
    }
}
