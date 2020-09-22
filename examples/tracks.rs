use rspotify::client::SpotifyBuilder;
use rspotify::oauth2::CredentialsBuilder;

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Set RSPOTIFY_CLIENT_ID, RSPOTIFY_CLIENT_SECRET and
    // RSPOTIFY_REDIRECT_URI in an .env file or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    //
    // These will then be read with `from_env`.
    //
    // Otherwise, set client_id and client_secret explictly:
    //
    // let creds = CredentialsBuilder::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build()
    //     .unwrap();
    let creds = CredentialsBuilder::from_env().build().unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .build()
        .unwrap();

    // Obtaining the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_client_token().await.unwrap();

    let birdy_uri1 = "spotify:track:3n3Ppam7vgaVa1iaRUc9Lp";
    let birdy_uri2 = "spotify:track:3twNvmDtFQtAd5gMKedhLD";
    let track_uris = vec![birdy_uri1, birdy_uri2];
    let tracks = spotify.tracks(track_uris, None).await;
    println!("Response: {:?}", tracks);
}
