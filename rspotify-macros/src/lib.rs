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

#[cfg(test)]
mod test {
    use crate::scopes;

    #[test]
    fn test_hashset() {
        let scopes = scopes!("hello", "world", "foo", "bar");
        assert_eq!(scopes.len(), 4);
        assert!(scopes.contains("hello"));
        assert!(scopes.contains("world"));
        assert!(scopes.contains("foo"));
        assert!(scopes.contains("bar"));
    }
}
