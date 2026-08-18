use rusqlite::{params, OptionalExtension, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::RecoveryId;

use super::authority::{ensure_current_authority, next_transition_at, trusted_now_in_transaction};
use super::recovery_ops::ensure_recovery_proof_current;
use super::security_effect_codes::owner_effect_from_code;
use super::security_entropy::opaque_id;
use super::support_recovery_kind_from_label::recovery_kind_from_label;
use super::support_recovery_kind_label::recovery_kind_label;
use super::{InviteRecoveryRepositoryError, RecoveryCompletion, RecoveryState};

impl SqliteAccountIdentityAuthorityRepository {
    pub fn complete_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &RecoveryId,
    ) -> Result<RecoveryCompletion, InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        let kind = ensure_recovery_proof_current(
            &transaction,
            recovery_id,
            authority.household_id().to_string(),
            now,
        )?;
        ensure_current_authority(&transaction, authority, now)?;
        let transition_at = next_transition_at(&transaction, recovery_id, now)?;
        let row = load_approved_recovery(
            &transaction,
            recovery_id,
            authority.household_id().to_string(),
        )?;
        let stored_kind = recovery_kind_from_label(&row.kind)
            .ok_or(InviteRecoveryRepositoryError::RecoveryRejected)?;
        if stored_kind != kind {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        let owner_effect = owner_effect_from_code(row.effect_code)
            .ok_or(InviteRecoveryRepositoryError::RecoveryRejected)?;
        enqueue_handoff(
            &transaction,
            recovery_id,
            authority,
            &row,
            kind,
            transition_at,
        )?;
        transaction
            .execute(
                "UPDATE account_identity_recovery
                 SET last_transition_at_epoch_millis = ?2
                 WHERE recovery_id = ?1 AND household_id = ?3 AND state = 'approved'",
                params![
                    recovery_id.as_str(),
                    transition_at,
                    authority.household_id().to_string(),
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(RecoveryCompletion {
            state: RecoveryState::Approved,
            handoff_enqueued: true,
            owner_effect,
        })
    }
}

struct ApprovedRecoveryRow {
    account_id: String,
    member_id: String,
    device_id: String,
    kind: String,
    effect_code: i64,
}

fn load_approved_recovery(
    transaction: &rusqlite::Transaction<'_>,
    recovery_id: &RecoveryId,
    household_id: String,
) -> Result<ApprovedRecoveryRow, InviteRecoveryRepositoryError> {
    transaction
        .query_row(
            "SELECT account_id, requester_member_id, requester_device_id, kind,
                    owner_effect_kind
             FROM account_identity_recovery
             WHERE recovery_id = ?1 AND household_id = ?2 AND state = 'approved'
             LIMIT 1",
            params![recovery_id.as_str(), household_id],
            |row| {
                Ok(ApprovedRecoveryRow {
                    account_id: row.get::<_, String>(0)?,
                    member_id: row.get::<_, String>(1)?,
                    device_id: row.get::<_, String>(2)?,
                    kind: row.get::<_, String>(3)?,
                    effect_code: row.get::<_, i64>(4)?,
                })
            },
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?
        .ok_or(InviteRecoveryRepositoryError::RecoveryRejected)
}

fn enqueue_handoff(
    transaction: &rusqlite::Transaction<'_>,
    recovery_id: &RecoveryId,
    authority: &VerifiedAccountIdentityAuthority,
    row: &ApprovedRecoveryRow,
    kind: crate::recovery_lifecycle::RecoveryKind,
    transition_at: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    let handoff_id = opaque_id("handoff-")?;
    let correlation_id = opaque_id("correlation-")?;
    transaction
        .execute(
            "INSERT INTO account_identity_recovery_custody_handoff (
                 handoff_id, correlation_id, recovery_id, household_id, account_id,
                 member_id, device_id, kind, requested_at_epoch_millis, state,
                 attempt_count, active_attempt_id
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'pending', 0, NULL)
             ON CONFLICT(recovery_id) DO NOTHING",
            params![
                handoff_id,
                correlation_id,
                recovery_id.as_str(),
                authority.household_id().to_string(),
                row.account_id.clone(),
                row.member_id.clone(),
                row.device_id.clone(),
                recovery_kind_label(kind),
                transition_at,
            ],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
}
