//! Offset object

use std::borrow::{ToOwned, Borrow};

use crate::{IdBuf, Id};

/// Offset object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Offset<T> {
    Position(u32),
    Uri(T),
}

// impl<B: Id + ToOwned> Offset<B::Owned> {
//     pub fn for_position(position: u32) -> Offset<B::Owned> {
//         Offset::Position(position)
//     }

//     pub fn for_uri(uri: &B) -> Offset<B::Owned> {
//         Offset::Uri(uri.to_owned())
//     }
// }

impl<O> Offset<O> {
    pub fn for_position(position: u32) -> Offset<O> {
        Offset::Position(position)
    }

    pub fn for_uri<B>(uri: &B) -> Offset<O>
        where
        O: Borrow<B>,
        B: ToOwned
    {
        Offset::Uri(uri.to_owned())
    }
}
