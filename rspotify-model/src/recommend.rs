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
