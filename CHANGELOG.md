## 0.11 (unreleased)

- Rewritten documentation in hopes that it's easier to get started with Rspotify.
- Reduced the number of examples. Instead of having an example for each endpoint, which is repetitive and unhelpful for newcomers, some real-life examples are now included. If you'd like to add your own example, please do! ([#113](https://github.com/ramsayleung/rspotify/pull/113))
- Add `add_item_to_queue` endpoint.
- Fix typo in `transfer_playback`: `device_id` to `device_ids`.
- Fix race condition when using a single client from multiple threads (see [#114](https://github.com/ramsayleung/rspotify/pull/114) for more information).
- Rspotify should now be considerably lighter and less bloated ([discussion in #108](https://github.com/ramsayleung/rspotify/issues/108)):
  + Remove unused dependencies: `base64`, `env_logger`, `derive_builder`, `random`, `url`. <!-- NOTE: derive_builder might not be removed after all -->
  + Remove `itertools` dependency by using the standard library.
  + Remove `rand` in place of `getrandom` to [reduce total dependencies and compile times](https://github.com/ramsayleung/rspotify/issues/108#issuecomment-673587185).
  + `webbrowser` and access to functions that use it (`util::get_token`, `util::get_token_without_cache` and `util::request_token`) can be disabled for the non-CLI applications with the `browser` feature. It's still enabled by default due to [its frequent usage](https://github.com/ramsayleung/rspotify/pull/110#issuecomment-674410604).
  + Cleanup, reduced repetitive code and boilerplate internally in several places ([#117](https://github.com/ramsayleung/rspotify/pull/117), [#113](https://github.com/ramsayleung/rspotify/pull/113), [#107](https://github.com/ramsayleung/rspotify/pull/107), [#106](https://github.com/ramsayleung/rspotify/pull/106)).
  + Updated dependencies to the latest versions, integrated Dependabot to keep track of them ([#105](https://github.com/ramsayleung/rspotify/pull/105), [#111](https://github.com/ramsayleung/rspotify/pull/111)).
+ Remove `convert_map_to_str` and `convert_str_to_map` from both `util.rs` and `blocking/util.rs`, replacing them with `reqwest`'s `query` and `Url::Parse`. Remove crate `percent-encoding`
+ Remove `generate_random_string` and `datetime_to_timestamp` from `blocking/util.rs`, using `generate_random_string` and `datetime_to_timestamp` from `util.rs` instead.
  
**Breaking changes:**
- `dotenv` support is now optional. You can enable it with the `env-file` feature to have the same behavior as before ([#108](https://github.com/ramsayleung/rspotify/issues/108)).
- Renamed environmental variables to `RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET` and `RSPOTIFY_REDIRECT_URI` to avoid name collisions with other libraries that use OAuth2 ([#118](https://github.com/ramsayleung/rspotify/issues/118)).
- Fix typo in `user_playlist_remove_specific_occurrenes_of_tracks`, now it's `user_playlist_remove_specific_occurrences_of_tracks`.
- All fallible calls in the client return a `ClientError` rather than using `failure`.
- Endpoints take `Vec<String>/&[String]` as parameter have changed to `impl IntoIterator<Item = &str>`, which is backward compatibility.
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
- Reexport `model` module to allow user to write `rspotify::model::FullAlbum` instead of  `rspotify::model::FullAlbum`.
- Split single `senum.rs` file into a seperate module named `enums` (which is more appropriate compared with `senum`) with three files `country.rs`, `types.rs`, `misc.rs`, check [enums mod.rs file](./src/enums/mod.rs) for details.

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
