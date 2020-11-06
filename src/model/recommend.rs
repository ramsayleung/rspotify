//! All objects related to recommendation
use super::track::SimplifiedTrack;
use serde::{Deserialize, Serialize};
use crate::model::RecommendationsSeedType;

/// Recommendations object
/// 
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#recommendations-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationsSeed>,
    pub tracks: Vec<SimplifiedTrack>,
}

/// Recommendations seed object
/// 
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#recommendations-seed-object)
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

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_recommendations_seed(){
        let json_str = r#"
        {
            "initialPoolSize": 500,
            "afterFilteringSize": 380,
            "afterRelinkingSize": 365,
            "href": "https://api.spotify.com/v1/artists/4NHQUGzhtTLFvgF5SZesLK",
            "id": "4NHQUGzhtTLFvgF5SZesLK",
            "type": "artist"
        }        
        "#;
        let seed: RecommendationsSeed = serde_json::from_str(&json_str).unwrap();
        assert_eq!(seed._type, RecommendationsSeedType::Artist);
    }
}