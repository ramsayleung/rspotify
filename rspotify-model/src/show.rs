use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::time::Duration;

use crate::{
    custom_serde::duration_ms, CopyrightType, DatePrecision, EpisodeId, Image, Page, ShowId,
};

/// Copyright object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Copyright {
    pub text: String,
    #[serde(rename = "type")]
    pub _type: CopyrightType,
}

/// Simplified show object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: ShowId<'static>,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
}

/// SimplifiedShows wrapped by `Vec`
#[derive(Deserialize)]
pub struct SeversalSimplifiedShows {
    pub shows: Vec<SimplifiedShow>,
}

/// Saved show object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Show {
    pub added_at: String,
    pub show: SimplifiedShow,
}

/// Full show object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub explicit: bool,
    pub episodes: Page<SimplifiedEpisode>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: ShowId<'static>,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
}

/// Simplified episode object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: EpisodeId<'static>,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    #[deprecated(note = "This `language` field is deprecated and might be \
        removed in the future by Spotify. Please use the languages field \
        instead")]
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
}

/// Full episode object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: EpisodeId<'static>,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    #[deprecated(note = "This `language` field is deprecated and might be \
        removed in the future by Spotify. Please use the languages field \
        instead")]
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    pub show: SimplifiedShow,
}

/// Intermediate episodes feature object wrapped by `Vec`
#[derive(Deserialize)]
pub struct EpisodesPayload {
    pub episodes: Vec<FullEpisode>,
}

/// Resume point object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ResumePoint {
    pub fully_played: bool,
    #[serde(with = "duration_ms", rename = "resume_position_ms")]
    pub resume_position: Duration,
}
