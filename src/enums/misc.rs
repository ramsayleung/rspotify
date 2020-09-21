use crate::enums::Error;
use crate::enums::ErrorKind;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// disallow: interrupting_playback, pausing, resuming, seeking, skipping_next, skipping_prev, toggling_repeat_context, toggling_shuffle, toggling_repeat_track, transferring_playback
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "snake_case")]
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
impl DisallowKey {
    pub fn as_str(&self) -> &str {
        match *self {
            DisallowKey::InterruptingPlayback => "interrupting_playback",
            DisallowKey::Pausing => "pausing",
            DisallowKey::Resuming => "resuming",
            DisallowKey::Seeking => "seeking",
            DisallowKey::SkippingNext => "skipping_next",
            DisallowKey::SkippingPrev => "skipping_prev",
            DisallowKey::TogglingRepeatContext => "toggling_repeat_context",
            DisallowKey::TogglingShuffle => "toggling_shuffle",
            DisallowKey::TogglingRepeatTrack => "toggling_repeat_track",
            DisallowKey::TransferringPlayback => "transferring_playback",
        }
    }
}
impl FromStr for DisallowKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interrupting_playback" => Ok(DisallowKey::InterruptingPlayback),
            "pausing" => Ok(DisallowKey::Pausing),
            "resuming" => Ok(DisallowKey::Resuming),
            "seeking" => Ok(DisallowKey::Seeking),
            "skipping_next" => Ok(DisallowKey::SkippingNext),
            "skipping_prev" => Ok(DisallowKey::SkippingPrev),
            "toggling_repeat_context" => Ok(DisallowKey::TogglingRepeatContext),
            "toggling_shuffle" => Ok(DisallowKey::TogglingShuffle),
            "toggling_repeat_track" => Ok(DisallowKey::TogglingRepeatTrack),
            "transferring_playback" => Ok(DisallowKey::TransferringPlayback),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

/// time range: long-term, medium-term, short-term
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

impl TimeRange {
    pub fn as_str(&self) -> &str {
        match *self {
            TimeRange::LongTerm => "long_term",
            TimeRange::MediumTerm => "medium_term",
            TimeRange::ShortTerm => "short_term",
        }
    }
}

impl FromStr for TimeRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "long_term" => Ok(TimeRange::LongTerm),
            "medium_term" => Ok(TimeRange::MediumTerm),
            "short_term" => Ok(TimeRange::ShortTerm),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_convert_time_range_from_str() {
    let time_range = TimeRange::from_str("long_term");
    assert_eq!(time_range.unwrap(), TimeRange::LongTerm);
    let empty_range = TimeRange::from_str("not exist enum");
    assert_eq!(empty_range.is_err(), true);
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(RepeatState::Off),
            "track" => Ok(RepeatState::Track),
            "context" => Ok(RepeatState::Context),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_convert_repeat_state_from_str() {
    let repeat_state = RepeatState::from_str("off");
    assert_eq!(repeat_state.unwrap(), RepeatState::Off);
    let unknown_state = RepeatState::from_str("not exist enum");
    assert_eq!(unknown_state.is_err(), true);
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audio" => Ok(IncludeExternal::Audio),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}
