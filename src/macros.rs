/// A macro to automatically run a blocking version of some async function,
/// used for the blocking module.
#[macro_export]
macro_rules! run_blocking {
    ($original: expr) => {
        $crate::blocking::RT
            .handle()
            .block_on(async move { $original.await })
    };
}

/// A more advanced macro that will implement an endpoint for both async code
/// and blocking.
#[macro_export]
macro_rules! endpoint_impl {
  (pub async fn $name:ident (&$self:ident, $($param:ident : $paramty:ty),*) -> $ret:ty $code:block) => {
    impl $crate::client::Spotify {
      pub async fn $name (&$self, $($param : $paramty),*) -> $ret $code
    }

    #[cfg(feature = "blocking")]
    impl $crate::blocking::client::Spotify {
      pub fn $name (&$self, $($param : $paramty),*) -> $ret {
        run_blocking! {
          $crate::client::Spotify::$name($self.0, $($param),*)
        }
      }
    }
  }
}
