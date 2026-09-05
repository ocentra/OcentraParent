pub struct EpochTimestampText(String);

pub trait FromEpochTimestampText {
    fn from_epoch_timestamp_text(value: EpochTimestampText) -> Self;
}

impl<T> FromEpochTimestampText for T
where
    T: From<String>,
{
    fn from_epoch_timestamp_text(value: EpochTimestampText) -> Self {
        T::from(value.0)
    }
}

pub fn timestamp_from_epoch_seconds<T>(epoch_seconds: u64) -> T
where
    T: FromEpochTimestampText,
{
    let epoch_seconds = i64::try_from(epoch_seconds).unwrap_or(i64::MAX);
    T::from_epoch_timestamp_text(EpochTimestampText(
        chrono::DateTime::from_timestamp(epoch_seconds, 0)
            .unwrap_or_else(chrono::Utc::now)
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
    ))
}
