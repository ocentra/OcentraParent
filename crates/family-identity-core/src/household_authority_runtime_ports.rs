use super::{
    ConsumedParentStepUp, CurrentChildDeviceTrustBinding, CurrentHouseholdCapability,
    CurrentHouseholdControllerLease, HouseholdAuthorityAction, HouseholdAuthorityCapabilitySource,
    HouseholdAuthorityControllerLeaseSource, HouseholdAuthorityParentStepUpSource,
    HouseholdAuthorityRuntimeFailure, ManualRequiredHouseholdAuthorityCapabilitySource,
    ManualRequiredHouseholdAuthorityControllerLeaseSource,
    ManualRequiredHouseholdAuthorityParentStepUpSource,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

impl HouseholdAuthorityCapabilitySource for ManualRequiredHouseholdAuthorityCapabilitySource {
    fn current_capability(
        &self,
        _account_authority: &VerifiedAccountIdentityAuthority,
        _device_binding: &CurrentChildDeviceTrustBinding,
        _action: HouseholdAuthorityAction,
    ) -> Result<CurrentHouseholdCapability, HouseholdAuthorityRuntimeFailure> {
        Err(HouseholdAuthorityRuntimeFailure::CapabilityUnavailable)
    }
}

impl HouseholdAuthorityControllerLeaseSource
    for ManualRequiredHouseholdAuthorityControllerLeaseSource
{
    fn current_controller_lease(
        &self,
        _account_authority: &VerifiedAccountIdentityAuthority,
        _device_binding: &CurrentChildDeviceTrustBinding,
        _action: HouseholdAuthorityAction,
    ) -> Result<CurrentHouseholdControllerLease, HouseholdAuthorityRuntimeFailure> {
        Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseUnavailable)
    }
}

impl HouseholdAuthorityParentStepUpSource for ManualRequiredHouseholdAuthorityParentStepUpSource {
    fn consume_current_parent_step_up(
        &mut self,
        _account_authority: &VerifiedAccountIdentityAuthority,
        _device_binding: &CurrentChildDeviceTrustBinding,
        _action: HouseholdAuthorityAction,
    ) -> Result<ConsumedParentStepUp, HouseholdAuthorityRuntimeFailure> {
        Err(HouseholdAuthorityRuntimeFailure::ParentStepUpUnavailable)
    }
}
