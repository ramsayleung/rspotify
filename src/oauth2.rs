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

    /// Gets the required URL to authorize the current client to start the
    /// [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    pub fn get_authorize_url(&self, show_dialog: bool) -> ClientResult<String> {
        let oauth = self.get_oauth()?;
        let mut payload: HashMap<&str, &str> = HashMap::new();
        let scope = oauth
            .scope
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ");
        payload.insert(headers::CLIENT_ID, &self.get_creds()?.id);
        payload.insert(headers::RESPONSE_TYPE, headers::RESPONSE_CODE);
        payload.insert(headers::REDIRECT_URI, &oauth.redirect_uri);
        payload.insert(headers::SCOPE, &scope);
        payload.insert(headers::STATE, &oauth.state);

        if show_dialog {
            payload.insert(headers::SHOW_DIALOG, "true");
        }

        let parsed = Url::parse_with_params(auth_urls::AUTHORIZE, payload)?;
        Ok(parsed.into_string())
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

    /// Refreshes the access token with the refresh token provided by the
    /// [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow),
    /// without saving it into the cache file.
    ///
    /// The obtained token will be saved internally.
    #[maybe_async]
    pub async fn refresh_user_token_without_cache(
        &mut self,
        refresh_token: &str,
    ) -> ClientResult<()> {
        let mut data = Form::new();
        data.insert(headers::REFRESH_TOKEN, refresh_token);
        data.insert(headers::GRANT_TYPE, headers::GRANT_REFRESH_TOKEN);

        let mut tok = self.fetch_access_token(&data).await?;
        tok.refresh_token = Some(refresh_token.to_string());
        self.token = Some(tok);

        Ok(())
    }

    /// The same as `refresh_user_token_without_cache`, but saves the token
    /// into the cache file if possible.
    #[maybe_async]
    pub async fn refresh_user_token(&mut self, refresh_token: &str) -> ClientResult<()> {
        self.refresh_user_token_without_cache(refresh_token).await?;

        Ok(())
    }

    /// Obtains the client access token for the app without saving it into the
    /// cache file. The resulting token is saved internally.
    #[maybe_async]
    pub async fn request_client_token_without_cache(&mut self) -> ClientResult<()> {
        let mut data = Form::new();
        data.insert(headers::GRANT_TYPE, headers::GRANT_CLIENT_CREDS);

        self.token = Some(self.fetch_access_token(&data).await?);

        Ok(())
    }

    /// The same as `request_client_token_without_cache`, but saves the token
    /// into the cache file if possible.
    #[maybe_async]
    pub async fn request_client_token(&mut self) -> ClientResult<()> {
        self.request_client_token_without_cache().await?;
        self.write_token_cache()
    }

    /// Parse the response code in the given response url. If the URL cannot be
    /// parsed or the `code` parameter is not present, this will return `None`.
    ///
    /// Step 2 of the [Authorization Code Flow
    /// ](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    pub fn parse_response_code(&self, url: &str) -> Option<String> {
        let url = Url::parse(url).ok()?;
        let mut params = url.query_pairs();
        let (_, url) = params.find(|(key, _)| key == "code")?;
        Some(url.to_string())
    }

    /// Obtains the user access token for the app with the given code without
    /// saving it into the cache file, as part of the OAuth authentication.
    /// The access token will be saved inside the Spotify instance.
    ///
    /// Step 3 of the [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    #[maybe_async]
    pub async fn request_user_token_without_cache(&mut self, code: &str) -> ClientResult<()> {
        let oauth = self.get_oauth()?;
        let mut data = Form::new();
        let scopes = oauth
            .scope
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ");
        data.insert(headers::GRANT_TYPE, headers::GRANT_AUTH_CODE);
        data.insert(headers::REDIRECT_URI, oauth.redirect_uri.as_ref());
        data.insert(headers::CODE, code);
        data.insert(headers::SCOPE, scopes.as_ref());
        data.insert(headers::STATE, oauth.state.as_ref());

        self.token = Some(self.fetch_access_token(&data).await?);

        Ok(())
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

    /// Tries to open the authorization URL in the user's browser, and returns
    /// the obtained code.
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    fn get_code_from_user(&self) -> ClientResult<String> {
        use crate::client::ClientError;

        let url = self.get_authorize_url(false)?;

        match webbrowser::open(&url) {
            Ok(_) => println!("Opened {} in your browser.", url),
            Err(why) => eprintln!(
                "Error when trying to open an URL in your browser: {:?}. \
                 Please navigate here manually: {}",
                why, url
            ),
        }

        println!("Please enter the URL you were redirected to: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let code = self
            .parse_response_code(&input)
            .ok_or_else(|| ClientError::Cli("unable to parse the response code".to_string()))?;

        Ok(code)
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
