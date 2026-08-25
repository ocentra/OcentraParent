use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority::AccountIdentityChildDeviceId;
use ocentra_schema::report_query_custody::ChildProfileId;

use crate::account_identity_authority::{
    account_identity_authority_value_mapping, VerifiedAccountIdentityAuthority,
};
use crate::family_identity::DeviceOwnershipScope;
use crate::household_authority::{
    authorize_household_actor_target_action, HouseholdActorTargetAuthorityInput,
    HouseholdAuthorityAction, HouseholdAuthorityDecision,
};

#[derive(Eq, PartialEq)]
pub enum AccountIdentityTarget {
    ChildProfile(ChildProfileId),
    ChildDevice {
        child_profile_id: ChildProfileId,
        child_device_id: AccountIdentityChildDeviceId,
    },
}

impl AccountIdentityTarget {
    pub fn child_profile(child_profile_id: ChildProfileId) -> Self {
        Self::ChildProfile(child_profile_id)
    }

    pub fn child_device(
        child_profile_id: ChildProfileId,
        child_device_id: AccountIdentityChildDeviceId,
    ) -> Self {
        Self::ChildDevice {
            child_profile_id,
            child_device_id,
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct AccountIdentityTargetActionRequest {
    action: HouseholdAuthorityAction,
    target: Option<AccountIdentityTarget>,
}

impl AccountIdentityTargetActionRequest {
    pub fn new(action: HouseholdAuthorityAction, target: Option<AccountIdentityTarget>) -> Self {
        Self { action, target }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccountIdentityTargetAuthorityFailure {
    SessionExpiryInvalid,
    SessionExpired,
    TargetRequired,
    TargetUnexpected,
    TargetProfileMismatch,
    TargetDeviceMismatch,
}

pub struct AccountIdentityTargetActionResolution<'a> {
    authority: &'a VerifiedAccountIdentityAuthority,
    action: HouseholdAuthorityAction,
    decision: HouseholdAuthorityDecision,
    target_required: bool,
}

impl AccountIdentityTargetActionResolution<'_> {
    pub fn action(&self) -> HouseholdAuthorityAction {
        self.action
    }

    pub fn decision(&self) -> HouseholdAuthorityDecision {
        self.decision
    }

    pub fn target_child_profile_id(&self) -> Option<&ChildProfileId> {
        self.target_required
            .then_some(self.authority.child_profile_id())
    }

    pub fn target_child_device_id(&self) -> Option<&AccountIdentityChildDeviceId> {
        self.target_required
            .then_some(self.authority.child_device_id())
    }

    pub fn session_generation(&self) -> u64 {
        self.authority.session_generation()
    }

    pub fn session_expires_at(&self) -> &str {
        self.authority.session_expires_at()
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority.authority_generation()
    }

    pub fn requires_parent_step_up(&self) -> bool {
        crate::household_authority::requires_parent_step_up(self.action)
    }
}

pub fn resolve_target_action_from_verified_authority<'a>(
    authority: &'a VerifiedAccountIdentityAuthority,
    request: &AccountIdentityTargetActionRequest,
) -> Result<AccountIdentityTargetActionResolution<'a>, AccountIdentityTargetAuthorityFailure> {
    resolve_target_action_at(authority, request, Utc::now())
}

pub(crate) fn resolve_target_action_at<'a>(
    authority: &'a VerifiedAccountIdentityAuthority,
    request: &AccountIdentityTargetActionRequest,
    now: DateTime<Utc>,
) -> Result<AccountIdentityTargetActionResolution<'a>, AccountIdentityTargetAuthorityFailure> {
    let session_expires_at = DateTime::parse_from_rfc3339(authority.session_expires_at())
        .map_err(|_| AccountIdentityTargetAuthorityFailure::SessionExpiryInvalid)?
        .with_timezone(&Utc);
    (session_expires_at > now)
        .then_some(())
        .ok_or(AccountIdentityTargetAuthorityFailure::SessionExpired)?;

    let target_required = action_requires_target(request.action);
    validate_target(
        target_required,
        action_requires_child_device_target(request.action),
        request.target.as_ref(),
        authority,
    )?;

    let (account_state, membership_state, device_trust_state, session_freshness_state, _, _, _, _) =
        authority.report_query_custody_states();
    let decision = authorize_household_actor_target_action(HouseholdActorTargetAuthorityInput {
        actor_role: account_identity_authority_value_mapping::map_role(authority.role()),
        same_family: authority.household_id() == &authority.current_binding().household_id,
        actor_account_state: account_identity_authority_value_mapping::map_account_state(
            account_state,
        ),
        membership_state: account_identity_authority_value_mapping::map_membership_state(
            membership_state,
        ),
        child_profile_binding_state: account_identity_authority_value_mapping::map_binding_state(
            authority.current_binding(),
        ),
        actor_device_ownership_scope: account_identity_authority_value_mapping::map_device_scope(
            authority.role(),
        ),
        // ViewChildStatus is profile-targeted at the request boundary, but
        // its device authority is still derived from the sealed binding.
        target_device_ownership_scope: target_required
            .then_some(DeviceOwnershipScope::ChildProfileDevice),
        device_trust_state: account_identity_authority_value_mapping::map_device_trust(
            device_trust_state,
        ),
        session_freshness_state: account_identity_authority_value_mapping::map_session_freshness(
            session_freshness_state,
        ),
        action: request.action,
    });

    Ok(AccountIdentityTargetActionResolution {
        authority,
        action: request.action,
        decision,
        target_required,
    })
}

fn validate_target(
    target_required: bool,
    child_device_required: bool,
    target: Option<&AccountIdentityTarget>,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), AccountIdentityTargetAuthorityFailure> {
    match (target_required, child_device_required, target) {
        (false, false, None) => Ok(()),
        (false, _, Some(_)) => Err(AccountIdentityTargetAuthorityFailure::TargetUnexpected),
        (true, false, Some(AccountIdentityTarget::ChildProfile(profile))) => {
            ensure_target_profile(profile, authority)
        }
        (true, false, Some(_)) => Err(AccountIdentityTargetAuthorityFailure::TargetUnexpected),
        (
            true,
            true,
            Some(AccountIdentityTarget::ChildDevice {
                child_profile_id,
                child_device_id,
            }),
        ) => {
            ensure_target_profile(child_profile_id, authority)?;
            ensure_target_device(child_device_id, authority)
        }
        (true, true, Some(_)) => Err(AccountIdentityTargetAuthorityFailure::TargetDeviceMismatch),
        (true, _, None) => Err(AccountIdentityTargetAuthorityFailure::TargetRequired),
        (false, _, None) => Ok(()),
    }
}

fn ensure_target_profile(
    target: &ChildProfileId,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), AccountIdentityTargetAuthorityFailure> {
    (target == authority.child_profile_id())
        .then_some(())
        .ok_or(AccountIdentityTargetAuthorityFailure::TargetProfileMismatch)
}

fn ensure_target_device(
    target: &AccountIdentityChildDeviceId,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), AccountIdentityTargetAuthorityFailure> {
    (target == authority.child_device_id())
        .then_some(())
        .ok_or(AccountIdentityTargetAuthorityFailure::TargetDeviceMismatch)
}

fn action_requires_target(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::PairChildDevice
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
            | HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::ViewChildStatus
            | HouseholdAuthorityAction::ChangePolicy
            | HouseholdAuthorityAction::ImportRestoreData
            | HouseholdAuthorityAction::StartRemoteView
            | HouseholdAuthorityAction::StartRemoteControl
    )
}

fn action_requires_child_device_target(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::PairChildDevice
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
            | HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::ChangePolicy
            | HouseholdAuthorityAction::ImportRestoreData
            | HouseholdAuthorityAction::StartRemoteView
            | HouseholdAuthorityAction::StartRemoteControl
    )
}
