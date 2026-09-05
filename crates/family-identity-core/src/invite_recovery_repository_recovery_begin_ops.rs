use rusqlite::{params, TransactionBehavior};

use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::RecoveryId;
use crate::setup_lifecycle::RecoverySupportChannel;

use super::authority::trusted_now_in_transaction;
use super::security_effect_codes::owner_effect_code;
use super::security_effects::owner_effect;
use super::security_entropy::opaque_id;
use super::security_rate_recovery::enforce_recovery_rate_limit;
use super::support_invite_identity::{provider_label, role_label};
use super::support_recovery_channel::support_channel_label;
use super::support_recovery_kind_label::recovery_kind_label;
use super::support_recovery_policy::{owner_approval_required, recovery_request_is_allowed};
use super::support_recovery_scope::support_authorization_scope_allows;
use super::support_recovery_scope_label::support_scope_label;
use super::{
    InviteRecoveryRepositoryError, VerifiedRecoveryIdentityProof,
    VerifiedSupportRecoveryAuthorization,
};

impl SqliteAccountIdentityAuthorityRepository {
    pub(crate) fn begin_recovery(
        &mut self,
        proof: &VerifiedRecoveryIdentityProof,
        support_authorization: Option<&VerifiedSupportRecoveryAuthorization>,
    ) -> Result<RecoveryId, InviteRecoveryRepositoryError> {
        validate_begin_request(proof, support_authorization)?;
        let recovery_id = RecoveryId::parse(opaque_id("recovery-")?)
            .map_err(InviteRecoveryRepositoryError::InvalidValue)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        validate_begin_expiry(proof, support_authorization, now)?;
        enforce_recovery_rate_limit(&transaction, &proof.provider, &proof.provider_subject, now)?;
        persist_recovery(
            &transaction,
            &recovery_id,
            proof,
            support_authorization,
            now,
        )?;
        transaction
            .commit()
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(recovery_id)
    }
}

fn validate_begin_request(
    proof: &VerifiedRecoveryIdentityProof,
    support_authorization: Option<&VerifiedSupportRecoveryAuthorization>,
) -> Result<(), InviteRecoveryRepositoryError> {
    let support_required = proof.support_channel == RecoverySupportChannel::SupportAssisted;
    if !recovery_request_is_allowed(proof.role, proof.kind, proof.support_channel)
        || (support_required && support_authorization.is_none())
        || proof.proof_id.trim().is_empty()
        || proof.member_id.trim().is_empty()
        || proof.device_id.trim().is_empty()
    {
        return Err(InviteRecoveryRepositoryError::RecoveryRejected);
    }
    Ok(())
}

fn validate_begin_expiry(
    proof: &VerifiedRecoveryIdentityProof,
    support_authorization: Option<&VerifiedSupportRecoveryAuthorization>,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    if proof.expires_at_epoch_millis <= now {
        return Err(InviteRecoveryRepositoryError::RecoveryRejected);
    }
    if let Some(authorization) = support_authorization {
        if authorization.expires_at_epoch_millis <= now
            || authorization.authorization_id.is_empty()
            || authorization.issuer.is_empty()
            || authorization.household_id != proof.household_id
            || authorization.account_id != proof.account_id
            || authorization.kind != proof.kind
            || !support_authorization_scope_allows(proof.kind, authorization.scope)
        {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
    }
    Ok(())
}

fn persist_recovery(
    transaction: &rusqlite::Transaction<'_>,
    recovery_id: &RecoveryId,
    proof: &VerifiedRecoveryIdentityProof,
    authorization: Option<&VerifiedSupportRecoveryAuthorization>,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    let state = if owner_approval_required(proof.kind, proof.support_channel) {
        "owner-approval-required"
    } else {
        "approved"
    };
    transaction
        .execute(
            "INSERT INTO account_identity_recovery (
                 recovery_id, household_id, account_id, requester_member_id,
                 requester_device_id, requester_role, kind, support_channel,
                 identity_proof_id, identity_proof_provider, identity_proof_subject,
                 identity_proof_expires_at_epoch_millis, identity_proof_state,
                 support_authorization_id, support_authorization_issuer,
                 support_authorization_scope, support_authorization_expires_at_epoch_millis,
                 owner_effect_kind, state, created_at_epoch_millis,
                 last_transition_at_epoch_millis, reserved_owner_receipt_id,
                 reserved_owner_transition_id, reserved_owner_receipt_expires_at_epoch_millis
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12,
                       'verified', ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?19, NULL, NULL, NULL)",
            params![
                recovery_id.as_str(),
                proof.household_id.to_string(),
                proof.account_id.to_string(),
                proof.member_id.as_str(),
                proof.device_id.as_str(),
                role_label(proof.role),
                recovery_kind_label(proof.kind),
                support_channel_label(proof.support_channel),
                proof.proof_id.as_str(),
                provider_label(&proof.provider),
                proof.provider_subject.as_str(),
                proof.expires_at_epoch_millis,
                authorization.map(|value| value.authorization_id.as_str()),
                authorization.map(|value| value.issuer.as_str()),
                authorization.map(|value| support_scope_label(value.scope)),
                authorization.map(|value| value.expires_at_epoch_millis),
                owner_effect_code(owner_effect(proof.kind)),
                state,
                now,
            ],
        )
        .map(|_| ())
        .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)
}
