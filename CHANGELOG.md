## 0.11 (unreleased)
This release contains *lots* of breaking changes. These were necessary to continue Rspotify's development, and no more versions like this should happen again. Lots of internal code was rewritten to make Rspotify more flexible, performant and easier to use. Sorry for the inconvenience!

If we missed any change or there's something you'd like to discuss about this version, please open a new issue and let us know.

- Rewritten documentation in hopes that it's easier to get started with Rspotify.
- Reduced the number of examples. Instead of having an example for each endpoint, which is repetitive and unhelpful for newcomers, some real-life examples are now included. If you'd like to add your own example, please do! ([#113](https://github.com/ramsayleung/rspotify/pull/113))
- Add `add_item_to_queue` endpoint.
- Add `category_playlists` endpoint ([#153](https://github.com/ramsayleung/rspotify/pull/153)).
- Fix race condition when using a single client from multiple threads ([#114](https://github.com/ramsayleung/rspotify/pull/114)).
- Rspotify should now be considerably lighter and less bloated ([discussion in #108](https://github.com/ramsayleung/rspotify/issues/108)):
  + Remove unused dependencies: `base64`, `env_logger`, `random`, `url`.
  + Remove `itertools` dependency by using the standard library.
  + Remove `rand` in place of `getrandom` to [reduce total dependencies and compile times](https://github.com/ramsayleung/rspotify/issues/108#issuecomment-673587185).
  + Cleanup, reduced repetitive code and boilerplate internally in several places ([#117](https://github.com/ramsayleung/rspotify/pull/117), [#113](https://github.com/ramsayleung/rspotify/pull/113), [#107](https://github.com/ramsayleung/rspotify/pull/107), [#106](https://github.com/ramsayleung/rspotify/pull/106)).
  + Updated dependencies to the latest versions, integrated Dependabot to keep track of them ([#105](https://github.com/ramsayleung/rspotify/pull/105), [#111](https://github.com/ramsayleung/rspotify/pull/111)).
- ([#145](https://github.com/ramsayleung/rspotify/pull/145)) Mark `SimplifiedEpisode.language` as deprecated.
- ([#145](https://github.com/ramsayleung/rspotify/pull/145)) Derive `PartialEq` and `Eq` for models:
  + `SimplifiedAlbum`
  + `Restrictions`
  + `FullAlbum`
  + `SimplifiedArtist`
  + `FullArtist`
  + `CursorPageFullArtists`
  + `AudioFeatures`
  + `AudioFeaturesPayload`
  + `AudioAnalysis`
  + `AudioAnalysisSection`
  + `AudioAnalysisMeta`
  + `AudioAnalysisSegment`
  + `AudioAnalysisTrack`
  + `Category`
  + `PageCategory`
  + `Context`
  + `FullPlayingContext`
  + `SimplifiedPlayingContext`
  + `CurrentlyPlayingContext`
  + `CurrentPlaybackContext`
  + `Actions`
  + `PlaylistResult`
  + `Device`
  + `DevicePayload`
  + `Image`
  + `PlayingItem` 
  + `Offset`
  + `Page`
  + `CursorBasedPage`
  + `Cursor`
  + `PlayHistory`
  + `SimplifiedPlaylist`
  + `FullPlaylist`
  + `PlaylistItem`
  + `FeaturedPlaylists`
  + `Recommendations`
  + `RecommendationsSeed`
  + `RecommendationsSeedType`
  + `SearchPlaylists`
  + `SearchAlbums`
  + `SearchArtists`
  + `SearchTracks`
  + `SearchShows`
  + `SearchEpisodes`
  + `SearchResult`
  + `Copyright`
  + `SimplifiedShow`
  + `Show`
  + `SeversalSimplifiedShows`
  + `FullShow`
  + `SimplifiedEpisode`
  + `FullEpisode`
  + `SeveralEpisodes`
  + `ResumePoint`
  + `FullTrack`
  + `TrackLink`
  + `SimplifiedTrack`
  + `TrackRestriction`
  + `SavedTrack`
  + `PublicUser`
  + `PrivateUser`
  + `ExplicitContent`
  + Fix broken model links refering to Spotify documentation
- ([#188](https://github.com/ramsayleung/rspotify/pull/188)) Replace html links with intra-documentation links

**Breaking changes:**
- `SpotifyClientCredentials` has been renamed to `Credentials` ([#129](https://github.com/ramsayleung/rspotify/pull/129)), and its members `client_id` and `client_secret` to `id` and `secret`, respectively.
- `TokenInfo` has been renamed to `Token`. It no longer has the `token_type` member, as it's always `Bearer` for now ([#129](https://github.com/ramsayleung/rspotify/pull/129)).
- `SpotifyOAuth` has been renamed to `OAuth`. It only contains the necessary parameters for OAuth authorization instead of repeating the items from `Credentials` and `Spotify`, so `client_id`, `client_secret` and `cache_path` are no longer in `OAuth` ([#129](https://github.com/ramsayleung/rspotify/pull/129)).
- `TokenBuilder` and `OAuthBuilder` will only read from environment variables when `from_env` is used, instead of `default`.
- `dotenv` support is now optional. You can enable it with the `env-file` feature to have the same behavior as before ([#108](https://github.com/ramsayleung/rspotify/issues/108)). It may be used with `from_env` as well.
- Renamed environmental variables to `RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET` and `RSPOTIFY_REDIRECT_URI` to avoid name collisions with other libraries that use OAuth2 ([#118](https://github.com/ramsayleung/rspotify/issues/118)).
- All fallible calls in the client return a `ClientResult` rather than using `failure`, which is equivalent to a `Result<T, ClientError>`.
- `ApiError` is now `APIError` for consistency.
- A real builder pattern is used now. For example, `Token` is constructed now with `TokenBuilder::default().access_token("...").build().unwrap()`. This has been applied to `Spotify`, `OAuth`, `Token` and `Credentials` ([#129](https://github.com/ramsayleung/rspotify/pull/129)).
- The `blocking` module has been removed, since Rspotify is able to use multiple HTTP clients now. `reqwest` and `ureq` are currently supported, meaning that you can still use blocking code by enabling the `client-ureq` feature and a TLS like `ureq-rustls-tls`. Read the docs for more information ([#129](https://github.com/ramsayleung/rspotify/pull/129)).
- The authentication process has been completely rewritten in order to make it more performant and robust. Please read the docs to learn more about how that works now ([#129](https://github.com/ramsayleung/rspotify/pull/129)). These are the main changes:
    + `TokenInfo::get_cached_token` is now `TokenBuilder::from_cache` and `Spotify::read_token_cache` (using the internal cache path). Instead of panicking, the resulting `TokenBuilder` may be empty (and `build` will fail).
    + `Spotify::save_token_info` is now `Token::write_cache` and `Spotify::write_token_cache`. The latter uses the client's set cache path for the write. These functions also now return `ClientResult` instead of panicking.
    + `Spotify::is_token_expired` is now `Token::is_expired`.
    + `SpotifyOAuth2::get_authorize_url` is now `Spotify::get_authorize_url`, and it returns `ClientResult<String>` instead of panicking.
    + `SpotifyOAuth2::refresh_access_token[_without_cache]` are now `Spotify::refresh_user_token[_with_cache]`. It returns `ClientResult<()>`, and the resulting token will be saved internally instead of returned.
    + `SpotifyOAuth2::request_client_token[_without_cache]` are now `Spotify::request_client_token[_with_cache]`. It returns `ClientResult<()>`, and the resulting token will be saved internally instead of returned.
    + `SpotifyOAuth2::get_access_token[_without_cache]` are now `Spotify::request_user_token[_with_cache]`. It returns `ClientResult<()>`, and the resulting token will be saved internally instead of returned.
    + `get_token[_without_cache]` is now `Spotify::prompt_for_user_token[_without_cache]`. It returns `ClientResult<()>`, and the resulting token will be saved internally instead of returned.
- CLI-exclusive functions and  are now optional under the `cli` feature:
    + `Spotify::prompt_for_user_token[_without_cache]`
    + The `ClientError::CLI` variant, for whenever user interaction goes wrong
- Fix typo in `user_playlist_remove_specific_occurrenes_of_tracks`, now it's `user_playlist_remove_specific_occurrences_of_tracks`.
- ([#123](https://github.com/ramsayleung/rspotify/pull/123))All fallible calls in the client return a `ClientError` rather than using `failure`.
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Endpoints take `Vec<String>/&[String]` as parameter have changed to `impl IntoIterator<Item = &str>`, which is backward compatibility.
  + The endpoints which changes parameter from `Vec<String>` to `impl IntoIterator<Item = &str>`:
	- `artists`
	- `albums`
	- `save_shows`
	- `get_several_episodes`
	- `check_users_saved_shows`
	- `remove_users_saved_shows`
  + The endpoints which changes parameter from `&[String]` to `impl IntoIterator<Item = &str>`:
	- `user_playlist_add_tracks`
	- `user_playlist_replace_tracks`
	- `user_playlist_remove_all_occurrences_of_tracks`
	- `current_user_saved_tracks_delete`
	- `current_user_saved_tracks_contains`
	- `current_user_saved_tracks_add`
	- `current_user_saved_albums_add`
	- `current_user_saved_albums_delete`
	- `current_user_saved_albums_contains`
	- `user_follow_artists`
	- `user_unfollow_artists`
	- `user_artist_check_follow`
	- `user_follow_users`
	- `user_unfollow_users`
	- `audios_features`
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Rename endpoints with more fitting name:
  + `audio_analysis` -> `track_analysis`
  + `audio_features` -> `track_features`
  + `audios_features` -> `tracks_features`
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Reexport `model` module to allow user to write `rspotify::model::FullAlbum` instead of  `rspotify::model::album::FullAlbum`.
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Split single `senum.rs` file into a separate module named `enums` (which is more appropriate compared with `senum`) with three files `country.rs`, `types.rs`, `misc.rs`, and move `enums` module into `model` module, which should be part of the `model` module, check [enums mod.rs file](src/model/enums/mod.rs) for details.
- ([#128](https://github.com/ramsayleung/rspotify/pull/128)) Refactor all enum files with `strum`, reduced boilerplate code.
   + All enums don't have a method named `as_str()` anymore, by leveraging `strum`, it's easy to convert strings to enum variants based on their name, with method `to_string()`.
- Fix typo in `transfer_playback`: `device_id` to `device_ids`.
- ([#145](https://github.com/ramsayleung/rspotify/pull/145))Refactor models to make it easier to use:
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
- ([#157](https://github.com/ramsayleung/rspotify/pull/157))Keep polishing models to make it easier to use:
  + Constrain visibility of `FullArtists` struct with `pub (in crate)`, make `artists` and `artist_related_artists` endpoints return a `Vec<FullArtist>` instead.
  + Constrain visibility of `FullTracks` struct with `pub (in crate)`, make `tracks` and `artist_top_tracks` endpoints return a `Vec<FullTrack>` instead.
  + Constrain visibility of `AudioFeaturesPayload` struct with `pub (in crate)`, make `tracks_features` endpoints return a `Vec<AudioFeatures>` instead.
  + Constrain visibility of `FullAlbums` struct with `pub (in crate)`, make `albums` endpoints return a `Vec<FullAlbum>` instead.
  + Constrain visibility of `PageSimpliedAlbums` struct with `pub (in crate)`, make `new_releases` endpoints return a `Page<SimplifiedAlbum>` instead.
  + Constrain visibility of `CursorPageFullArtists` struct with `pub (in crate)`, make `current_user_followed_artists` endpoints return a `CursorBasedPage<FullArtist>` instead.
  + Constrain visibility of `PageCategory` struct with `pub (in crate)`, make `categories` endpoints return a `Page<Category>` instead.
  + Constrain visibility of `DevicePayload` struct with `pub (in crate)`, make `device` endpoints return a `Vec<Device>` instead.
  + Constrain visibility of `SeversalSimplifiedShows` struct with `pub (in crate)`, make `get_several_shows` endpoints return a `Vec<SimplifiedShow>` instead.
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
  + Change `Token.scope` from `String` to `HashSet`.
  + Change `OAuth.scope` from `String` to `HashSet`.
  + Change `SimplifiedPlaylist::tracks` from `HashMap` to `PlaylistTracksRef`

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
