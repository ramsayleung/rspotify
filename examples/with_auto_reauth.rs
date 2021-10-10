//! Automatically re-authentication means you only need to authenticate the
//! usual way at most once to get token. Then everytime you send a request to
//! Spotify server, it will check whether the token is expired and automatically
//! re-authenticate by refresh_token if set `Token.token_refreshing` to true.

use chrono::offset::Utc;
use chrono::Duration;
use rspotify::{
    model::AlbumId, model::ArtistId, prelude::*, scopes, AuthCodeSpotify, ClientCredsSpotify,
    Config, Credentials, OAuth,
};

// Sample request that will follow some artists, print the user's
// followed artists, and then unfollow the artists.
async fn auth_code_do_things(spotify: &AuthCodeSpotify) {
    let artists = [
        &ArtistId::from_id("3RGLhK1IP9jnYFH4BRFJBS").unwrap(), // The Clash
        &ArtistId::from_id("0yNLKJebCb8Aueb54LYya3").unwrap(), // New Order
        &ArtistId::from_id("2jzc5TC5TVFLXQlBNiIUzE").unwrap(), // a-ha
    ];
    spotify
        .user_follow_artists(artists)
        .await
        .expect("couldn't follow artists");
    println!("Followed {} artists successfully.", artists.len());

    // Printing the followed artists
    let followed = spotify
        .current_user_followed_artists(None, None)
        .await
        .expect("couldn't get user followed artists");
    println!(
        "User currently follows at least {} artists.",
        followed.items.len()
    );

    spotify
        .user_unfollow_artists(artists)
        .await
        .expect("couldn't unfollow artists");
    println!("Unfollowed {} artists successfully.", artists.len());
}

async fn client_creds_do_things(spotify: &ClientCredsSpotify) {
    // Running the requests
    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(&birdy_uri).await;
    println!("Get ablums: {}", albums.unwrap().id);
}

async fn expire_token<S: BaseClient>(spotify: &S) {
    let token_mutex = spotify.get_token().await;
    let mut token = token_mutex.lock().await.unwrap();
    assert!(token.is_some());
    token.as_mut().map(|x| {
        // In a regular case, the token would expire with time. Here we just do
        // it manually.
        let now = Utc::now().checked_sub_signed(Duration::seconds(10));
        x.expires_at = now;
        // We also use a garbage access token to make sure it's actually
        // refreshed.
        x.access_token = "garbage".to_owned();
    });
}

async fn with_auth(creds: Credentials, oauth: OAuth, config: Config) {
    // In the first session of the application we authenticate and obtain the
    // refresh token.
    println!(">>> Session one, obtaining refresh token and running some requests:");
    let mut spotify = AuthCodeSpotify::with_config(creds.clone(), oauth, config.clone());
    let url = spotify.get_authorize_url(false).unwrap();
    spotify
        .prompt_for_token(&url)
        .await
        .expect("couldn't authenticate successfully");

    // We can now perform requests
    auth_code_do_things(&spotify).await;

    // Manually expiring the token.
    expire_token(&spotify).await;

    // Without automatically refreshing tokens, this would cause an
    // authentication error when making a request, because the auth token is
    // invalid. However, since it will be refreshed automatically, this will
    // work.
    println!(">>> Session two, the token should expire, then re-auth automatically");
    auth_code_do_things(&spotify).await;
}

async fn with_client_credentials(creds: Credentials, config: Config) {
    // Same with client-credential based spotify client
    println!(">>> New Session one from ClientCredsSpotify, obtaining token and doing things");
    let mut spotify = ClientCredsSpotify::with_config(creds, config);
    spotify.request_token().await.unwrap();

    // We can now perform requests
    client_creds_do_things(&spotify).await;

    // Manually expiring the token.
    expire_token(&spotify).await;

    // Same as before, this should work just fine.
    println!(">>> New Session two from ClientCredsSpotify, expiring the token and then re-auth automatically");
    client_creds_do_things(&spotify).await;
}

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Enabling automatic token refreshing in the config
    let mut config = Config::default();
    config.token_refreshing = true;

    // The default credentials from the `.env` file will be used by default.
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-follow-read user-follow-modify")).unwrap();

    with_auth(creds.clone(), oauth, config.clone()).await;
    with_client_credentials(creds, config).await;
}
