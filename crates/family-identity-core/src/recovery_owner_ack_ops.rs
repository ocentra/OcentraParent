use rusqlite::{params, OptionalExtension, Transaction, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::RecoveryId;
use crate::setup_lifecycle::RecoveryKind;

use super::authority::{ensure_current_authority, next_transition_at, trusted_now_in_transaction};
use super::recovery_ops::ensure_recovery_proof_current;
use super::security_effect_codes::owner_effect_code;
use super::support_recovery_kind_from_label::recovery_kind_from_label;
use super::{
    DeviceTrustReinstallOwnerReceipt, DeviceTrustRevokeOwnerReceipt,
    HouseholdAuthorityMutationOwnerReceipt, InviteRecoveryRepositoryError,
    ProviderCredentialSessionOwnerReceipt, RecoveryHandoffDeliveryAttempt, RecoveryOwnerEffect,
};

impl SqliteAccountIdentityAuthorityRepository {
    pub(crate) fn acknowledge_provider_credential_session(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &ProviderCredentialSessionOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        acknowledge_owner_receipt(
            self,
            authority,
            attempt,
            receipt.handoff_id.as_str(),
            receipt.correlation_id.as_str(),
            &receipt.recovery_id,
            receipt.attempt_id.as_str(),
            receipt.transition_id.as_str(),
            receipt.receipt_digest.as_str(),
            RecoveryOwnerEffect::ProviderCredentialSession,
        )
    }

    pub(crate) fn acknowledge_device_trust_revoke(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &DeviceTrustRevokeOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        acknowledge_owner_receipt(
            self,
            authority,
            attempt,
            receipt.handoff_id.as_str(),
            receipt.correlation_id.as_str(),
            &receipt.recovery_id,
            receipt.attempt_id.as_str(),
            receipt.transition_id.as_str(),
            receipt.receipt_digest.as_str(),
            RecoveryOwnerEffect::DeviceTrustRevoke,
        )
    }

    pub(crate) fn acknowledge_device_trust_reinstall(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &DeviceTrustReinstallOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        acknowledge_owner_receipt(
            self,
            authority,
            attempt,
            receipt.handoff_id.as_str(),
            receipt.correlation_id.as_str(),
            &receipt.recovery_id,
            receipt.attempt_id.as_str(),
            receipt.transition_id.as_str(),
            receipt.receipt_digest.as_str(),
            RecoveryOwnerEffect::DeviceTrustReinstall,
        )
    }

    pub(crate) fn acknowledge_household_authority_mutation(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &HouseholdAuthorityMutationOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        acknowledge_owner_receipt(
            self,
            authority,
            attempt,
            receipt.handoff_id.as_str(),
            receipt.correlation_id.as_str(),
            &receipt.recovery_id,
            receipt.attempt_id.as_str(),
            receipt.transition_id.as_str(),
            receipt.receipt_digest.as_str(),
            RecoveryOwnerEffect::HouseholdAuthorityMutation,
        )
    }
}

fn acknowledge_owner_receipt(
    repository: &mut SqliteAccountIdentityAuthorityRepository,
    authority: &VerifiedAccountIdentityAuthority,
    attempt: &RecoveryHandoffDeliveryAttempt,
    handoff_id: &str,
    correlation_id: &str,
    recovery_id: &RecoveryId,
    attempt_id: &str,
    transition_id: &str,
    receipt_digest: &str,
    effect: RecoveryOwnerEffect,
) -> Result<(), InviteRecoveryRepositoryError> {
    validate_owner_receipt_input(
        attempt,
        handoff_id,
        correlation_id,
        recovery_id,
        attempt_id,
        transition_id,
        receipt_digest,
    )?;
    let transaction = repository
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
    validate_owner_effect(&transaction, attempt, effect, kind)?;
    let transition_at = next_transition_at(&transaction, recovery_id, now)?;
    persist_owner_receipt(
        &transaction,
        handoff_id,
        correlation_id,
        recovery_id,
        authority.household_id().to_string(),
        attempt_id,
        transition_id,
        receipt_digest,
        now,
        transition_at,
    )?;
    transaction
        .commit()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
}

fn persist_owner_receipt(
    transaction: &Transaction<'_>,
    handoff_id: &str,
    correlation_id: &str,
    recovery_id: &RecoveryId,
    household_id: String,
    attempt_id: &str,
    transition_id: &str,
    receipt_digest: &str,
    now: i64,
    transition_at: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_recovery_custody_handoff
             SET state = 'delivered', lease_expires_at_epoch_millis = NULL,
                 active_attempt_id = NULL, owner_transition_id = ?4,
                 owner_receipt_digest = ?5
             WHERE handoff_id = ?1 AND correlation_id = ?2 AND recovery_id = ?3
               AND household_id = ?6 AND state = 'in-flight' AND active_attempt_id = ?7
               AND lease_expires_at_epoch_millis > ?8",
            params![
                handoff_id,
                correlation_id,
                recovery_id.as_str(),
                transition_id,
                receipt_digest,
                household_id.as_str(),
                attempt_id,
                now,
            ],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    if changed != 1 {
        return Err(InviteRecoveryRepositoryError::HandoffConflict);
    }
    let completed = transaction
        .execute(
            "UPDATE account_identity_recovery
             SET state = 'completed', last_transition_at_epoch_millis = ?2,
                 reserved_owner_receipt_id = ?3, reserved_owner_transition_id = ?4
             WHERE recovery_id = ?1 AND household_id = ?5 AND state = 'approved'",
            params![
                recovery_id.as_str(),
                transition_at,
                receipt_digest,
                transition_id,
                household_id.as_str(),
            ],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    (completed == 1)
        .then_some(())
        .ok_or(InviteRecoveryRepositoryError::RecoveryRejected)
}

fn validate_owner_receipt_input(
    attempt: &RecoveryHandoffDeliveryAttempt,
    handoff_id: &str,
    correlation_id: &str,
    recovery_id: &RecoveryId,
    attempt_id: &str,
    transition_id: &str,
    receipt_digest: &str,
) -> Result<(), InviteRecoveryRepositoryError> {
    if handoff_id != attempt.handoff.handoff_id()
        || correlation_id != attempt.handoff.correlation_id()
        || recovery_id != attempt.handoff.recovery_id()
        || attempt_id != attempt.attempt_id
        || transition_id.trim().is_empty()
        || !hex_digest(receipt_digest)
    {
        return Err(InviteRecoveryRepositoryError::HandoffConflict);
    }
    Ok(())
}

fn validate_owner_effect(
    transaction: &Transaction<'_>,
    attempt: &RecoveryHandoffDeliveryAttempt,
    effect: RecoveryOwnerEffect,
    kind: RecoveryKind,
) -> Result<(), InviteRecoveryRepositoryError> {
    let row = transaction
        .query_row(
            "SELECT h.kind, r.owner_effect_kind, r.state
             FROM account_identity_recovery_custody_handoff h
             JOIN account_identity_recovery r ON r.recovery_id = h.recovery_id
             WHERE h.handoff_id = ?1 AND h.recovery_id = ?2 LIMIT 1",
            params![
                attempt.handoff.handoff_id(),
                attempt.handoff.recovery_id().as_str()
            ],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?
        .ok_or(InviteRecoveryRepositoryError::Missing)?;
    let stored_kind =
        recovery_kind_from_label(&row.0).ok_or(InviteRecoveryRepositoryError::RecoveryRejected)?;
    if stored_kind != kind || row.1 != owner_effect_code(effect) || row.2 != "approved" {
        return Err(InviteRecoveryRepositoryError::RecoveryRejected);
    }
    Ok(())
}

fn hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
