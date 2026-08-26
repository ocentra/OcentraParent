#![forbid(unsafe_code)]

use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Transaction;

use super::SessionLifecycleRepositoryError;

pub(crate) fn trusted_now_epoch_millis() -> Result<i64, SessionLifecycleRepositoryError> {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SessionLifecycleRepositoryError::ClockUnavailable)?;
    i64::try_from(elapsed.as_millis())
        .map_err(|_| SessionLifecycleRepositoryError::ClockUnavailable)
}

/// Read and advance the Account-owned durable clock inside the operation
/// transaction. Bridge rows must not be judged against an unpersisted wall
/// clock: rollback or an implausible forward jump would otherwise make old
/// rows appear current (or evict current rows) after restart.
pub(crate) fn trusted_now_in_transaction(
    transaction: &Transaction<'_>,
) -> Result<i64, SessionLifecycleRepositoryError> {
    crate::account_identity_authority_repository::invite_recovery_repository::authority::
        trusted_now_in_transaction(transaction)
        .map(|(now, _)| now)
        .map_err(|error| match error {
            crate::account_identity_authority_repository::invite_recovery_repository::
                InviteRecoveryRepositoryError::Unavailable =>
            {
                SessionLifecycleRepositoryError::Unavailable
            }
            crate::account_identity_authority_repository::invite_recovery_repository::
                InviteRecoveryRepositoryError::ClockUnavailable =>
            {
                SessionLifecycleRepositoryError::ClockUnavailable
            }
            _ => SessionLifecycleRepositoryError::ClockUnavailable,
        })
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
