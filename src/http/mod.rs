//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients interchangeably.

// Disable all modules when both client features are enabled or when none are.
// This way only the compile error below gets shown instead of a whole list of
// confusing errors..

// If no client is configured the crate is empty to avoid compilation errors
#[cfg(any(feature = "__async", feature = "__sync"))]
mod common;
#[cfg(any(feature = "__async", feature = "__sync"))]
pub use common::{BaseHttpClient, Form, Headers, HttpError, HttpResult, Query};

#[cfg(feature = "client-reqwest")]
pub mod reqwest;
#[cfg(feature = "client-reqwest")]
pub use self::reqwest::ReqwestClient;

#[cfg(feature = "client-ureq")]
pub mod ureq;
#[cfg(feature = "client-ureq")]
pub use self::ureq::UreqClient;
