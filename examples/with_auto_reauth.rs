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
        ArtistId::from_id("3RGLhK1IP9jnYFH4BRFJBS").unwrap(), // The Clash
        ArtistId::from_id("0yNLKJebCb8Aueb54LYya3").unwrap(), // New Order
        ArtistId::from_id("2jzc5TC5TVFLXQlBNiIUzE").unwrap(), // a-ha
    ];
    let num_artists = artists.len();
    spotify
        .user_follow_artists(artists.iter().map(|a| a.as_ref()))
        .await
        .expect("couldn't follow artists");
    println!("Followed {num_artists} artists successfully.");

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
    println!("Unfollowed {num_artists} artists successfully.");
}

async fn client_creds_do_things(spotify: &ClientCredsSpotify) {
    // Running the requests
    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(birdy_uri, None).await;
    println!("Get albums: {}", albums.unwrap().id);
}

async fn expire_token<S: BaseClient>(spotify: &S) {
    let token_mutex = spotify.get_token();
    let mut token = token_mutex.lock().await.unwrap();
    let token = token.as_mut().expect("Token can't be empty as this point");
    // In a regular case, the token would expire with time. Here we just do
    // it manually.
    let now = Utc::now().checked_sub_signed(Duration::try_seconds(10).unwrap());
    token.expires_at = now;
    token.expires_in = Duration::try_seconds(0).unwrap();
    // We also use a garbage access token to make sure it's actually
    // refreshed.
    token.access_token = "garbage".to_owned();
}

async fn with_auth(creds: Credentials, oauth: OAuth, config: Config) {
    // In the first session of the application we authenticate and obtain the
    // refresh token.
    println!(">>> Session one, obtaining refresh token and running some requests:");
    let spotify = AuthCodeSpotify::with_config(creds.clone(), oauth, config.clone());
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
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
    let spotify = ClientCredsSpotify::with_config(creds, config);
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
    let config = Config {
        ..Default::default()
    };

    // May require the `env-file` feature enabled if the environment variables
    // aren't configured manually.
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-follow-read user-follow-modify")).unwrap();

    with_auth(creds.clone(), oauth, config.clone()).await;
    with_client_credentials(creds, config).await;
}
