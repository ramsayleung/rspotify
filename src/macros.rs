/// Create a [`HashSet`](std::collections::HashSet) from a list of `&str` (which
/// will be converted to String internally), to easily create scopes for
/// [`Token`](crate::oauth2::Token) or
/// [`OAuthBuilder`](crate::oauth2::OAuthBuilder).
///
/// Example:
///
/// ```
/// use rspotify::oauth2::TokenBuilder;
/// use rspotify::scopes;
/// use std::collections::HashSet;
/// use chrono::prelude::*;
/// use chrono::Duration;
///
/// let scope = scopes!("playlist-read-private", "playlist-read-collaborative");
/// let tok = TokenBuilder::default()
///     .access_token("test-access_token")
///     .expires_in(Duration::seconds(1))
///     .expires_at(Utc::now())
///     .scope(scope)
///     .refresh_token("...")
///     .build()
///     .unwrap();
/// ```
#[macro_export]
macro_rules! scopes {
    ($($key:expr),*) => {{
        let mut container = ::std::collections::HashSet::new();
        $(
            container.insert($key.to_owned());
        )*
        container
    }};
}

/// If there's an optional value, the macro will return its value. Otherwise, a
/// default value will be returned. This is helpful to handle `$( expr )?`
/// cases in macros.
#[doc(hidden)]
#[macro_export]
macro_rules! opt {
    (, $default:expr) => {
        $default
    };
    ($optional:expr, $default:expr) => {
        $optional
    };
}

/// Private macro to insert either required or optional fields. Pattern matching
/// will accordingly pick the branch, and then insert ($key, $val) into $map.
#[doc(hidden)]
#[macro_export]
macro_rules! params_internal {
    ($map:ident, req, $name:ident, $key:expr, $val:expr) => {
        $map.insert($key, $val);
    };
    ($map:ident, opt, $name:ident, $key:expr, $val:expr) => {
        // Will only insert when `$name` is not None.
        if let Some(ref $name) = $name {
            $map.insert($key, $val);
        }
    };
}

/// TODO: use with_capacity?
/// This macro and [`map_json`] help make the endpoints as concise as possible
/// and boilerplate-free, which is specially important when initializing the
/// parameters of the query with a HashMap/similar. Their items follow the
/// syntax:
///
///   [req|opt] key [=> value]
/// 
/// The first keyword is just to distinguish between a direct insert into the
/// hashmap (required parameter), and an insert only if the value is
/// `Some(...)`, respectively (optional parameter). This is followed by the
/// variable to be inserted, which shall have the same name as the key in the
/// map (meaning that `r#type` may have to be used).
///
/// It also works similarly to how struct initialization works. You may provide
/// a key and a value with `MyStruct { key: value }`, or if both have the same
/// name as the key, `MyStruct { key }` is enough.
///
/// For more information, please refer to the `test::test_map_query` function in
/// this module to see an example, or the real usages in Rspotify's client.
#[doc(hidden)]
#[macro_export]
macro_rules! map_query {
    (
        $(
            $kind:ident $name:ident $( => $val:expr )?
        ),* $(,)?
    ) => ({
        let mut params = $crate::http::Query::new();
        $(
            $crate::params_internal!(
                params,
                $kind,
                $name,
                stringify!($name),
                $crate::opt!($( $val )?, $name)
            );
        )*
        params
    });
}

/// Refer to the [`map_query`] documentation; this is the same but for JSON
/// maps.
#[doc(hidden)]
#[macro_export]
macro_rules! map_json {
    (
        $(
            $kind:ident $name:ident $( => $val:expr )?
        ),* $(,)?
    ) => ({
        let mut params = ::serde_json::map::Map::new();
        $(
            $crate::params_internal!(
                params,
                $kind,
                $name,
                stringify!($name).to_string(),
                json!($crate::opt!($( $val )?, $name))
            );
        )*
        ::serde_json::Value::from(params)
    });
}

#[cfg(test)]
mod test {
    use crate::{map_query, scopes};
    use crate::model::{AlbumId, Market};
    use crate::http::Query;

    #[test]
    fn test_hashset() {
        let scope = scopes!("hello", "world", "foo", "bar");
        assert_eq!(scope.len(), 4);
        assert!(scope.contains(&"hello".to_owned()));
        assert!(scope.contains(&"world".to_owned()));
        assert!(scope.contains(&"foo".to_owned()));
        assert!(scope.contains(&"bar".to_owned()));
    }

    fn test_map_query() {
        // Passed as parameters, for example.
        let id = "Pink Lemonade";
        let artist: Option<&str> = None;
        let album = Some(AlbumId::from_uri("spotify:album:0XOseclZGO4NnaBz5Shjxp").unwrap());
        let market = Some(Market::FromToken);

        let with_macro = map_query! {
            // Mandatory (not an `Option<T>`)
            req id,
            // Is `None`, so it won't be inserted
            opt artist,
            // Can be used directly
            opt album,
            // `Market` needs to be converted to &str
            opt market => market.as_ref(),
        };

        let mut manually = Query::new();
        manually.insert("id", id);
        if let Some(ref artist) = artist {
            manually.insert("artist", artist);
        }
        if let Some(ref market) = market {
            manually.insert("market", market.as_ref());
        }

        assert_eq!(with_macro, manually);
    }
}
