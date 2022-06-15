//! General internal utilities used across this crate.

use std::collections::HashMap;

use serde::Serialize;

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

pub(crate) struct JsonBuilder(serde_json::Map<String, serde_json::Value>);

impl JsonBuilder {
    pub(crate) fn new() -> Self {
        Self(serde_json::Map::new())
    }

    pub(crate) fn required(mut self, name: &str, value: impl Serialize) -> Self {
        self.0
            .insert(name.to_owned(), serde_json::to_value(value).unwrap());
        self
    }

    pub(crate) fn optional(self, name: &str, value: Option<impl Serialize>) -> Self {
        if let Some(value) = value {
            self.required(name, value)
        } else {
            self
        }
    }

    pub(crate) fn build(self) -> serde_json::Value {
        serde_json::Value::Object(self.0)
    }
}
