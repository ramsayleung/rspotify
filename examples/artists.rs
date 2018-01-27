extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;

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
    let birdy_uri1 = String::from("spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy");
    let birdy_uri2 = String::from("spotify:artist:3dBVyJ7JuOMt4GE9607Qin");
    let artist_uris = vec![birdy_uri1, birdy_uri2];
    let artists = spotify.artists(artist_uris);
    println!("{:?}", artists);
}
