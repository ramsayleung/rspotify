//! Synchronization primitives that have both synchronous and asynchronous variants under the same
//! interface.

#[cfg(feature = "__sync")]
mod blocking;
#[cfg(feature = "__async")]
mod futures;

#[cfg(feature = "__sync")]
use self::blocking as imp;
#[cfg(feature = "__async")]
use self::futures as imp;

/// A type alias for either an asynchronous mutex or [`std::sync::Mutex`], depending on whether
/// this library is compiled in asynchronous or synchronous mode.
pub type Mutex<T> = imp::Mutex<T>;
