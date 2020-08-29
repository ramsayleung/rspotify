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

/// A more advanced macro that will implement an endpoint for both the async
/// client and the blocking one.
///
/// The macro takes a variable number of functions with the signature you'd
/// expect for an endpoint: public, async, and with a docstring. In order to
/// capture the docstring, it has to use the `#[doc]` macro.
#[macro_export]
macro_rules! endpoint_impl {
    (
        $(
            $(
                #[$attr:meta]
            )*
            pub async fn $name:ident
            // Taking into account basic generic parameters
            $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?
            (
                // With this, it's possible to use `self` in the functions
                // declared inside this macro, but it's limited to an
                // immutable reference for now.
                &$self:ident $(,)?
                // The function may take a variable number of arguments, which
                // may have a trailing comma.
                $($param:ident : $paramty:ty),* $(,)?
            ) -> $ret:ty $code:block
        )*
    ) => {
        impl $crate::client::Spotify {
            $(
                $(#[$attr])*
                pub async fn $name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (&$self, $($param : $paramty),*) -> $ret $code
            )*
        }

        #[cfg(feature = "blocking")]
        impl $crate::blocking::client::Spotify {
            $(
                $(#[$attr])*
                pub fn $name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (&$self, $($param : $paramty),*) -> $ret {
                    $crate::run_blocking! {
                        $crate::client::Spotify::$name($self.0, $($param),*)
                    }
                }
            )*
        }
    };
}
