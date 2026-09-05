#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimestampText(pub(crate) String);

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

pub fn timestamp_text_now() -> TimestampText {
    TimestampText(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
}
