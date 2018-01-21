use serde_json;
use super::album_item::Item;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Albums {
    pub href: String,
    pub items: Vec<Item>,
    pub limit: u16,
    pub next: String,
    pub offset: i32,
    pub previous: Option<String>,
    pub total: u32,
}
