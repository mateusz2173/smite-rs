use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_non_empty_string<'de, D, T>(data: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
{
    if let Some(val) = Option::<String>::deserialize(data)? {
        let trimmed = val.trim();
        if val.trim().is_empty() || trimmed == "\"\"" {
            return Ok(None);
        }

        T::from_str(&val)
            .map(Some)
            .map_err(|_| serde::de::Error::custom(format!("Unable to parse value: {val:?}")))
            .map_err(From::from)
    } else {
        Ok(None)
    }
}

pub(crate) fn timestamp_from_string<'de, D>(data: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = String::deserialize(data)?;
    let timestamp = format!("{timestamp} +0000");

    DateTime::parse_from_str(&timestamp, "%-m/%d/%Y %I:%M:%S %p %z")
        .map(|datetime| datetime.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}
