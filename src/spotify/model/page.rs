#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub href: String,
    pub items: Vec<T>,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
}
