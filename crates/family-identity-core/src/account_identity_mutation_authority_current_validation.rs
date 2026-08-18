use chrono::{DateTime, Utc};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::{
    AccountIdentityMutationAction, AccountIdentityMutationAuthorityRequest,
    AccountIdentityMutationTarget,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn validate_against_current_authority(
    authority: &VerifiedAccountIdentityAuthority,
    request: &AccountIdentityMutationAuthorityRequest,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if !matches!(
        authority.role(),
        ocentra_schema::account_identity_authority::AccountIdentityRole::ParentOwner
            | ocentra_schema::account_identity_authority::AccountIdentityRole::CoParentGuardian
    ) {
        return Err(AccountIdentityMutationAuthorityError::RoleNotAuthorized);
    }

    let expires_at = DateTime::parse_from_rfc3339(authority.session_expires_at())
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidAuthority)?
        .with_timezone(&Utc);
    if expires_at <= Utc::now() {
        return Err(AccountIdentityMutationAuthorityError::AuthorityExpired);
    }

    if let AccountIdentityMutationTarget::ChildDevice {
        child_profile_id,
        child_device_id,
    } = &request.target
    {
        if child_profile_id != authority.child_profile_id().as_str()
            || child_device_id != authority.child_device_id().as_str()
        {
            return Err(AccountIdentityMutationAuthorityError::TargetMismatch);
        }
    }

    // Device revocation is a high-risk action. The current Account binding
    // intentionally has no parent step-up proof yet, so this producer must
    // remain unavailable instead of turning a role/session snapshot into a
    // mutation capability.
    if request.action == AccountIdentityMutationAction::RevokeChildDevice {
        return Err(AccountIdentityMutationAuthorityError::StepUpUnavailable);
    }

    Ok(())
}
