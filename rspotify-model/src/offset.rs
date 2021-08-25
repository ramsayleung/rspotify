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

impl<T> Offset<T> {
    pub fn for_position(position: u32) -> Offset<T> {
        Offset::Position(position)
    }

    pub fn for_uri<Brw, Own>(uri: &Brw) -> Offset<Brw::Owned>
      where
        Brw: ToOwned,
        Own: Borrow<Brw>
    {
        Offset::Uri(uri.to_owned())
    }
}
