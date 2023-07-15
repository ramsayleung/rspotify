//! If developer specify a callback function for token, this callback function
//! will be invoked whenever client succeeds to request or refetch a token.
//! Therefore, developer could write token into file or save token into database
//! after fetch the token with their own callback function.

use std::sync::Arc;

use rspotify::{
    clients::OAuthClient, scopes, AuthCodePkceSpotify, AuthCodeSpotify, CallbackError,
    ClientCredsSpotify, Config, Credentials, OAuth, TokenCallback,
};

async fn _with_pkce(creds: Credentials, oauth: OAuth) {
    let operate_token_fn = |token| {
        println!(">>> From token callback function with AuthCodePkceSpotify");
        println!(">>> Let's manipulate it. Oooh, we could only read it");
        println!(">>> token: {:?}", token);
        Err(CallbackError::CustomizedError(
            "oooh, there is an error".to_string(),
        ))
    };
    let token_callback = TokenCallback(Box::new(operate_token_fn));

    // Enabling automatic token refreshing in the config
    let config = Config {
        token_callback_fn: Arc::new(Some(token_callback)),
        ..Default::default()
    };

    let mut spotify = AuthCodePkceSpotify::with_config(creds, oauth, config);
    let url = spotify.get_authorize_url(None).unwrap();
    // This function requires the `cli` feature enabled.
    spotify
        .prompt_for_token(&url)
        .await
        .expect("couldn't authenticate successfully");
}

async fn with_auth(creds: Credentials, oauth: OAuth) {
    let operate_token_fn = |token| {
        println!(">>> From token callback function with AuthCodeSpotify");
        println!(">>> Let's manipulate it. Oooh, we could only read it");
        println!(">>> token: {:?}", token);
        Ok(())
    };
    let token_callback = TokenCallback(Box::new(operate_token_fn));

    // Enabling automatic token refreshing in the config
    let config = Config {
        token_callback_fn: Arc::new(Some(token_callback)),
        ..Default::default()
    };

    println!(">>> Fetch token with AuthCodeSpotify");
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify
        .prompt_for_token(&url)
        .await
        .expect("couldn't authenticate successfully");
}

async fn with_client_credentials(creds: Credentials) {
    let operate_token_fn = |token| {
        println!(">>> From token callback function with ClientCredsSpotify");
        println!(">>> Let's manipulate it. Oooh, we could only read it");
        println!(">>> token: {:?}", token);
        Ok(())
    };
    let token_callback = TokenCallback(Box::new(operate_token_fn));

    // Enabling automatic token refreshing in the config
    let config = Config {
        token_callback_fn: Arc::new(Some(token_callback)),
        ..Default::default()
    };
    // Same with client-credential based spotify client
    println!(">>> Fetch token with ClientCredsSpotify");
    let spotify = ClientCredsSpotify::with_config(creds, config);
    spotify.request_token().await.unwrap();
}

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // May require the `env-file` feature enabled if the environment variables
    // aren't configured manually.
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-follow-read user-follow-modify")).unwrap();

    with_auth(creds.clone(), oauth.clone()).await;
    // with_pkce(creds.clone(), oauth).await;
    with_client_credentials(creds).await;
}
