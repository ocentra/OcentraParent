use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub(super) const IO_POLL_SLICE: Duration = Duration::from_millis(50);

pub(super) fn poll_timeout(
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<Duration> {
    if unavailable(deadline, cancellation) {
        return None;
    }
    Some(
        deadline
            .saturating_duration_since(Instant::now())
            .min(IO_POLL_SLICE),
    )
}

pub(super) fn unavailable(deadline: Instant, cancellation: Option<&AtomicBool>) -> bool {
    cancellation.is_some_and(|value| value.load(Ordering::Acquire)) || Instant::now() >= deadline
}

pub(super) fn retryable(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::Interrupted
            | std::io::ErrorKind::WouldBlock
            | std::io::ErrorKind::TimedOut
    )
}
