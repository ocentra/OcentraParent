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

    let month = parse_time_component(field, &value[5..7])?;
    let day = parse_time_component(field, &value[8..10])?;
    if month == 0 || month > 12 || day == 0 || day > 31 {
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

fn parse_time_component(field: &'static str, value: &str) -> Result<u8, EventingError> {
    value
        .parse::<u8>()
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })
}
