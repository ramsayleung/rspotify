use rspotify::client::SpotifyBuilder;
use rspotify::oauth2::ClientCredentialsBuilder;

#[tokio::main]
async fn main() {
    // Set client_id and client_secret in .env file or
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    let creds = ClientCredentialsBuilder::from_env().build().unwrap();

    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    let spotify = SpotifyBuilder::default()
        .credentials(creds)
        .build()
        .unwrap();
    let birdy_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let albums = spotify.album(birdy_uri).await;

    println!("Response: {:#?}", albums);
}
