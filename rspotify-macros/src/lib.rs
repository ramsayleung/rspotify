/// Create a [`HashSet`](std::collections::HashSet) from a list of `&str` to
/// easily create scopes for `Token` or `OAuth`.
///
/// Example:
///
/// ```
/// use rspotify_macros::scopes;
/// use std::collections::HashSet;
///
/// let with_macro = scopes!("playlist-read-private", "playlist-read-collaborative");
/// let mut manually = HashSet::new();
/// manually.insert("playlist-read-private".to_owned());
/// manually.insert("playlist-read-collaborative".to_owned());
/// assert_eq!(with_macro, manually);
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
    use crate::{build_json, scopes};
    use serde_json::{json, Map, Value};

    #[test]
    fn test_hashset() {
        let scopes = scopes!("hello", "world", "foo", "bar");
        assert_eq!(scopes.len(), 4);
        assert!(scopes.contains("hello"));
        assert!(scopes.contains("world"));
        assert!(scopes.contains("foo"));
        assert!(scopes.contains("bar"));
    }

    #[test]
    fn test_json_query() {
        // Passed as parameters, for example.
        let id = "Pink Lemonade";
        let artist = Some("The Wombats");
        let market: Option<i32> = None;

        let with_macro = build_json! {
            "id": id,
            optional "artist": artist,
            optional "market": market.map(|x| x.to_string()),
        };

        let mut manually = Map::with_capacity(3);
        manually.insert("id".to_string(), json!(id));
        if let Some(val) = artist.map(|x| json!(x)) {
            manually.insert("artist".to_string(), val);
        }
        if let Some(val) = market.map(|x| x.to_string()).map(|x| json!(x)) {
            manually.insert("market".to_string(), val);
        }

        assert_eq!(with_macro, Value::from(manually));
    }
}
