extern crate rspotify;

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::SpotifyClientCredentials;
use rspotify::util::get_token;

fn main() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();

    match get_token(&mut oauth) {
        // Some auth stuff
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // instantiate the Spotify client
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            // Example uri to a song(Hydrogen by Moon)
            let hydrogen_uri = String::from("spotify:track:38rMZCtAPuRgOuV3pyFDmF");
            let res = spotify.add_item_to_queue(hydrogen_uri, None);
            // If stressful, should print "add to queue results:Ok(())"
            println!("add to queue results:{:?}", res);
        }
        None => println!("auth failed"),
    }
}
