///https://developer.spotify.com/web-api/object-model/#paging-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub href: String,
    pub items: Vec<T>,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
}
///https://developer.spotify.com/web-api/object-model/#cursor-based-paging-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CursorBasedPage<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Cursor,
    pub total: u32,
}
///https://developer.spotify.com/web-api/object-model/#cursor-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cursor {
    pub after: Option<String>,
}
