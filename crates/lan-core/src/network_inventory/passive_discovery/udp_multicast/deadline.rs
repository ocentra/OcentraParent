use std::time::{Duration, Instant};

pub(super) fn remaining_read_timeout_at(
    deadline: Instant,
    now: Instant,
) -> Option<Duration> {
    let remaining = deadline.saturating_duration_since(now);
    if remaining.is_zero() {
        return None;
    }
    #[cfg(windows)]
    if remaining < Duration::from_millis(1) {
        return None;
    }
    Some(remaining)
}
