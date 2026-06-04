use chrono::{SecondsFormat, Utc};

pub fn timestamp_now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn timestamp_from_epoch_seconds(epoch_seconds: u64) -> String {
    let epoch_seconds = i64::try_from(epoch_seconds).unwrap_or(i64::MAX);
    chrono::DateTime::from_timestamp(epoch_seconds, 0)
        .unwrap_or_else(Utc::now)
        .to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn timestamp_after_epoch_seconds(epoch_seconds: u64, delta_seconds: u64) -> String {
    timestamp_from_epoch_seconds(epoch_seconds.saturating_add(delta_seconds))
}
