use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use super::super::super::SsdpDiscoveryError;

const SSDP_IO_POLL_SLICE: Duration = Duration::from_millis(50);

pub(super) fn poll_timeout(
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<Duration, SsdpDiscoveryError> {
    ensure_active(deadline, cancellation)?;
    let remaining = deadline.saturating_duration_since(Instant::now());
    (!remaining.is_zero())
        .then_some(remaining.min(SSDP_IO_POLL_SLICE))
        .ok_or(SsdpDiscoveryError::Timeout)
}

pub(super) fn ensure_active(
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<(), SsdpDiscoveryError> {
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) || Instant::now() >= deadline
    {
        Err(SsdpDiscoveryError::Timeout)
    } else {
        Ok(())
    }
}

pub(super) fn is_retryable_timeout(error: &io::Error) -> bool {
    matches!(
        error.kind(),
        io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
    )
}
