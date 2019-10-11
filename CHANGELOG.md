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
