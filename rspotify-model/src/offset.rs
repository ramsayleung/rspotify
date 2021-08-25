//! Offset object

use std::borrow::ToOwned;

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

    pub fn for_uri<Brw: ToOwned>(uri: &Brw) -> Offset<Brw::Owned> {
        Offset::Uri(uri.to_owned())
    }
}
