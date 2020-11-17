use serde::{Deserialize, Serialize};
use strum::ToString;

/// Disallows object:
/// `interrupting_playback`, `pausing`, `resuming`, `seeking`, `skipping_next`,
/// `skipping_prev`, `toggling_repeat_context`, `toggling_shuffle`, `toggling_repeat_track`, `transferring_playback`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#disallows-object)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash, ToString)]
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

/// Time range: `long-term`, `medium-term`, `short-term`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/personalization/get-users-top-artists-and-tracks/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
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
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}

/// Type for include_external: `audio`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/search/search/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IncludeExternal {
    Audio,
}

/// Date precision: `year`, `month`, `day`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/):
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

/// The reason for the restriction: `market`, `product`, `explicit`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-restriction-object)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RestrictionReason {
    Market,
    Product,
    Explict,
}
