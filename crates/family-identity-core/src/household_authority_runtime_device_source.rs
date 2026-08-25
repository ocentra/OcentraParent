use super::{
    HouseholdAuthorityDeviceTrustSource, HouseholdAuthorityRuntimeFailure,
    ManualRequiredHouseholdAuthorityDeviceTrustSource,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::device_trust_current_binding::CurrentChildDeviceTrustBinding;
use crate::device_trust_lifecycle::DeviceTrustLifecycleRepository;

impl HouseholdAuthorityDeviceTrustSource for ManualRequiredHouseholdAuthorityDeviceTrustSource {
    fn current_device_trust_binding(
        &self,
        _account_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<CurrentChildDeviceTrustBinding, HouseholdAuthorityRuntimeFailure> {
        Err(HouseholdAuthorityRuntimeFailure::DeviceTrustUnavailable)
    }
}

/// The durable Device Trust repository is a legal adapter for the current opaque child binding.
/// It resolves identity from the freshly verified Account authority and never accepts a caller
/// supplied child/parent/generation tuple.
impl HouseholdAuthorityDeviceTrustSource for DeviceTrustLifecycleRepository {
    fn current_device_trust_binding(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<CurrentChildDeviceTrustBinding, HouseholdAuthorityRuntimeFailure> {
        let family_id = account_authority.household_id().to_string();
        let current = self
            .current_signer_authority(
                &family_id,
                account_authority.provider_subject().as_str(),
                account_authority.device_id().as_str(),
                account_authority.child_device_id().as_str(),
            )
            .map_err(super::household_authority_runtime_device_validation::map_error)?;
        Ok(current.into_current_child_device_trust_binding())
    }
}
