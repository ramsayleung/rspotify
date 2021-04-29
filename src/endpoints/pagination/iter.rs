//! Synchronous implementation of automatic pagination requests.

use crate::{model::Page, ClientError, ClientResult};

/// Alias for `Iterator<Item = T>`, since sync mode is enabled.
pub trait Paginator<T>: Iterator<Item = T> {}
impl<T, I: Iterator<Item = T>> Paginator<T> for I {}

/// This is used to handle paginated requests automatically.
pub fn paginate<'a, T, Request, It>(
    req: Request,
    page_size: u32,
) -> It
where
    T: 'a,
    Request: Fn(u32, u32) -> ClientResult<Page<T>> + 'a,
    It: Iterator<Item = ClientResult<T>> + 'a
{
    let pages = PageIterator {
        req,
        offset: 0,
        done: false,
        page_size,
    };

    pages.flat_map(|result| ResultIter::new(result.map(|page| page.items.into_iter())))
}

/// Iterator that repeatedly calls a function that returns a page until an empty
/// page is returned.
struct PageIterator<Request> {
    req: Request,
    offset: u32,
    done: bool,
    page_size: u32,
}

impl<T, Request> Iterator for PageIterator<Request>
where
    Request: Fn(u32, u32) -> ClientResult<Page<T>>,
{
    type Item = ClientResult<Page<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        match (self.req)(self.page_size, self.offset) {
            Ok(page) if page.items.is_empty() => {
                self.done = true;
                None
            }
            Ok(page) => {
                self.offset += page.items.len() as u32;
                Some(Ok(page))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

/// Helper to transform a `Result<Iterator<Item = T>, E>` into an `Iterator<Item
/// = Result<T, E>>`.
struct ResultIter<T, I: Iterator<Item = T>> {
    inner: Option<I>,
    err: Option<ClientError>,
}

impl<T, I: Iterator<Item = T>> ResultIter<T, I> {
    pub fn new(res: ClientResult<I>) -> Self {
        match res {
            Ok(inner) => ResultIter {
                inner: Some(inner),
                err: None,
            },
            Err(err) => ResultIter {
                inner: None,
                err: Some(err),
            },
        }
    }
}

impl<T, I: Iterator<Item = T>> Iterator for ResultIter<T, I> {
    type Item = ClientResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.err.take(), &mut self.inner) {
            (Some(err), _) => Some(Err(err)),
            (None, Some(inner)) => inner.next().map(Ok),
            _ => None, // Error already taken
        }
    }
}
