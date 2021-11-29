//! Offset object

/// Offset object
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Offset {
    Position(u32),
    Uri(String),
}

impl Offset {
    pub fn for_position(position: u32) -> Offset {
        Offset::Position(position)
    }

    pub fn for_uri(uri: &str) -> Offset {
        Offset::Uri(uri.to_owned())
    }
}
