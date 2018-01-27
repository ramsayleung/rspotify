extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;
use rspotify::spotify::spotify_enum::ALBUM_TYPE;

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
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let mut birdy_uri = String::from("spotify:artist:2WX2uTcsvV5OnS0inACecP");
    let albums = spotify.artist_albums(&mut birdy_uri, Some(ALBUM_TYPE::Album), None, None, None);
    println!("{:?}", albums);
}
