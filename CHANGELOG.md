## 0.13.0 (2024.03.08)

**New features**
- ([#458](https://github.com/ramsayleung/rspotify/pull/458)) Support for the `wasm32-unknown-unknown` build target

**Bugfixes**
- ([#440](https://github.com/ramsayleung/rspotify/issues/440)) Add Smartwatch device type, fix for json parse error: unknown variant Smartwatch.
- ([#447](https://github.com/ramsayleung/rspotify/pull/447)) Replace the deprecated `dotenv` crate with `dotenvy`

## 0.13.0 (2023.08.26)
**New features**
- ([#390](https://github.com/ramsayleung/rspotify/pull/390)) The `scopes!` macro supports to split the scope by whitespace.
- ([#418](https://github.com/ramsayleung/rspotify/pull/418)) Add a user-settable callback function whenever token is updated.

**Breaking changes**
- ([#409](https://github.com/ramsayleung/rspotify/pull/409)) Change type of `position` parameter in `playlist_add_items` endpoint from `Opinion<Duration>` to `Opinion<u32>`
- ([#421](https://github.com/ramsayleung/rspotify/issues/421)) Change type of `AudioFeaturesPayload.audio_features` from `Vec<AudioFeatures>` to `Vec<Option<AudioFeatures>>`
- ([#429](https://github.com/ramsayleung/rspotify/pull/429)) Enable Token refreshing by default.
- ([#432](https://github.com/ramsayleung/rspotify/pull/432)) Add optional `market` field to `track`, `album`, `albums`, and `album_track[_manual]`. Make `market` field in `artist_top_tracks` optional.

**Bugfixes**
- ([#419](https://github.com/ramsayleung/rspotify/issues/419)) Base64url encode instead of plain base64 encode for PKCE challenge code.
- ([#421](https://github.com/ramsayleung/rspotify/issues/421)) Filter `null`s on `tracks_features` requests
- ([#424](https://github.com/ramsayleung/rspotify/pull/424)) Fix PKCE refresh token invalid error
- ([#428](https://github.com/ramsayleung/rspotify/pull/428)) Fix PKCE url in doc

## 0.11.7 (2023.04.26)

- ([#399](https://github.com/ramsayleung/rspotify/pull/399)) Add a new variant `Collectionyourepisodes` for `Type` enum.
- ([#375](https://github.com/ramsayleung/rspotify/pull/375)) We now use `chrono::Duration` in more places for consistency and usability: `start_uris_playback`, `start_context_playback`, `rspotify_model::Offset`, `resume_playback`, `seek_track`. Some of these fields have been renamed from `position_ms` to `position`.
- ((#356)[https://github.com/ramsayleung/rspotify/pull/356]) We now support custom authentication base URLs. `Config::prefix` has been renamed to `Config::api_base_url`, and we've introduced `Config::auth_base_url`.
- ([#384](https://github.com/ramsayleung/rspotify/pull/384)) Add STB alias for Stb device type, fix for `json parse error: unknown variant STB`.
- ([#386](https://github.com/ramsayleung/rspotify/pull/386)) Support `BaseClient::artist_albums` with zero or more `AlbumType`.
- ([#393](https://github.com/ramsayleung/rspotify/pull/393)) Add `ureq-rustls-tls-native-certs` feature flag.
- ([#402](https://github.com/ramsayleung/rspotify/pull/402)) Add `ureq-native-tls` feature flag.

**Bugfixes**
- ([#394](https://github.com/ramsayleung/rspotify/pull/394)) Set a common 10 second timeout for both http clients.

## 0.11.6 (2022.12.14)

- ([#331](https://github.com/ramsayleung/rspotify/pull/331)) `Market` is now `Copy`
- ([#366](https://github.com/ramsayleung/rspotify/pull/366)) Replace all `std::time::Duration` with `chrono::Duration` to support negative duration.

**Bugfixes**:
- ([#332](https://github.com/ramsayleung/rspotify/pull/332)) Fix typo in `RestrictionReason` enum values

**Breaking changes**:
- ([#336](https://github.com/ramsayleung/rspotify/pull/336)) `Offset::for_position` and `Offset::for_uri` have been removed, as they were unnecessary. Use `Offset::Position` and `Offset::Uri` instead, respectively.
- ([#305](https://github.com/ramsayleung/rspotify/pull/305)) The `Id` types have been refactored to maximize usability. Instead of focusing on having an object-safe trait and using `dyn Id`, we now have enums to group up the IDs. This is based on how [`enum_dispatch`](https://docs.rs/enum_dispatch) works, and it's not only easier to use, but also more efficient. It makes it possible to have borrowed IDs again, so we've chosen to use `Cow` internally for flexibility. Check out the docs for more information!

  Please let us know if there is anything that could be improved. Unfortunately, this breaks many methods in `BaseClient` and `OAuthClient`, but the errors should occur at compile-time only.
- ([#325](https://github.com/ramsayleung/rspotify/pull/325)) The `auth_code`, `auth_code_pkce`, `client_creds`, `clients::base` and `clients::oauth` modules have been removed from the public API; you should access the same types from their parent modules instead
- ([#326](https://github.com/ramsayleung/rspotify/pull/326)) The `rspotify::clients::mutex` module has been renamed to `rspotify::sync`
- ([#330](https://github.com/ramsayleung/rspotify/pull/330)) `search` now accepts `Option<IncludeExternal>` instead of `Option<&IncludeExternal>`
- ([#330](https://github.com/ramsayleung/rspotify/pull/330)) `featured_playlists` now accepts `Option<chrono::DateTime<chrono::Utc>>` instead of `Option<&chrono::DateTime<chrono::Utc>>`
- ([#330](https://github.com/ramsayleung/rspotify/pull/330)) `current_user_top_artists[_manual]` and `current_user_top_tracks[_manual]` now accept `Option<TimeRange>` instead of `Option<&TimeRange>`
- ([#331](https://github.com/ramsayleung/rspotify/pull/331)) All enums now implement `Into<&'static str>` instead of `AsRef<str>`
- ([#331](https://github.com/ramsayleung/rspotify/pull/331)) `Option<&Market>` parameters have been changed to `Option<Market>`

**New features**
- ([362](https://github.com/ramsayleung/rspotify/pull/362)) Add "Get the User's Queue" endpoint

## 0.11.5 (2022.03.28)

**Breaking changes**:
- ([#306](https://github.com/ramsayleung/rspotify/pull/306)) The `collection` variant has been added to `Type`

## 0.11.4 (2022.03.08)

- ([#295](https://github.com/ramsayleung/rspotify/pull/295)) The `Tv` variant in `DeviceType` is actually case insensitive.
- ([#296](https://github.com/ramsayleung/rspotify/pull/296)) The `Avr` variant in `DeviceType` is actually case insensitive.
- ([#302](https://github.com/ramsayleung/rspotify/pull/302)) Added undocumented `label` field to `FullAlbum`.

**Breaking changes:**
- ([#303](https://github.com/ramsayleung/rspotify/pull/303)) The `cursors` field in `CursorBasedPage` is now optional.

## 0.11.3 (2021.11.29)

- ([#281](https://github.com/ramsayleung/rspotify/issues/281)) The documentation website for the Spotify API has been changed, so the links in the `rspotify` crate have been updated. Unfortunately, the object model section has been removed, so we've had to delete most reference links in `rspotify-model`. From now on, you should refer to the endpoints where they are used to see their definition.

**Breaking changes:**
- ([#286](https://github.com/ramsayleung/rspotify/pull/286)) Make `available_markets` optional for `FullAlbum`
- ([#282](https://github.com/ramsayleung/rspotify/pull/282)) Make `id` optional for `FullTrack`

## 0.11 (2021.10.14)

This release contains *lots* of breaking changes. These were necessary to continue RSpotify's development, and this shouldn't happen again. From now on we'll work on getting closer to the first stable release. Lots of internal code was rewritten to make RSpotify more flexible, performant and easier to use. Sorry for the inconvenience!

If we missed any change or there's something you'd like to discuss about this version, please open a new issue and let us know.

### Upgrade guide

This guide should make it easier to upgrade your code, rather than checking out the changelog line by line. The most important changes are:

* Support for **multiple HTTP clients**. Instead of using `rspotify::blocking` for synchronous access, you just need to configure the `ureq-client` feature and its TLS (learn more in the docs). However, note that this is subject to changes after [#221](https://github.com/ramsayleung/rspotify/issues/221) is fixed.
* No need for the builder pattern anymore: `Spotify` has been split up into **multiple clients depending on the authentication process** you want to follow. This means that you'll be required to `use rspotify::prelude::*` in order to access the traits with the endpoints, since they're written in a base trait now.
    * [Client Credentials Flow](https://developer.spotify.com/documentation/general/guides/authorization/client-credentials/): see `ClientCredsSpotify`.
    * [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization/code-flow): see `AuthCodeSpotify`.
    * [Authorization Code Flow with Proof Key for Code Exchange (PKCE)](https://developer.spotify.com/documentation/general/guides/authorization/code-flow): see `AuthCodePkceSpotify`. This is new! You might be interested in using PKCE for your app if you don't want to expose your client secret.
    * [Implicit Grant Flow](https://developer.spotify.com/documentation/general/guides/authorization/implicit-grant): unimplemented, as RSpotify has not been tested on a browser yet. If you'd like support for it, let us know in an issue!
* There's now support for (both sync and async) **automatic pagination**! Make sure you upgrade to these after checking out the [`pagination_async.rs`](https://github.com/ramsayleung/rspotify/blob/auth-rewrite-part4/examples/pagination_async.rs) and [`pagination_sync.rs`](https://github.com/ramsayleung/rspotify/blob/auth-rewrite-part4/examples/pagination_sync.rs) examples. You can use the `_manual`-suffixed endpoints for the previous pagination style.
* We've **renamed** a few structs and endpoints. The new names are quite similar, so the Rust compiler should suggest you what to change after an error. The only one you might not notice is the **environmental variables**: they're now `RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET` and `RSPOTIFY_REDIRECT_URI` to avoid collisions with other libraries.
* We always use **`Option<T>`** for optional parameters now. This means that you might have to add `Some(...)` to some of your parameters. We were using both `Into<Option<T>>` and `Option<T>` but decided that either of these would be best as long as it's *consistent*. `Option<T>` has less magic, so we went for that one.
* The core library has been split up with **features**. If you need `dotenv` just activate `env-file`, and if you need CLI functionality (`prompt_for_token` and similars), activate `cli`.
* We use **custom errors** now instead of the `failure` crate.
* The Spotify clients now have a `Config` struct for configuration. This includes **cached tokens** (saved/loaded automatically in a JSON file), and **refreshing tokens** (reauthenticated automatically when they become expired).

Now to a quick example: here's how you *used to* query the current user saved tracks:

```rust
extern crate rspotify;

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token;

fn main() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-read").build(); // Turns out this reads from the environment variables!
    let token_info = get_token(&mut oauth).unwrap(); // How does it get the token? Why is it not in the client if it makes a request?

    let client_credential = SpotifyClientCredentials::default() // This also accesses the environment variables with no warning.
        .token_info(token_info)
        .build(); // So verbose...

    let spotify = Spotify::default() // So verbose and easy to mess up... What auth flow is this again?
        .client_credentials_manager(client_credential)
        .build();
    let tracks = spotify.current_user_saved_tracks(10, 0); // Iterating is hard
    println!("{:?}", tracks);
}
```

And here's how you do it now:

```rust
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

fn main() {
    let oauth = OAuth::from_env(scopes!("user-library-read")).unwrap(); // Concise & explicit with `from_env`
    let creds = Credentials::from_env().unwrap(); // Same, concise & explicit

    let mut spotify = AuthCodeSpotify::new(creds, oauth); // Simpler initialization

    let url = spotify.get_authorize_url(false).unwrap(); // More flexible, lets us implement PKCE
    spotify.prompt_for_token(&url).unwrap(); // Explicit: the token is obtained by interacting with the user

    let stream = spotify.current_user_saved_tracks(None);
    println!("Items:");
    for item in stream { // Easy iteration instead of manual pagination
        println!("* {}", item.unwrap().track.name);
    }
}
```

Hopefully this will convince you that the new breaking changes are good; you'll find the new interface easier to read, more intuitive and less error prone.

Here are a few examples of upgrades:

| Name                                         | Old                                                                                                                                                                                      | New                                                                                                                  |
| -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| [Sync] device                                | [`examples/blocking/device.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/blocking/device.rs)                                       | [`examples/ureq/device.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/ureq/device.rs)             |
| [Sync] me                                    | [`examples/blocking/me.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/blocking/me.rs)                                               | [`examples/ureq/me.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/ureq/me.rs)                     |
| [Sync] search                                | [`examples/blocking/search_track.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/blocking/search_track.rs)                           | [`examples/ureq/search.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/ureq/search.rs)             |
| [Sync] seek_track                            | [`examples/blocking/seek_track.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/blocking/seek_track.rs)                               | [`examples/ureq/seek_track.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/ureq/seek_track.rs)     |
| [Sync] current_user_saved_tracks             | [`examples/blocking/current_user_saved_tracks.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/blocking/current_user_saved_tracks.rs) | [`examples/pagination_sync.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/pagination_sync.rs)     |
| [Async] current_user_saved_tracks            | [`examples/current_user_saved_tracks.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/current_user_saved_tracks.rs)                   | [`examples/pagination_async.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/pagination_async.rs)   |
| [Async] current_user_saved_tracks (manually) | [`examples/current_user_saved_tracks.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/current_user_saved_tracks.rs)                   | [`examples/pagination_manual.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/pagination_manual.rs) |
| [Async] current_playing                      | [`examples/current_playing.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/current_playing.rs)                                       | [`examples/auth_code.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/auth_code.rs)                 |
| [Async] current_playback                     | [`examples/current_playback.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/current_playback.rs)                                     | [`examples/auth_code_pkce.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/auth_code_pkce.rs)       |
| [Async] album                                | [`examples/album.rs`](https://github.com/ramsayleung/rspotify/blob/22a995a061dffbce9f5069fd603e266d7ed3a252/examples/album.rs)                                                           | [`examples/client_creds.rs`](https://github.com/ramsayleung/rspotify/blob/master/examples/client_creds.rs)           |
| [Async] webapp with Rocket                   | [`examples/webapp`](https://github.com/ramsayleung/rspotify/tree/4c1c3366630a8b2b37668a17878b746108c93fd0/examples/webapp)                                                               | [`examples/webapp`](https://github.com/ramsayleung/rspotify/tree/master/examples/webapp)                             |

More in the [`examples` directory](https://github.com/ramsayleung/rspotify/tree/master/examples)!

### Full changelog

- Rewritten documentation in hopes that it's easier to get started with RSpotify.
- Reduced the number of examples. Instead of having an example for each endpoint, which is repetitive and unhelpful for newcomers, some real-life examples are now included. If you'd like to add your own example, please do! ([#113](https://github.com/ramsayleung/rspotify/pull/113))
- RSpotify now uses macros internally to make the endpoints as concise as possible and nice to read.
- Add `add_item_to_queue` endpoint.
- Add `category_playlists` endpoint ([#153](https://github.com/ramsayleung/rspotify/pull/153)).
- Add `resume_playback` endpoint.
- Fix race condition when using a single client from multiple threads ([#114](https://github.com/ramsayleung/rspotify/pull/114)).
- RSpotify should now be considerably lighter and less bloated ([discussion in #108](https://github.com/ramsayleung/rspotify/issues/108)):
  + Remove unused dependencies: `base64`, `env_logger`, `random`, `url`.
  + Remove `itertools` dependency by using the standard library.
  + Remove `rand` in place of `getrandom` to [reduce total dependencies and compile times](https://github.com/ramsayleung/rspotify/issues/108#issuecomment-673587185).
  + Cleanup, reduced repetitive code and boilerplate internally in several places ([#117](https://github.com/ramsayleung/rspotify/pull/117), [#113](https://github.com/ramsayleung/rspotify/pull/113), [#107](https://github.com/ramsayleung/rspotify/pull/107), [#106](https://github.com/ramsayleung/rspotify/pull/106)).
  + Added internal zero-copy type for Spotify ids, reduced number of allocations/clones ([#161](https://github.com/ramsayleung/rspotify/pull/161)).
  + Updated dependencies to the latest versions, integrated Dependabot to keep track of them ([#105](https://github.com/ramsayleung/rspotify/pull/105), [#111](https://github.com/ramsayleung/rspotify/pull/111)).
- ([#145](https://github.com/ramsayleung/rspotify/pull/145)) Mark `SimplifiedEpisode.language` as deprecated.
- ([#145](https://github.com/ramsayleung/rspotify/pull/145)) Derive `PartialEq` and `Eq` for models:
  + `Actions`
  + `AudioAnalysisMeta`
  + `AudioAnalysisSection`
  + `AudioAnalysisSegment`
  + `AudioAnalysisTrack`
  + `AudioAnalysis`
  + `AudioFeaturesPayload`
  + `AudioFeatures`
  + `Category`
  + `Context`
  + `Copyright`
  + `CurrentPlaybackContext`
  + `CurrentlyPlayingContext`
  + `CursorBasedPage`
  + `CursorPageFullArtists`
  + `Cursor`
  + `DevicePayload`
  + `Device`
  + `ExplicitContent`
  + `FeaturedPlaylists`
  + `FullAlbum`
  + `FullArtist`
  + `FullEpisode`
  + `FullPlayingContext`
  + `FullPlaylist`
  + `FullShow`
  + `FullTrack`
  + `Image`
  + `Offset`
  + `PageCategory`
  + `Page`
  + `PlayHistory`
  + `PlayableItem`
  + `PlayingItem`
  + `PlaylistItem`
  + `PlaylistResult`
  + `PrivateUser`
  + `PublicUser`
  + `RecommendationsSeedType`
  + `RecommendationsSeed`
  + `Recommendations`
  + `Restrictions`
  + `ResumePoint`
  + `SavedTrack`
  + `SearchAlbums`
  + `SearchArtists`
  + `SearchEpisodes`
  + `SearchPlaylists`
  + `SearchResult`
  + `SearchShows`
  + `SearchTracks`
  + `SeversalSimplifiedShows`
  + `Show`
  + `SimplifiedAlbum`
  + `SimplifiedArtist`
  + `SimplifiedEpisode`
  + `SimplifiedPlayingContext`
  + `SimplifiedPlaylist`
  + `SimplifiedShow`
  + `SimplifiedTrack`
  + `TrackLink`
  + `TrackRestriction`
  + Fix broken model links refering to Spotify documentation
- ([#188](https://github.com/ramsayleung/rspotify/pull/188)) Replace html links with intra-documentation links
- ([#189](https://github.com/ramsayleung/rspotify/pull/189)) Add `scopes!` macro to generate scopes for `Token` from string literal
- RSpotify has now been split up into independent crates, so that it can be used without the client. See `rspotify-macros` and `rspotify-model`.
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Reexport `model` module to allow user to write `rspotify::model::FullAlbum` instead of  `rspotify::model::album::FullAlbum`.
- ([#246](https://github.com/ramsayleung/rspotify/pull/246)) Add support for PKCE, see `AuthCodePkceSpotify`.
- ([#257](https://github.com/ramsayleung/rspotify/pull/257)) `parse_response_code` now checks that the state is the same in the request and in the callback.

**Breaking changes:**
- ([#202](https://github.com/ramsayleung/rspotify/pull/202)) RSpotify now consistently uses `Option<T>` for optional parameters. Those generic over `Into<Option<T>>` have been changed, which makes calling endpoints a bit ugiler but more consistent and simpler.
- `SpotifyClientCredentials` has been renamed to `Credentials` ([#129](https://github.com/ramsayleung/rspotify/pull/129)), and its members `client_id` and `client_secret` to `id` and `secret`, respectively.
- `TokenInfo` has been renamed to `Token`. It no longer has the `token_type` member, as it's always `Bearer` for now ([#129](https://github.com/ramsayleung/rspotify/pull/129)).
- `SpotifyOAuth` has been renamed to `OAuth`. It only contains the necessary parameters for OAuth authorization instead of repeating the items from `Credentials` and `Spotify`, so `client_id`, `client_secret` and `cache_path` are no longer in `OAuth` ([#129](https://github.com/ramsayleung/rspotify/pull/129)).
- `TokenBuilder` and `OAuthBuilder` will only read from environment variables when `from_env` is used, instead of `default`.
- `dotenv` support is now optional. You can enable it with the `env-file` feature to have the same behavior as before ([#108](https://github.com/ramsayleung/rspotify/issues/108)). It may be used with `from_env` as well.
- Renamed environmental variables to `RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET` and `RSPOTIFY_REDIRECT_URI` to avoid name collisions with other libraries that use OAuth2 ([#118](https://github.com/ramsayleung/rspotify/issues/118)).
- The `blocking` module has been removed, since RSpotify is able to use multiple HTTP clients now. `reqwest` and `ureq` are currently supported, meaning that you can still use blocking code by enabling the `client-ureq` feature and a TLS like `ureq-rustls-tls`. Read the docs for more information ([#129](https://github.com/ramsayleung/rspotify/pull/129)).
- The `Spotify` client has been split up by authorization flows (`ClientCredsSpotify`, `AuthCodeSpotify`, `AuthCodePkceSpotify`), which allows us to remove the builder pattern. The authentication process has been rewritten. ([#216](https://github.com/ramsayleung/rspotify/pull/216)).
- Fix typo in `user_playlist_remove_specific_occurrenes_of_tracks`, now it's `user_playlist_remove_specific_occurrences_of_tracks`.
- ([#123](https://github.com/ramsayleung/rspotify/pull/123)) All fallible calls in the client return a `ClientError` rather than using `failure`.
- ([#244](https://github.com/ramsayleung/rspotify/pull/244)) Model objects like `FullTrack` or `AudioFeatures` have had their `_type` and `uri` fields removed. These can be accessed instead with the `id` field: `id._type()` or `id.uri()`.
- ([#244](https://github.com/ramsayleung/rspotify/pull/244)) Endpoints taking `Vec<String>/&[String]` as parameter have changed to `impl IntoIterator<Item = &Id>`.
  + The endpoints which changes parameter from `Vec<String>` to `impl IntoIterator<Item = &Id>`:
    - `albums`
    - `artists`
    - `check_users_saved_shows`
    - `get_several_episodes`
    - `remove_users_saved_shows`
    - `save_shows`
  + The endpoints which changes parameter from `&[String]` to `impl IntoIterator<Item = &Id>`:
    - `audios_features`
    - `current_user_saved_albums_add`
    - `current_user_saved_albums_contains`
    - `current_user_saved_albums_delete`
    - `current_user_saved_tracks_add`
    - `current_user_saved_tracks_contains`
    - `current_user_saved_tracks_delete`
    - `user_artist_check_follow`
    - `user_follow_artists`
    - `user_follow_users`
    - `user_playlist_add_tracks`
    - `user_playlist_remove_all_occurrences_of_tracks`
    - `user_playlist_replace_tracks`
    - `user_unfollow_artists`
    - `user_unfollow_users`
  + The endpoints which changes parameter from `String` to `&Id`:
        - `get_a_show`
        - `get_an_episode`
        - `get_shows_episodes`
  + The endpoint which changes parameter from `Vec<Map<String, Value>>` to `Vec<TrackPositions>`:
        - `playlist_remove_specific_occurrences_of_tracks`
- The `Offset` type is now an enum to match API logic, `Offset::Position` is `u32` now (it's not a position in time, it's a position in a playlist, and you can't have both `position` and `uri` fields at the same time).
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Rename endpoints with more fitting name:
  + `audio_analysis` -> `track_analysis`
  + `audio_features` -> `track_features`
  + `audios_features` -> `tracks_features`
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Split single `senum.rs` file into a separate module named `enums` (which is more appropriate compared with `senum`) with three files `country.rs`, `types.rs`, `misc.rs`, and move `enums` module into `model` module, which should be part of the `model` module, check [enums mod.rs file](src/model/enums/mod.rs) for details.
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Refactor all enum files with `strum`, reduced boilerplate code.
   + All enums don't have a method named `as_str()` anymore, by leveraging `strum`, it's easy to convert strings to enum variants based on their name, with method `as_ref()`.
- Fix typo in `transfer_playback`: `device_id` to `device_ids`.
- ([#249](https://github.com/ramsayleung/rspotify/pull/249)) The `recommendations` endpoint has been made simpler to use; the attributes are now serialized with `RecommendationsAttribute`.
- ([#145](https://github.com/ramsayleung/rspotify/pull/145)) Refactor models to make it easier to use:
  + Changed type of `track` in `PlayHistory` to `FullTrack` ([#139](https://github.com/ramsayleung/rspotify/pull/139)).
  + Rename model `CurrentlyPlaybackContext` to `CurrentPlaybackContext`
  + Change `copyrights` from `Vec<HashMap<String, String>>` to `Vec<Copyright>`
  + Add missing field `is_private_session` for `Device`
  + Change `PublicUser.images` from `Option<Vec<Image>>` to `Vec<Image>`
  + Add three missing fields `is_playable`, `linked_from`, `restrictions` for `SimplifiedTrack`
  + Delete deprecated field `birthday` and Add missing fields `product` and `explicit_content` for `PrivateUser`
  + Rename PlayingTrack to PlayingItem and change `added_at` to Option
  + Replace `Playing` with `CurrentlyPlayingContext`, since it's the same
  + Make `Device.id` and `Device.volume_percent`, since they would be null
  + Rename `Restrictions` to `Restriction` and move it to top level of `model` module
  + Rename `AudioAnalysisMeasure` to `TimeInterval`
  + Replace `start`, `duration`, `confidence` fields from `AudioAnalysisSection` and `AudioAnalysisSegment` to `TimeInterval` field
  + Remove useless `FullPlayingContext`, since it has been replaced with `CurrentPlayingContext`
  + Rename `CUDResult` to `PlaylistResult`, since this original name isn't self-explaining
  + Change `{FullArtist, FullPlaylist, PublicUser, PrivateUser}::followers` from `HashMap<String, Option<Value>>` to struct `Followers`
  + Replace `Actions::disallows` with a `Vec<DisallowKey>` by removing all entires whose value is false, which will result in a simpler API
  + Replace `{FullAlbum, SimplifiedEpisode, FullEpisode}::release_date_precision` from `String` to `DatePrecision` enum, makes it easier to use.
  + Id and URI parameters are type-safe now everywhere thanks to the `Id` trait and its implementations.
- ([#157](https://github.com/ramsayleung/rspotify/pull/157)) Keep polishing models to make it easier to use:
  + Constrain visibility of `FullArtists` struct with `pub (in crate)`, make `artists` and `artist_related_artists` endpoints return a `Vec<FullArtist>` instead.
  + Constrain visibility of `FullTracks` struct with `pub (in crate)`, make `tracks` and `artist_top_tracks` endpoints return a `Vec<FullTrack>` instead.
  + Constrain visibility of `AudioFeaturesPayload` struct with `pub (in crate)`, make `tracks_features` endpoints return a `Vec<AudioFeatures>` instead.
  + Constrain visibility of `FullAlbums` struct with `pub (in crate)`, make `albums` endpoints return a `Vec<FullAlbum>` instead.
  + Constrain visibility of `PageSimpliedAlbums` struct with `pub (in crate)`, make `new_releases` endpoints return a `Page<SimplifiedAlbum>` instead.
  + Constrain visibility of `CursorPageFullArtists` struct with `pub (in crate)`, make `current_user_followed_artists` endpoints return a `CursorBasedPage<FullArtist>` instead.
  + Constrain visibility of `PageCategory` struct with `pub (in crate)`, make `categories` endpoints return a `Page<Category>` instead.
  + Constrain visibility of `DevicePayload` struct with `pub (in crate)`, make `device` endpoints return a `Vec<Device>` instead.
  + Constrain visibility of `SeversalSimplifiedShows` struct with `pub (in crate)`, make `get_several_shows` endpoints return a `Vec<SimplifiedShow>` instead.
  + Constrain visibility of `SeversalEpisodes` struct with `pub (in crate)`, make `get_several_episodes` endpoints return a `Vec<FullEpisode>` instead.
  + Rename `AudioFeatures.duration_ms` to `duration`, and change its type from `u32` to `std::time::Duration`.
  + Rename `FullEpisode.duration_ms` to `duration`, and change its type from `u32` to `std::time::Duration`.
  + Rename `SimplifiedEpisode.duration_ms` to `duration`, and change its type from `u32` to `std::time::Duration`.
  + Rename `FullTrack.duration_ms` to `duration`, and change its type from `u32` to `std::time::Duration`.
  + Rename `SimplifiedTrack.duration_ms` to `duration`, and change its type from `u32` to `std::time::Duration`.
  + Rename `ResumePoint.resume_position_ms` to `resume_position`, and change its type from `u32` to `std::time::Duration`.
  + Rename `CurrentlyPlayingContext.progress_ms` to `progress`, and change its type from `Option<u32>` to `Option<std::time::Duration>`.
  + Rename `CurrentPlaybackContext.progress_ms` to `progress`, and change its type from `Option<u32>` to `Option<std::time::Duration>`.
  + Change `CurrentlyPlayingContext.timestamp`'s type from `u64` to `chrono::DateTime<Utc>`.
  + Change `CurrentPlaybackContext.timestamp`'s type from `u64` to `chrono::DateTime<Utc>`.
  + Change `Offset.position`'s type from `Option<u32>` to `Option<std::time::Duration>`
  + Remove `SimplifiedPlayingContext`, since it's useless.
- ([#177](https://github.com/ramsayleung/rspotify/pull/157)) Change `mode` from `f32` to `enum Modality`:
  + Change `AudioAnalysisSection::mode`, `AudioAnalysisTrack::mode` and `AudioFeatures::mode` from `f32` to `enum Modality`.
- ([#185](https://github.com/ramsayleung/rspotify/pull/185)) Polish the `Token.expires_at`, `Token.expires_in` fields
  + Change `Token.expires_in` from u32 to `chrono::Duration`
  + Change `Token.expires_at` from i64 to `chrono::DateTime<Utc>`
  + Change `Token.scopes` from `String` to `HashSet`.
  + Change `OAuth.scopes` from `String` to `HashSet`.
  + Change `SimplifiedPlaylist::tracks` from `HashMap` to `PlaylistTracksRef`
- ([#194](https://github.com/ramsayleung/rspotify/pull/194)) Rename `PlayingItem` to `PlayableItem`, `PlaylistItem::track` type changed to `Option<PlayableItem>`, so playlists can contain episodes as well
- ([#197](https://github.com/ramsayleung/rspotify/pull/197)) Making acronyms lowercase:
  + Rename `ClientError::ParseJSON` to `ClientError::ParseJson`
  + Rename `ClientError::ParseURL` to `ClientError::ParseUrl`
  + Rename `ClientError::IO` to `ClientError::Io`
  + Rename `ClientError::CLI` to `ClientError::Cli`
  + Rename `BaseHTTPClient` to `BaseHttpClient`
- ([#166](https://github.com/ramsayleung/rspotify/pull/166) [#201](https://github.com/ramsayleung/rspotify/pull/201)) Add automatic pagination, which is now enabled by default. You can still use the methods with the `_manual` suffix to have access to manual pagination. There are three new examples for this, check out `examples/pagination*` to learn more!

  As a side effect, some methods now take references instead of values (so that they can be used multiple times when querying), and the parameters have been reordered so that the `limit` and `offset` are consistently the last two.

  The pagination chunk size can be configured with the `Spotify::pagination_chunks` field, which is set to 50 items by default.
- No default values are set from RSpotify now, they will be left to the Spotify API.
- ([#202](https://github.com/ramsayleung/rspotify/pull/202)) Add a `collaborative` parameter to `user_playlist_create`.
- ([#202](https://github.com/ramsayleung/rspotify/pull/202)) Add a `uris` parameter to `playlist_reorder_tracks`.
- ([#206](https://github.com/ramsayleung/rspotify/pull/206)) Update the endpoint signatures to pass parameters by reference, affected endpoint list:
    + `tracks`
    + `artist_albums`
    + `artist_albums_manual`
    + `artist_top_tracks`
    + `search`
    + `playlist`
    + `playlist_remove_specific_occurences_of_tracks`
    + `featured_playlists`
    + `recommendations`
    + `current_playlist`
    + `repeat`
    + `get_seversal_shows`
    + `get_an_episode`
    + `get_several_episodes`
    + `remove_users_saved_shows`
- ([#261](https://github.com/ramsayleung/rspotify/pull/261/files)) `PageSimpliedAlbums` has been renamed to `PageSimplifiedAlbums`
- ([#261](https://github.com/ramsayleung/rspotify/pull/261/files)) `Default` has been implemented for all the possible models
- ([#257](https://github.com/ramsayleung/rspotify/pull/257)) Fix naming for most playlist-related endpoints. They used to work only for tracks, but they've been extended to episodes as well, so we call the contents of a playlist "items" instead of "tracks".
    + `playlist_add_tracks` is now `playlist_add_items`
    + `playlist_tracks` is now `playlist_items`
    + `playlist_tracks_manual` is now `playlist_items_manual`
    + `playlist_replace_tracks` is now `playlist_replace_items`
    + `playlist_reorder_tracks` is now `playlist_reorder_items`
    + `playlist_remove_all_occurrences_of_tracks` is now `playlist_remove_all_occurrences_of_items`
    + `playlist_remove_specific_occurrences_of_tracks` is now `playlist_remove_specific_occurrences_of_items`
    + `model::TrackPositions` is now `model::ItemPositions`
    + `current_user_playing_track` is now `current_user_playing_item`
- ([#260](https://github.com/ramsayleung/rspotify/pull/260)) The `current_user_saved_albums` and `current_user_saved_tracks` now have a `market` parameter
- ([#256](https://github.com/ramsayleung/rspotify/pull/256), [#198](https://github.com/ramsayleung/rspotify/pull/198)) Added missing `before` and `after` parameters from `current_user_recently_played`

## 0.10 (2020/07/01)

- Add `get_access_token_without_cache` and `refresh_access_token_without_cache` to get and refresh access token without caching it.
- Add `cross compile` support.
- Add `podcast` support:
  + add `save_shows` endpoint.
  + add `get_saved_shows` endpoint.
  + add `get_a_show` endpoint.
  + add `get_several_shows` endpoint.
  + add `get_shows_episodes` endpoint.
  + add `get_an_episode` endpoint.
  + add `get_several_episodes` endpoint.
  + add `check_users_saved_shows` endpoint.
  + add `remove_users_saved_shows` endpoint

  **Breaking Change**
  + update the `current_playing` endpoint, add a new parameter named `additional_types`, and add some new fields for return object, change the return object type from `SimplifiedPlayingContext` to `CurrentlyPlaybackContext.`
  + update the `current_playback` endpoint, add a new parameter named `current_playback`, and add some new fields for return object, change the return object type from `FullPlayingContext` to `CurrentlyPlaybackContext`.
  + update the `search` endpoint, which adds support of podcast shows and spisodes, add a new parameter named `include_external`, change `search` function from private to public.
  + remove `search_album`, `search_artist`, `search_playlist`, and `search_track`, now there is only search method left, it's the `search` endpoint.

## 0.9 (2020/02/28)

- Adds `async/await` support.
- Keeps the previous synchronous API, enabled by extra feature `blocking`, disabled by default.
- Shorten the import path.
- Add missing `Show` and `Episode` types.

## 0.8 (2020/01/30)

- Provide more informational error strings for API.
- Fix tuneable attribute passing for recommendations.
- Support system proxy setting.
- Add two endpoint functions: `current_user_saved_albums_contains`, `user_artist_check_follow`.
- (Breaking change)Change `AlbumType::from_str`, `SearchType::from_str`, `RepeatState::from_str`, `Country::from_str`, `TimeRange::from_str`, `Type::from_str`, `AlbumType::from_str`, implement `FromStr` trait for all of them.

## 0.7 (2019/10/11)

- Code optimize, remove a unnecessary mut and add a missing reference
- Fix reqwest breaking change
- Add missing devices type
- Add `position_ms` to `start_playback`

## 0.6 (2019/07/22)

- Make PrivateUser country optional

## 0.5 (2019/04/30)

- Replace println! with log!
- FIx errors when email or birthdate are missing
- Fix de-serialization panics for null values
- Implement unsaving albums and unfollowing users/artists

## 0.4 (2019/03/23)

- Allow application to perform error handling

## 0.3 (2019/02/20)

- update dependencies to fix issus on Windows
- fix failed test

## 0.2.6 (2018/12/20)

- Hide warning on successful authentication
- Remove unneeded extern crate from  examples/new_releases.rs
- Changes to Spotify.user_playlist and Spotify.playlist methods
- add new field `Unknown` for `DeviceType` enum

## 0.2.5 (2018/10/05)

- update reqwest to 0.9

## 0.2.4 (2018/08/19)

- add debug and clone derives to spotify client and credentials
- Change state field for authorization URL to have default
- Fix show_dialog field to be checked before setting true
_ Fix typo in show_dialog

## 0.1.0 (2018/02/20)

- rspotify first release, Cheers!
