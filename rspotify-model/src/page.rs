//! All kinds of page object
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Custom deserializer to handle `Vec<Option<T>>` and filter out `None` values
/// This is useful for deserializing lists that may contain null values that are not relevants
fn vec_without_nulls<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let v = Vec::<Option<T>>::deserialize(deserializer)?;
    Ok(v.into_iter().flatten().collect())
}

/// Paging object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Page<T: DeserializeOwned> {
    pub href: String,
    #[serde(deserialize_with = "vec_without_nulls")]
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    /// This field could mismatch the actual number of items in `items` field 
    /// because sometimes the API returns `null` items that are not included in the `items` field.
    pub total: u32,
}

/// Cursor-based paging object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CursorBasedPage<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Option<Cursor>,
    /// Absent if it has read all data items. This field doesn't match what
    /// Spotify document says
    pub total: Option<u32>,
}

/// Cursor object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Cursor {
    pub after: Option<String>,
}
