//! User authorization and client credentials management.


/// Authorization-related methods for the client.
impl Spotify {
    /// Updates the cache file at the internal cache path.
    pub fn write_token_cache(&self) -> ClientResult<()> {
        if let Some(tok) = self.token.as_ref() {
            tok.write_cache(&self.cache_path)?;
        }

        Ok(())
    }

    /// Tries to read the cache file's token, which may not exist.
    #[maybe_async]
    pub async fn read_token_cache(&mut self) -> Option<Token> {
        let tok = TokenBuilder::from_cache(&self.cache_path).build().ok()?;

        if !self.get_oauth().ok()?.scope.is_subset(&tok.scope) || tok.is_expired() {
            // Invalid token, since it doesn't have at least the currently
            // required scopes or it's expired.
            None
        } else {
            Some(tok)
        }
    }

    /// Sends a request to Spotify for an access token.
    #[maybe_async]
    async fn fetch_access_token(&self, payload: &Form<'_>) -> ClientResult<Token> {
        // This request uses a specific content type, and the client ID/secret
        // as the authentication, since the access token isn't available yet.
        let mut head = Headers::new();
        let (key, val) = headers::basic_auth(&self.get_creds()?.id, &self.get_creds()?.secret);
        head.insert(key, val);

        let response = self
            .post_form(auth_urls::TOKEN, Some(&head), payload)
            .await?;
        let mut tok = serde_json::from_str::<Token>(&response)?;
        tok.expires_at = Utc::now().checked_add_signed(tok.expires_in);
        Ok(tok)
    }



    /// The same as `request_user_token_without_cache`, but saves the token into
    /// the cache file if possible.
    #[maybe_async]
    pub async fn request_user_token(&mut self, code: &str) -> ClientResult<()> {
        self.request_user_token_without_cache(code).await?;
        self.write_token_cache()
    }

    /// Opens up the authorization URL in the user's browser so that it can
    /// authenticate. It also reads from the standard input the redirect URI
    /// in order to obtain the access token information. The resulting access
    /// token will be saved internally once the operation is successful.
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    #[maybe_async]
    pub async fn prompt_for_user_token_without_cache(&mut self) -> ClientResult<()> {
        let code = self.get_code_from_user()?;
        self.request_user_token_without_cache(&code).await?;

        Ok(())
    }

    /// The same as the `prompt_for_user_token_without_cache` method, but it
    /// will try to use the user token into the cache file, and save it in
    /// case it didn't exist/was invalid.
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    #[maybe_async]
    pub async fn prompt_for_user_token(&mut self) -> ClientResult<()> {
        // TODO: shouldn't this also refresh the obtained token?
        self.token = self.read_token_cache().await;

        // Otherwise following the usual procedure to get the token.
        if self.token.is_none() {
            let code = self.get_code_from_user()?;
            // Will write to the cache file if successful
            self.request_user_token(&code).await?;
        }

        Ok(())
    }

}

#[cfg(test)]
mod test {
    use super::generate_random_string;
    use std::collections::HashSet;

    #[test]
    fn test_generate_random_string() {
        let mut containers = HashSet::new();
        for _ in 1..101 {
            containers.insert(generate_random_string(10));
        }
        assert_eq!(containers.len(), 100);
    }
}
