use super::AuthenticatedDeliveryGrantInstant;

mod date;
mod offset;
mod time;

pub(super) fn parse(value: &str) -> Option<AuthenticatedDeliveryGrantInstant> {
    let bytes = value.as_bytes();
    if !matches!(bytes.get(10), Some(b'T' | b't')) {
        return None;
    }
    let (year, month, day) = date::parse(bytes.get(..10)?)?;
    let (hour, minute, second, nanos, offset) = time::parse(bytes.get(11..)?)?;
    let days = date::days_since_unix_epoch(year, month, day)?;
    let seconds = days
        .checked_mul(86_400)?
        .checked_add(i64::from(hour) * 3_600 + i64::from(minute) * 60 + i64::from(second))?
        .checked_sub(i64::from(offset::parse(offset)?))?;
    Some(AuthenticatedDeliveryGrantInstant(
        i128::from(seconds)
            .checked_mul(1_000_000_000)?
            .checked_add(nanos)?,
    ))
}
