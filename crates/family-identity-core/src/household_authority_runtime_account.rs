use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority::{
    AccountIdentityAccountState, AccountIdentityBindingLifecycleState,
    AccountIdentityBindingRevocationState, AccountIdentityDeviceTrustState,
    AccountIdentityInstallState, AccountIdentityMembershipState, AccountIdentityPairingState,
    AccountIdentityRole, AccountIdentitySessionFreshnessState,
};

use super::{HouseholdAuthorityAction, HouseholdAuthorityRuntimeFailure};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

pub(super) fn validate_current(
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), HouseholdAuthorityRuntimeFailure> {
    let (
        account_state,
        membership_state,
        device_trust_state,
        session_freshness_state,
        pairing_state,
        install_state,
        lifecycle_state,
        revocation_state,
    ) = authority.report_query_custody_states();

    if account_state != AccountIdentityAccountState::Active
        || membership_state != AccountIdentityMembershipState::Active
    {
        return Err(HouseholdAuthorityRuntimeFailure::AccountAuthorityRevoked);
    }
    if device_trust_state != AccountIdentityDeviceTrustState::Trusted {
        return Err(HouseholdAuthorityRuntimeFailure::DeviceTrustRevoked);
    }
    if session_freshness_state != AccountIdentitySessionFreshnessState::Fresh {
        return Err(HouseholdAuthorityRuntimeFailure::SessionStale);
    }
    if pairing_state != AccountIdentityPairingState::Paired
        || install_state != AccountIdentityInstallState::Installed
        || lifecycle_state != AccountIdentityBindingLifecycleState::Active
        || revocation_state != AccountIdentityBindingRevocationState::Active
    {
        return Err(HouseholdAuthorityRuntimeFailure::DeviceTrustBindingMismatch);
    }
    if authority.authority_generation() == 0
        || authority.session_generation() == 0
        || authority.current_binding().authority_generation != authority.authority_generation()
    {
        return Err(HouseholdAuthorityRuntimeFailure::AccountAuthorityGenerationMismatch);
    }

    let expires_at = DateTime::parse_from_rfc3339(authority.session_expires_at())
        .map_err(|_error| HouseholdAuthorityRuntimeFailure::SessionStale)?
        .with_timezone(&Utc);
    if expires_at <= Utc::now() {
        return Err(HouseholdAuthorityRuntimeFailure::SessionStale);
    }

    let binding = authority.current_binding();
    if &binding.account_id != authority.account_id()
        || &binding.household_id != authority.household_id()
        || &binding.child_profile_id != authority.child_profile_id()
        || binding.child_device_id.as_str() != authority.child_device_id().as_str()
    {
        return Err(HouseholdAuthorityRuntimeFailure::AccountAuthorityStale);
    }
    Ok(())
}

pub(super) fn role_can_authorize(
    role: AccountIdentityRole,
    action: HouseholdAuthorityAction,
) -> bool {
    matches!(
        (role, action),
        (
            AccountIdentityRole::ParentOwner,
            HouseholdAuthorityAction::SealParentDeviceTrust
        ) | (
            AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian,
            HouseholdAuthorityAction::PairChildDevice
                | HouseholdAuthorityAction::RegisterLanSignerAnchor
                | HouseholdAuthorityAction::RevokeChildDevice
                | HouseholdAuthorityAction::ChangePolicy
        ) | (
            AccountIdentityRole::ParentOwner
                | AccountIdentityRole::CoParentGuardian
                | AccountIdentityRole::Observer,
            HouseholdAuthorityAction::ViewChildStatus | HouseholdAuthorityAction::StartRemoteView
        ) | (
            AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian,
            HouseholdAuthorityAction::StartRemoteControl
        ) | (
            AccountIdentityRole::ParentOwner,
            HouseholdAuthorityAction::ExportDeleteData
                | HouseholdAuthorityAction::ImportRestoreData
                | HouseholdAuthorityAction::ManageBilling
        )
    )
}
