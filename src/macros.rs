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

#[macro_export]
macro_rules! params_internal {
    ($map:ident, req, $name:ident, $key:expr, $val:expr) => {
        $map.insert($key, $val);
    };
    ($map:ident, opt, $name:ident, $key:expr, $val:expr) => {
        if let Some(ref $name) = $name {
            $map.insert($key, $val);
        }
    };
}

/// TODO: use with_capacity?
#[macro_export]
macro_rules! map_query {
    (
        $(
            $kind:ident $name:ident => $val:expr
        ),* $(,)?
    ) => ({
        let mut params = crate::http::Query::new();
        $(
            crate::params_internal!(
                params,
                $kind,
                $name,
                stringify!($name),
                $val
            );
        )*
        params
    });
}

#[macro_export]
macro_rules! map_json {
    (
        $(
            $kind:ident $name:ident => $val:expr
        ),* $(,)?
    ) => ({
        let mut params = ::serde_json::map::Map::new();
        $(

            crate::params_internal!(
                params,
                $kind,
                $name,
                stringify!($name).to_string(),
                json!($val)
            );
        )*
        ::serde_json::Value::from(params)
    });
}

#[cfg(test)]
mod test {
    use crate::{json_insert, scopes};
    use serde_json::json;

    #[test]
    fn test_hashset() {
        let scope = scopes!("hello", "world", "foo", "bar");
        assert_eq!(scope.len(), 4);
        assert!(scope.contains(&"hello".to_owned()));
        assert!(scope.contains(&"world".to_owned()));
        assert!(scope.contains(&"foo".to_owned()));
        assert!(scope.contains(&"bar".to_owned()));
    }

    #[test]
    fn test_json_insert() {
        let mut params = json!({});
        let name = "ramsay";
        json_insert!(params, "name", name);
        assert_eq!(params["name"], name);
    }
}
