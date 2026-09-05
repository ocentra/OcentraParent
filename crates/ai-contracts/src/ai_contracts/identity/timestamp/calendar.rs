fn leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 if leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 0,
    }
}

fn days_before_year(year: u32) -> i64 {
    let mut days = 0_i64;
    for current in 0..year {
        days += if leap_year(current) { 366 } else { 365 };
    }
    days
}

fn days_before_month(year: u32, month: u32) -> i64 {
    (1..month)
        .map(|current| i64::from(days_in_month(year, current)))
        .sum()
}

pub(super) fn epoch_days(year: u32, month: u32, day: u32) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=days_in_month(year, month)).contains(&day) {
        return None;
    }
    let epoch = days_before_year(1970) + days_before_month(1970, 1);
    Some(days_before_year(year) + days_before_month(year, month) + i64::from(day - 1) - epoch)
}
