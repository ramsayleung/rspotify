use rspotify::model::*;
use std::time::Duration;
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
    "available_markets": ["US"],
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

#[test]
fn test_public_user() {
    let json_str = r#"
        {
            "display_name": "Ronald Pompa",
            "external_urls": {
                "spotify": "https://open.spotify.com/user/wizzler"
            },
            "followers": {
                "href": null,
                "total": 4050
            },
            "href": "https://api.spotify.com/v1/users/wizzler",
            "id": "wizzler",
            "images": [
                {
                "height": null,
                "url": "https://i.scdn.co/image/ab6775700000ee85b5d374d281b9e510eda15fdf",
                "width": null
                }
            ],
            "type": "user",
            "uri": "spotify:user:wizzler"
        }
        "#;
    let user: PublicUser = serde_json::from_str(&json_str).unwrap();
    assert_eq!(user.id, "wizzler".to_string());
}
#[test]
fn test_private_user() {
    let json_str = r#"
        {
            "country": "US",
            "display_name": "Sergey",
            "email": "vixatew967@top-email.org",
            "explicit_content": {
              "filter_enabled": false,
              "filter_locked": false
            },
            "external_urls": {
              "spotify": "https://open.spotify.com/user/waq5aexykhm6nlv0cnwdieng0"
            },
            "followers": {
              "href": null,
              "total": 0
            },
            "href": "https://api.spotify.com/v1/users/waq5aexykhm6nlv0cnwdieng0",
            "id": "waq5aexykhm6nlv0cnwdieng0",
            "images": [],
            "product": "open",
            "type": "user",
            "uri": "spotify:user:waq5aexykhm6nlv0cnwdieng0"
          } 
        "#;
    let private_user: PrivateUser = serde_json::from_str(&json_str).unwrap();
    assert_eq!(private_user.country.unwrap(), Country::UnitedStates);
}

#[test]
fn test_full_artist() {
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
        "description": "En ny tysk ",
        "duration_ms": 1502795,
        "explicit": false,
        "external_urls": {
            "spotify": "https://open.spotify.com/episode/512ojhOuo1ktJprKbVcKyQ"
        },
        "href": "https://api.spotify.com/v1/episodes/512ojhOuo1ktJprKbVcKyQ",
        "id": "512ojhOuo1ktJprKbVcKyQ",
        "images": [
            {
                "height": 64,
                "url": "https://i.scdn.co/image/e29c75799cad73927fad713011edad574868d8da",
                "width": 64
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
                    "height": 64,
                    "url": "https://i.scdn.co/image/3dc007829bc0663c24089e46743a9f4ae15e65f8",
                    "width": 64
                }
            ],
            "is_externally_hosted": false,
            "languages": [
                "sv"
            ],
            "media_type": "audio",
            "name": "Vetenskapsradion Historia",
            "publisher": "Sveriges Radio",
            "type": "show",
            "uri": "spotify:show:38bS44xjbVVZ3No3ByF1dJ"
        },
        "type": "episode",
        "uri": "spotify:episode:512ojhOuo1ktJprKbVcKyQ"
    }
        "#;
    let full_episode: FullEpisode = serde_json::from_str(&json_str).unwrap();
    assert_eq!(full_episode.release_date_precision, DatePrecision::Day);
    let duration = Duration::from_millis(1502795);
    assert_eq!(full_episode.duration, duration);
}

#[test]
fn test_copyright() {
    let json_str = r#"
	[ {
	    "text" : "(P) 2000 Sony Music Entertainment Inc.",
	    "type" : "P"
	} ]

"#;
    let copyrights: Vec<Copyright> = serde_json::from_str(&json_str).unwrap();
    assert_eq!(copyrights[0]._type, CopyrightType::Performance);
}

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
                0.15
            ],
            "timbre": [
                -19.037
            ]
            }
            "#;
    let segment: AudioAnalysisSegment = serde_json::from_str(&json_str).unwrap();
    assert_eq!(segment.time_interval.start, 252.15601);
}

#[test]
fn test_actions() {
    let json_str = r#"
        {
            "disallows": {
                "resuming": true
            }
        }
        "#;
    let actions: Actions = serde_json::from_str(&json_str).unwrap();
    assert_eq!(actions.disallows[0], DisallowKey::Resuming);
}

#[test]
fn test_recommendations_seed() {
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

#[test]
fn test_full_playlist() {
    let json_str_images = r#"
[
    {
	"height": null,
	"url": "https://i.scdn.co/image/ab67706c0000bebb8d0ce13d55f634e290f744ba",
	"width": null
    }
]
"#;
    let json_str_simplified_artists = r#"
[
    {
	"external_urls": {
	    "spotify": "https://open.spotify.com/artist/5I8r2w4hf7OYp2cunjihxJ"
	},
	"href": "https://api.spotify.com/v1/artists/5I8r2w4hf7OYp2cunjihxJ",
	"id": "5I8r2w4hf7OYp2cunjihxJ",
	"name": "Kularis",
	"type": "artist",
	"uri": "spotify:artist:5I8r2w4hf7OYp2cunjihxJ"
    }
]
"#;
    let json_str = r#"
        {
            "collaborative": false,
            "description": "A playlist for testing pourposes",
            "external_urls": {
                "spotify": "https://open.spotify.com/playlist/3cEYpjA9oz9GiPac4AsH4n"
            },
            "followers": {
                "href": null,
                "total": 109
            },
            "href": "https://api.spotify.com/v1/playlists/3cEYpjA9oz9GiPac4AsH4n",
            "id": "3cEYpjA9oz9GiPac4AsH4n",
            "images": json_str_images,
            "name": "Spotify Web API Testing playlist",
            "owner": {
                "display_name": "JMPerez²",
                "external_urls": {
                    "spotify": "https://open.spotify.com/user/jmperezperez"
                },
                "href": "https://api.spotify.com/v1/users/jmperezperez",
                "id": "jmperezperez",
                "type": "user",
                "uri": "spotify:user:jmperezperez"
            },
            "primary_color": null,
            "public": true,
            "snapshot_id": "MTgsZWFmNmZiNTIzYTg4ODM0OGQzZWQzOGI4NTdkNTJlMjU0OWFkYTUxMA==",
            "tracks": {
                "href": "https://api.spotify.com/v1/playlists/3cEYpjA9oz9GiPac4AsH4n/tracks?offset=0&limit=100",
                "items": [
                    {
                        "added_at": "2015-01-15T12:39:22Z",
                        "added_by": {
                            "external_urls": {
                                "spotify": "https://open.spotify.com/user/jmperezperez"
                            },
                            "href": "https://api.spotify.com/v1/users/jmperezperez",
                            "id": "jmperezperez",
                            "type": "user",
                            "uri": "spotify:user:jmperezperez"
                        },
                        "is_local": false,
                        "primary_color": null,
                        "track": {
                            "album": {
                                "album_type": "album",
                                "artists": json_str_simplified_artists,
                                "available_markets": [
                                    "US"
                                ],
                                "external_urls": {
                                    "spotify": "https://open.spotify.com/album/2pANdqPvxInB0YvcDiw4ko"
                                },
                                "href": "https://api.spotify.com/v1/albums/2pANdqPvxInB0YvcDiw4ko",
                                "id": "2pANdqPvxInB0YvcDiw4ko",
                                "images": json_str_images,
                                "name": "Progressive Psy Trance Picks Vol.8",
                                "release_date": "2012-04-02",
                                "release_date_precision": "day",
                                "total_tracks": 20,
                                "type": "album",
                                "uri": "spotify:album:2pANdqPvxInB0YvcDiw4ko"
                            },
                            "artists": json_str_simplified_artists,
                            "available_markets": [
                                "US"
                            ],
                            "disc_number": 1,
                            "duration_ms": 376000,
                            "episode": false,
                            "explicit": false,
                            "external_ids": {
                                "isrc": "DEKC41200989"
                            },
                            "external_urls": {
                                "spotify": "https://open.spotify.com/track/4rzfv0JLZfVhOhbSQ8o5jZ"
                            },
                            "href": "https://api.spotify.com/v1/tracks/4rzfv0JLZfVhOhbSQ8o5jZ",
                            "id": "4rzfv0JLZfVhOhbSQ8o5jZ",
                            "is_local": false,
                            "name": "Api",
                            "popularity": 2,
                            "preview_url": "https://p.scdn.co/mp3-preview/c440fa9ff320fb74629f8477bff45b1a377897ab?cid=774b29d4f13844c495f206cafdad9c86",
                            "track": true,
                            "track_number": 10,
                            "type": "track",
                            "uri": "spotify:track:4rzfv0JLZfVhOhbSQ8o5jZ"
                        },
                        "video_thumbnail": {
                            "url": null
                        }
                    }
                ],
                "limit": 100,
                "next": null,
                "offset": 0,
                "previous": null,
                "total": 5
            },
            "type": "playlist",
            "uri": "spotify:playlist:3cEYpjA9oz9GiPac4AsH4n"
        }
        "#.replace("json_str_images", json_str_images).replace("json_str_simplified_artists", json_str_simplified_artists);
    let full_playlist: FullPlaylist = serde_json::from_str(&json_str).unwrap();
    assert_eq!(
        full_playlist.uri,
        "spotify:playlist:3cEYpjA9oz9GiPac4AsH4n".to_string()
    );
    assert_eq!(full_playlist.followers.total, 109);
}

#[test]
fn test_audio_features() {
    let json = r#"
    {
        "duration_ms" : 255349,
        "key" : 5,
        "mode" : 0,
        "time_signature" : 4,
        "acousticness" : 0.514,
        "danceability" : 0.735,
        "energy" : 0.578,
        "instrumentalness" : 0.0902,
        "liveness" : 0.159,
        "loudness" : -11.840,
        "speechiness" : 0.0461,
        "valence" : 0.624,
        "tempo" : 98.002,
        "id" : "06AKEBrKUckW0KREUWRnvT",
        "uri" : "spotify:track:06AKEBrKUckW0KREUWRnvT",
        "track_href" : "https://api.spotify.com/v1/tracks/06AKEBrKUckW0KREUWRnvT",
        "analysis_url" : "https://api.spotify.com/v1/audio-analysis/06AKEBrKUckW0KREUWRnvT",
        "type" : "audio_features"
    }
    "#;
    let audio_features: AudioFeatures = serde_json::from_str(json).unwrap();
    let duration = Duration::from_millis(255349);
    assert_eq!(audio_features.duration, duration);
}
