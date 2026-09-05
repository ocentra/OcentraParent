use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::{
    AccountIdentityAuthorityService, AccountIdentityAuthorityServiceError,
};
use crate::device_trust_current_binding::CurrentChildDeviceTrustBinding;
use crate::household_authority::requires_parent_step_up;

use super::{
    ConsumedParentStepUp, CurrentHouseholdCapability, CurrentHouseholdControllerLease,
    HouseholdAuthorityAction, HouseholdAuthorityCapabilitySource,
    HouseholdAuthorityControllerLeaseSource, HouseholdAuthorityParentStepUpSource,
    HouseholdAuthorityRuntimeFailure,
};

pub(super) fn account_authority(
    account_service: &AccountIdentityAuthorityService,
    presented_account_authority: &VerifiedAccountIdentityAuthority,
) -> Result<VerifiedAccountIdentityAuthority, HouseholdAuthorityRuntimeFailure> {
    account_service
        .resolve_current(
            presented_account_authority.provider(),
            presented_account_authority.provider_subject(),
        )
        .map_err(|error| map_account_authority_error(&error))
}

pub(super) fn capability(
    source: &impl HouseholdAuthorityCapabilitySource,
    authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
    action: HouseholdAuthorityAction,
) -> Result<Option<CurrentHouseholdCapability>, HouseholdAuthorityRuntimeFailure> {
    if !super::household_authority_runtime_requirements::requires_capability(action) {
        return Ok(None);
    }
    let capability = source.current_capability(authority, device_binding, action)?;
    capability.validate_for(authority, device_binding, action)?;
    Ok(Some(capability))
}

pub(super) fn controller_lease(
    source: &impl HouseholdAuthorityControllerLeaseSource,
    authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
    action: HouseholdAuthorityAction,
) -> Result<Option<CurrentHouseholdControllerLease>, HouseholdAuthorityRuntimeFailure> {
    if !super::household_authority_runtime_requirements::requires_controller_lease(action) {
        return Ok(None);
    }
    let lease = source.current_controller_lease(authority, device_binding, action)?;
    lease.validate_for(authority, device_binding, action)?;
    Ok(Some(lease))
}

pub(super) fn parent_step_up(
    source: &mut impl HouseholdAuthorityParentStepUpSource,
    authority: &VerifiedAccountIdentityAuthority,
    device_binding: &CurrentChildDeviceTrustBinding,
    action: HouseholdAuthorityAction,
) -> Result<Option<ConsumedParentStepUp>, HouseholdAuthorityRuntimeFailure> {
    if !requires_parent_step_up(action) {
        return Ok(None);
    }
    let step_up = source.consume_current_parent_step_up(authority, device_binding, action)?;
    step_up.validate_for(authority, device_binding, action)?;
    Ok(Some(step_up))
}

fn map_account_authority_error(
    error: &AccountIdentityAuthorityServiceError,
) -> HouseholdAuthorityRuntimeFailure {
    match error {
        AccountIdentityAuthorityServiceError::Repository(_) => {
            HouseholdAuthorityRuntimeFailure::AccountAuthorityUnavailable
        }
        AccountIdentityAuthorityServiceError::Missing => {
            HouseholdAuthorityRuntimeFailure::AccountAuthorityRevoked
        }
        AccountIdentityAuthorityServiceError::InvalidAuthority => {
            HouseholdAuthorityRuntimeFailure::AccountAuthorityStale
        }
    }
}
