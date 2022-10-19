use rspotify::search::{
    AlbumSearchFilter, ArtistSearchFilter, SearchQuery, TrackSearchFilter, Tracks,
};

#[tokio::main]
async fn main() {
    let query: String = SearchQuery::<Tracks>::default()
        .any("Exemple any")
        .artist("Lisa")
        .track("Demon slayer")
        .album("Another Album")
        .into();

    println!("{}", query);
}
