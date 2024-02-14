//! Utilities for pagination requests. If the configured client is asynchronous,
//! it'll be  based on `futures::stream::Stream`, if it's synchronous it'll just
//! use `std::iter::Iterator`.
//!
//! All implementations export:
//!
//! * A `Paginator` struct which wraps the iterable of items
//! * A `paginate` function, which returns a `Paginator` based on a request that
//!   may be repeated in order to return a continuous sequence of `Page`s
//! * A `paginate_with_ctx` function that does the same as the `paginate`
//!   function, but accepts a generic context that works around lifetime issues
//!   in the async version due to restrictions in HRTBs
//!   (<https://kevincox.ca/2022/04/16/rust-generic-closure-lifetimes/>)
//!
//! Note that `Paginator` should actually be a trait so that a dynamic
//! allocation can be avoided when returning it with `-> impl Iterator<T>`, as
//! opposed to `-> Box<dyn Iterator<T>>`. But since the Spotify clients are
//! trait-based, they can't return anonymous types, and the former option is
//! impossible for now. This is the same small overhead introduced by the
//! `async_trait` crate and that will hopefully be fixed in the future.
//!
//! Both `Paginator` and `paginate` have a lifetime of `'a`. This is because the
//! pagination may borrow the client itself in order to make requests, and said
//! lifetime helps ensure the `Paginator` struct won't outlive the client.

#[cfg(feature = "__sync")]
mod iter;

#[cfg(all(feature = "__async", not(target_arch = "wasm32")))]
mod stream;

#[cfg(all(feature = "__async", target_arch = "wasm32"))]
mod wasm_stream;

#[cfg(feature = "__sync")]
pub use iter::{paginate, paginate_with_ctx, Paginator};

#[cfg(all(feature = "__async", not(target_arch = "wasm32")))]
pub use stream::{paginate, paginate_with_ctx, Paginator};

#[cfg(all(feature = "__async", target_arch = "wasm32"))]
pub use wasm_stream::{paginate, paginate_with_ctx, Paginator};
