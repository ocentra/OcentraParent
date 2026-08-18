use chrono::Duration;

use super::{
    AccountIdentityMutationAuthorityError, AccountIdentityMutationAuthorityRequest,
    AccountIdentityMutationTarget,
};

#[path = "account_identity_mutation_authority_current_validation.rs"]
mod current_validation;

pub(super) use current_validation::validate_against_current_authority;

pub(super) const MAX_IDEMPOTENCY_KEY_BYTES: usize = 256;
pub(super) const MAX_TARGET_ID_BYTES: usize = 256;
pub(super) const MAX_AUTHORITY_LIFETIME: Duration = Duration::minutes(5);

pub(super) fn validate_request(
    request: &AccountIdentityMutationAuthorityRequest,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if request.idempotency_key.trim().is_empty()
        || request.idempotency_key.len() > MAX_IDEMPOTENCY_KEY_BYTES
        || request
            .idempotency_key
            .bytes()
            .any(|byte| byte.is_ascii_control())
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidRequest);
    }

    match &request.target {
        AccountIdentityMutationTarget::ChildDevice {
            child_profile_id,
            child_device_id,
        } => validate_target_ids([child_profile_id, child_device_id])?,
        AccountIdentityMutationTarget::SetupInvite(invite_id)
        | AccountIdentityMutationTarget::Recovery(invite_id) => validate_target_ids([invite_id])?,
    }
    let target_matches_action = matches!(
        (request.action, &request.target),
        (
            super::AccountIdentityMutationAction::RevokeChildDevice,
            AccountIdentityMutationTarget::ChildDevice { .. }
        ) | (
            super::AccountIdentityMutationAction::RevokeSetupInvite,
            AccountIdentityMutationTarget::SetupInvite(_)
        ) | (
            super::AccountIdentityMutationAction::RevokeRecovery,
            AccountIdentityMutationTarget::Recovery(_)
        )
    );
    if !target_matches_action {
        return Err(AccountIdentityMutationAuthorityError::InvalidRequest);
    }
    Ok(())
}

pub(super) fn validate_lifetime(
    issued_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if expires_at <= issued_at || expires_at - issued_at > MAX_AUTHORITY_LIFETIME {
        return Err(AccountIdentityMutationAuthorityError::InvalidRequest);
    }
    Ok(())
}

fn validate_target_ids<const N: usize>(
    values: [&str; N],
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if values.iter().any(|value| {
        value.trim().is_empty()
            || value.len() > MAX_TARGET_ID_BYTES
            || value.bytes().any(|byte| byte.is_ascii_control())
    }) {
        return Err(AccountIdentityMutationAuthorityError::InvalidRequest);
    }
    Ok(())
}
