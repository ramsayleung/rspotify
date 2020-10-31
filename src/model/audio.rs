//! All objects related to artist defined by Spotify API
use serde::{Deserialize, Serialize};

/// [audio feature object](https://developer.spotify.com/documentation/web-api/reference/object-model/#audio-features-object)
/// Audio Feature object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioFeaturesPayload {
    pub audio_features: Vec<AudioFeatures>,
}

/// Audio Analysis Object
/// [audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysis {
    pub bars: Vec<TimeInterval>,
    pub beats: Vec<TimeInterval>,
    pub meta: AudioAnalysisMeta,
    pub sections: Vec<AudioAnalysisSection>,
    pub segments: Vec<AudioAnalysisSegment>,
    pub tatums: Vec<TimeInterval>,
    pub track: AudioAnalysisTrack,
}

/// [Time interval](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#time-interval-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeInterval {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
}

/// [Audio section](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#section-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisSection {
    #[serde(flatten)]
    pub time_interval: TimeInterval,
    pub loudness: f32,
    pub tempo: f32,
    pub tempo_confidence: f32,
    pub key: i32,
    pub key_confidence: f32,
    pub mode: f32,
    pub mode_confidence: f32,
    pub time_signature: i32,
    pub time_signature_confidence: f32,
}

/// [audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisMeta {
    pub analyzer_version: String,
    pub platform: String,
    pub detailed_status: String,
    pub status_code: i32,
    pub timestamp: u64,
    pub analysis_time: f32,
    pub input_process: String,
}
///[Audio segment](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#segment-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisSegment {
    #[serde(flatten)]
    pub time_interval: TimeInterval,
    pub loudness_start: f32,
    pub loudness_max_time: f32,
    pub loudness_max: f32,
    pub loudness_end: Option<f32>,
    pub pitches: Vec<f32>,
    pub timbre: Vec<f32>,
}

/// [audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisTrack {
    pub num_samples: u32,
    pub duration: f32,
    pub sample_md5: String,
    pub offset_seconds: u32,
    pub window_seconds: u32,
    pub analysis_sample_rate: i32,
    pub analysis_channels: u32,
    pub end_of_fade_in: f32,
    pub start_of_fade_out: f32,
    pub loudness: f32,
    pub tempo: f32,
    pub tempo_confidence: f32,
    pub time_signature: i32,
    pub time_signature_confidence: f32,
    pub key: u32,
    pub key_confidence: f32,
    pub mode: f32,
    pub mode_confidence: f32,
    pub codestring: String,
    pub code_version: f32,
    pub echoprintstring: String,
    pub echoprint_version: f32,
    pub synchstring: String,
    pub synch_version: f32,
    pub rhythmstring: String,
    pub rhythm_version: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_audio_analysis_section() {
        let json_str = r#"
        {
            "start": 237.02356,
            "duration": 18.32542,
            "confidence": 1,
            "loudness": -20.074,
            "tempo": 98.253,
            "tempo_confidence": 0.767,
            "key": 5,
            "key_confidence": 0.327,
            "mode": 1,
            "mode_confidence": 0.566,
            "time_signature": 4,
            "time_signature_confidence": 1
        }
        "#;
        let session: AudioAnalysisSection = serde_json::from_str(&json_str).unwrap();
        assert_eq!(session.time_interval.duration, 18.32542);
    }
    #[test]
    fn test_audio_analysis_segments() {
        let json_str = r#"
         {
            "start": 252.15601,
            "duration": 3.19297,
            "confidence": 0.522,
            "loudness_start": -23.356,
            "loudness_max_time": 0.06971,
            "loudness_max": -18.121,
            "loudness_end": -60,
            "pitches": [
                0.709,
                0.092,
                0.196,
                0.084,
                0.352,
                0.134,
                0.161,
                1,
                0.17,
                0.161,
                0.211,
                0.15
            ],
            "timbre": [
                23.312,
                -7.374,
                -45.719,
                294.874,
                51.869,
                -79.384,
                -89.048,
                143.322,
                -4.676,
                -51.303,
                -33.274,
                -19.037
            ]
            }
            "#;
        let segment: AudioAnalysisSegment = serde_json::from_str(&json_str).unwrap();
        assert_eq!(segment.time_interval.start, 252.15601);
    }
}
