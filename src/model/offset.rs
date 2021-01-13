//! Offset object
use crate::model::{idtypes, Id, IdBuf, PlayableIdType};
use std::time::Duration;

/// Offset object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Offset<T> {
    Position(Duration),
    Uri(IdBuf<T>),
}

impl<T> Offset<T> {
    pub fn for_position(position: u64) -> Offset<idtypes::Track> {
        Offset::Position(Duration::from_millis(position))
    }

    pub fn for_uri(uri: &Id<T>) -> Offset<T>
    where
        T: PlayableIdType,
    {
        Offset::Uri(uri.to_owned())
    }
}
