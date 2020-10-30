//! All objects related to album defined by Spotify API
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::artist::SimplifiedArtist;
use super::image::Image;
use super::page::Page;
use super::track::SimplifiedTrack;
use super::Restriction;
use crate::model::{AlbumType, Copyright, DatePrecision, Type};

/// [link to album object simplified](https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-simplified)
/// Simplified Album Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedAlbum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_group: Option<String>,
    pub album_type: Option<String>,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restriction>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: Option<String>,
}

/// [link to album object full](https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-full)
/// Full Album Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullAlbum {
    pub artists: Vec<SimplifiedArtist>,
    pub album_type: AlbumType,
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub tracks: Page<SimplifiedTrack>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Full Albums
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullAlbums {
    pub albums: Vec<FullAlbum>,
}

/// [link to get list new releases](https://developer.spotify.com/web-api/get-list-new-releases/)
/// Simplified Albums wrapped by Page object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PageSimpliedAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

/// [link to save album object](https://developer.spotify.com/documentation/web-api/reference/object-model/#save-album-object)
/// Saved Album object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SavedAlbum {
    pub added_at: DateTime<Utc>,
    pub album: FullAlbum,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::CopyrightType;
    #[test]
    fn test_full_album() {
        let json_str = r#"
{
  "album_type" : "album",
  "artists" : [ {
    "external_urls" : {
      "spotify" : "https://open.spotify.com/artist/2BTZIqw0ntH9MvilQ3ewNY"
    },
    "href" : "https://api.spotify.com/v1/artists/2BTZIqw0ntH9MvilQ3ewNY",
    "id" : "2BTZIqw0ntH9MvilQ3ewNY",
    "name" : "Cyndi Lauper",
    "type" : "artist",
    "uri" : "spotify:artist:2BTZIqw0ntH9MvilQ3ewNY"
  } ],
  "available_markets" : [ "AD", "AR", "AT", "AU", "BE", "BG", "BO", "BR", "CA", "CH", "CL", "CO", "CR", "CY", "CZ", "DE", "DK", "DO", "EC", "EE", "ES", "FI", "FR", "GB", "GR", "GT", "HK", "HN", "HU", "IE", "IS", "IT", "LI", "LT", "LU", "LV", "MC", "MT", "MX", "MY", "NI", "NL", "NO", "NZ", "PA", "PE", "PH", "PT", "PY", "RO", "SE", "SG", "SI", "SK", "SV", "TW", "UY" ],
  "copyrights" : [ {
    "text" : "(P) 2000 Sony Music Entertainment Inc.",
    "type" : "P"
  } ],
  "external_ids" : {
    "upc" : "5099749994324"
  },
  "external_urls" : {
    "spotify" : "https://open.spotify.com/album/0sNOF9WDwhWunNAHPD3Baj"
  },
  "genres" : [ ],
  "href" : "https://api.spotify.com/v1/albums/0sNOF9WDwhWunNAHPD3Baj",
  "id" : "0sNOF9WDwhWunNAHPD3Baj",
  "images" : [ {
    "height" : 640,
    "url" : "https://i.scdn.co/image/07c323340e03e25a8e5dd5b9a8ec72b69c50089d",
    "width" : 640
  }, {
    "height" : 300,
    "url" : "https://i.scdn.co/image/8b662d81966a0ec40dc10563807696a8479cd48b",
    "width" : 300
  }, {
    "height" : 64,
    "url" : "https://i.scdn.co/image/54b3222c8aaa77890d1ac37b3aaaa1fc9ba630ae",
    "width" : 64
  } ],
  "name" : "She's So Unusual",
  "popularity" : 39,
  "release_date" : "1983",
  "release_date_precision" : "year",
  "tracks" : {
    "href" : "https://api.spotify.com/v1/albums/0sNOF9WDwhWunNAHPD3Baj/tracks?offset=0&limit=50",
    "items" : [ {
      "artists" : [ {
        "external_urls" : {
          "spotify" : "https://open.spotify.com/artist/2BTZIqw0ntH9MvilQ3ewNY"
        },
        "href" : "https://api.spotify.com/v1/artists/2BTZIqw0ntH9MvilQ3ewNY",
        "id" : "2BTZIqw0ntH9MvilQ3ewNY",
        "name" : "Cyndi Lauper",
        "type" : "artist",
        "uri" : "spotify:artist:2BTZIqw0ntH9MvilQ3ewNY"
      } ],
      "available_markets" : [ "AD", "AR", "AT", "AU", "BE", "BG", "BO", "BR", "CA", "CH", "CL", "CO", "CR", "CY", "CZ", "DE", "DK", "DO", "EC", "EE", "ES", "FI", "FR", "GB", "GR", "GT", "HK", "HN", "HU", "IE", "IS", "IT", "LI", "LT", "LU", "LV", "MC", "MT", "MX", "MY", "NI", "NL", "NO", "NZ", "PA", "PE", "PH", "PT", "PY", "RO", "SE", "SG", "SI", "SK", "SV", "TW", "UY" ],
      "disc_number" : 1,
      "duration_ms" : 305560,
      "explicit" : false,
      "external_urls" : {
        "spotify" : "https://open.spotify.com/track/3f9zqUnrnIq0LANhmnaF0V"
      },
      "href" : "https://api.spotify.com/v1/tracks/3f9zqUnrnIq0LANhmnaF0V",
      "id" : "3f9zqUnrnIq0LANhmnaF0V",
      "name" : "Money Changes Everything",
      "preview_url" : "https://p.scdn.co/mp3-preview/01bb2a6c9a89c05a4300aea427241b1719a26b06",
      "track_number" : 1,
      "type" : "track",
      "uri" : "spotify:track:3f9zqUnrnIq0LANhmnaF0V",
      "is_local": true
    }],
    "limit" : 50,
    "next" : null,
    "offset" : 0,
    "previous" : null,
    "total" : 13
  },
  "type" : "album",
  "uri" : "spotify:album:0sNOF9WDwhWunNAHPD3Baj"
}

"#;
        let album: FullAlbum = serde_json::from_str(&json_str).unwrap();
        assert_eq!(album.copyrights[0]._type, CopyrightType::Performance);
    }
}
