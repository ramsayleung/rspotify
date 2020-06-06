use super::image::Image;
use super::page::Page;
use std::collections::HashMap;

/// Show object(simplified)
/// [Show object simplified](https://developer.spotify.com/documentation/web-api/reference/object-model/#show-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<HashMap<String, String>>,
    pub description: String,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeversalSimplifiedShows {
    pub shows: Vec<SimplifiedShow>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Show {
    pub added_at: String,
    pub show: SimplifiedShow,
}

/// [Show object(full)](https://developer.spotify.com/documentation/web-api/reference/object-model/#show-object-full)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<HashMap<String, String>>,
    pub description: String,
    pub explicit: bool,
    pub episodes: Page<SimplifiedEpisode>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

/// [Episode object simplified](https://developer.spotify.com/documentation/web-api/reference/object-model/#episode-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    /// Note: This field is deprecated and might be removed in the future. Please use the languages field instead
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub resume_point: Option<ResumePoint>,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

/// [Episode object full](https://developer.spotify.com/documentation/web-api/reference/object-model/#episode-object-full)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    /// Note: This field is deprecated and might be removed in the future. Please use the languages field instead
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub resume_point: Option<ResumePoint>,
    pub show: SimplifiedShow,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeveralEpisodes {
    pub episodes: Vec<FullEpisode>,
}

/// [](https://developer.spotify.com/documentation/web-api/reference/object-model/#resume-point-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: u32,
}
