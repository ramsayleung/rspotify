//! All objects related to recommendation
use super::track::SimplifiedTrack;
use serde::{Deserialize, Serialize};

/// [Recommendations object](https://developer.spotify.com/documentation/web-api/reference/object-model/#recommendations-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationsSeed>,
    pub tracks: Vec<SimplifiedTrack>,
}
/// [Recommendations seed object](https://developer.spotify.com/documentation/web-api/reference/object-model/#recommendations-seed-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecommendationsSeed {
    #[serde(rename = "afterFilteringSize")]
    pub after_filtering_size: u32,
    #[serde(rename = "afterRelinkingSize")]
    pub after_relinking_size: u32,
    pub href: Option<String>,
    pub id: String,
    #[serde(rename = "initialPoolSize")]
    pub initial_pool_size: u32,
    #[serde(rename = "type")]
    pub _type: RecommendationsSeedType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecommendationsSeedType {
    #[serde(rename = "ARTIST")]
    Artist,
    #[serde(rename = "TRACK")]
    Track,
    #[serde(rename = "GENRE")]
    Genre,
}
