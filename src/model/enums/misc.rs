use serde::{Deserialize, Serialize};
use strum::ToString;

/// [Disallows object](https://developer.spotify.com/documentation/web-api/reference/object-model/#disallows-object):
/// interrupting_playback, pausing, resuming, seeking, skipping_next,
/// skipping_prev, toggling_repeat_context, toggling_shuffle, toggling_repeat_track, transferring_playback
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

/// Time range: long-term, medium-term, short-term
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

/// Repeat state: track, context or off.
/// - track will repeat the current track.
/// - context will repeat the current context.
/// - off will turn repeat off.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}

/// Type for include_external: audio
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IncludeExternal {
    Audio,
}

/// [Date precision](https://developer.spotify.com/documentation/web-api/reference/object-model/):
/// year, month, day.
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

/// [The reason for the restriction](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-restriction-object)
/// Supported values: market, product, explicit
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RestrictionReason {
    Market,
    Product,
    Explict,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_external() {
        let audio = IncludeExternal::Audio;
        assert_eq!("audio".to_string(), audio.to_string());
    }
    #[test]
    fn test_repeat_state() {
        let context = RepeatState::Context;
        assert_eq!(context.to_string(), "context".to_string());
    }

    #[test]
    fn test_disallow_key() {
        let toggling_shuffle = DisallowKey::TogglingShuffle;
        assert_eq!(toggling_shuffle.to_string(), "toggling_shuffle".to_string());
    }

    #[test]
    fn test_time_range() {
        let medium_range = TimeRange::MediumTerm;
        assert_eq!(medium_range.to_string(), "medium_term".to_string());
    }
    #[test]
    fn test_date_precision() {
        let month = DatePrecision::Month;
        assert_eq!(month.to_string(), "month".to_string());
    }
}
