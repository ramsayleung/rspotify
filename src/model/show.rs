use super::image::Image;
use super::page::Page;
use crate::model::{duration_ms, CopyrightType, DatePrecision};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Copyright object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-copyrightobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Copyright {
    pub text: String,
    #[serde(rename = "type")]
    pub _type: CopyrightType,
}

/// Simplified show object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedshowobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
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

/// SimplifiedShows wrapped by `Vec`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-multiple-shows)
#[derive(Deserialize)]
pub(in crate) struct SeversalSimplifiedShows {
    pub shows: Vec<SimplifiedShow>,
}

/// Saved show object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-savedshowobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Show {
    pub added_at: String,
    pub show: SimplifiedShow,
}

/// Full show object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-showobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
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

/// Simplified episode object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedepisodeobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    #[deprecated(
        note = "This `language` field is deprecated and might be removed in the future by Spotify. Please use the languages field instead"
    )]
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

/// Full episode object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-episodeobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    /// Note: This field is deprecated and might be removed in the future.
    /// Please use the languages field instead.
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    pub show: SimplifiedShow,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeveralEpisodes {
    pub episodes: Vec<FullEpisode>,
}

/// Resume point object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-resumepointobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResumePoint {
    pub fully_played: bool,
    #[serde(with = "duration_ms", rename = "resume_position_ms")]
    pub resume_position: Duration,
}
