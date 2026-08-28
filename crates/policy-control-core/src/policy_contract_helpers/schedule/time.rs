#![forbid(unsafe_code)]

use super::PolicyContractValidationResult;

pub(super) fn assert_local_time(
    value: &str,
    field_name: &'static str,
) -> PolicyContractValidationResult {
    if value.len() != 5
        || !value.is_ascii()
        || value.as_bytes()[2] != b':'
        || parse_time_component(&value[0..2]).is_none_or(|hour| hour > 23)
        || parse_time_component(&value[3..5]).is_none_or(|minute| minute > 59)
    {
        return Err(match field_name {
            "localTime" => "localTime must use HH:MM 24-hour local time",
            _ => "policy contract local time must use HH:MM 24-hour local time",
        }
        .into());
    }

    Ok(())
}

pub(super) fn assert_utc_timestamp(
    value: &str,
    field_name: &'static str,
) -> PolicyContractValidationResult {
    let bytes = value.as_bytes();
    if value.len() != 20
        || !value.is_ascii()
        || !bytes[0..4].iter().all(|byte| byte.is_ascii_digit())
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(match field_name {
            "evaluatedAt" => "evaluatedAt must be an ISO-8601 timestamp",
            "reviewedAt" => "reviewedAt must be an ISO-8601 timestamp",
            _ => "policy contract timestamps must be ISO-8601 UTC values",
        }
        .into());
    }

    let year = value[0..4].parse::<i32>().unwrap_or(-1);
    let month = parse_time_component(&value[5..7]).unwrap_or(0);
    let day = parse_time_component(&value[8..10]).unwrap_or(0);
    let seconds = parse_time_component(&value[17..19]).unwrap_or(60);
    if !valid_calendar_date(year, month, day) || seconds > 59 {
        return Err("policy contract timestamps must be ISO-8601 UTC values".into());
    }

    assert_local_time(&value[11..16], "policy timestamp inner local time")
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
    year >= 0 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn parse_time_component(value: &str) -> Option<u8> {
    value.parse::<u8>().ok()
}
