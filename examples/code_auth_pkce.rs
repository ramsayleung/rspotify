use rspotify::{prelude::*, scopes, CodeAuthPKCESpotify, Credentials, OAuth};

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
    // ```
    // let creds = Credentials {
    //     id: "this-is-my-client-id".to_string(),
    //     secret: "this-is-my-client-secret".to_string()
    // };
    // ```
    let creds = Credentials::from_env().unwrap();

    // Or set the redirect_uri explictly:
    //
    // ```
    // let oauth = OAuth {
    //     redirect_uri: "http://localhost:8888/callback".to_string(),
    //     scope: scopes!("user-read-recently-played"),
    //     ..Default::default(),
    // };
    // ```
    let oauth = OAuth::from_env().unwrap();
    oauth.scope = scopes!("user-read-recently-played");

    let mut spotify = CodeAuthPKCESpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url().unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    // Running the requests
    let history = spotify.current_playback(None, None).await;

    println!("Response: {:?}", history);
}
