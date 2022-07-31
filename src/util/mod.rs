//! General internal utilities used across this crate.

use std::collections::HashMap;

use serde::Serialize;
use std::marker::PhantomData;

pub fn build_map<'key, 'value, const N: usize>(
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

/// The `Len` parameter is a type-level natural number (encoded as a Peano
/// number using the `Zero` and `Successor<T>` types) holding the number of
/// times `optional` or `required` must be called before the builder is
/// complete. It is used to give a correct value to
/// `serde_json::Map::with_capacity`, and it will be generally figured out by
/// type inference so you don't have to specify it yourself.
pub struct JsonBuilder<Len> {
    map: serde_json::Map<String, serde_json::Value>,
    len: PhantomData<fn() -> Len>,
}

impl<Len: Natural> JsonBuilder<Len> {
    fn from_map(map: serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            map,
            len: PhantomData,
        }
    }

    pub fn new() -> Self {
        Self::from_map(serde_json::Map::with_capacity(Len::VALUE))
    }
}

// This `impl` block only applies to `JsonBuilder`s that have the capability to
// add one more field, and all the methods return a `JsonBuilder` with the
// capability to add one less field than before.
impl<Len: Natural> JsonBuilder<Successor<Len>> {
    pub fn required(mut self, name: &str, value: impl Serialize) -> JsonBuilder<Len> {
        self.map
            .insert(name.to_owned(), serde_json::to_value(value).unwrap());

        JsonBuilder::from_map(self.map)
    }

    pub fn optional(self, name: &str, value: Option<impl Serialize>) -> JsonBuilder<Len> {
        if let Some(value) = value {
            self.required(name, value)
        } else {
            JsonBuilder::from_map(self.map)
        }
    }
}

impl JsonBuilder<Zero> {
    pub fn build(self) -> serde_json::Value {
        serde_json::Value::Object(self.map)
    }
}

/// A type-level Peano integer representing zero.
pub struct Zero;

/// A type-level Peano integer representing one plus an exsting number; for
/// example, `Successor<Zero>` is one and
/// `Successor<Successor<Successor<Zero>>>` is three.
pub struct Successor<T>(T);

/// A trait implemented on `Zero` and `Successor` to allow obtaining the actual
/// value behind the type-level number.
pub trait Natural {
    const VALUE: usize;
}

impl Natural for Zero {
    const VALUE: usize = 0;
}

impl<T: Natural> Natural for Successor<T> {
    const VALUE: usize = T::VALUE + 1;
}
