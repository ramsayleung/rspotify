//! All objects related to artist defined by Spotify API

///[audio feature object](https://developer.spotify.com/web-api/object-model/#audio-features-object)
/// Audio Feature object
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
    pub mode: f32,
    pub speechiness: f32,
    pub tempo: f32,
    pub time_signature: i32,
    pub track_href: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
    pub valence: f32,
}

/// Audio Feature Vector
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioFeaturesPayload {
    pub audio_features: Vec<AudioFeatures>,
}

/// Audio Analysis Object
///[audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysis {
    pub bars: Vec<AudioAnalysisMeasure>,
    pub beats: Vec<AudioAnalysisMeasure>,
    pub meta: AudioAnalysisMeta,
    pub sections: Vec<AudioAnalysisSection>,
    pub segments: Vec<AudioAnalysisSegment>,
    pub tatums: Vec<AudioAnalysisMeasure>,
    pub track: AudioAnalysisTrack,
}

///[audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysisMeasure {
    pub start: f32,
    pub duration: f32,
    pub confidence: Option<f32>,
}

///[audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysisSection {
    pub start: Option<f32>,
    pub duration: Option<f32>,
    pub confidence: Option<f32>,
    pub loudness: Option<f32>,
    pub tempo: Option<f32>,
    pub tempo_confidence: Option<f32>,
    pub key: Option<i32>,
    pub key_confidence: Option<f32>,
    pub mode: Option<f32>,
    pub mode_confidence: Option<f32>,
    pub time_signature: Option<i32>,
    pub time_signature_confidence: Option<f32>,
}

///[audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysisMeta {
    pub analyzer_version: Option<String>,
    pub platform: Option<String>,
    pub detailed_status: Option<String>,
    pub timestamp: Option<String>,
    pub analysis_time: Option<f32>,
    pub input_process: Option<String>,
}

///[audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysisSegment {
    pub start: Option<f32>,
    pub duration: Option<f32>,
    pub confidence: Option<f32>,
    pub loudness_start: Option<f32>,
    pub loudness_max_time: Option<f32>,
    pub loudness_max: Option<f32>,
    pub loudness_end: Option<f32>,
    pub pitches: Option<Vec<f32>>,
    pub timbre: Option<Vec<f32>>,
}

///[audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysisTrack {
    pub num_samples: Option<String>,
    pub duration: Option<f32>,
    pub sample_md5: Option<String>,
    pub offset_seconds: Option<u32>,
    pub window_seconds: Option<u32>,
    pub analysis_sample_rate: Option<f32>,
    pub analysis_channels: Option<u32>,
    pub end_of_fade_in: Option<f32>,
    pub start_of_fade_out: Option<f32>,
    pub loudness: Option<f32>,
    pub tempo: Option<f32>,
    pub tempo_confidence: Option<f32>,
    pub time_signature: Option<i32>,
    pub time_signature_confidence: Option<f32>,
    pub key: Option<u32>,
    pub key_confidence: Option<f32>,
    pub mode: Option<f32>,
    pub mode_confidence: Option<f32>,
    pub codestring: Option<String>,
    pub code_version: Option<f32>,
    pub echoprintstring: Option<String>,
    pub echoprint_version: Option<f32>,
    pub synchstring: Option<String>,
    pub synch_version: Option<f32>,
    pub rhythmstring: Option<String>,
    pub rhythm_version: Option<f32>,
}
