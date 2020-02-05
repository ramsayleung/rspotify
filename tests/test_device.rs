#[cfg(feature = "default-tls")]
extern crate reqwest_default_tls as reqwest;

#[cfg(feature = "native-tls-crate")]
extern crate reqwest_native_tls as reqwest;

#[cfg(feature = "native-tls-vendored")]
extern crate reqwest_native_tls_vendored as reqwest;

#[cfg(feature = "rustls-tls")]
extern crate reqwest_rustls_tls as reqwest;

extern crate rspotify;

//TODO waitting for mutable mockito test framework
#[test]
#[ignore]
fn test_device() {}
