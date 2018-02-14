use super::track::SimplifiedTrack;
///https://developer.spotify.com/web-api/object-model/#recommendations-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationsSeed>,
    pub tracks: Vec<SimplifiedTrack>,
}
///https://developer.spotify.com/web-api/object-model/#recommendations-seed-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecommendationsSeed {
    pub afterFilteringSize: u32,
    pub afterRelinkingSize: u32,
    pub href: String,
    pub id: String,
    pub initialPoolSize: u32,
    #[serde(rename = "type")]
    pub _type: RecommendationsSeedType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationsSeedType {
    Artist,
    Album,
    Genre,
}
