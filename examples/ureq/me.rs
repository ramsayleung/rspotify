use rspotify::{prelude::*, scopes, CodeAuthSpotify, Credentials, OAuth};

fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

    let mut spotify = CodeAuthSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).unwrap();

    let user = spotify.me();
    println!("Request: {:?}", user);
}
