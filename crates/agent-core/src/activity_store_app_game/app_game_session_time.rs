use chrono::{DateTime, Duration, Utc};

pub(super) fn timestamp_ms(value: &str) -> Option<i64> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc).timestamp_millis())
}

pub(super) fn rollup_date(value: &str) -> Option<String> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.date_naive().to_string())
}

pub(super) fn add_millis(value: &str, millis: i64) -> Option<String> {
    let timestamp = DateTime::parse_from_rfc3339(value).ok()?;
    Some(
        (timestamp.with_timezone(&Utc) + Duration::milliseconds(millis))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    )
}
