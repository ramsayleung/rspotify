use crate::{Credentials, HTTPClient, Token};
use std::collections::HashMap;

pub trait BaseClient {
    fn get_http(&self) -> &HTTPClient;
    fn get_token(&self) -> Option<&Token>;
    fn get_creds(&self) -> &Credentials;

    // Existing
    fn request(&self, mut params: HashMap<String, String>) {
        let http = self.get_http();
        params.insert("url".to_string(), "...".to_string());
        http.request(params);
    }

    // Existing
    fn endpoint_request(&self) {
        let mut params = HashMap::new();
        params.insert("token".to_string(), self.get_token().unwrap().0.clone());
        self.request(params);
    }

    fn base_endpoint(&self) {
        println!("Performing base request");
        self.endpoint_request();
    }
}
