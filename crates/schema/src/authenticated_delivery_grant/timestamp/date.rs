pub(super) fn parse(value: &[u8]) -> Option<(i32, u8, u8)> {
    if value.len() != 10 || value.get(4) != Some(&b'-') || value.get(7) != Some(&b'-') {
        return None;
    }
    let year = decimal_i32(value.get(..4)?)?;
    let month = decimal_u8(value.get(5..7)?)?;
    let day = decimal_u8(value.get(8..10)?)?;
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
        return None;
    }
    Some((year, month, day))
}

pub(super) fn days_since_unix_epoch(year: i32, month: u8, day: u8) -> Option<i64> {
    let adjusted_year = year - i32::from(month <= 2);
    let era = adjusted_year.div_euclid(400);
    let year_of_era = adjusted_year - era * 400;
    let month_index = i32::from(month) + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month_index + 2) / 5 + i32::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(i64::from(era * 146_097 + day_of_era - 719_468))
}

fn days_in_month(year: i32, month: u8) -> u8 {
    let mut days = [31_u8, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if month == 2 && leap_year(year) {
        days[1] = 29;
    }
    days[usize::from(month - 1)]
}

fn leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn decimal_u8(value: &[u8]) -> Option<u8> {
    if !value.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(value).ok()?.parse().ok()
}

fn decimal_i32(value: &[u8]) -> Option<i32> {
    if !value.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(value).ok()?.parse().ok()
}
