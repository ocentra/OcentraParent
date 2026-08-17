use ocentra_schema::account_identity_authority::{
    AccountIdentityBindingLifecycleState, AccountIdentityBindingRevocationState,
    AccountIdentityDeviceTrustState, AccountIdentityHouseholdChildDeviceBinding,
    AccountIdentityInstallState, AccountIdentityPairingState, AccountIdentitySessionFreshnessState,
};

use crate::family_identity::{ChildProfileBindingState, DeviceTrustState, SessionFreshnessState};

pub(super) fn map_binding_state(
    binding: &AccountIdentityHouseholdChildDeviceBinding,
) -> ChildProfileBindingState {
    if binding.pairing_state == AccountIdentityPairingState::Paired
        && binding.install_state == AccountIdentityInstallState::Installed
        && binding.lifecycle_state == AccountIdentityBindingLifecycleState::Active
        && binding.revocation_state == AccountIdentityBindingRevocationState::Active
    {
        ChildProfileBindingState::Bound
    } else {
        ChildProfileBindingState::Missing
    }
}

pub(super) fn map_device_trust(state: AccountIdentityDeviceTrustState) -> DeviceTrustState {
    match state {
        AccountIdentityDeviceTrustState::Pending => DeviceTrustState::Pending,
        AccountIdentityDeviceTrustState::Trusted => DeviceTrustState::Trusted,
        AccountIdentityDeviceTrustState::Revoked => DeviceTrustState::Revoked,
        AccountIdentityDeviceTrustState::ResetRequired => DeviceTrustState::ResetRequired,
        AccountIdentityDeviceTrustState::Disabled => DeviceTrustState::Disabled,
    }
}

pub(super) fn map_session_freshness(
    state: AccountIdentitySessionFreshnessState,
) -> SessionFreshnessState {
    match state {
        AccountIdentitySessionFreshnessState::Fresh => SessionFreshnessState::Fresh,
        AccountIdentitySessionFreshnessState::Stale => SessionFreshnessState::Stale,
        AccountIdentitySessionFreshnessState::Expired => SessionFreshnessState::Expired,
    }
}
