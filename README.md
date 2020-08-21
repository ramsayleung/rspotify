[![Continuous Integration](https://github.com/ramsayleung/rspotify/workflows/Continuous%20Integration/badge.svg)](https://github.com/ramsayleung/rspotify/actions)
[![License](https://img.shields.io/github/license/ramsayleung/rspotify)](https://github.com/ramsayleung/rspotify/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/rspotify.svg)](https://crates.io/crates/rspotify)
[![Docs](https://docs.rs/rspotify/badge.svg)](https://docs.rs/crate/rspotify/)

# Rspotify - a Rust client for The Spotify Web API 

## Disclaimer

This crate is heavily inspired by [spotipy](https://github.com/plamere/spotipy) - A Spotify API wrapper implemented in Python.

## Description

Rspotify is a lightweight wrapper for the [Spotify Web API](https://developer.spotify.com/web-api/) It includes helper functions for **all Spotify's endpoints**, such as fetching metadata (search and look-up of albums, artists, tracks, playlists, new releases) and user's information (follow users, artists and playlists, and saved tracks management).

## Features

*Rspotify* supports all the features of the Spotify Web API, including access to all endpoints, and support for user authorization. Note that before accessing to any endpoints, you need to be authorized. For details on the capabilities you are encouraged to review the [Spotify Web Endpoint Reference](https://developer.spotify.com/web-api/endpoint-reference/) documentation.

Nowadays, thanks to [`reqwest`](https://docs.rs/reqwest/0.10.1/reqwest/#proxies), `rspotify` supports system proxy by default. `reqwest` system proxies look in environment variables to set HTTP or HTTPS proxies. `HTTP_PROXY` or `http_proxy` provide HTTP proxies for HTTP connections while `HTTPS_PROXY` or `https_proxy` provide HTTPS proxies for HTTPS connections (~~note that `reqwest` system proxy doesn't support socks proxy for now,~~ check [this issue](https://github.com/seanmonstar/reqwest/issues/790) for more details).

## Usage

Add this to your `Cargo.toml`:

``` toml
[dependencies]
rspotify = "0.10.0"
```

Or you could get it from [GitHub](https://github.com/samrayleung/rspotify).

## Getting Started

### Authorization

Since all methods require user authorization now, you will need to generate an authorization token that indicates that the user has granted permission for your application to perform the given task. You will need to register your app to get the credentials necessary to make authorized calls. Read the [official guide for a detailed explanation](https://developer.spotify.com/documentation/general/guides/authorization-guide/).

Even if your script does not have an accessible URL, you need to specify one when registering your application where the Spotify authentication API will redirect to after successful login. The URL doesn't need to work or be accessible, you can specify "http://localhost/", and after the user's successful login the access token will be given as the `code` HTTP parameter: "http://localhost/?code=...". For example:

![demo](./doc/images/rspotify.gif)

In order to help other developers to get used to `rspotify`, I registered a Spotify account with temporary email. You can test `rspotify` with this account's `CLIENT_ID` and `CLIENT_SECRET`, check the [`.env` file](./.env) for more details.

### Examples

There are some [available examples](./examples) which can serve as a learning tool. The following snippet will obtain the top tracks for an artist:

``` toml
[dependencies]
rspotify = { version = "0.10.0" }
tokio = { version = "0.2", features = ["full"] }
```

``` rust
use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::senum::Country;

#[tokio::main]
async fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    let client_credential = SpotifyClientCredentials::default().build();

    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let tracks = spotify
        .artist_top_tracks(birdy_uri, Country::UnitedStates)
        .await;
    println!("{:?}", tracks.unwrap());
}
```

### Blocking API example

There is an optional client API that can be enabled for non-asynchronous requests:

``` toml
[dependencies]
rspotify = { version = "0.10.0", features = ["blocking"] }
```

``` rust
use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::SpotifyClientCredentials;
use rspotify::senum::Country;

fn main() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let tracks = spotify.artist_top_tracks(birdy_uri, Country::UnitedStates);
    println!("{:?}", tracks.unwrap());
}
```

## API Documentation

For more API information, check the [Rspotify API documentation](https://docs.rs/crate/rspotify).

## CHANGELOG

Please see the [changelog](./CHANGELOG.md) for a release history and indications on how to upgrade from one version to another.

## Contribution

If you find any problems or have suggestions about this crate, please submit an issue. Moreover, any pull request, code review and feedback are welcome.

## License

[MIT](./LICENSE)
