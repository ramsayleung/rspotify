use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, CodeAuthSpotify, Credentials, OAuth,
};

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
    // let mut oauth = OAuth {
    //     redirect_uri: "http://localhost:8888/callback".to_string(),
    //     scope: scopes!("user-read-recently-played"),
    //     ..Default::default(),
    // };
    // ```
    let mut oauth = OAuth::from_env().unwrap();
    oauth.scope = scopes!("user-read-currently-playing");
    println!("{:?}", oauth);

    let mut spotify = CodeAuthSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    // Running the requests
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(&market), Some(&additional_types))
        .await;

    println!("Response: {:?}", artists);
}
