use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use serde::Deserialize;

use super::AiTimestamp;

mod calendar;
mod format;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct AiUtcInstant {
    seconds: i64,
    nanos: u32,
}

fn parse_canonical_utc(value: &str) -> Option<AiUtcInstant> {
    let bytes = value.as_bytes();
    if bytes.len() < 20
        || bytes.get(4) != Some(&b'-')
        || bytes.get(7) != Some(&b'-')
        || bytes.get(10) != Some(&b'T')
        || bytes.get(13) != Some(&b':')
        || bytes.get(16) != Some(&b':')
    {
        return None;
    }
    let year = format::ascii_digits(&bytes[0..4])?;
    let month = format::ascii_digits(&bytes[5..7])?;
    let day = format::ascii_digits(&bytes[8..10])?;
    let hour = format::ascii_digits(&bytes[11..13])?;
    let minute = format::ascii_digits(&bytes[14..16])?;
    let second = format::ascii_digits(&bytes[17..19])?;
    if hour >= 24 || minute >= 60 || second >= 60 {
        return None;
    }
    let (nanos, suffix) = match bytes.get(19) {
        Some(b'Z') if bytes.len() == 20 => (0, 20),
        Some(b'.') => {
            let z = bytes.iter().position(|byte| *byte == b'Z')?;
            if z != bytes.len() - 1 {
                return None;
            }
            (format::fraction_nanos(&bytes[20..z])?, z + 1)
        }
        _ => return None,
    };
    if suffix != bytes.len() {
        return None;
    }
    let days = calendar::epoch_days(year, month, day)?;
    Some(AiUtcInstant {
        seconds: days * 86_400
            + i64::from(hour) * 3_600
            + i64::from(minute) * 60
            + i64::from(second),
        nanos,
    })
}

impl AiTimestamp {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        parse_canonical_utc(&value)?;
        Some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn is_well_formed(&self) -> bool {
        parse_canonical_utc(self.as_str()).is_some()
    }

    pub(crate) fn precedes(&self, other: &Self) -> bool {
        self.compare(other) == Some(Ordering::Less)
    }

    fn compare(&self, other: &Self) -> Option<Ordering> {
        Some(parse_canonical_utc(self.as_str())?.cmp(&parse_canonical_utc(other.as_str())?))
    }
}

impl<'de> Deserialize<'de> for AiTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value)
            .ok_or_else(|| serde::de::Error::custom("AI timestamp must be a canonical UTC instant"))
    }
}

impl Display for AiTimestamp {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
