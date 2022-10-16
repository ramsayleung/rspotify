use rspotify::search::SearchQuery;

#[tokio::main]
async fn main() {
    let query: String = SearchQuery::default()
        .any("any")
        .artist("Lisa")
        .track("Demon slayer")
        .album("Another Album")
        .into();

    println!("{}", query);
}
