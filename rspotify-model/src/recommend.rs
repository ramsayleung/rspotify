//! All objects related to recommendation

use serde::{Deserialize, Serialize};

use crate::{Id, RecommendationsSeedType, SimplifiedTrack};

/// Recommendations object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-recommendationsobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recommendations<T: Id> {
    pub seeds: Vec<RecommendationsSeed<T>>,
    pub tracks: Vec<SimplifiedTrack>,
}

/// Recommendations seed object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-recommendationseedobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecommendationsSeed<T: Id> {
    #[serde(rename = "afterFilteringSize")]
    pub after_filtering_size: u32,
    #[serde(rename = "afterRelinkingSize")]
    pub after_relinking_size: u32,
    pub href: Option<String>,
    pub id: T,
    #[serde(rename = "initialPoolSize")]
    pub initial_pool_size: u32,
    #[serde(rename = "type")]
    pub _type: RecommendationsSeedType,
}
