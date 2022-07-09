//! Offset object

/// Offset object
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Offset {
    Position(u32),
    Uri(String),
}
