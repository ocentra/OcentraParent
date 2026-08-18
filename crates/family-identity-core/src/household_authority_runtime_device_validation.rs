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
    {
        return Err(HouseholdAuthorityRuntimeFailure::DeviceTrustBindingMismatch);
    }
    Ok(())
}
