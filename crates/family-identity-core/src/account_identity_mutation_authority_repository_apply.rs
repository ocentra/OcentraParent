use rusqlite::Transaction;

use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::AccountIdentityMutationResult;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

#[path = "account_identity_mutation_authority_repository_apply_invite.rs"]
mod invite;
#[path = "account_identity_mutation_authority_repository_apply_recovery.rs"]
mod recovery;

pub(super) fn apply(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<AccountIdentityMutationResult, AccountIdentityMutationAuthorityError> {
    match (envelope.action.as_str(), envelope.target_kind.as_str()) {
        ("revoke-setup-invite", "setup-invite") => invite::apply(transaction, envelope, now),
        ("revoke-recovery", "recovery") => recovery::apply(transaction, envelope, now),
        ("revoke-child-device", "child-device") => {
            Err(AccountIdentityMutationAuthorityError::StepUpUnavailable)
        }
        _ => Err(AccountIdentityMutationAuthorityError::InvalidEnvelope),
    }
}
