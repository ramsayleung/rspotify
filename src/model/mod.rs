//! All Spotify API endpoint response object
pub mod album;
pub mod artist;
pub mod audio;
pub mod category;
pub mod context;
pub mod device;
pub mod enums;
pub mod image;
pub mod offset;
pub mod page;
pub mod playing;
pub mod playlist;
pub mod recommend;
pub mod search;
pub mod show;
pub mod track;
pub mod user;
use serde::{Deserialize, Serialize};

pub(in crate) mod duration_ms {
    use serde::{de, Serializer};
    use std::{fmt, time::Duration};

    /// Vistor to help deserialize duration represented as millisecond to
    /// `std::time::Duration`.
    pub(in crate) struct DurationVisitor;
    impl<'de> de::Visitor<'de> for DurationVisitor {
        type Value = Duration;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a milliseconds represents std::time::Duration")
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Duration::from_millis(v))
        }
    }

    /// Deserialize `std::time::Duration` from milliseconds (represented as u64)
    pub(in crate) fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_u64(DurationVisitor)
    }

    /// Serialize `std::time::Duration` to milliseconds (represented as u64)
    pub(in crate) fn serialize<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_u64(x.as_millis() as u64)
    }
}
pub(in crate) mod millisecond_timestamp {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{de, Serializer};
    use std::fmt;

    /// Vistor to help deserialize unix millisecond timestamp to
    /// `chrono::DateTime`.
    struct DateTimeVisitor;

    impl<'de> de::Visitor<'de> for DateTimeVisitor {
        type Value = DateTime<Utc>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "an unix millisecond timestamp represents DataTime<UTC>"
            )
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let second = (v - v % 1000) / 1000;
            let nanosecond = ((v % 1000) * 1000000) as u32;
            // The maximum value of i64 is large enough to hold milliseconds,
            // so it would be safe to convert it i64.
            let dt = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(second as i64, nanosecond),
                Utc,
            );
            Ok(dt)
        }
    }

    /// Deserialize Unix millisecond timestamp to `DateTime<Utc>`
    pub(in crate) fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_u64(DateTimeVisitor)
    }

    /// Serialize DateTime<Utc> to Unix millisecond timestamp
    pub(in crate) fn serialize<S>(x: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(x.timestamp_millis())
    }
}
pub(in crate) mod option_duration_ms {
    use super::duration_ms;
    use serde::{de, Serializer};
    use std::{fmt, time::Duration};

    /// Vistor to help deserialize duration represented as milliseconds to
    /// `Option<std::time::Duration>`
    struct OptionDurationVisitor;

    impl<'de> de::Visitor<'de> for OptionDurationVisitor {
        type Value = Option<Duration>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "a optional milliseconds represents std::time::Duration"
            )
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Ok(Some(
                deserializer.deserialize_u64(duration_ms::DurationVisitor)?,
            ))
        }
    }

    /// Deserialize `Option<std::time::Duration>` from milliseconds
    /// (represented as u64)
    pub(in crate) fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_option(OptionDurationVisitor)
    }

    /// Serialize `Option<std::time::Duration>` to milliseconds (represented as
    /// u64)
    pub(in crate) fn serialize<S>(x: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *x {
            Some(duration) => s.serialize_u64(duration.as_millis() as u64),
            None => s.serialize_none(),
        }
    }
}
/// Deserialize/Serialize `Modality` to integer(0, 1, -1).
pub(in crate) mod modality {
    use super::enums::Modality;
    use serde::{de, Deserialize, Serializer};

    pub fn deserialize<'de, D>(d: D) -> Result<Modality, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v = i8::deserialize(d)?;
        match v {
            0 => Ok(Modality::Minor),
            1 => Ok(Modality::Major),
            -1 => Ok(Modality::NoResult),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Signed(v.into()),
                &"valid value: 0, 1, -1",
            )),
        }
    }

    pub fn serialize<S>(x: &Modality, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match x {
            Modality::Minor => s.serialize_i8(0),
            Modality::Major => s.serialize_i8(1),
            Modality::NoResult => s.serialize_i8(-1),
        }
    }
}

/// Restriction object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub reason: RestrictionReason,
}

/// Followers object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-followersobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

/// A full track object or a full episode object
///
/// + [Reference to full track](https://developer.spotify.com/documentation/web-api/reference/#object-trackobject)
/// + [Reference to full episode](https://developer.spotify.com/documentation/web-api/reference/#object-episodeobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayingItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
}

pub use {
    album::*, artist::*, audio::*, category::*, context::*, device::*, enums::*, image::*,
    offset::*, page::*, playing::*, playlist::*, recommend::*, search::*, show::*, track::*,
    user::*,
};
