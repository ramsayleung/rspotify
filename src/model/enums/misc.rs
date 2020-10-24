use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

/// Disallow key: interrupting_playback, pausing, resuming, seeking, skipping_next, skipping_prev, toggling_repeat_context, toggling_shuffle, toggling_repeat_track, transferring_playback
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash, EnumString, AsRefStr, Display,
)]
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
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
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
#[derive(
    Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}

/// Type for include_external: audio
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IncludeExternal {
    Audio,
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_include_external() {
        let audio_from_str = IncludeExternal::from_str("audio");
        assert_eq!(audio_from_str.unwrap(), IncludeExternal::Audio);
        let audio = IncludeExternal::Audio;
        assert_eq!("audio", audio.as_ref());
        assert_eq!("audio".to_string(), audio.to_string());
    }
    #[test]
    fn test_repeat_state() {
        let repeat_state = RepeatState::from_str("off");
        assert_eq!(repeat_state.unwrap(), RepeatState::Off);
        let unknown_state = RepeatState::from_str("not exist enum");
        assert!(unknown_state.is_err());
        let context = RepeatState::Context;
        assert_eq!(context.as_ref(), "context");
        assert_eq!(context.to_string(), "context".to_string());
    }

    #[test]
    fn test_disallow_key() {
        let interrupting_playback = DisallowKey::InterruptingPlayback;
        assert_eq!(interrupting_playback.as_ref(), "interrupting_playback");
        let toggling_shuffle = DisallowKey::from_str("toggling_shuffle");
        assert_eq!(toggling_shuffle.unwrap(), DisallowKey::TogglingShuffle);
        assert_eq!(
            toggling_shuffle.unwrap().to_string(),
            "toggling_shuffle".to_string()
        );
    }

    #[test]
    fn test_time_range() {
        let time_range = TimeRange::from_str("long_term");
        assert_eq!(time_range.unwrap(), TimeRange::LongTerm);
        let empty_range = TimeRange::from_str("not exist enum");
        assert!(empty_range.is_err());
        let medium_range = TimeRange::MediumTerm;
        assert_eq!(medium_range.as_ref(), "medium_term");
        assert_eq!(medium_range.to_string(), "medium_term".to_string());
    }
}
