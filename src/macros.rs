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
/// parameters of the query with a HashMap/similar. Their items follow the
/// syntax:
///
///   [optional] "key": value
///
/// The first keyword is just to distinguish between a direct insert into the
/// hashmap (required parameter), and an insert only if the value is `Some(...)`
/// (optional parameter). This is followed by the variable to be inserted, which
/// shall have the same name as the key in the map (meaning that `r#type` may
/// have to be used).
///
/// It also works similarly to how struct initialization works. You may provide
/// a key and a value with `MyStruct { key: value }`, or if both have the same
/// name as the key, `MyStruct { key }` is enough.
///
/// For more information, please refer to the `test::test_build_map` function in
/// this module to see an example, or the real usages in Rspotify's client.
#[doc(hidden)]
#[macro_export]
macro_rules! build_map {
    (@/* required */, $map:ident, $key:expr, $val:expr) => {
        $map.insert($key, $val);
    };
    (@optional, $map:ident, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $map.insert($key, val);
        }
    };

    (
        $(
            $( $kind:ident )? $key:literal : $val:expr
        ),+ $(,)?
    ) => {{
        let mut params = $crate::http::Query::with_capacity(
            $crate::count_items!($( $key ),*)
        );
        $(
            $crate::build_map!(
                @$( $kind )?,
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
macro_rules! build_json {
    (@/* required */, $map:ident, $key:expr, $val:expr) => {
        $map.insert($key.to_string(), json!($val));
    };
    (@optional, $map:ident, $key:expr, $val:expr) => {
        if let Some(val) = $val {
            $map.insert($key.to_string(), json!(val));
        }
    };

    (
        $(
            $( $kind:ident )? $key:literal : $val:expr
        ),+ $(,)?
    ) => {{
        let mut params = ::serde_json::map::Map::with_capacity(
            $crate::count_items!($( $key ),*)
        );
        $(
            $crate::build_json!(
                @$( $kind )?,
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
    use crate::http::Query;
    use crate::model::Market;
    use crate::{build_json, build_map, scopes};
    use serde_json::{json, Map, Value};

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
        let market: Option<Market> = None;

        let with_macro = build_map! {
            // Mandatory (not an `Option<T>`)
            "id": id,
            // Can be used directly
            optional "artist": artist,
            // `Modality` needs to be converted to &str
            optional "market": market.map(|x| x.as_ref()),
        };

        let mut manually = Query::with_capacity(3);
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
        let market: Option<Market> = None;

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
