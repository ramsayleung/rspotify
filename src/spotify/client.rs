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
use super::model::album::{Albums, Album};
use super::model::track::{Tracks, Track};
use super::util::convert_map_to_string;
pub struct Spotify {
    pub prefix: String,
    pub access_token: Option<String>,
    pub client_credentials_manager: Option<SpotifyClientCredentials>,
}
// struct Converter<T>;

// impl Converter{
//     pub fn convert_result(input:Option<String>)
// }
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

    pub fn track(&self, track_id: &mut str) -> Option<Track> {
        let trid = self.get_id(TYPE::TRACK, track_id);
        let mut url = String::from("tracks/");
        url.push_str(&trid);
        let result = self.get(&mut url, None, &mut HashMap::new());
        self.convert_result::<Track>(&result.unwrap_or_default())
    }

    pub fn tracks(&self, tracks: Vec<String>, market: Option<&str>) -> Option<Tracks> {
        let mut ids: Vec<String> = vec![];
        for mut track_id in tracks {
            ids.push(self.get_id(TYPE::TRACK, &mut track_id));
        }
        let mut url = String::from("tracks/?ids=");
        url.push_str(&ids.join(","));
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_market) = market {
            params.insert("market", _market.to_owned());
        }
        println!("{:?}", &url);
        let result = self.get(&mut url, None, &mut params);
        self.convert_result::<Tracks>(&result.unwrap_or_default())
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
                         -> Option<Albums> {
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
        let trid = self.get_id(TYPE::ARTIST, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/albums");
        let result = self.get(&mut url, None, &mut params);
        self.convert_result::<Albums>(&result.unwrap_or_default())

    }
    fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Option<T> {
        match serde_json::from_str::<T>(input) {
            Ok(result) => Some(result),
            Err(why) => {
                println!("convert result failed {:?}", why);
                None
            }
        }
    }

    /// Get Spotify catalog information about an artist's top 10 tracks
    ///    by country.

    ///    Parameters:
    ///        - artist_id - the artist ID, URI or URL
    ///        - country - limit the response to one particular country.
    pub fn artist_top_tracks(&self, artist_id: &mut str, country: Option<&str>) -> Option<Tracks> {
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_country) = country {
            params.insert("country", _country.to_string());
        } else {
            params.insert("country", "US".to_owned());
        }
        println!("{:?}", &params);
        let trid = self.get_id(TYPE::ARTIST, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/top-tracks");
        match self.get(&mut url, None, &mut params) {
            Some(result) => {
                // let mut albums: Albums = ;
                match serde_json::from_str::<Tracks>(&result) {
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
        let id = spotify.get_id(TYPE::ARTIST, &mut artist_id);
        assert_eq!("2WX2uTcsvV5OnS0inACecP", &id);
        // assert album
        let mut artist_id_a = String::from("spotify/album/2WX2uTcsvV5OnS0inACecP");
        assert_eq!("2WX2uTcsvV5OnS0inACecP",
                   &spotify.get_id(TYPE::ALBUM, &mut artist_id_a));

        // mismatch type
        let mut artist_id_b = String::from("spotify:album:2WX2uTcsvV5OnS0inACecP");
        assert_eq!("spotify:album:2WX2uTcsvV5OnS0inACecP",
                   &spotify.get_id(TYPE::ARTIST, &mut artist_id_b));

        // could not split
        let mut artist_id_c = String::from("spotify-album-2WX2uTcsvV5OnS0inACecP");
        assert_eq!("spotify-album-2WX2uTcsvV5OnS0inACecP",
                   &spotify.get_id(TYPE::ARTIST, &mut artist_id_c));
    }
}
