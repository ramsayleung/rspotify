#[cfg(feature = "__sync")]
pub use test as maybe_async_test;

#[cfg(feature = "__async")]
pub use tokio::test as maybe_async_test;
