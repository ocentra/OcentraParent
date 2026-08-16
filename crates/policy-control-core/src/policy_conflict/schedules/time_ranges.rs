#![forbid(unsafe_code)]

use crate::policy_source::PolicyScheduleWindow;

pub(super) fn normalized_time_ranges(schedule: &PolicyScheduleWindow) -> Vec<(u16, u16)> {
    let start = parse_clock_minutes(&schedule.starts_at).unwrap_or(0);
    let end = parse_clock_minutes(&schedule.ends_at).unwrap_or(start);

    match start.cmp(&end) {
        std::cmp::Ordering::Equal => vec![(0, 24 * 60)],
        std::cmp::Ordering::Less => vec![(start, end)],
        std::cmp::Ordering::Greater => vec![(start, 24 * 60), (0, end)],
    }
}

pub(super) fn parse_clock_minutes(value: &str) -> Option<u16> {
    let (hours, minutes) = value.split_once(':')?;
    let hours = parse_clock_component(hours, 23)?;
    let minutes = parse_clock_component(minutes, 59)?;
    Some(hours * 60 + minutes)
}

fn parse_clock_component(value: &str, max_value: u16) -> Option<u16> {
    let parsed = value.parse::<u16>().ok()?;
    (parsed <= max_value).then_some(parsed)
}
