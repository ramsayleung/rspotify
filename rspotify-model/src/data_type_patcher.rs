// Workaround for Spotify API bug which causes dome uint fields
// being return as floats
// TODO: remove this workaround after Spotify fix the [issue](https://github.com/ramsayleung/rspotify/issues/452)

use serde::{Deserialize, Deserializer};

pub fn as_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let float_data: f64 = Deserialize::deserialize(deserializer)?;

    let u32_data = float_data as u32;

    Ok(u32_data)
}

pub fn as_some_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let float_data: Option<f64> = Deserialize::deserialize(deserializer)?;

    match float_data {
        Some(f) => Ok(Some(f as u32)),
        None => Ok(None),
    }
}
