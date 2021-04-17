//! Utilities for pagination requests. If the configured client is asynchronous,
//! it'll be  based on `futures::stream::Stream`, if it's synchronous it'll just
//! use `std::iter::Iterator`.
//!
//! In the future, this may be extended to use the same interface for all
//! manual, synchronous and asynchronous iteration with a nicer interface. The
//! problem is that it requires GATs to be implemented. See this comment:
//! `<https://github.com/ramsayleung/rspotify/pull/166#issuecomment-793698024>`

#[cfg(feature = "__sync")]
mod iter;
#[cfg(feature = "__async")]
mod stream;

#[cfg(feature = "__sync")]
pub use iter::{paginate, Paginator};
#[cfg(feature = "__async")]
pub use stream::{paginate, Paginator};
