#![forbid(unsafe_code)]

//! Owner-bound claim, acknowledgement, retry, and startup recovery for the
//! parent-local bridge audit outbox.

use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentitySessionId,
};
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeAudience;
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};
use rusqlite::{params, OptionalExtension, Transaction, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::session_lifecycle_custody::audit_delivery::{
    SessionAuditDeliveryAttemptId, SessionAuditEventId,
};
use crate::session_lifecycle_custody::parent_local_bridge_audit::{
    ParentLocalBridgeAuditAction, ParentLocalBridgeAuditDeliveryClaim, ParentLocalBridgeAuditEvent,
    ParentLocalBridgeStartupRecovery,
};

use super::super::{authority, clock, SessionLifecycleRepositoryError};
use super::audit::{self, MAX_MAINTENANCE_ROWS};

const RETRY_BASE_MILLIS: i64 = 1_000;
const RETRY_MAX_MILLIS: i64 = 5 * 60 * 1_000;

impl super::super::SqliteAccountIdentityAuthorityRepository {
    pub fn claim_parent_local_bridge_audit_delivery(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Option<ParentLocalBridgeAuditDeliveryClaim>, SessionLifecycleRepositoryError> {
        let lease_millis = self.session_policy.audit_delivery_lease_millis;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now = clock::trusted_now_in_transaction(&transaction)?;
        let binding = audit_owner_binding(&transaction, current_authority, now)?;
        requeue_expired_claims(&transaction, now)?;
        let Some(event) = read_next_pending(
            &transaction,
            &binding,
            now,
            self.session_policy.clock_skew_millis,
        )?
        else {
            audit::cleanup(&transaction, now)?;
            transaction
                .commit()
                .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
            return Ok(None);
        };
        let attempt_id = SessionAuditDeliveryAttemptId::generate()
            .map_err(|_| SessionLifecycleRepositoryError::EntropyUnavailable)?;
        let lease_expires = now
            .checked_add(lease_millis)
            .ok_or(SessionLifecycleRepositoryError::ClockUnavailable)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_parent_local_bridge_audit_outbox
                    SET delivery_state = 'in-flight', delivery_attempt_id = ?2,
                        delivery_attempt_count = delivery_attempt_count + 1,
                        delivery_claimed_at_epoch_millis = ?3,
                        delivery_lease_expires_at_epoch_millis = ?4
                  WHERE event_id = ?1 AND account_id = ?5 AND household_id = ?6
                    AND delivery_state = 'pending' AND delivery_attempt_id IS NULL
                    AND delivery_claimed_at_epoch_millis IS NULL
                    AND delivery_lease_expires_at_epoch_millis IS NULL
                    AND delivered_at_epoch_millis IS NULL
                    AND next_delivery_at_epoch_millis <= ?3",
                params![
                    event.event_id.as_str(),
                    attempt_id.as_str(),
                    now,
                    lease_expires,
                    binding.account_id.to_string(),
                    binding.household_id.to_string(),
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::DeliveryConflict);
        }
        let attempt_count = read_attempt_count(&transaction, event.event_id.as_str())?;
        audit::cleanup(&transaction, now)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(Some(ParentLocalBridgeAuditDeliveryClaim {
            event,
            attempt_id,
            attempt_count,
            claimed_at_epoch_millis: now,
            lease_expires_at_epoch_millis: lease_expires,
        }))
    }

    pub fn acknowledge_parent_local_bridge_audit_delivery(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
        claim: ParentLocalBridgeAuditDeliveryClaim,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now = clock::trusted_now_in_transaction(&transaction)?;
        let binding = audit_owner_binding(&transaction, current_authority, now)?;
        ensure_claim_owner(&claim, &binding)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_parent_local_bridge_audit_outbox
                    SET delivery_state = 'delivered', delivery_attempt_id = NULL,
                        delivery_claimed_at_epoch_millis = NULL,
                        delivery_lease_expires_at_epoch_millis = NULL,
                        next_delivery_at_epoch_millis = ?5,
                        delivered_at_epoch_millis = ?5
                  WHERE event_id = ?1 AND delivery_attempt_id = ?2
                    AND account_id = ?3 AND household_id = ?4
                    AND delivery_state = 'in-flight'
                    AND delivery_claimed_at_epoch_millis = ?6
                    AND delivery_lease_expires_at_epoch_millis = ?7
                    AND delivery_lease_expires_at_epoch_millis > ?5
                    AND delivered_at_epoch_millis IS NULL",
                params![
                    claim.event.event_id.as_str(),
                    claim.attempt_id().as_str(),
                    binding.account_id.to_string(),
                    binding.household_id.to_string(),
                    now,
                    claim.claimed_at_epoch_millis,
                    claim.lease_expires_at_epoch_millis,
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::DeliveryConflict);
        }
        audit::cleanup(&transaction, now)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }

    pub fn release_parent_local_bridge_audit_delivery(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
        claim: ParentLocalBridgeAuditDeliveryClaim,
    ) -> Result<(), SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now = clock::trusted_now_in_transaction(&transaction)?;
        let binding = audit_owner_binding(&transaction, current_authority, now)?;
        ensure_claim_owner(&claim, &binding)?;
        let next_delivery = now
            .checked_add(retry_delay(claim.attempt_count))
            .ok_or(SessionLifecycleRepositoryError::ClockUnavailable)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_parent_local_bridge_audit_outbox
                    SET delivery_state = 'pending', delivery_attempt_id = NULL,
                        delivery_claimed_at_epoch_millis = NULL,
                        delivery_lease_expires_at_epoch_millis = NULL,
                        next_delivery_at_epoch_millis = ?5
                  WHERE event_id = ?1 AND delivery_attempt_id = ?2
                    AND account_id = ?3 AND household_id = ?4
                    AND delivery_state = 'in-flight'
                    AND delivery_claimed_at_epoch_millis = ?6
                    AND delivery_lease_expires_at_epoch_millis = ?7
                    AND delivered_at_epoch_millis IS NULL",
                params![
                    claim.event.event_id.as_str(),
                    claim.attempt_id().as_str(),
                    binding.account_id.to_string(),
                    binding.household_id.to_string(),
                    next_delivery,
                    claim.claimed_at_epoch_millis,
                    claim.lease_expires_at_epoch_millis,
                ],
            )
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(SessionLifecycleRepositoryError::DeliveryConflict);
        }
        audit::cleanup(&transaction, now)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
    }

    pub fn recover_parent_local_bridge_startup(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<ParentLocalBridgeStartupRecovery, SessionLifecycleRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now = clock::trusted_now_in_transaction(&transaction)?;
        audit_owner_binding(&transaction, current_authority, now)?;
        let requeued = requeue_expired_claims(&transaction, now)?;
        let cleanup = audit::cleanup(&transaction, now)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(ParentLocalBridgeStartupRecovery {
            expired_claims_requeued: requeued,
            terminal_sessions_removed: cleanup.terminal_sessions_removed,
            delivered_audits_removed: cleanup.delivered_audits_removed,
            more_recovery_work: requeued == MAX_MAINTENANCE_ROWS as u64 || cleanup.more_work,
        })
    }
}

fn audit_owner_binding(
    transaction: &Transaction<'_>,
    authority_value: &VerifiedAccountIdentityAuthority,
    now: i64,
) -> Result<
    crate::session_lifecycle_custody::record::SessionAuthorityBinding,
    SessionLifecycleRepositoryError,
> {
    if !super::is_parent_owner(authority_value.role()) {
        return Err(SessionLifecycleRepositoryError::WrongCredentialClass);
    }
    authority::parent_local_bridge_binding_from_verified(transaction, authority_value, now)
}

fn ensure_claim_owner(
    claim: &ParentLocalBridgeAuditDeliveryClaim,
    binding: &crate::session_lifecycle_custody::record::SessionAuthorityBinding,
) -> Result<(), SessionLifecycleRepositoryError> {
    (&claim.event.account_id == &binding.account_id
        && &claim.event.household_id == &binding.household_id)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

fn requeue_expired_claims(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<u64, SessionLifecycleRepositoryError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_parent_local_bridge_audit_outbox
                SET delivery_state = 'pending', delivery_attempt_id = NULL,
                    delivery_claimed_at_epoch_millis = NULL,
                    delivery_lease_expires_at_epoch_millis = NULL,
                    next_delivery_at_epoch_millis = ?1
              WHERE sequence IN (
                  SELECT sequence
                    FROM account_identity_parent_local_bridge_audit_outbox
                   WHERE delivery_state = 'in-flight'
                     AND delivery_lease_expires_at_epoch_millis <= ?1
                     AND delivered_at_epoch_millis IS NULL
                   ORDER BY sequence LIMIT ?2
              )",
            params![now, MAX_MAINTENANCE_ROWS],
        )
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    u64::try_from(changed).map_err(|_| SessionLifecycleRepositoryError::Unavailable)
}

fn read_next_pending(
    transaction: &Transaction<'_>,
    binding: &crate::session_lifecycle_custody::record::SessionAuthorityBinding,
    now: i64,
    clock_skew_millis: i64,
) -> Result<Option<ParentLocalBridgeAuditEvent>, SessionLifecycleRepositoryError> {
    transaction
        .query_row(
            "SELECT event_id, account_id, household_id, member_id, device_id,
                    authority_session_id, audience, bridge_revoke_epoch, action,
                    occurred_at_epoch_millis, retain_until_epoch_millis
               FROM account_identity_parent_local_bridge_audit_outbox
              WHERE account_id = ?1 AND household_id = ?2
                AND delivery_state = 'pending' AND delivery_attempt_id IS NULL
                AND delivery_claimed_at_epoch_millis IS NULL
                AND delivery_lease_expires_at_epoch_millis IS NULL
                AND delivered_at_epoch_millis IS NULL
                AND next_delivery_at_epoch_millis <= ?3
              ORDER BY sequence LIMIT 1",
            params![
                binding.account_id.to_string(),
                binding.household_id.to_string(),
                now
            ],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, String>(8)?,
                    row.get::<_, i64>(9)?,
                    row.get::<_, i64>(10)?,
                ))
            },
        )
        .optional()
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?
        .map(|row| decode_event(row, now, clock_skew_millis))
        .transpose()
}

type StoredAuditEvent = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    i64,
);

fn decode_event(
    row: StoredAuditEvent,
    now: i64,
    clock_skew_millis: i64,
) -> Result<ParentLocalBridgeAuditEvent, SessionLifecycleRepositoryError> {
    let future_ceiling = now
        .checked_add(clock_skew_millis)
        .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?;
    if row.9 <= 0
        || row.9 > future_ceiling
        || row.10
            != row
                .9
                .checked_add(audit::BRIDGE_AUDIT_RETENTION_MILLIS)
                .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?
    {
        return Err(SessionLifecycleRepositoryError::InvalidAuditRecord);
    }
    Ok(ParentLocalBridgeAuditEvent {
        event_id: SessionAuditEventId::parse(row.0)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        account_id: ParentAccountId::parse(row.1)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        household_id: FamilyId::parse(row.2)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        member_id: AccountIdentityMemberId::parse(row.3)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        device_id: AccountIdentityDeviceId::parse(row.4)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        authority_session_id: AccountIdentitySessionId::parse(row.5)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        audience: (row.6 == AccountIdentityParentLocalBridgeAudience::fixed().as_str())
            .then_some(AccountIdentityParentLocalBridgeAudience::fixed())
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        bridge_revoke_epoch: u64::try_from(row.7)
            .ok()
            .filter(|value| *value > 0)
            .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)?,
        action: decode_action(&row.8)?,
        occurred_at_epoch_millis: row.9,
    })
}

fn decode_action(
    value: &str,
) -> Result<ParentLocalBridgeAuditAction, SessionLifecycleRepositoryError> {
    match value {
        "issued" => Ok(ParentLocalBridgeAuditAction::Issued),
        "authenticated" => Ok(ParentLocalBridgeAuditAction::Authenticated),
        "revoked" => Ok(ParentLocalBridgeAuditAction::Revoked),
        "globally-revoked" => Ok(ParentLocalBridgeAuditAction::GloballyRevoked),
        _ => Err(SessionLifecycleRepositoryError::InvalidAuditRecord),
    }
}

fn read_attempt_count(
    transaction: &Transaction<'_>,
    event_id: &str,
) -> Result<u64, SessionLifecycleRepositoryError> {
    let value = transaction
        .query_row(
            "SELECT delivery_attempt_count
               FROM account_identity_parent_local_bridge_audit_outbox
              WHERE event_id = ?1 LIMIT 1",
            [event_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    u64::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)
}

fn retry_delay(attempt_count: u64) -> i64 {
    let shift = u32::try_from(attempt_count.saturating_sub(1).min(8)).unwrap_or(8);
    RETRY_BASE_MILLIS
        .saturating_mul(1_i64.checked_shl(shift).unwrap_or(i64::MAX))
        .min(RETRY_MAX_MILLIS)
}
