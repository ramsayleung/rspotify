use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // May require the `env-file` feature enabled if the environment variables
    // aren't configured manually.
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).unwrap();

    let devices = spotify.device();

    println!("Request: {devices:?}");
}
