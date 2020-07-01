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
- Add `position_ms` to `start_playback

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
