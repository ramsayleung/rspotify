///https://developer.spotify.com/web-api/get-list-categories/#categoryobject
use super::image::Image;
use super::page::Page;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}

///https://developer.spotify.com/web-api/get-list-categories/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageCategory {
    pub categories: Page<Category>,
}
