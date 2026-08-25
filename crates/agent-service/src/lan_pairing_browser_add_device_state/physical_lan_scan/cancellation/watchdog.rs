use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

const CANCELLATION_POLL: Duration = Duration::from_millis(10);

pub(super) fn wait(
    deadline: Instant,
    external_cancellation: &AtomicBool,
    scan_cancellation: &AtomicBool,
    finished: &AtomicBool,
) {
    while !finished.load(Ordering::Acquire) {
        if external_cancellation.load(Ordering::Acquire) || Instant::now() >= deadline {
            scan_cancellation.store(true, Ordering::Release);
            return;
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        thread::sleep(CANCELLATION_POLL.min(remaining));
    }
}
