use rspotify::Credentials;

#[cfg(not(target_arch = "wasm32"))]
pub fn get_credentials() -> Credentials {
    // The credentials must be available in the environment.
    Credentials::from_env().unwrap_or_else(|| {
        panic!(
            "No credentials configured. Make sure that either the `env-file` \
            feature is enabled, or that the required environment variables are \
            exported (`RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET`)"
        )
    })
}

#[cfg(target_arch = "wasm32")]
pub fn get_credentials() -> Credentials {
    // The credentials must be available in the environment. Panics if they are not available
    let id = dotenvy_macro::dotenv!("RSPOTIFY_CLIENT_ID");
    let secret = dotenvy_macro::dotenv!("RSPOTIFY_CLIENT_SECRET");
    Credentials::new(&id, &secret)
}
