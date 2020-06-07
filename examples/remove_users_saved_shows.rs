extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::senum::Country;
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    // export REDIRECT_URI=your-direct-uri

    // Or set client_id, client_secret,redirect_uri explictly
    // let oauth = SpotifyOAuth::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build();

    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let ids = vec![
                "5AvwZVawapvyhJUIx71pdJ".to_owned(),
                "6ups0LMt1G8n81XLlkbsPo".to_owned(),
                "5AvwZVawapvyhJUIx71pdJ".to_owned(),
            ];
            let result = spotify
                .remove_users_saved_shows(ids, Some(Country::France))
                .await;
            match result {
                Ok(_) => println!("success to remove user's saved shows"),
                Err(err) => println!("failed to remove user's saved shows, {:?}", err),
            }
        }
        None => println!("auth failed"),
    };
}
