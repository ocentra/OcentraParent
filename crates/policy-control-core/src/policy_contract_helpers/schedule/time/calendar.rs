#![forbid(unsafe_code)]

pub(super) fn valid_calendar_date(year: i32, month: u8, day: u8) -> bool {
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
