///https://developer.spotify.com/web-api/object-model/#audio-features-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioFeatures {
    pub acousticness: f32,
    pub analysis_url: String,
    pub danceability: f32,
    pub duration_ms: u32,
    pub energy: f32,
    pub id: String,
    pub instrumentalness: f32,
    pub key: i32,
    pub liveness: f32,
    pub loudness: f32,
    pub mode: i32,
    pub speechiness: f32,
    pub tempo: f32,
    pub time_signature: i32,
    pub track_href: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
    pub valence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioFeaturesList {
    pub audio_features: Vec<AudioFeatures>,
}
