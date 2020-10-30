//! All kinds of tracks object
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::album::SimplifiedAlbum;
use super::artist::SimplifiedArtist;
use super::Restriction;
use crate::model::Type;

/// [Track object full](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullTrack {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_local: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<TrackLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restriction>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// [Link to track link] https://developer.spotify.com/documentation/web-api/reference/object-model/#track-link
/// Track Link
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackLink {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullTracks {
    pub tracks: Vec<FullTrack>,
}
/// [track object simplified](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    #[serde(default)]
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_local: bool,
    // These three fields are only present when track relinking is applied.
    //-------------------//
    pub is_playable: Option<bool>,
    pub linked_from: Option<TrackLink>,
    pub restrictions: Option<Restriction>,
    //-------------------//
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// [Saved track object](https://developer.spotify.com/documentation/web-api/reference/object-model/#saved-track-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedTrack {
    pub added_at: DateTime<Utc>,
    pub track: FullTrack,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simplified_track() {
        let json_str = r#"
{
    "artists": [ {
      "external_urls": {
        "spotify": "https://open.spotify.com/artist/08td7MxkoHQkXnWAYD8d6Q"
      },
      "href": "https://api.spotify.com/v1/artists/08td7MxkoHQkXnWAYD8d6Q",
      "id": "08td7MxkoHQkXnWAYD8d6Q",
      "name": "Tania Bowra",
      "type": "artist",
      "uri": "spotify:artist:08td7MxkoHQkXnWAYD8d6Q"
    } ],
    "available_markets": [ "AD", "AR", "AT", "AU", "BE", "BG", "BO", "BR", "CA", "CH", "CL", "CO", "CR", "CY", "CZ", "DE", "DK", "DO", "EC", "EE", "ES", "FI", "FR", "GB", "GR", "GT", "HK", "HN", "HU", "IE", "IS", "IT", "LI", "LT", "LU", "LV", "MC", "MT", "MX", "MY", "NI", "NL", "NO", "NZ", "PA", "PE", "PH", "PL", "PT", "PY", "RO", "SE", "SG", "SI", "SK", "SV", "TR", "TW", "US", "UY" ],
    "disc_number": 1,
    "duration_ms": 276773,
    "explicit": false,
    "external_urls": {
      "spotify": "https://open.spotify.com/track/2TpxZ7JUBn3uw46aR7qd6V"
    },
    "href": "https://api.spotify.com/v1/tracks/2TpxZ7JUBn3uw46aR7qd6V",
    "id": "2TpxZ7JUBn3uw46aR7qd6V",
    "name": "All I Want",
    "preview_url": "https://p.scdn.co/mp3-preview/6d00206e32194d15df329d4770e4fa1f2ced3f57",
    "track_number": 1,
    "type": "track",
    "uri": "spotify:track:2TpxZ7JUBn3uw46aR7qd6V",
    "is_local": false
  }

"#;
        let track: SimplifiedTrack = serde_json::from_str(&json_str).unwrap();
        assert_eq!(track.duration_ms, 276773);
    }
}
