use ocentra_schema::account_identity_authority::AccountIdentityRole;
use rusqlite::{params, OptionalExtension, TransactionBehavior};

use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::RecoveryId;
use crate::setup_lifecycle::RecoveryKind;

use super::authority::{ensure_current_authority, next_transition_at, trusted_now_in_transaction};
use super::support_recovery_kind_from_label::recovery_kind_from_label;
use super::support_recovery_scope::support_authorization_scope_allows;
use super::support_recovery_scope_from_label::support_scope_from_label;
use super::{InviteRecoveryRepositoryError, VerifiedAccountIdentityAuthority};

impl SqliteAccountIdentityAuthorityRepository {
    pub fn approve_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &RecoveryId,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        if authority.role() != AccountIdentityRole::ParentOwner {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        let household_id = authority.household_id().to_string();
        ensure_recovery_proof_current(&transaction, recovery_id, household_id.as_str(), now)?;
        ensure_current_authority(&transaction, authority, now)?;
        let transition_at = next_transition_at(&transaction, recovery_id, now)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery
                 SET state = 'approved', last_transition_at_epoch_millis = ?3
                 WHERE recovery_id = ?1 AND household_id = ?2
                   AND state = 'owner-approval-required'
                   AND identity_proof_state = 'verified'",
                params![
                    recovery_id.as_str(),
                    authority.household_id().to_string(),
                    transition_at
                ],
            )
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        transaction
            .commit()
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)
    }
}

pub(crate) fn ensure_recovery_proof_current(
    transaction: &rusqlite::Transaction<'_>,
    recovery_id: &RecoveryId,
    household_id: &str,
    now: i64,
) -> Result<RecoveryKind, InviteRecoveryRepositoryError> {
    let row = transaction
        .query_row(
            "SELECT kind, identity_proof_expires_at_epoch_millis, identity_proof_state,
                    support_channel, support_authorization_id, support_authorization_issuer,
                    support_authorization_scope,
                    support_authorization_expires_at_epoch_millis
             FROM account_identity_recovery
             WHERE recovery_id = ?1 AND household_id = ?2 LIMIT 1",
            params![recovery_id.as_str(), household_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, Option<String>>(6)?,
                    row.get::<_, Option<i64>>(7)?,
                ))
            },
        )
        .optional()
        .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?
        .ok_or(InviteRecoveryRepositoryError::Missing)?;
    let kind =
        recovery_kind_from_label(&row.0).ok_or(InviteRecoveryRepositoryError::RecoveryRejected)?;
    if row.1 <= now || row.2 != "verified" {
        return Err(InviteRecoveryRepositoryError::RecoveryRejected);
    }
    if row.3 == "support-assisted" {
        if row.4.as_deref().is_none_or(|value| value.trim().is_empty())
            || row.5.as_deref().is_none_or(|value| value.trim().is_empty())
        {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        let scope = row
            .6
            .as_deref()
            .and_then(support_scope_from_label)
            .ok_or(InviteRecoveryRepositoryError::RecoveryRejected)?;
        if row.7.is_none_or(|expires_at| expires_at <= now)
            || !support_authorization_scope_allows(kind, scope)
        {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
    }
    Ok(kind)
}
