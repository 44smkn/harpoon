use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let n = i64::deserialize(deserializer)?;
    Ok(Some(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(n, 0),
        Utc,
    )))
}
