//! Custom serialization methods used throughout the crate

pub mod duration_ms {
    use chrono::Duration;
    use serde::{de, Serializer};
    use std::convert::TryFrom;
    use std::fmt;

    /// Vistor to help deserialize duration represented as millisecond to
    /// `chrono::Duration`.
    pub struct DurationVisitor;
    impl<'de> de::Visitor<'de> for DurationVisitor {
        type Value = Duration;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a milliseconds represents chrono::Duration")
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Duration::try_milliseconds(v).ok_or_else(|| {
                E::invalid_value(
                    serde::de::Unexpected::Signed(v),
                    &"an invalid duration in milliseconds",
                )
            })
        }

        // JSON deserializer calls visit_u64 for non-negative intgers
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match i64::try_from(v) {
                Ok(val) => Duration::try_milliseconds(val).ok_or_else(|| {
                    E::invalid_value(
                        serde::de::Unexpected::Signed(val),
                        &"a valid duration in
        milliseconds",
                    )
                }),
                Err(_) => Err(E::custom(format!(
                    "Conversion error: u64 to i64 conversion failed for value {}",
                    v
                ))),
            }
        }
    }

    /// Deserialize `chrono::Duration` from milliseconds (represented as i64)
    pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_i64(DurationVisitor)
    }

    /// Serialize `chrono::Duration` to milliseconds (represented as i64)
    pub fn serialize<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(x.num_milliseconds())
    }
}

pub mod option_duration_ms {
    use crate::custom_serde::duration_ms;
    use chrono::Duration;
    use serde::{de, Serializer};
    use std::fmt;

    /// Vistor to help deserialize duration represented as milliseconds to
    /// `Option<chrono::Duration>`
    struct OptionDurationVisitor;

    impl<'de> de::Visitor<'de> for OptionDurationVisitor {
        type Value = Option<Duration>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "a optional milliseconds represents chrono::Duration"
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
                deserializer.deserialize_i64(duration_ms::DurationVisitor)?,
            ))
        }
    }

    /// Deserialize `Option<chrono::Duration>` from milliseconds
    /// (represented as i64)
    pub fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_option(OptionDurationVisitor)
    }

    /// Serialize `Option<chrono::Duration>` to milliseconds (represented as
    /// i64)
    pub fn serialize<S>(x: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *x {
            Some(duration) => s.serialize_i64(duration.num_milliseconds()),
            None => s.serialize_none(),
        }
    }
}

/// Deserialize/Serialize `Modality` to integer(0, 1, -1).
pub mod modality {
    use crate::Modality;
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

pub mod duration_second {
    use chrono::Duration;
    use serde::{de, Deserialize, Serializer};

    /// Deserialize `chrono::Duration` from seconds (represented as u64)
    pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let duration: i64 = Deserialize::deserialize(d)?;
        Duration::try_seconds(duration).ok_or(serde::de::Error::invalid_value(
            serde::de::Unexpected::Signed(duration),
            &"an invalid duration in seconds",
        ))
    }

    /// Serialize `chrono::Duration` to seconds (represented as u64)
    pub fn serialize<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(x.num_seconds())
    }
}

pub mod space_separated_scopes {
    use serde::{de, Deserialize, Serializer};
    use std::collections::HashSet;

    pub fn deserialize<'de, D>(d: D) -> Result<HashSet<String>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let scopes: String = Deserialize::deserialize(d)?;
        Ok(scopes.split_whitespace().map(ToOwned::to_owned).collect())
    }

    pub fn serialize<S>(scopes: &HashSet<String>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let scopes = scopes.clone().into_iter().collect::<Vec<_>>().join(" ");
        s.serialize_str(&scopes)
    }
}
