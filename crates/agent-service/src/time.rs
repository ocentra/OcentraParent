#[path = "time/epoch.rs"]
mod epoch;
#[path = "time/now.rs"]
mod now;

pub type TimestampText = now::TimestampText;

pub trait FromTimestampText {
    fn from_timestamp_text(value: TimestampText) -> Self;
}

struct ConvertedTimestamp<T>(T);

impl<T> now::FromTimestampText for ConvertedTimestamp<T>
where
    T: FromTimestampText,
{
    fn from_timestamp_text(value: TimestampText) -> Self {
        Self(T::from_timestamp_text(value))
    }
}

impl<T> FromTimestampText for T
where
    T: now::FromTimestampText,
{
    fn from_timestamp_text(value: TimestampText) -> Self {
        now::FromTimestampText::from_timestamp_text(value)
    }
}

pub fn timestamp_now<T>() -> T
where
    T: FromTimestampText,
{
    now::timestamp_now::<ConvertedTimestamp<T>>().0
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
    T::from_timestamp_text(now::TimestampText(epoch::timestamp_from_epoch_seconds(
        epoch_seconds,
    )))
}
