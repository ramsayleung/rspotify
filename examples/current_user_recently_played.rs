use rspotify::{prelude::*, scopes, CodeAuthSpotify, Credentials, OAuth};

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
    //     client_id: "this-is-my-client-id".to_string(),
    //     client_secret: "this-is-my-client-secret".to_string()
    // };
    // ```
    let creds = Credentials::from_env().unwrap();

    // Or set the redirect_uri explictly:
    //
    // let oauth = OAuthBuilder::default()
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build()
    //     .unwrap();
    let mut oauth = OAuth::from_env().unwrap();
    oauth.scope = scopes!("user-read-recently-played");

    let mut spotify = CodeAuthSpotify::new(creds, oauth);

    // Obtaining the access token
    spotify.prompt_for_token().await.unwrap();

    // Running the requests
    let history = spotify.current_user_recently_played(Some(10)).await;

    println!("Response: {:?}", history);
}
