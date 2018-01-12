use std::collections::HashMap;
use reqwest::Client;
use reqwest::Method;
pub struct Spotify {
    pub prefix: String,
    pub requests_time: u8,
    pub client: Client,
}
impl Spotify {
    fn auth_headers(&self) -> HashMap {
        HashMap::new()
    }
    fn internal_call(&self,method:Method,url:String)->HashMap{

    }
}
