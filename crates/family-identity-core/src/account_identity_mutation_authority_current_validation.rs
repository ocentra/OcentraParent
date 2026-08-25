use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority::{
    AccountIdentityRole, AccountIdentitySupportReceiptRevocationState,
};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::{
    AccountIdentityMutationAction, AccountIdentityMutationAuthorityRequest,
    AccountIdentityMutationTarget,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn validate_against_current_authority(
    authority: &VerifiedAccountIdentityAuthority,
    request: &AccountIdentityMutationAuthorityRequest,
    trusted_now_epoch_millis: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if !matches!(
        authority.role(),
        AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian
    ) {
        return Err(AccountIdentityMutationAuthorityError::RoleNotAuthorized);
    }

    let expires_at = DateTime::parse_from_rfc3339(authority.session_expires_at())
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidAuthority)?
        .with_timezone(&Utc);
    let trusted_now = DateTime::<Utc>::from_timestamp_millis(trusted_now_epoch_millis)
        .ok_or(AccountIdentityMutationAuthorityError::ClockUnavailable)?;
    if expires_at <= trusted_now {
        return Err(AccountIdentityMutationAuthorityError::AuthorityExpired);
    }

    validate_support_receipt(authority, trusted_now)?;

    if let AccountIdentityMutationTarget::ChildDevice {
        child_profile_id,
        child_device_id,
    } = request.target()
    {
        if child_profile_id != authority.child_profile_id().to_string().as_str()
            || child_device_id.as_str() != authority.child_device_id().as_str()
        {
            return Err(AccountIdentityMutationAuthorityError::TargetMismatch);
        }
    }

    // Device revocation is a high-risk action. The current Account binding
    // intentionally has no parent step-up proof yet, so this producer must
    // remain unavailable instead of turning a role/session snapshot into a
    // mutation capability.
    if request.action() == AccountIdentityMutationAction::RevokeChildDevice {
        return Err(AccountIdentityMutationAuthorityError::StepUpUnavailable);
    }

    Ok(())
}

fn validate_support_receipt(
    authority: &VerifiedAccountIdentityAuthority,
    trusted_now: DateTime<Utc>,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let Some(receipt) = authority.support_receipt() else {
        return Ok(());
    };
    let issued_at = DateTime::parse_from_rfc3339(&receipt.issued_at)
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidAuthority)?
        .with_timezone(&Utc);
    let expires_at = DateTime::parse_from_rfc3339(&receipt.expires_at)
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidAuthority)?
        .with_timezone(&Utc);
    if receipt.revocation_state != AccountIdentitySupportReceiptRevocationState::Active
        || issued_at > trusted_now
        || trusted_now >= expires_at
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidAuthority);
    }
    Ok(())
}
