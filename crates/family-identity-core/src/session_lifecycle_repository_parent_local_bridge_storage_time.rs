#![forbid(unsafe_code)]

//! Trusted-time shape validation for durable parent-local bridge rows.

use super::super::SessionLifecycleRepositoryError;

pub(super) struct ParentLocalBridgeRecordTimeShape {
    pub(super) issued_at_epoch_millis: i64,
    pub(super) expires_at_epoch_millis: i64,
    pub(super) authority_expires_at_epoch_millis: i64,
    pub(super) last_transition_at_epoch_millis: i64,
    pub(super) now_epoch_millis: i64,
    pub(super) clock_skew_millis: i64,
    pub(super) freshness_ttl_millis: i64,
    pub(super) active: bool,
}

pub(super) fn validate_record_time_shape(
    shape: ParentLocalBridgeRecordTimeShape,
) -> Result<(), SessionLifecycleRepositoryError> {
    if shape.clock_skew_millis <= 0 || shape.freshness_ttl_millis <= 0 {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    let future_ceiling = shape
        .now_epoch_millis
        .checked_add(shape.clock_skew_millis)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?;
    let backdate_window = shape
        .freshness_ttl_millis
        .checked_add(shape.clock_skew_millis)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?;
    let oldest_issued_at = shape
        .now_epoch_millis
        .checked_sub(backdate_window)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?;
    let max_expiry = shape
        .issued_at_epoch_millis
        .checked_add(shape.freshness_ttl_millis)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?;
    if shape.issued_at_epoch_millis <= 0
        || shape.expires_at_epoch_millis <= shape.issued_at_epoch_millis
        || shape.expires_at_epoch_millis > shape.authority_expires_at_epoch_millis
        || shape.issued_at_epoch_millis < oldest_issued_at
        || shape.issued_at_epoch_millis > future_ceiling
        || shape.last_transition_at_epoch_millis < oldest_issued_at
        || shape.last_transition_at_epoch_millis > future_ceiling
        || shape.expires_at_epoch_millis > max_expiry
        || shape.last_transition_at_epoch_millis < shape.issued_at_epoch_millis
        || (shape.active && shape.last_transition_at_epoch_millis != shape.issued_at_epoch_millis)
        || (!shape.active && shape.last_transition_at_epoch_millis <= shape.issued_at_epoch_millis)
    {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}
