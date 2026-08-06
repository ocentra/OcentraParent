use crate::parent_presence::{ParentPresenceObservedAt, ParentPresenceTimestampParseFailureReason};
use crate::trust_bootstrap_clock::{days_from_civil, days_in_month};

impl ParentPresenceObservedAt {
    pub fn from_canonical_utc(
        value: &str,
    ) -> Result<Self, ParentPresenceTimestampParseFailureReason> {
        parse_canonical_utc(value)
    }

    pub fn is_after(&self, other: &Self) -> bool {
        self.epoch_millis > other.epoch_millis
    }

    pub fn is_before(&self, other: &Self) -> bool {
        self.epoch_millis < other.epoch_millis
    }
}

fn parse_canonical_utc(
    value: &str,
) -> Result<ParentPresenceObservedAt, ParentPresenceTimestampParseFailureReason> {
    let bytes = value.as_bytes();
    let millisecond = match bytes {
        [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'-', b'0'..=b'9', b'0'..=b'9', b'-', b'0'..=b'9', b'0'..=b'9', b'T', b'0'..=b'9', b'0'..=b'9', b':', b'0'..=b'9', b'0'..=b'9', b':', b'0'..=b'9', b'0'..=b'9', b'Z'] => {
            0
        }
        [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'-', b'0'..=b'9', b'0'..=b'9', b'-', b'0'..=b'9', b'0'..=b'9', b'T', b'0'..=b'9', b'0'..=b'9', b':', b'0'..=b'9', b'0'..=b'9', b':', b'0'..=b'9', b'0'..=b'9', b'.', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'Z'] => {
            parse_slice(value, 20, 23)?
        }
        _ => return Err(classify_timestamp_shape(value)),
    };

    let year = parse_slice(value, 0, 4)?;
    let month = parse_slice(value, 5, 7)?;
    let day = parse_slice(value, 8, 10)?;
    let hour = parse_slice(value, 11, 13)?;
    let minute = parse_slice(value, 14, 16)?;
    let second = parse_slice(value, 17, 19)?;

    if month == 0 || month > 12 {
        return Err(ParentPresenceTimestampParseFailureReason::Malformed);
    }
    if hour > 23 || minute > 59 || second > 59 || millisecond > 999 {
        return Err(ParentPresenceTimestampParseFailureReason::Malformed);
    }

    let max_day = days_in_month(year, month);
    if day == 0 || day > max_day {
        return Err(ParentPresenceTimestampParseFailureReason::Malformed);
    }

    let epoch_millis = days_from_civil(year, month, day) as i128 * 86_400_000
        + hour as i128 * 3_600_000
        + minute as i128 * 60_000
        + second as i128 * 1_000
        + millisecond as i128;

    Ok(ParentPresenceObservedAt {
        epoch_millis,
        canonical: value.to_owned(),
    })
}

fn classify_timestamp_shape(value: &str) -> ParentPresenceTimestampParseFailureReason {
    if value.len() > 19 && matches!(value.as_bytes()[19], b'+' | b'-') {
        ParentPresenceTimestampParseFailureReason::OffsetNotAllowed
    } else if value.ends_with('Z') {
        ParentPresenceTimestampParseFailureReason::NonCanonical
    } else {
        ParentPresenceTimestampParseFailureReason::Malformed
    }
}

fn parse_slice(
    value: &str,
    start: usize,
    end: usize,
) -> Result<u32, ParentPresenceTimestampParseFailureReason> {
    value
        .get(start..end)
        .ok_or(ParentPresenceTimestampParseFailureReason::Malformed)?
        .parse()
        .map_err(|_error| ParentPresenceTimestampParseFailureReason::Malformed)
}
