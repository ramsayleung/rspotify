//! Offset object
///[offset object](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Offset {
    pub position: Option<u32>,
    pub uri: Option<String>,
}

pub fn for_position(position: u32) -> Option<Offset> {
    Some(Offset {
        position: Some(position),
        uri: None,
    })
}

pub fn for_uri(uri: String) -> Option<Offset> {
    Some(Offset {
        position: None,
        uri: Some(uri),
    })
}
