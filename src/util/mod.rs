//! General internal utilities used across this crate.

use std::collections::HashMap;

pub(crate) fn build_map<'key, 'value, const N: usize>(
    array: [(&'key str, Option<&'value str>); N],
) -> HashMap<&'key str, &'value str> {
    // Use a manual for loop instead of iterators so we can call `with_capacity`
    // and avoid reallocating.
    let mut map = HashMap::with_capacity(N);
    for (key, value) in array {
        if let Some(value) = value {
            map.insert(key, value);
        }
    }
    map
}
