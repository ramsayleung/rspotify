//! This example showcases how the RSpotify client can be used to perform
//! multithreaded requests as well.

use rspotify::{model::AlbumId, prelude::*, ClientCredsSpotify, Credentials};
use std::{
    sync::{mpsc::channel, Arc},
    thread,
};

fn main() {
    // May require the `env-file` feature enabled if the environment variables
    // aren't configured manually.
    let creds = Credentials::from_env().unwrap();

    let spotify = ClientCredsSpotify::new(creds);
    let ids = [
        AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap(),
        AlbumId::from_uri("spotify:album:5EBb7SSkPgxO9Lmt8NjAPT").unwrap(),
        AlbumId::from_uri("spotify:album:3jtOeny1Xh5fp6aSOHahe2").unwrap(),
    ];
    let mut handles = Vec::with_capacity(ids.len());
    let (wr, rd) = channel();

    spotify.request_token().unwrap();

    // Performing the requests concurrently
    let spotify = Arc::new(spotify);
    for id in ids {
        let spotify = Arc::clone(&spotify);
        let wr = wr.clone();
        let handle = thread::spawn(move || {
            let albums = spotify.album(id, None).unwrap();
            wr.send(albums).unwrap();
        });

        handles.push(handle);
    }
    drop(wr); // Automatically closed channel

    while let Ok(album) = rd.recv() {
        println!("Album: {}", album.name);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
