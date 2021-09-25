use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::Country;

/// Disallows object: `interrupting_playback`, `pausing`, `resuming`, `seeking`,
/// `skipping_next`, `skipping_prev`, `toggling_repeat_context`,
/// `toggling_shuffle`, `toggling_repeat_track`, `transferring_playback`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-disallowsobject)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DisallowKey {
    InterruptingPlayback,
    Pausing,
    Resuming,
    Seeking,
    SkippingNext,
    SkippingPrev,
    TogglingRepeatContext,
    TogglingShuffle,
    TogglingRepeatTrack,
    TransferringPlayback,
}

/// Time range: `long-term`, `medium-term`, `short-term`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/personalization/get-users-top-artists-and-tracks/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

/// Repeat state: `track`, `context` or `off`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/set-repeat-mode-on-users-playback/)
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}

/// Type for include_external: `audio`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-search)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IncludeExternal {
    Audio,
}

/// Date precision: `year`, `month`, `day`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/):
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

/// The reason for the restriction: `market`, `product`, `explicit`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RestrictionReason {
    Market,
    Product,
    Explict,
}

/// Indicates the modality (major or minor) of a track.
///
/// This field will contain a 0 for `minor`, a 1 for `major` or a -1 for `no
/// result`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#objects-index)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, AsRefStr)]
pub enum Modality {
    Minor = 0,
    Major = 1,
    NoResult = -1,
}

/// Limit the response to a particular market
///
/// FromToken is the same thing as setting the market parameter to the user's country.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Market {
    Country(Country),
    FromToken,
}

impl AsRef<str> for Market {
    fn as_ref(&self) -> &str {
        match self {
            Market::Country(country) => country.as_ref(),
            Market::FromToken => "from_token",
        }
    }
}

/// Time limits in miliseconds (unix timestamps)
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-recently-played)
#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub enum TimeLimits {
    Before(i64),
    After(i64),
}
