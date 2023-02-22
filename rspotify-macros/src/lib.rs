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
/// Note: the scopes! macro also support to split the word by whitespace
/// so the scope can't contain any whitespace
/// ```
/// use rspotify_macros::scopes;
/// use std::collections::HashSet;
///
/// let macro_with_whitespace = scopes!("playlist-read-private playlist-read-collaborative");
/// let mut manually = HashSet::new();
/// manually.insert("playlist-read-private".to_owned());
/// manually.insert("playlist-read-collaborative".to_owned());
/// assert_eq!(macro_with_whitespace, manually);
/// ```
#[macro_export]
macro_rules! scopes {
    ($($key:expr),*) => {{
        let mut container = ::std::collections::HashSet::new();
        $(
            for scope in $key.split_whitespace(){
            container.insert(scope);
            }
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

    #[test]
    fn test_scopes_with_whitespace() {
        let scopes = scopes!("      hello world foo bar");

        assert_eq!(scopes.len(), 4);
        assert!(scopes.contains("hello"));
        assert!(scopes.contains("world"));
        assert!(scopes.contains("foo"));
        assert!(scopes.contains("bar"));
    }
}
