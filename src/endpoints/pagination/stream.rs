//! Asynchronous implementation of automatic pagination requests.

use crate::ClientResult;
use crate::model::Page;
use futures::future::Future;
use futures::stream::Stream;

/// Alias for `futures::stream::Stream<Item = T>`, since async mode is enabled.
pub trait Paginator<T>: Stream<Item = T> {}
impl<T, I: Stream<Item = T>> Paginator<T> for I {}

/// This is used to handle paginated requests automatically.
pub fn paginate<'a, T, Fut, Request, S>(
    req: Request,
    page_size: u32,
) -> S
where
    T: Unpin + 'a,
    Fut: Future<Output = ClientResult<Page<T>>>,
    Request: Fn(u32, u32) -> Fut + 'a,
    S: Stream<Item = ClientResult<T>> + 'a
{
    use async_stream::stream;
    let mut offset = 0;
    stream! {
        loop {
            let page = req(page_size, offset).await?;
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
