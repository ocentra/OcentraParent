#![forbid(unsafe_code)]

//! Redacted durable lifecycle audit for Account-owned parent-local bridges.

use ocentra_schema::account_identity_authority::AccountIdentityProviderSubject;
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeAudience;
use rusqlite::{params, ErrorCode, Transaction};
use sha2::{Digest, Sha256};

use crate::session_lifecycle_custody::audit_delivery::SessionAuditEventId;
use crate::session_lifecycle_custody::record::SessionAuthorityBinding;

use super::super::SessionLifecycleRepositoryError;
use super::StoredParentLocalBridgeSession;

pub(super) const BRIDGE_AUDIT_RETENTION_MILLIS: i64 = 30 * 24 * 60 * 60 * 1_000;
pub(super) const MAX_MAINTENANCE_ROWS: i64 = 256;
const PROVIDER_SUBJECT_DIGEST_DOMAIN: &str = "ocentra-account-parent-local-bridge-audit-subject-v1";

pub(super) struct CleanupProgress {
    pub(super) terminal_sessions_removed: u64,
    pub(super) delivered_audits_removed: u64,
    pub(super) more_work: bool,
}

pub(super) fn insert_session_event(
    transaction: &Transaction<'_>,
    record: &StoredParentLocalBridgeSession,
    action: &'static str,
    occurred_at_epoch_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    insert_binding_event(
        transaction,
        &record.binding,
        record.audience,
        record.bridge_revoke_epoch,
        action,
        occurred_at_epoch_millis,
    )
}

pub(super) fn insert_global_event(
    transaction: &Transaction<'_>,
    binding: &SessionAuthorityBinding,
    bridge_revoke_epoch: u64,
    occurred_at_epoch_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    insert_binding_event(
        transaction,
        binding,
        AccountIdentityParentLocalBridgeAudience::fixed(),
        bridge_revoke_epoch,
        "globally-revoked",
        occurred_at_epoch_millis,
    )
}

fn insert_binding_event(
    transaction: &Transaction<'_>,
    binding: &SessionAuthorityBinding,
    audience: AccountIdentityParentLocalBridgeAudience,
    bridge_revoke_epoch: u64,
    action: &'static str,
    occurred_at_epoch_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    if audience != AccountIdentityParentLocalBridgeAudience::fixed()
        || bridge_revoke_epoch == 0
        || occurred_at_epoch_millis <= 0
    {
        return Err(SessionLifecycleRepositoryError::InvalidAuditRecord);
    }
    let retain_until_epoch_millis = occurred_at_epoch_millis
        .checked_add(BRIDGE_AUDIT_RETENTION_MILLIS)
        .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?;
    let event_id = SessionAuditEventId::generate()
        .map_err(|_| SessionLifecycleRepositoryError::EntropyUnavailable)?;
    let changed = transaction
        .execute(
            "INSERT INTO account_identity_parent_local_bridge_audit_outbox (
                 event_id, account_id, provider, provider_subject_digest,
                 household_id, member_id, device_id, authority_session_id,
                 audience, bridge_revoke_epoch, action,
                 occurred_at_epoch_millis, retain_until_epoch_millis,
                 delivery_state, delivery_attempt_id, delivery_attempt_count,
                 delivery_claimed_at_epoch_millis,
                 delivery_lease_expires_at_epoch_millis,
                 next_delivery_at_epoch_millis, delivered_at_epoch_millis
             ) VALUES (
                 ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12,
                 ?13, 'pending', NULL, 0, NULL, NULL, ?12, NULL
             )",
            params![
                event_id.as_str(),
                binding.account_id.to_string(),
                crate::account_identity_authority_repository::session_lifecycle_repository::labels::provider_label(&binding.provider).0,
                provider_subject_digest(&binding.provider_subject),
                binding.household_id.to_string(),
                binding.member_id.as_str(),
                binding.device_id.as_str(),
                binding.authority_session_id.as_str(),
                audience.as_str(),
                crate::account_identity_authority_repository::session_lifecycle_repository::codec::to_sql_generation(bridge_revoke_epoch)?,
                action,
                occurred_at_epoch_millis,
                retain_until_epoch_millis,
            ],
        )
        .map_err(map_audit_insert_error)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::AuditConflict)
}

pub(in crate::account_identity_authority_repository::session_lifecycle_repository) fn provider_subject_digest(
    provider_subject: &AccountIdentityProviderSubject,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(PROVIDER_SUBJECT_DIGEST_DOMAIN.as_bytes());
    hasher.update([0]);
    hasher.update(provider_subject.as_str().as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Remove only terminal session rows and delivered audit rows in fixed-size
/// batches. Pending and in-flight evidence is not eligible for deletion.
pub(super) fn cleanup(
    transaction: &Transaction<'_>,
    now_epoch_millis: i64,
) -> Result<CleanupProgress, SessionLifecycleRepositoryError> {
    let terminal_cutoff = now_epoch_millis
        .checked_sub(BRIDGE_AUDIT_RETENTION_MILLIS)
        .ok_or(SessionLifecycleRepositoryError::ClockUnavailable)?;
    let terminal_sessions_removed = transaction
        .execute(
            "DELETE FROM account_identity_parent_local_bridge_session
             WHERE capability_digest IN (
                 SELECT capability_digest
                   FROM account_identity_parent_local_bridge_session
                  WHERE expires_at_epoch_millis <= ?1
                     OR (state IN ('consumed','revoked')
                         AND last_transition_at_epoch_millis <= ?2)
                  ORDER BY last_transition_at_epoch_millis, capability_digest
                  LIMIT ?3
             )",
            params![now_epoch_millis, terminal_cutoff, MAX_MAINTENANCE_ROWS],
        )
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    let delivered_audits_removed = transaction
        .execute(
            "DELETE FROM account_identity_parent_local_bridge_audit_outbox
             WHERE sequence IN (
                 SELECT sequence
                   FROM account_identity_parent_local_bridge_audit_outbox
                  WHERE delivery_state = 'delivered'
                    AND retain_until_epoch_millis <= ?1
                  ORDER BY sequence
                  LIMIT ?2
             )",
            params![now_epoch_millis, MAX_MAINTENANCE_ROWS],
        )
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    Ok(CleanupProgress {
        terminal_sessions_removed: u64::try_from(terminal_sessions_removed)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?,
        delivered_audits_removed: u64::try_from(delivered_audits_removed)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?,
        more_work: terminal_sessions_removed == MAX_MAINTENANCE_ROWS as usize
            || delivered_audits_removed == MAX_MAINTENANCE_ROWS as usize,
    })
}

fn map_audit_insert_error(error: rusqlite::Error) -> SessionLifecycleRepositoryError {
    match error {
        rusqlite::Error::SqliteFailure(failure, _)
            if failure.code == ErrorCode::ConstraintViolation =>
        {
            SessionLifecycleRepositoryError::AuditConflict
        }
        _ => SessionLifecycleRepositoryError::Unavailable,
    }
}
