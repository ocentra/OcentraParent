use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parent_presence::ParentPresenceObservedAt;

impl ParentPresenceObservedAt {
    pub fn from_system_time(value: SystemTime) -> Self {
        let epoch_millis = match value.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as i128 * 1_000 + duration.subsec_millis() as i128,
            Err(error) => {
                let duration = error.duration();
                -((duration.as_secs() as i128 * 1_000) + duration.subsec_millis() as i128)
            }
        };
        Self {
            epoch_millis,
            canonical: format_canonical_utc(epoch_millis),
        }
    }
}

impl fmt::Debug for ParentPresenceObservedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParentPresenceObservedAt")
            .field("canonical", &self.canonical)
            .finish()
    }
}

impl fmt::Display for ParentPresenceObservedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical)
    }
}

impl From<ParentPresenceObservedAt> for String {
    fn from(value: ParentPresenceObservedAt) -> Self {
        value.canonical
    }
}

pub(crate) fn days_in_month(year: u32, month: u32) -> u32 {
    const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if month == 2 && is_leap_year(year) {
        29
    } else {
        DAYS[(month - 1) as usize]
    }
}

pub(crate) fn days_from_civil(year: u32, month: u32, day: u32) -> i64 {
    let year = year as i64 - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_adjust = month as i64 + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month_adjust + 2) / 5 + day as i64 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn format_canonical_utc(epoch_millis: i128) -> String {
    let total_seconds = epoch_millis.div_euclid(1_000);
    let milliseconds = epoch_millis.rem_euclid(1_000) as u32;
    let days = total_seconds.div_euclid(86_400);
    let seconds_of_day = total_seconds.rem_euclid(86_400) as u32;
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;

    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{milliseconds:03}Z")
}

fn civil_from_days(days: i128) -> (i32, u32, u32) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + i128::from(month <= 2);

    (year as i32, month as u32, day as u32)
}
