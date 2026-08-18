use super::{
    ConsumedParentStepUp, CurrentChildDeviceTrustBinding, CurrentHouseholdCapability,
    CurrentHouseholdControllerLease, HouseholdAuthorityAction, HouseholdAuthorityCapabilitySource,
    HouseholdAuthorityControllerLeaseSource, HouseholdAuthorityParentStepUpSource,
    HouseholdAuthorityRuntimeCasFence, HouseholdAuthorityRuntimeEffectAuthorization,
    HouseholdAuthorityRuntimeFailure, ManualRequiredHouseholdAuthorityCapabilitySource,
    ManualRequiredHouseholdAuthorityControllerLeaseSource,
    ManualRequiredHouseholdAuthorityParentStepUpSource,
    ManualRequiredHouseholdAuthorityRuntimeCasFence,
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

impl HouseholdAuthorityRuntimeCasFence for ManualRequiredHouseholdAuthorityRuntimeCasFence {
    fn compare_and_consume(
        &mut self,
        _authorization: super::HouseholdAuthorityRuntimeAuthorization,
        _current_account_authority: crate::account_identity_authority::VerifiedAccountIdentityAuthority,
        _current_device_binding: CurrentChildDeviceTrustBinding,
        _current_capability: Option<CurrentHouseholdCapability>,
        _current_controller_lease: Option<CurrentHouseholdControllerLease>,
        _current_parent_step_up: Option<ConsumedParentStepUp>,
        _consumption_nonce: &[u8; 32],
    ) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeFailure>
    {
        Err(HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable)
    }
}
