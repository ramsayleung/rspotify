use serde_json;
use serde::de::Deserialize;
use reqwest::header::{Authorization, Bearer, ContentType, Headers};
use reqwest::Client;
use reqwest::Method::{self, Put, Get, Post, Delete};

//  built-in battery
use std::collections::HashMap;
use std::io::Read;
use std::borrow::Cow;


use super::oauth2::SpotifyClientCredentials;
use super::spotify_enum::{ALBUM_TYPE, TYPE};
use super::model::album::{SimplifiedAlbum, FullAlbum, FullAlbums};
use super::model::page::Page;
use super::model::track::{FullTracks, FullTrack, SimplifiedTrack};
use super::model::artist::{FullArtist, FullArtists};
use super::model::user::PublicUser;
use super::model::playlist::SimplifiedPlaylist;
use super::util::convert_map_to_string;
pub struct Spotify {
    pub prefix: String,
    pub access_token: Option<String>,
    pub client_credentials_manager: Option<SpotifyClientCredentials>,
}
impl Spotify {
    pub fn default() -> Spotify {
        Spotify {
            prefix: "https://api.spotify.com/v1/".to_owned(),
            access_token: None,
            client_credentials_manager: None,
        }
    }

    pub fn prefix(mut self, prefix: &str) -> Spotify {
        self.prefix = prefix.to_owned();
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Spotify {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn client_credentials_manager(mut self,
                                      client_credential_manager: SpotifyClientCredentials)
                                      -> Spotify {
        self.client_credentials_manager = Some(client_credential_manager);
        self
    }
    pub fn build(self) -> Spotify {
        if self.access_token.is_none() && self.client_credentials_manager.is_none() {
            panic!("access_token and client_credentials_manager are none!!!");
        }
        self
    }
    fn auth_headers(&self) -> Authorization<Bearer> {
        match self.access_token {
            Some(ref token) => Authorization(Bearer { token: token.to_owned() }),
            None => {
                match self.client_credentials_manager {
                    Some(ref client_credentials_manager) => {
                        let token = client_credentials_manager.get_access_token();
                        Authorization(Bearer { token: token })
                    }
                    None => panic!("client credentials manager is none"),
                }
            }
        }
    }
    fn internal_call(&self,
                     method: Method,
                     url: &str,
                     payload: Option<&HashMap<&str, &str>>,
                     params: &mut HashMap<&str, String>)
                     -> Option<String> {
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["https://api.spotify.com/v1/", &url].concat().into();
        }
        if let Some(data) = payload {
            match serde_json::to_string(&data) {
                Ok(payload_string) => {
                    params.insert("data", payload_string);
                }
                Err(why) => {
                    panic!("couldn't convert payload to string: {} ", why);
                }
            }
        }
        println!("{:?}", &url);
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(self.auth_headers());
        headers.set(ContentType::json());
        let mut response = client
            .request(method, &url.into_owned())
            .headers(headers)
            .json(&payload)
            .send()
            .expect("send request failed");

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("failed to read response");
        if response.status().is_success() {
            Some(buf)
        } else {
            eprintln!("response: {:?}", &response);
            eprintln!("content: {:?}", &buf);
            None
        }

    }
    fn get(&self,
           url: &mut str,
           payload: Option<&HashMap<&str, &str>>,
           params: &mut HashMap<&str, String>)
           -> Option<String> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = String::from(url.to_owned());
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Get, &url_with_params, payload, params)
        } else {

            self.internal_call(Get, url, payload, params)
        }
    }

    fn post(&self,
            url: &mut str,
            payload: Option<&HashMap<&str, &str>>,
            params: &mut HashMap<&str, String>)
            -> Option<String> {
        self.internal_call(Post, url, payload, params)
    }
    fn put(&self,
           url: &mut str,
           payload: Option<&HashMap<&str, &str>>,
           params: &mut HashMap<&str, String>)
           -> Option<String> {
        self.internal_call(Put, url, payload, params)
    }
    fn delete(&self,
              url: &mut str,
              payload: Option<&HashMap<&str, &str>>,
              params: &mut HashMap<&str, String>)
              -> Option<String> {
        self.internal_call(Delete, url, payload, params)
    }
    ///returns a single track given the track's ID, URI or URL
    ///Parameters:
    ///- track_id - a spotify URI, URL or ID
    pub fn track(&self, track_id: &mut str) -> Option<FullTrack> {
        let trid = self.get_id(TYPE::Track, track_id);
        let mut url = String::from("tracks/");
        url.push_str(&trid);
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<FullTrack>(&result.unwrap_or_default())
    }

    ///returns a list of tracks given a list of track IDs, URIs, or URLs
    ///Parameters:
    ///- track_ids - a list of spotify URIs, URLs or IDs
    ///- market - an ISO 3166-1 alpha-2 country code.
    pub fn tracks(&self, track_ids: Vec<String>, market: Option<&str>) -> Option<FullTracks> {
        let mut ids: Vec<String> = vec![];
        for mut track_id in track_ids {
            ids.push(self.get_id(TYPE::Track, &mut track_id));
        }
        let mut url = String::from("tracks/?ids=");
        url.push_str(&ids.join(","));
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_market) = market {
            params.insert("market", _market.to_owned());
        }
        println!("{:?}", &url);
        let result = self.get(&mut url, None, &mut params);
        self.convert_result::<FullTracks>(&result.unwrap_or_default())
    }
    ///returns a single artist given the artist's ID, URI or URL
    ///Parameters:
    ///- artist_id - an artist ID, URI or URL
    pub fn artist(&self, artist_id: &mut str) -> Option<FullArtist> {
        let trid = self.get_id(TYPE::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<FullArtist>(&result.unwrap_or_default())
    }

    ///returns a list of artists given the artist IDs, URIs, or URLs
    ///Parameters:
    ///- artist_ids - a list of  artist IDs, URIs or URLs
    pub fn artists(&self, artist_ids: Vec<String>) -> Option<FullArtists> {
        let mut ids: Vec<String> = vec![];
        for mut artist_id in artist_ids {
            ids.push(self.get_id(TYPE::Artist, &mut artist_id));
        }
        let mut url = String::from("artists/?ids=");
        url.push_str(&ids.join(","));
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<FullArtists>(&result.unwrap_or_default())
    }

    ///  Get Spotify catalog information about an artist's albums
    /// - artist_id - the artist ID, URI or URL
    /// - album_type - 'album', 'single', 'appears_on', 'compilation'
    /// - country - limit the response to one particular country.
    /// - limit  - the number of albums to return
    /// - offset - the index of the first album to return

    pub fn artist_albums(&self,
                         artist_id: &mut str,
                         album_type: Option<ALBUM_TYPE>,
                         country: Option<&str>,
                         limit: Option<u32>,
                         offset: Option<u32>)
                         -> Option<Page<SimplifiedAlbum>> {
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_limit) = limit {
            params.insert("limit", _limit.to_string());
        }
        if let Some(_album_type) = album_type {
            params.insert("album_type", _album_type.as_str().to_owned());
        }
        if let Some(_offset) = offset {
            params.insert("offset", _offset.to_string());
        }
        if let Some(_country) = country {
            params.insert("country", _country.to_string());
        }
        let trid = self.get_id(TYPE::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/albums");
        let result = self.get(&mut url, None, &mut params);
        self.convert_result::<Page<SimplifiedAlbum>>(&result.unwrap_or_default())

    }

    /// Get Spotify catalog information about an artist's top 10 tracks by country.
    ///    Parameters:
    ///        - artist_id - the artist ID, URI or URL
    ///        - country - limit the response to one particular country.
    pub fn artist_top_tracks(&self,
                             artist_id: &mut str,
                             country: Option<&str>)
                             -> Option<FullTracks> {
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_country) = country {
            params.insert("country", _country.to_string());
        } else {
            params.insert("country", "US".to_owned());
        }
        println!("{:?}", &params);
        let trid = self.get_id(TYPE::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/top-tracks");
        match self.get(&mut url, None, &mut params) {
            Some(result) => {
                // let mut albums: Albums = ;
                match serde_json::from_str::<FullTracks>(&result) {
                    Ok(_tracks) => Some(_tracks),
                    Err(why) => {
                        eprintln!("convert albums from String to Albums failed {:?}", why);
                        None
                    }
                }
            }
            None => None,
        }
    }

    ///Get Spotify catalog information about artists similar to an
    ///identified artist. Similarity is based on analysis of the
    ///Spotify community's listening history.
    ///Parameters:
    ///- artist_id - the artist ID, URI or URL
    pub fn artist_related_artists(&self, artist_id: &mut str) -> Option<FullArtists> {
        let trid = self.get_id(TYPE::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/related-artists");
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<FullArtists>(&result.unwrap_or_default())
    }

    ///returns a single album given the album's ID, URIs or URL
    ///Parameters:
    ///- album_id - the album ID, URI or URL
    pub fn album(&self, album_id: &mut str) -> Option<FullAlbum> {
        let trid = self.get_id(TYPE::Album, album_id);
        let mut url = String::from("albums/");
        url.push_str(&trid);
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<FullAlbum>(&result.unwrap_or_default())

    }
    ///returns a list of albums given the album IDs, URIs, or URLs
    ///Parameters:
    ///- albums_ids - a list of  album IDs, URIs or URLs
    pub fn albums(&self, album_ids: Vec<String>) -> Option<FullAlbums> {
        let mut ids: Vec<String> = vec![];
        for mut album_id in album_ids {
            ids.push(self.get_id(TYPE::Album, &mut album_id));
        }
        let mut url = String::from("albums/?ids=");
        url.push_str(&ids.join(","));
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<FullAlbums>(&result.unwrap_or_default())
    }

    ///Get Spotify catalog information about an album's tracks
    ///Parameters:
    ///- album_id - the album ID, URI or URL
    ///- limit  - the number of items to return
    ///- offset - the index of the first item to return
    pub fn album_track(&self,
                       album_id: &mut str,
                       limit: Option<u16>,
                       offset: Option<u32>)
                       -> Option<Page<SimplifiedTrack>> {
        let mut params = HashMap::new();
        let trid = self.get_id(TYPE::Album, album_id);
        let mut url = String::from("albums/");
        url.push_str(&trid);
        url.push_str("/tracks");
        if let Some(_limit) = limit {
            params.insert("limit", _limit.to_string());
        } else {
            params.insert("limit", "50".to_string());
        }
        if let Some(_offset) = offset {
            params.insert("offset", _offset.to_string());
        } else {
            params.insert("offset", "0".to_string());
        }
        let result = self.get(&mut url, None, &mut params);
        self.convert_result::<Page<SimplifiedTrack>>(&result.unwrap_or_default())

    }

    ///Gets basic profile information about a Spotify User
    ///Parameters:
    ///- user - the id of the usr
    pub fn user(&self, user_id: &str) -> Option<PublicUser> {
        let mut url = String::from("users/");
        url.push_str(&user_id);
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<PublicUser>(&result.unwrap_or_default())
    }

    ///Get current user playlists without required getting his profile
    ///Parameters:
    ///- limit  - the number of items to return
    ///- offset - the index of the first item to return
    pub fn current_user_playlists(&self,
                                  limit: Option<u32>,
                                  offset: Option<u32>)
                                  -> Option<Page<SimplifiedPlaylist>> {
        let mut params = HashMap::new();
        if let Some(_limit) = limit {
            params.insert("limit", _limit.to_string());
        } else {
            params.insert("limit", "50".to_string());
        }
        if let Some(_offset) = offset {
            params.insert("offset", _offset.to_string());
        } else {
            params.insert("offset", "0".to_string());
        }
        let mut url = String::from("me/playlists");
        let result = self.get(&mut url, None, &mut params);
        self.convert_result::<Page<SimplifiedPlaylist>>(&result.unwrap_or_default())
    }

    ///Gets playlists of a user
    ///Parameters:
    ///- user_id - the id of the usr
    ///- limit  - the number of items to return
    ///- offset - the index of the first item to return
    pub fn user_playlists(&self,
                          user_id: &str,
                          limit: Option<u32>,
                          offset: Option<u32>)
                          -> Option<Page<SimplifiedPlaylist>> {
        let mut params = HashMap::new();
        if let Some(_limit) = limit {
            params.insert("limit", _limit.to_string());
        } else {
            params.insert("limit", "50".to_string());
        }
        if let Some(_offset) = offset {
            params.insert("offset", _offset.to_string());
        } else {
            params.insert("offset", "0".to_string());
        }
        let mut url = String::from(format!("users/{}/playlists", user_id));
        let result = self.get(&mut url, None, &mut params);
        self.convert_result::<Page<SimplifiedPlaylist>>(&result.unwrap_or_default())
    }


    fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Option<T> {
        match serde_json::from_str::<T>(input) {
            Ok(result) => Some(result),
            Err(why) => {
                eprintln!("convert result failed {:?}", why);
                eprintln!("content: {:?}", &input);
                None
            }
        }
    }


    fn get_id(&self, artist_type: TYPE, artist_id: &mut str) -> String {
        let fields: Vec<&str> = artist_id.split(":").collect();
        let len = fields.len();
        if len >= 3 {
            if artist_type.as_str() != fields[len - 2] {
                eprintln!("expected id of type {:?} but found type {:?} {:?}",
                          artist_type,
                          fields[len - 2],
                          artist_id);
            } else {
                return fields[len - 1].to_owned();
            }
        }
        let sfields: Vec<&str> = artist_id.split("/").collect();
        let len: usize = sfields.len();
        if len >= 3 {
            if artist_type.as_str() != sfields[len - 2] {
                eprintln!("expected id of type {:?} but found type {:?} {:?}",
                          artist_type,
                          sfields[len - 2],
                          artist_id);
            } else {
                return sfields[len - 1].to_owned();
            }
        }
        return artist_id.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_id() {
        // assert artist
        let spotify = Spotify::default().access_token("test-access").build();
        let mut artist_id = String::from("spotify:artist:2WX2uTcsvV5OnS0inACecP");
        let id = spotify.get_id(TYPE::Artist, &mut artist_id);
        assert_eq!("2WX2uTcsvV5OnS0inACecP", &id);
        // assert album
        let mut artist_id_a = String::from("spotify/album/2WX2uTcsvV5OnS0inACecP");
        assert_eq!("2WX2uTcsvV5OnS0inACecP",
                   &spotify.get_id(TYPE::Album, &mut artist_id_a));

        // mismatch type
        let mut artist_id_b = String::from("spotify:album:2WX2uTcsvV5OnS0inACecP");
        assert_eq!("spotify:album:2WX2uTcsvV5OnS0inACecP",
                   &spotify.get_id(TYPE::Artist, &mut artist_id_b));

        // could not split
        let mut artist_id_c = String::from("spotify-album-2WX2uTcsvV5OnS0inACecP");
        assert_eq!("spotify-album-2WX2uTcsvV5OnS0inACecP",
                   &spotify.get_id(TYPE::Artist, &mut artist_id_c));
    }
}
