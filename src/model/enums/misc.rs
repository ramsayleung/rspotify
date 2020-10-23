use crate::model::EnumError;
use crate::model::ErrorKind;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumString};

/// disallow: interrupting_playback, pausing, resuming, seeking, skipping_next, skipping_prev, toggling_repeat_context, toggling_shuffle, toggling_repeat_track, transferring_playback
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash, EnumString, AsRefStr, Display)]
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

/// time range: long-term, medium-term, short-term
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

///repeat state: track, context or off.
/// - track will repeat the current track.
/// - context will repeat the current context.
/// - off will turn repeat off.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}
impl RepeatState {
    pub fn as_str(&self) -> &str {
        match *self {
            RepeatState::Off => "off",
            RepeatState::Track => "track",
            RepeatState::Context => "context",
        }
    }
}
impl FromStr for RepeatState {
    type Err = EnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(RepeatState::Off),
            "track" => Ok(RepeatState::Track),
            "context" => Ok(RepeatState::Context),
            _ => Err(EnumError::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

/// Type for include_external: audio
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IncludeExternal {
    Audio,
}
impl IncludeExternal {
    pub fn as_str(&self) -> &str {
        match *self {
            IncludeExternal::Audio => "audio",
        }
    }
}
impl FromStr for IncludeExternal {
    type Err = EnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audio" => Ok(IncludeExternal::Audio),
            _ => Err(EnumError::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_repeat_state_from_str() {
        let repeat_state = RepeatState::from_str("off");
        assert_eq!(repeat_state.unwrap(), RepeatState::Off);
        let unknown_state = RepeatState::from_str("not exist enum");
        assert_eq!(unknown_state.is_err(), true);
    }

    #[test]
    fn test_disallow_key(){
        let interrupting_playback = DisallowKey::InterruptingPlayback;
        assert_eq!(interrupting_playback.as_ref(), "interrupting_playback");
        let toggling_shuffle = DisallowKey::from_str("toggling_shuffle");
        assert_eq!(toggling_shuffle.unwrap(), DisallowKey::TogglingShuffle);
        assert_eq!(toggling_shuffle.unwrap().to_string(), "toggling_shuffle".to_string());
    }

    #[test]
    fn test_time_range() {
        let time_range = TimeRange::from_str("long_term");
        assert_eq!(time_range.unwrap(), TimeRange::LongTerm);
        let empty_range = TimeRange::from_str("not exist enum");
        assert!(empty_range.is_err());
        let medium_range = TimeRange::MediumTerm;
        assert_eq!(medium_range.as_ref(), "medium_term");
        // assert_eq!(medium_range.to_string(), "medium_range".to_string());
    }
}
