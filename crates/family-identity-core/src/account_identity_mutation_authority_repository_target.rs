use rusqlite::Transaction;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::{
    AccountIdentityMutationAuthorityRequest, AccountIdentityMutationTarget,
    ResolvedAccountIdentityMutationTarget,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use crate::family_identity::{RecoveryId, SetupInviteId};

#[path = "account_identity_mutation_authority_repository_target_invite.rs"]
mod invite;
#[path = "account_identity_mutation_authority_repository_target_recovery.rs"]
mod recovery;

pub(super) fn resolve_request(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    request: &AccountIdentityMutationAuthorityRequest,
    now: i64,
) -> Result<ResolvedAccountIdentityMutationTarget, AccountIdentityMutationAuthorityError> {
    match request.target() {
        AccountIdentityMutationTarget::ChildDevice { .. } => {
            Err(AccountIdentityMutationAuthorityError::StepUpUnavailable)
        }
        AccountIdentityMutationTarget::SetupInvite(value) => {
            let invite_id = SetupInviteId::parse(value.clone())
                .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidRequest)?;
            invite::resolve(transaction, authority, &invite_id, now)
        }
        AccountIdentityMutationTarget::Recovery(value) => {
            let recovery_id = RecoveryId::parse(value.clone())
                .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidRequest)?;
            recovery::resolve(transaction, authority, &recovery_id, now)
        }
    }
}

pub(super) fn validate_consumed(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    match envelope.target_kind.as_str() {
        "child-device" => Err(AccountIdentityMutationAuthorityError::StepUpUnavailable),
        "setup-invite" => invite::validate_consumed(transaction, envelope, now),
        "recovery" => recovery::validate_consumed(transaction, envelope, now),
        _ => Err(AccountIdentityMutationAuthorityError::InvalidEnvelope),
    }
}
