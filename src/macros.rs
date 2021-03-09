/// Create a **Hashset** from a list of &str(which will be converted to
/// String internally), be used to create scope for
/// [`Token`](crate::oauth2::Token).
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

/// Reduce boilerplate when inserting new elements in a JSON object.
#[doc(hidden)]
#[macro_export]
macro_rules! json_insert {
    ($json:expr, $p1:expr, $p2:expr) => {
        $json
            .as_object_mut()
            .unwrap()
            .insert($p1.to_string(), json!($p2))
    };
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
