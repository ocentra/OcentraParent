use chrono::{SecondsFormat, Utc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TimestampText(String);

pub trait FromTimestampText {
    fn from_timestamp_text(value: TimestampText) -> Self;
}

impl<T> FromTimestampText for T
where
    T: From<String>,
{
    fn from_timestamp_text(value: TimestampText) -> Self {
        T::from(value.0)
    }
}

pub fn timestamp_now<T>() -> T
where
    T: FromTimestampText,
{
    T::from_timestamp_text(timestamp_text_now())
}

pub fn timestamp_after_epoch_seconds<T>(epoch_seconds: u64, delta_seconds: u64) -> T
where
    T: FromTimestampText,
{
    timestamp_from_epoch_seconds(epoch_seconds.saturating_add(delta_seconds))
}

pub fn timestamp_from_epoch_seconds<T>(epoch_seconds: u64) -> T
where
    T: FromTimestampText,
{
    T::from_timestamp_text(timestamp_text_from_epoch_seconds(epoch_seconds))
}

fn timestamp_text_now() -> TimestampText {
    TimestampText(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true))
}

fn timestamp_text_from_epoch_seconds(epoch_seconds: u64) -> TimestampText {
    let epoch_seconds = i64::try_from(epoch_seconds).unwrap_or(i64::MAX);
    TimestampText(
        chrono::DateTime::from_timestamp(epoch_seconds, 0)
            .unwrap_or_else(Utc::now)
            .to_rfc3339_opts(SecondsFormat::Millis, true),
    )
}
