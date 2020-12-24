use crate::model::Type;

mod private {
    pub trait Sealed {}
}

pub trait IdType: private::Sealed {
    const TYPE: Type;
}

impl IdType for Artist {
    const TYPE: Type = Type::Artist;
}
impl IdType for Album {
    const TYPE: Type = Type::Album;
}
impl IdType for Track {
    const TYPE: Type = Type::Track;
}
impl IdType for Playlist {
    const TYPE: Type = Type::Playlist;
}
impl IdType for User {
    const TYPE: Type = Type::User;
}
impl IdType for Show {
    const TYPE: Type = Type::Show;
}
impl IdType for Episode {
    const TYPE: Type = Type::Episode;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Artist {}
impl private::Sealed for Artist {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Album {}
impl private::Sealed for Album {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Track {}
impl private::Sealed for Track {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Playlist {}
impl private::Sealed for Playlist {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum User {}
impl private::Sealed for User {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Show {}
impl private::Sealed for Show {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Episode {}
impl private::Sealed for Episode {}
