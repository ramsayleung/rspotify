//! All object related to category
use super::image::Image;
use super::page::Page;
use serde::{Deserialize, Serialize};
/// Category object
///
/// [Reference](https://developer.spotify.com/web-api/get-list-categories/#categoryobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}

/// Categories wrapped by page object
///
/// [Reference](https://developer.spotify.com/web-api/get-list-categories/)
// TODO: Reduce this wrapper object to `Page<Category>`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PageCategory {
    pub categories: Page<Category>,
}
