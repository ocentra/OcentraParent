#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

pub(crate) fn assert_local_time(field: &'static str, value: &str) -> Result<(), EventingError> {
    if value.len() != 5 || !value.is_ascii() || value.as_bytes()[2] != b':' {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    let hour = parse_time_component(field, &value[0..2])?;
    let minute = parse_time_component(field, &value[3..5])?;
    if hour > 23 || minute > 59 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    Ok(())
}

pub(crate) fn assert_utc_timestamp(field: &'static str, value: &str) -> Result<(), EventingError> {
    if value.len() != 20
        || !value.is_ascii()
        || !value.as_bytes()[0..4]
            .iter()
            .all(|byte| byte.is_ascii_digit())
        || value.as_bytes()[4] != b'-'
        || value.as_bytes()[7] != b'-'
        || value.as_bytes()[10] != b'T'
        || value.as_bytes()[13] != b':'
        || value.as_bytes()[16] != b':'
        || value.as_bytes()[19] != b'Z'
    {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    let year = value[0..4]
        .parse::<i32>()
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })?;
    let month = parse_time_component(field, &value[5..7])?;
    let day = parse_time_component(field, &value[8..10])?;
    if !valid_calendar_date(year, month, day) {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    assert_local_time(field, &value[11..16])?;
    let seconds = parse_time_component(field, &value[17..19])?;
    if seconds > 59 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    Ok(())
}

fn valid_calendar_date(year: i32, month: u8, day: u8) -> bool {
    let Some(max_day) = days_in_month(year, month) else {
        return false;
    };
    (1..=max_day).contains(&day)
}

fn days_in_month(year: i32, month: u8) -> Option<u8> {
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return None,
    };
    Some(days)
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn parse_time_component(field: &'static str, value: &str) -> Result<u8, EventingError> {
    value
        .parse::<u8>()
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })
}
