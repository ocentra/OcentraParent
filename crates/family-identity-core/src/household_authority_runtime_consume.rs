use super::{
    CurrentChildDeviceTrustBinding, HouseholdAuthorityCapabilitySource,
    HouseholdAuthorityControllerLeaseSource, HouseholdAuthorityDeviceTrustSource,
    HouseholdAuthorityParentStepUpSource, HouseholdAuthorityRuntimeAuthorization,
    HouseholdAuthorityRuntimeCasFence, HouseholdAuthorityRuntimeCasInput,
    HouseholdAuthorityRuntimeConsumeInput, HouseholdAuthorityRuntimeEffectAuthorization,
    HouseholdAuthorityRuntimeFailure,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

pub(super) fn consume<
    DeviceTrustSource: HouseholdAuthorityDeviceTrustSource,
    CapabilitySource: HouseholdAuthorityCapabilitySource,
    ControllerLeaseSource: HouseholdAuthorityControllerLeaseSource,
    ParentStepUpSource: HouseholdAuthorityParentStepUpSource,
    CasFence: HouseholdAuthorityRuntimeCasFence,
>(
    input: HouseholdAuthorityRuntimeConsumeInput<
        '_,
        DeviceTrustSource,
        CapabilitySource,
        ControllerLeaseSource,
        ParentStepUpSource,
        CasFence,
    >,
) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeFailure> {
    let HouseholdAuthorityRuntimeConsumeInput {
        account_service,
        presented_account_authority,
        device_trust_source,
        capability_source,
        controller_lease_source,
        parent_step_up_source,
        cas_fence,
        authorization,
    } = input;
    let current_account = super::household_authority_runtime_resolution::account_authority(
        account_service,
        presented_account_authority,
    )?;
    super::household_authority_runtime_account::validate_current(&current_account)?;
    if !super::household_authority_runtime_account::role_can_authorize(
        current_account.role(),
        authorization.action(),
    ) {
        return Err(HouseholdAuthorityRuntimeFailure::RoleNotAuthorized);
    }
    authorization.validate_current_account(&current_account)?;
    let current_device = device_trust_source.current_device_trust_binding(&current_account)?;
    super::household_authority_runtime_device_validation::validate_current(
        &current_account,
        &current_device,
    )?;
    authorization.validate_current_device(&current_device)?;
    let current_capability = super::household_authority_runtime_resolution::capability(
        capability_source,
        &current_account,
        &current_device,
        authorization.action(),
    )?;
    authorization.validate_current_capability(current_capability.as_ref())?;

    let current_controller_lease = super::household_authority_runtime_resolution::controller_lease(
        controller_lease_source,
        &current_account,
        &current_device,
        authorization.action(),
    )?;
    authorization.validate_current_controller_lease(current_controller_lease.as_ref())?;

    let current_parent_step_up = revalidate_parent_step_up(
        parent_step_up_source,
        &authorization,
        &current_account,
        &current_device,
    )?;

    let consumption_nonce = *authorization.consumption_nonce();
    cas_fence.compare_and_consume(HouseholdAuthorityRuntimeCasInput {
        authorization,
        current_account_authority: current_account,
        current_device_binding: current_device,
        current_capability,
        current_controller_lease,
        current_parent_step_up,
        consumption_nonce,
    })
}

fn revalidate_parent_step_up(
    source: &impl HouseholdAuthorityParentStepUpSource,
    authorization: &HouseholdAuthorityRuntimeAuthorization,
    account_authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
) -> Result<Option<super::ConsumedParentStepUp>, HouseholdAuthorityRuntimeFailure> {
    let Some(step_up) = authorization.parent_step_up() else {
        return Ok(None);
    };
    step_up.validate_for(account_authority, device_binding, authorization.action())?;
    source
        .revalidate_current_parent_step_up(
            account_authority,
            device_binding,
            authorization.action(),
            step_up,
        )
        .and_then(|current| {
            current.validate_for(account_authority, device_binding, authorization.action())?;
            if !step_up.same_current(&current) {
                return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch);
            }
            Ok(Some(current))
        })
}
