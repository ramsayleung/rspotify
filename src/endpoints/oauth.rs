use crate::rspotify::{prelude::*, OAuth};

pub trait OAuthClient: BaseClient {
    fn get_oauth(&self) -> &OAuth;

    fn user_endpoint(&self) {
        println!("Performing OAuth request");
        self.endpoint_request();
    }
}
