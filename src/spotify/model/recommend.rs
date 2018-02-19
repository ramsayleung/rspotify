//! All objects related to recommendation
use super::track::SimplifiedTrack;
///[recommendations object](https://developer.spotify.com/web-api/object-model/#recommendations-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationsSeed>,
    pub tracks: Vec<SimplifiedTrack>,
}
///[recommendations seed object](https://developer.spotify.com/web-api/object-model/#recommendations-seed-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecommendationsSeed {
    #[serde(rename = "afterFilteringSize")]
    pub after_filtering_size: u32,
    #[serde(rename = "afterRelinkingSize")]
    pub after_relinking_size: u32,
    pub href: String,
    pub id: String,
    #[serde(rename = "initialPoolSize")]
    pub initial_pool_size: u32,
    #[serde(rename = "type")]
    pub _type: RecommendationsSeedType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RecommendationsSeedType {
    #[serde(rename = "ARTIST")]
    Artist,
    #[serde(rename = "TRACK")]
    Track,
    #[serde(rename = "GENRE")]
    Genre,
}
