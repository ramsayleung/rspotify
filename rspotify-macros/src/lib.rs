/// Create a [`HashSet`](std::collections::HashSet) from a list of `&str` to
/// easily create scopes for `Token` or `OAuth`.
///
/// Example:
///
/// ```
/// use rspotify::{Token, scopes};
/// use std::collections::HashSet;
/// use chrono::{Duration, prelude::*};
///
/// let scope = scopes!("playlist-read-private", "playlist-read-collaborative");
/// let tok = Token {
///     scope,
///     access_token: "test-access_token".to_owned(),
///     expires_in: Duration::seconds(1),
///     expires_at: Some(Utc::now().to_owned()),
///     refresh_token: Some("...".to_owned()),
/// };
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

/// Count items in a list of items within a macro, taken from here:
/// https://danielkeep.github.io/tlborm/book/blk-counting.html
#[doc(hidden)]
#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! count_items {
    ($($item:expr),*) => {<[()]>::len(&[$($crate::replace_expr!($item ())),*])};
}

/// This macro and [`build_json`] help make the endpoints as concise as possible
/// and boilerplate-free, which is specially important when initializing the
/// parameters of the query. In the case of `build_map` this will construct a
/// `HashMap<&str, &str>`, and `build_json` will initialize a
/// `HashMap<String, serde_json::Value>`.
///
/// The syntax is the following:
///
///   [optional] "key": value
///
/// For an example, refer to the `test::test_build_map` function in this module,
/// or the real usages in Rspotify's client.
///
/// The `key` and `value` parameters are what's to be inserted in the HashMap.
/// If `optional` is used, the value will only be inserted if it's a
/// `Some(...)`.
#[doc(hidden)]
#[macro_export]
macro_rules! internal_build_map {
    (/* required */, $map:ident, $key:expr, $val:expr) => {
        $map.insert($key, $val);
    };
    (optional, $map:ident, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $map.insert($key, val);
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! build_map {
    (
        $(
            $( $kind:ident )? $key:literal : $val:expr
        ),+ $(,)?
    ) => {{
        let mut params = ::std::collections::HashMap::<&str, &str>::with_capacity(
            $crate::count_items!($( $key ),*)
        );
        $(
            $crate::internal_build_map!(
                $( $kind )?,
                params,
                $key,
                $val
            );
        )+
        params
    }};
}

/// Refer to the [`build_map`] documentation; this is the same but for JSON
/// maps.
#[doc(hidden)]
#[macro_export]
macro_rules! internal_build_json {
    (/* required */, $map:ident, $key:expr, $val:expr) => {
        $map.insert($key.to_string(), json!($val));
    };
    (optional, $map:ident, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $map.insert($key.to_string(), json!(val));
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! build_json {
    (
        $(
            $( $kind:ident )? $key:literal : $val:expr
        ),+ $(,)?
    ) => {{
        let mut params = ::serde_json::map::Map::with_capacity(
            $crate::count_items!($( $key ),*)
        );
        $(
            $crate::internal_build_json!(
                $( $kind )?,
                params,
                $key,
                $val
            );
        )+
        ::serde_json::Value::from(params)
    }};
}

#[cfg(test)]
mod test {
    use crate::{build_json, build_map, scopes};
    use rspotify::model::Market;
    use serde_json::{json, Map, Value};
    use std::collections::HashMap;

    #[test]
    fn test_hashset() {
        let scope = scopes!("hello", "world", "foo", "bar");
        assert_eq!(scope.len(), 4);
        assert!(scope.contains("hello"));
        assert!(scope.contains("world"));
        assert!(scope.contains("foo"));
        assert!(scope.contains("bar"));
    }

    #[test]
    fn test_build_map() {
        // Passed as parameters, for example.
        let id = "Pink Lemonade";
        let artist = Some("The Wombats");
        let market: Option<&Market> = None;

        let with_macro = build_map! {
            // Mandatory (not an `Option<T>`)
            "id": id,
            // Can be used directly
            optional "artist": artist,
            // `Modality` needs to be converted to &str
            optional "market": market.map(|x| x.as_ref()),
        };

        let mut manually = HashMap::<&str, &str>::with_capacity(3);
        manually.insert("id", id);
        if let Some(val) = artist {
            manually.insert("artist", val);
        }
        if let Some(val) = market.map(|x| x.as_ref()) {
            manually.insert("market", val);
        }

        assert_eq!(with_macro, manually);
    }

    #[test]
    fn test_json_query() {
        // Passed as parameters, for example.
        let id = "Pink Lemonade";
        let artist = Some("The Wombats");
        let market: Option<&Market> = None;

        let with_macro = build_json! {
            "id": id,
            optional "artist": artist,
            optional "market": market.map(|x| x.as_ref()),
        };

        let mut manually = Map::with_capacity(3);
        manually.insert("id".to_string(), json!(id));
        if let Some(val) = artist.map(|x| json!(x)) {
            manually.insert("artist".to_string(), val);
        }
        if let Some(val) = market.map(|x| x.as_ref()).map(|x| json!(x)) {
            manually.insert("market".to_string(), val);
        }

        assert_eq!(with_macro, Value::from(manually));
    }
}
