//! Offset object

use crate::{Id, IdBuf, PlayableId};

/// Offset object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Offset<T: IdBuf> {
    Position(u32),
    Uri(T),
}

impl<T: PlayableId> Offset<T> {
    pub fn for_position(position: u32) -> Offset<T> {
        Offset::Position(position)
    }

    pub fn for_uri(uri: &T) -> Offset<T> {
        Offset::Uri(uri.to_owned())
    }
}
