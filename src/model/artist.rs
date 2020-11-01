//! All objects related to artist defined by Spotify API
use serde::{Deserialize, Serialize};

use super::image::Image;
use super::page::CursorBasedPage;
use crate::model::{Type, Followers};
use std::collections::HashMap;
/// [artist object simplified](https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified)
/// Simplified Artist Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedArtist {
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: Option<String>,
}

/// [artist object full](https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-full)
/// Full Artist Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullArtist {
    pub external_urls: HashMap<String, String>,
    pub followers: Followers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Full artist vector
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullArtists {
    pub artists: Vec<FullArtist>,
}

/// Full Artists vector wrapped by cursor-based-page object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CursorPageFullArtists {
    pub artists: CursorBasedPage<FullArtist>,
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_full_artist(){
        let json_str = r#"
        {
            "external_urls": {
                "spotify": "https://open.spotify.com/artist/0OdUWJ0sBjDrqHygGUXeCF"
            },
            "followers": {
                "href": null,
                "total": 833247
            },
            "genres": [
                "indie folk"
            ],
            "href": "https://api.spotify.com/v1/artists/0OdUWJ0sBjDrqHygGUXeCF",
            "id": "0OdUWJ0sBjDrqHygGUXeCF",
            "images": [
                {
                    "height": 640,
                    "url": "https://i.scdn.co/image/0f9a5013134de288af7d49a962417f4200539b47",
                    "width": 640
                }
            ],
            "name": "Band of Horses",
            "popularity": 65,
            "type": "artist",
            "uri": "spotify:artist:0OdUWJ0sBjDrqHygGUXeCF"
        }
        "#;
        let full_artist: FullArtist = serde_json::from_str(&json_str).unwrap();
        assert_eq!(full_artist.name, "Band of Horses");
        assert_eq!(full_artist.followers.total, 833247);
    }
}