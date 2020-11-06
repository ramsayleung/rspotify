use super::image::Image;
use super::page::Page;
use crate::model::{CopyrightType, DatePrecision};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Copyright object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#copyright-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Copyright {
    pub text: String,
    #[serde(rename = "type")]
    pub _type: CopyrightType,
}

/// Simplified show object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#show-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

/// SimplifiedShows wrapped by `Vec`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/shows/get-several-shows/)
// TODO: Reduce such wrapper object to `Vec<SimplifiedShow>`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeversalSimplifiedShows {
    pub shows: Vec<SimplifiedShow>,
}

/// Saved show object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#saved-show-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Show {
    pub added_at: String,
    pub show: SimplifiedShow,
}

/// Full show object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#show-object-full)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub explicit: bool,
    pub episodes: Page<SimplifiedEpisode>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

/// Simplified episode object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#episode-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    #[deprecated(
        note = "This `language` field is deprecated and might be removed in the future by Spotify. Please use the languages field instead"
    )]
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

/// Full episode object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#episode-object-full)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    /// Note: This field is deprecated and might be removed in the future. Please use the languages field instead
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    pub show: SimplifiedShow,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeveralEpisodes {
    pub episodes: Vec<FullEpisode>,
}

/// Resume point object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#resume-point-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: u32,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simplified_episode() {
        let json_str = r#"
        {
            "audio_preview_url": "https://p.scdn.co/mp3-preview/d8b916e1872de2bb0285d8c7bfe2b4b57011c85c",
            "description": "En unik barockträdgård från 1600-talet gömmer sig på Södermalm i Stockholm och nu gräver arkeologerna ut parken och kvarteret där Bellman lekte som barn.  Nu grävs Carl Michael Bellmans kvarter fram på Södermalm i Stockholm. Under dagens jordyta döljer sig en rik barockträdgård, men också tunga industrier från en tid då Söder var stockholmarnas sommarnöje. Dessutom om hur arkeologer ska kunna bli bättre att hitta de fattigas kulturarv. För vid sidan av slott, borgar och hallar finns torpen och backstugorna som utgör ett fortfarande okänt kulturarv som angår oss alla. Programledare Tobias Svanelid.",
            "duration_ms": 2685023,
            "explicit": false,
            "external_urls": {
                "spotify": "https://open.spotify.com/episode/3brfPv3PaUhspkm1T9ZVl8"
            },
            "href": "https://api.spotify.com/v1/episodes/3brfPv3PaUhspkm1T9ZVl8",
            "id": "3brfPv3PaUhspkm1T9ZVl8",
            "images": [
                {
                "height": 640,
                "url": "https://i.scdn.co/image/65497c8c1bef1b783d2be6a1c73b294d953f9406",
                "width": 640
                }
            ],
            "is_externally_hosted": false,
            "is_playable": true,
            "language": "sv",
            "languages": [
                "sv"
            ],
            "name": "På Bellmans bakgata",
            "release_date": "2020-10-20",
            "release_date_precision": "day",
            "resume_point": {
                "fully_played": false,
                "resume_position_ms": 0
            },
            "type": "episode",
            "uri": "spotify:episode:3brfPv3PaUhspkm1T9ZVl8"
        }
        "#;
        let simplified_episode: SimplifiedEpisode = serde_json::from_str(&json_str).unwrap();
        assert_eq!(
            simplified_episode.release_date_precision,
            DatePrecision::Day
        );
    }

    #[test]
    fn test_full_episode() {
        let json_str = r#"
        {
            "audio_preview_url": "https://p.scdn.co/mp3-preview/566fcc94708f39bcddc09e4ce84a8e5db8f07d4d",
            "description": "En ny tysk bok granskar för ",
            "duration_ms": 1502795,
            "explicit": false,
            "external_urls": {
                "spotify": "https://open.spotify.com/episode/512ojhOuo1ktJprKbVcKyQ"
            },
            "href": "https://api.spotify.com/v1/episodes/512ojhOuo1ktJprKbVcKyQ",
            "id": "512ojhOuo1ktJprKbVcKyQ",
            "images": [
                {
                "height": 640,
                "url": "https://i.scdn.co/image/de4a5f115ac6f6ca4cae4fb7aaf27bacac7a0b8a",
                "width": 640
                }
            ],
            "is_externally_hosted": false,
            "is_playable": true,
            "language": "sv",
            "languages": [
                "sv"
            ],
            "name": "Tredje rikets knarkande granskas",
            "release_date": "2015-10-01",
            "release_date_precision": "day",
            "resume_point": {
                "fully_played": false,
                "resume_position_ms": 0
            },
            "show": {
                "available_markets": [
                "ZA"
                ],
                "copyrights": [],
                "description": "Vi är där historien är. Ansvarig utgivare: Nina Glans",
                "explicit": false,
                "external_urls": {
                "spotify": "https://open.spotify.com/show/38bS44xjbVVZ3No3ByF1dJ"
                },
                "href": "https://api.spotify.com/v1/shows/38bS44xjbVVZ3No3ByF1dJ",
                "id": "38bS44xjbVVZ3No3ByF1dJ",
                "images": [
                {
                    "height": 640,
                    "url": "https://i.scdn.co/image/84bc7407c7a61e805284314bef8b9ed5a7c31426",
                    "width": 640
                }
                ],
                "is_externally_hosted": false,
                "languages": [
                "sv"
                ],
                "media_type": "audio",
                "name": "Vetenskapsradion Historia",
                "publisher": "Sveriges Radio",
                "total_episodes": 500,
                "type": "show",
                "uri": "spotify:show:38bS44xjbVVZ3No3ByF1dJ"
            },
            "type": "episode",
            "uri": "spotify:episode:512ojhOuo1ktJprKbVcKyQ"
        }
        "#;
        let full_episode: FullEpisode = serde_json::from_str(&json_str).unwrap();
        assert_eq!(full_episode.release_date_precision, DatePrecision::Day);
    }
}
