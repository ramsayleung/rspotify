//! Asynchronous implementation of automatic pagination requests.

use crate::{model::Page, ClientResult};

use std::pin::Pin;

use futures::{future::Future, stream::Stream};

/// Alias for `futures::stream::Stream<Item = T>`, since async mode is enabled.
pub type Paginator<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a + Send>>;

pub type RequestFuture<'a, T> = Pin<Box<dyn 'a + Future<Output = ClientResult<Page<T>>> + Send>>;

/// This is used to handle paginated requests automatically.
pub fn paginate_with_ctx<'a, Ctx: 'a + Send, T, Request>(
    ctx: Ctx,
    req: Request,
    page_size: u32,
) -> Paginator<'a, ClientResult<T>>
where
    T: 'a + Unpin + Send,
    Request: 'a + for<'ctx> Fn(&'ctx Ctx, u32, u32) -> RequestFuture<'ctx, T> + Send,
{
    use async_stream::stream;
    let mut offset = 0;
    Box::pin(stream! {
        loop {
            let request = req(&ctx, page_size, offset);
            let page = request.await?;
            offset += page.items.len() as u32;
            for item in page.items {
                yield Ok(item);
            }
            if page.next.is_none() {
                break;
            }
        }
    })
}

pub fn paginate<'a, T, Fut, Request>(req: Request, page_size: u32) -> Paginator<'a, ClientResult<T>>
where
    T: 'a + Unpin + Send,
    Fut: Future<Output = ClientResult<Page<T>>> + Send,
    Request: 'a + Fn(u32, u32) -> Fut + Send,
{
    use async_stream::stream;
    let mut offset = 0;
    Box::pin(stream! {
        loop {
            let request = req(page_size, offset);
            let page = request.await?;
            offset += page.items.len() as u32;
            for item in page.items {
                yield Ok(item);
            }
            if page.next.is_none() {
                break;
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::paginate;
    use crate::model::Page;
    use futures::{future, StreamExt};
    use std::future::Future;

    fn schedule_future<'a, F>(fut: F)
    where
        F: Future + Send + 'a,
    {
        futures::executor::block_on(fut);
    }

    #[test]
    fn test_mt_scheduling() {
        async fn test() {
            let mut paginator = paginate(
                |_, offset| {
                    let fake_page = Page {
                        items: vec![offset, offset + 1, offset + 2],
                        ..Page::default()
                    };
                    future::ok(fake_page)
                },
                32,
            );

            let mut expected = [0, 1, 2].into_iter();
            while let Some(item) = paginator.next().await {
                assert_eq!(expected.next().unwrap(), item.unwrap());
            }
        }
        schedule_future(test());
    }
}
