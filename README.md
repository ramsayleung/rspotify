# Rspotify - a Rust client for The Spotify Web API [WIP]
## Disclaimer
This crate is heavily inspired by [spotipy](https://github.com/plamere/spotipy)-
A spotify api wrapper implemented in Python
## Description
Rspotify is a lightweight wrapper for the [Spotify Web API](https://developer.spotify.com/web-api/) It includes helper functions for
**all Spotify's endpoints**, such as fetching metadata (search and look-up of
albums, artists, tracks, playlists, new releases) and user's information (follow
users, artists and playlists, and saved tracks management).

The wrapper includes helper functions to do the following:

#### Music metadata
- Albums, artists, tracks and playlists
- Audio features and audio analysis for tracks [WIP]
- Albums for a specific artist
- Top tracks for a specific artist
- Artists similar to a specific artist [WIP]

#### Profiles
- User's emails, product type, display name, birthdate, image

#### Search[WIP]
- Albums, artists, tracks, and playlists

#### Playlist manipulation
- Get a user's playlists
- Create playlists
- Change playlist details
- Add tracks to a playlist
- Remove tracks from a playlist
- Replace tracks in a playlist
- Reorder tracks in a playlist
- Upload custom playlist cover image [WIP]

#### Your Music library
- Add, remove, and get tracks that are in the signed in user's Your Music library
- Check if a track is in the signed in user's Your Music library

#### Personalization
- Get a user’s top artists and tracks based on calculated affinity
- Get current user’s recently played tracks

#### Browse
- Get new releases
- Get featured playlists
- Get a list of categories
- Get a category
- Get a category's playlists
- Get recommendations based on seeds [WIP]
- Get available genre seeds [WIP]

#### Follow
- Follow and unfollow users
- Follow and unfollow artists
- Check if the logged in user follows a user or artist
- Follow a playlist
- Unfollow a playlist
- Get followed artists
- Check if users are following a Playlist

#### Player
- Get a user's available devices
- Get information about the user's current playback
- Get the user's currently playing track
- Transfer a user's playback [WIP]
- Start/Resume a user's playback [WIP]
- Pause a user's playback [WIP]
- Skip user's playback to next track [WIP]
- Skip user's playback to previous track [WIP]
- Seek to position in currently playing track [WIP]
- Set repeat mode on user's playback [WIP]
- Set volume for user's playback [WIP]
- Toggle shuffle for user's playback [WIP]

## Installation
Since this crate is still under developing, it isn't pushed to
[crates.io](https://crates.io/), but it is close to finish. Just be patient
## Examples

If you have a use case you are intertested in, you could check the
[examples](./examples), which has all kinds of examples. For example, If you
want to get Spotify catalog information about an artist's top 10 tracks by
country, you could check [artist_top_tracks](./examples/artist_top_tracks). This
is the example code:
``` rust
extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;
use rspotify::spotify::spotify_enum::Country;

fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    let client_credential = SpotifyClientCredentials::default().build();

    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    println!("{:?}:{:?}:{:?}",
             &client_credential.client_id,
             &client_credential.client_secret,
             &client_credential.get_access_token());
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let mut birdy_uri = String::from("spotify:artist:2WX2uTcsvV5OnS0inACecP");
    let tracks = spotify.artist_top_tracks(&mut birdy_uri, Country::UnitedStates);
    println!("{:?}", tracks.unwrap());
}

```

