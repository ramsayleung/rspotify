//! All objects related to recommendation

use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

use crate::{RecommendationsSeedType, SimplifiedTrack};

/// Recommendations object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationsSeed>,
    pub tracks: Vec<SimplifiedTrack>,
}

/// Recommendations seed object
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

/// The attributes for recommendations
#[derive(Clone, Copy, Debug, Serialize, PartialEq, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RecommendationsAttribute {
    MinAcousticness(f32),
    MaxAcousticness(f32),
    TargetAcousticness(f32),
    MinDanceability(f32),
    MaxDanceability(f32),
    TargetDanceability(f32),
    MinDurationMs(i32),
    MaxDurationMs(i32),
    TargetDurationMs(i32),
    MinEnergy(f32),
    MaxEnergy(f32),
    TargetEnergy(f32),
    MinInstrumentalness(f32),
    MaxInstrumentalness(f32),
    TargetInstrumentalness(f32),
    MinKey(i32),
    MaxKey(i32),
    TargetKey(i32),
    MinLiveness(f32),
    MaxLiveness(f32),
    TargetLiveness(f32),
    MinLoudness(f32),
    MaxLoudness(f32),
    TargetLoudness(f32),
    MinMode(i32),
    MaxMode(i32),
    TargetMode(i32),
    MinPopularity(i32),
    MaxPopularity(i32),
    TargetPopularity(i32),
    MinSpeechiness(f32),
    MaxSpeechiness(f32),
    TargetSpeechiness(f32),
    MinTempo(f32),
    MaxTempo(f32),
    TargetTempo(f32),
    MinTimeSignature(i32),
    MaxTimeSignature(i32),
    TargetTimeSignature(i32),
    MinValence(f32),
    MaxValence(f32),
    TargetValence(f32),
}

impl RecommendationsAttribute {
    /// Obtains the value of the enum as a String, which may be helpful when
    /// serializing it.
    pub fn value_string(&self) -> String {
        use RecommendationsAttribute::*;

        match self {
            MinAcousticness(x) => x.to_string(),
            MaxAcousticness(x) => x.to_string(),
            TargetAcousticness(x) => x.to_string(),
            MinDanceability(x) => x.to_string(),
            MaxDanceability(x) => x.to_string(),
            TargetDanceability(x) => x.to_string(),
            MinDurationMs(x) => x.to_string(),
            MaxDurationMs(x) => x.to_string(),
            TargetDurationMs(x) => x.to_string(),
            MinEnergy(x) => x.to_string(),
            MaxEnergy(x) => x.to_string(),
            TargetEnergy(x) => x.to_string(),
            MinInstrumentalness(x) => x.to_string(),
            MaxInstrumentalness(x) => x.to_string(),
            TargetInstrumentalness(x) => x.to_string(),
            MinKey(x) => x.to_string(),
            MaxKey(x) => x.to_string(),
            TargetKey(x) => x.to_string(),
            MinLiveness(x) => x.to_string(),
            MaxLiveness(x) => x.to_string(),
            TargetLiveness(x) => x.to_string(),
            MinLoudness(x) => x.to_string(),
            MaxLoudness(x) => x.to_string(),
            TargetLoudness(x) => x.to_string(),
            MinMode(x) => x.to_string(),
            MaxMode(x) => x.to_string(),
            TargetMode(x) => x.to_string(),
            MinPopularity(x) => x.to_string(),
            MaxPopularity(x) => x.to_string(),
            TargetPopularity(x) => x.to_string(),
            MinSpeechiness(x) => x.to_string(),
            MaxSpeechiness(x) => x.to_string(),
            TargetSpeechiness(x) => x.to_string(),
            MinTempo(x) => x.to_string(),
            MaxTempo(x) => x.to_string(),
            TargetTempo(x) => x.to_string(),
            MinTimeSignature(x) => x.to_string(),
            MaxTimeSignature(x) => x.to_string(),
            TargetTimeSignature(x) => x.to_string(),
            MinValence(x) => x.to_string(),
            MaxValence(x) => x.to_string(),
            TargetValence(x) => x.to_string(),
        }
    }
}
