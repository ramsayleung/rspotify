//! This mutex wraps both the synchronous and asynchronous versions under the
//! same interface.

#[cfg(feature = "__async")]
mod futures;
#[cfg(feature = "__sync")]
mod sync;

#[cfg(feature = "__async")]
pub use self::futures::FuturesMutex as Mutex;
#[cfg(feature = "__sync")]
pub use self::sync::SyncMutex as Mutex;