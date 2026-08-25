use super::{
    CurrentChildDeviceTrustBinding, CurrentHouseholdCapability, CurrentHouseholdControllerLease,
    HouseholdAuthorityRuntimeAuthorization, HouseholdAuthorityRuntimeFailure,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

impl HouseholdAuthorityRuntimeAuthorization {
    pub(super) fn validate_current_account(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<(), HouseholdAuthorityRuntimeFailure> {
        let binding = authority.current_binding();
        if self.account_authority_generation != authority.authority_generation()
            || self.session_generation != authority.session_generation()
        {
            return Err(HouseholdAuthorityRuntimeFailure::AccountAuthorityGenerationMismatch);
        }
        if self.session_id != authority.session_id().as_str()
            || self.session_expires_at != authority.session_expires_at()
        {
            return Err(HouseholdAuthorityRuntimeFailure::SessionStale);
        }
        if self.installation_id != binding.installation_id.as_str()
            || self.pairing_id != binding.pairing_id.as_str()
            || self.route_id != binding.selected_route_id.as_str()
        {
            return Err(HouseholdAuthorityRuntimeFailure::AccountAuthorityStale);
        }
        Ok(())
    }

    pub(super) fn validate_current_device(
        &self,
        binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<(), HouseholdAuthorityRuntimeFailure> {
        if !super::household_authority_runtime_device_validation::same_current(
            &self.device_binding,
            binding,
        ) {
            return Err(HouseholdAuthorityRuntimeFailure::DeviceTrustGenerationMismatch);
        }
        Ok(())
    }

    pub(super) fn validate_current_capability(
        &self,
        capability: Option<&CurrentHouseholdCapability>,
    ) -> Result<(), HouseholdAuthorityRuntimeFailure> {
        match (&self.capability, capability) {
            (None, None) => Ok(()),
            (Some(expected), Some(current)) if expected.same_current(current) => Ok(()),
            _ => Err(HouseholdAuthorityRuntimeFailure::CapabilityBindingMismatch),
        }
    }

    pub(super) fn validate_current_controller_lease(
        &self,
        lease: Option<&CurrentHouseholdControllerLease>,
    ) -> Result<(), HouseholdAuthorityRuntimeFailure> {
        match (&self.controller_lease, lease) {
            (None, None) => Ok(()),
            (Some(expected), Some(current)) if expected.same_current(current) => Ok(()),
            _ => Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch),
        }
    }
}
