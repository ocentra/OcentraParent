use super::{CurrentChildDeviceTrustBinding, HouseholdAuthorityRuntimeFailure};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::device_trust_lifecycle::{DeviceTrustLifecycleError, DeviceTrustLifecycleState};

pub(super) fn map_error(error: DeviceTrustLifecycleError) -> HouseholdAuthorityRuntimeFailure {
    match error {
        DeviceTrustLifecycleError::RevokedDevice | DeviceTrustLifecycleError::InvalidState => {
            HouseholdAuthorityRuntimeFailure::DeviceTrustRevoked
        }
        DeviceTrustLifecycleError::ParentReauthorizationRequired
        | DeviceTrustLifecycleError::InvalidGeneration => {
            HouseholdAuthorityRuntimeFailure::DeviceTrustGenerationMismatch
        }
        DeviceTrustLifecycleError::Unavailable
        | DeviceTrustLifecycleError::DuplicateRegistration
        | DeviceTrustLifecycleError::RegistrationMissing
        | DeviceTrustLifecycleError::InvalidIdentity
        | DeviceTrustLifecycleError::InvalidSignerKey
        | DeviceTrustLifecycleError::DuplicateSignerRegistration
        | DeviceTrustLifecycleError::SignerRegistrationConflict
        | DeviceTrustLifecycleError::SignerRegistrationMissing => {
            HouseholdAuthorityRuntimeFailure::DeviceTrustUnavailable
        }
    }
}

pub(super) fn validate_current(
    authority: &VerifiedAccountIdentityAuthority,
    binding: &CurrentChildDeviceTrustBinding,
) -> Result<(), HouseholdAuthorityRuntimeFailure> {
    if binding.state() != DeviceTrustLifecycleState::Trusted {
        return Err(HouseholdAuthorityRuntimeFailure::DeviceTrustRevoked);
    }
    if binding.authority_generation() == 0
        || binding.lifecycle_generation() == 0
        || binding.installation_binding_generation() == 0
    {
        return Err(HouseholdAuthorityRuntimeFailure::DeviceTrustGenerationMismatch);
    }
    if binding.authority_generation() != authority.authority_generation()
        || binding.family_id() != authority.household_id().to_string()
        || binding.trust_subject() != authority.provider_subject().as_str()
        || binding.parent_device_id() != authority.device_id().as_str()
        || binding.child_device_id() != authority.child_device_id().as_str()
        || binding.installation_id() != authority.current_binding().installation_id.as_str()
    {
        return Err(HouseholdAuthorityRuntimeFailure::DeviceTrustBindingMismatch);
    }
    Ok(())
}

pub(super) fn same_current(
    expected: &CurrentChildDeviceTrustBinding,
    current: &CurrentChildDeviceTrustBinding,
) -> bool {
    expected.family_id() == current.family_id()
        && expected.trust_subject() == current.trust_subject()
        && expected.parent_device_id() == current.parent_device_id()
        && expected.child_device_id() == current.child_device_id()
        && expected.installation_id() == current.installation_id()
        && expected.signer_key_id() == current.signer_key_id()
        && expected.signer_key_sha256() == current.signer_key_sha256()
        && expected.lifecycle_generation() == current.lifecycle_generation()
        && expected.installation_binding_generation() == current.installation_binding_generation()
        && expected.authority_generation() == current.authority_generation()
        && expected.state() == current.state()
}
