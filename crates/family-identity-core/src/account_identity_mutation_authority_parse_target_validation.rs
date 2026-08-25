use super::super::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn validate_envelope(
    envelope: &CanonicalMutationEnvelope,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    match envelope.target_kind.as_str() {
        "child-device" => validate_child_target(envelope),
        "setup-invite" => validate_invite_target(envelope),
        "recovery" => validate_recovery_target(envelope),
        _ => Err(AccountIdentityMutationAuthorityError::InvalidEnvelope),
    }
}

fn validate_child_target(
    envelope: &CanonicalMutationEnvelope,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if !envelope.target_id.is_empty()
        || envelope.target_child_profile_id.trim().is_empty()
        || envelope.target_child_device_id.trim().is_empty()
        || envelope.target_expires_at_epoch_millis != 0
        || !target_support_is_empty(envelope)
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(())
}

fn validate_invite_target(
    envelope: &CanonicalMutationEnvelope,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if envelope.target_id.trim().is_empty()
        || !envelope.target_child_profile_id.is_empty()
        || !envelope.target_child_device_id.is_empty()
        || envelope.target_expires_at_epoch_millis <= 0
        || !target_support_is_empty(envelope)
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(())
}

fn validate_recovery_target(
    envelope: &CanonicalMutationEnvelope,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if envelope.target_id.trim().is_empty()
        || !envelope.target_child_profile_id.is_empty()
        || !envelope.target_child_device_id.is_empty()
        || envelope.target_expires_at_epoch_millis <= 0
        || !["self-serve", "household-owner-assisted", "support-assisted"]
            .contains(&envelope.target_support_channel.as_str())
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    let has_support = envelope.target_support_channel == "support-assisted";
    if has_support == target_support_authorization_is_empty(envelope) {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    if has_support
        && !["household", "device-control"]
            .contains(&envelope.target_support_authorization_scope.as_str())
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(())
}

fn target_support_is_empty(envelope: &CanonicalMutationEnvelope) -> bool {
    envelope.target_support_channel.is_empty() && target_support_authorization_is_empty(envelope)
}

fn target_support_authorization_is_empty(envelope: &CanonicalMutationEnvelope) -> bool {
    envelope.target_support_authorization_id.is_empty()
        && envelope.target_support_authorization_issuer.is_empty()
        && envelope.target_support_authorization_scope.is_empty()
        && envelope.target_support_authorization_expires_at_epoch_millis == 0
}
