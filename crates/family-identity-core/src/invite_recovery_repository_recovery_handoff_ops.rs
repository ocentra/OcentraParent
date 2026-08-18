use rusqlite::{params, OptionalExtension, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::RecoveryId;

use super::authority::{ensure_current_authority, timestamp, trusted_now_in_transaction};
use super::recovery_ops::ensure_recovery_proof_current;
use super::security_entropy::opaque_id;
use super::support_recovery_handoff::durable_handoff;
use super::{InviteRecoveryRepositoryError, RecoveryHandoffDeliveryAttempt, HANDOFF_LEASE_MILLIS};

impl SqliteAccountIdentityAuthorityRepository {
    pub fn claim_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Option<RecoveryHandoffDeliveryAttempt>, InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        let lease_expires = now
            .checked_add(HANDOFF_LEASE_MILLIS)
            .ok_or(InviteRecoveryRepositoryError::HandoffConflict)?;
        let Some(row) = load_ready_handoff(&transaction, authority, now)? else {
            transaction
                .commit()
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
            return Ok(None);
        };
        let recovery_id = RecoveryId::parse(row.recovery_id.clone())
            .map_err(InviteRecoveryRepositoryError::InvalidValue)?;
        ensure_recovery_proof_current(
            &transaction,
            &recovery_id,
            authority.household_id().to_string(),
            now,
        )?;
        let attempt_id = claim_handoff(&transaction, &row, authority, lease_expires, now)?;
        let handoff = durable_handoff(
            row.handoff_id,
            row.correlation_id,
            row.recovery_id,
            authority.household_id().to_string(),
            row.account_id,
            row.member_id,
            row.device_id,
            row.kind,
            row.requested_at,
        )?;
        let lease_expires_at = timestamp(lease_expires)?;
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(Some(RecoveryHandoffDeliveryAttempt {
            handoff,
            attempt_id,
            lease_expires_at,
        }))
    }

    pub fn release_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery_custody_handoff
                 SET state = 'pending', lease_expires_at_epoch_millis = NULL,
                     active_attempt_id = NULL
                 WHERE handoff_id = ?1 AND household_id = ?2 AND state = 'in-flight'
                   AND correlation_id = ?3 AND recovery_id = ?4
                   AND account_id = ?5 AND member_id = ?6 AND device_id = ?7
                   AND kind = ?8 AND active_attempt_id = ?9
                   AND lease_expires_at_epoch_millis > ?10",
                params![
                    attempt.handoff.handoff_id(),
                    authority.household_id().to_string(),
                    attempt.handoff.correlation_id(),
                    attempt.handoff.recovery_id().as_str(),
                    attempt.handoff.account_id().to_string(),
                    attempt.handoff.member_id().to_string(),
                    attempt.handoff.device_id().to_string(),
                    super::support_recovery_kind_label::recovery_kind_label(attempt.handoff.kind()),
                    attempt.attempt_id,
                    now,
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
    }
}

struct HandoffRow {
    handoff_id: String,
    correlation_id: String,
    recovery_id: String,
    account_id: String,
    member_id: String,
    device_id: String,
    kind: String,
    requested_at: i64,
}

fn load_ready_handoff(
    transaction: &rusqlite::Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    now: i64,
) -> Result<Option<HandoffRow>, InviteRecoveryRepositoryError> {
    transaction
        .query_row(
            "SELECT h.handoff_id, h.correlation_id, h.recovery_id, h.account_id,
                    h.member_id, h.device_id, h.kind, h.requested_at_epoch_millis
             FROM account_identity_recovery_custody_handoff h
             JOIN account_identity_recovery r ON r.recovery_id = h.recovery_id
             WHERE h.household_id = ?1 AND r.state = 'approved'
               AND (h.state = 'pending'
                OR (h.state = 'in-flight' AND h.lease_expires_at_epoch_millis <= ?2))
             ORDER BY h.requested_at_epoch_millis, h.handoff_id LIMIT 1",
            params![authority.household_id().to_string(), now],
            |row| {
                Ok(HandoffRow {
                    handoff_id: row.get::<_, String>(0)?,
                    correlation_id: row.get::<_, String>(1)?,
                    recovery_id: row.get::<_, String>(2)?,
                    account_id: row.get::<_, String>(3)?,
                    member_id: row.get::<_, String>(4)?,
                    device_id: row.get::<_, String>(5)?,
                    kind: row.get::<_, String>(6)?,
                    requested_at: row.get::<_, i64>(7)?,
                })
            },
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
}

fn claim_handoff(
    transaction: &rusqlite::Transaction<'_>,
    row: &HandoffRow,
    authority: &VerifiedAccountIdentityAuthority,
    lease_expires: i64,
    now: i64,
) -> Result<String, InviteRecoveryRepositoryError> {
    let attempt_id = opaque_id("attempt-")?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_recovery_custody_handoff
             SET state = 'in-flight', lease_expires_at_epoch_millis = ?2,
                 attempt_count = attempt_count + 1, active_attempt_id = ?3
             WHERE handoff_id = ?1 AND household_id = ?4
               AND (state = 'pending'
                 OR (state = 'in-flight' AND lease_expires_at_epoch_millis <= ?5))",
            params![
                row.handoff_id,
                lease_expires,
                attempt_id,
                authority.household_id().to_string(),
                now,
            ],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    if changed != 1 {
        return Err(InviteRecoveryRepositoryError::HandoffConflict);
    }
    Ok(attempt_id)
}
