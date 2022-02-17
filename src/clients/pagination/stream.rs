//! Asynchronous implementation of automatic pagination requests.

use crate::{model::Page, ClientError, ClientResult};

use std::{fmt::Debug, fmt::Display, pin::Pin};

use futures::{future::Future, stream::Stream};

/// Alias for `futures::stream::Stream<Item = T>`, since async mode is enabled.
pub type Paginator<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a>>;

/// This is used to handle paginated requests automatically.
pub fn paginate<'a, T: 'a, Fut, Request: 'a>(
    req: Request,
    page_size: u32,
) -> Paginator<'a, ClientResult<T>>
where
    T: Unpin,
    Fut: Future<Output = ClientResult<Page<T>>>,
    Request: Fn(u32, u32) -> Fut,
{
    use async_stream::stream;
    let mut offset = 0;
    Box::pin(stream! {
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
    })
}

pub async fn write_paginate<'a, Item, Writer: 'a>(
    writer: Writer,
    items: Vec<Item>,
    chunk_size: u32,
) -> String
where
    Writer: Fn(
        &[Item],
        u32,
    ) -> Pin<Box<dyn futures::Future<Output = Result<String, ClientError>> + Send>>,
    Item: Display + Debug,
{
    let mut last_page = String::new();
    for chunk in items.chunks(chunk_size as usize) {
        last_page = writer(chunk, chunk_size).await.unwrap();
    }
    last_page
}
