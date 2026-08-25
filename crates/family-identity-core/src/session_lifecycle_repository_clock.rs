#![forbid(unsafe_code)]

use std::time::{SystemTime, UNIX_EPOCH};

use super::SessionLifecycleRepositoryError;

pub(crate) fn trusted_now_epoch_millis() -> Result<i64, SessionLifecycleRepositoryError> {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SessionLifecycleRepositoryError::ClockUnavailable)?;
    i64::try_from(elapsed.as_millis())
        .map_err(|_| SessionLifecycleRepositoryError::ClockUnavailable)
}

pub(crate) fn monotonic_transition_epoch_millis(
    trusted_now: i64,
    last_transition: i64,
) -> Result<i64, SessionLifecycleRepositoryError> {
    let next_after_prior = last_transition
        .checked_add(1)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?;
    Ok(trusted_now.max(next_after_prior))
}
