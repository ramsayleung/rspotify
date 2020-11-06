use rspotify::model::*;
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
fn test_playlist_item() {
    let json_str = r#"
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
            "artists": [
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
            ],
            "external_urls": {
                "spotify": "https://open.spotify.com/album/2pANdqPvxInB0YvcDiw4ko"
            },
            "href": "https://api.spotify.com/v1/albums/2pANdqPvxInB0YvcDiw4ko",
            "id": "2pANdqPvxInB0YvcDiw4ko",
            "images": [
                {
                "height": 640,
                "url": "https://i.scdn.co/image/ab67616d0000b273ce6d0eef0c1ce77e5f95bbbc",
                "width": 640
                },
                {
                "height": 300,
                "url": "https://i.scdn.co/image/ab67616d00001e02ce6d0eef0c1ce77e5f95bbbc",
                "width": 300
                },
                {
                "height": 64,
                "url": "https://i.scdn.co/image/ab67616d00004851ce6d0eef0c1ce77e5f95bbbc",
                "width": 64
                }
            ],
            "name": "Progressive Psy Trance Picks Vol.8",
            "release_date": "2012-04-02",
            "release_date_precision": "day",
            "total_tracks": 20,
            "type": "album",
            "uri": "spotify:album:2pANdqPvxInB0YvcDiw4ko"
            },
            "artists": [
            {
                "external_urls": {
                "spotify": "https://open.spotify.com/artist/6eSdhw46riw2OUHgMwR8B5"
                },
                "href": "https://api.spotify.com/v1/artists/6eSdhw46riw2OUHgMwR8B5",
                "id": "6eSdhw46riw2OUHgMwR8B5",
                "name": "Odiseo",
                "type": "artist",
                "uri": "spotify:artist:6eSdhw46riw2OUHgMwR8B5"
            }
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
            "is_playable": true,
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
        "#;
    let playlist_item: PlaylistItem = serde_json::from_str(&json_str).unwrap();
    assert_eq!(
        playlist_item.added_by.unwrap().id,
        "jmperezperez".to_string()
    );
}
#[test]
fn test_full_playlist() {
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
            "images": [
                {
                    "height": null,
                    "url": "https://i.scdn.co/image/ab67706c0000bebb8d0ce13d55f634e290f744ba",
                    "width": null
                }
            ],
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
                                "artists": [
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
                                ],
                                "available_markets": [
                                    "US"
                                ],
                                "external_urls": {
                                    "spotify": "https://open.spotify.com/album/2pANdqPvxInB0YvcDiw4ko"
                                },
                                "href": "https://api.spotify.com/v1/albums/2pANdqPvxInB0YvcDiw4ko",
                                "id": "2pANdqPvxInB0YvcDiw4ko",
                                "images": [
                                    {
                                        "height": 640,
                                        "url": "https://i.scdn.co/image/ab67616d0000b273ce6d0eef0c1ce77e5f95bbbc",
                                        "width": 640
                                    },
                                    {
                                        "height": 300,
                                        "url": "https://i.scdn.co/image/ab67616d00001e02ce6d0eef0c1ce77e5f95bbbc",
                                        "width": 300
                                    },
                                    {
                                        "height": 64,
                                        "url": "https://i.scdn.co/image/ab67616d00004851ce6d0eef0c1ce77e5f95bbbc",
                                        "width": 64
                                    }
                                ],
                                "name": "Progressive Psy Trance Picks Vol.8",
                                "release_date": "2012-04-02",
                                "release_date_precision": "day",
                                "total_tracks": 20,
                                "type": "album",
                                "uri": "spotify:album:2pANdqPvxInB0YvcDiw4ko"
                            },
                            "artists": [
                                {
                                    "external_urls": {
                                        "spotify": "https://open.spotify.com/artist/6eSdhw46riw2OUHgMwR8B5"
                                    },
                                    "href": "https://api.spotify.com/v1/artists/6eSdhw46riw2OUHgMwR8B5",
                                    "id": "6eSdhw46riw2OUHgMwR8B5",
                                    "name": "Odiseo",
                                    "type": "artist",
                                    "uri": "spotify:artist:6eSdhw46riw2OUHgMwR8B5"
                                }
                            ],
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
        "#;
    let full_playlist: FullPlaylist = serde_json::from_str(&json_str).unwrap();
    assert_eq!(
        full_playlist.uri,
        "spotify:playlist:3cEYpjA9oz9GiPac4AsH4n".to_string()
    );
    assert_eq!(full_playlist.followers.total, 109);
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
