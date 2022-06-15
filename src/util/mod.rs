//! General internal utilities used across this crate.

use std::collections::HashMap;

pub(crate) fn build_map<'key, 'value, const N: usize>(
    array: [(&'key str, Option<&'value str>); N],
) -> HashMap<&'key str, &'value str> {
    let mut map = HashMap::with_capacity(N);
    for (key, value) in array {
        if let Some(value) = value {
            map.insert(key, value);
        }
    }
    map
}
