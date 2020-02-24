extern crate rspotify;

use futures;
use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use tokio;

#[tokio::main]
async fn main() {
    let mut handlers = vec![];
    for _ in 0..20 {
        let handler = tokio::spawn(async move {
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
            let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
            let artist = spotify.artist(birdy_uri).await;
            println!("{:?}", artist);
            return;
        });
        handlers.push(handler);
    }
    futures::future::join_all(handlers).await;
}
