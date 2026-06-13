use ocentra_family_identity_core::{
    authorize_household_action, ActorAccountState, AuditRequirementState, ChildProfileBindingState,
    DeviceOwnershipScope, DeviceTrustState, ElevatedConfirmationState, FamilyActorRole,
    HouseholdAuthorityAction, HouseholdAuthorityInput, HouseholdAuthorizationFailureReason,
    HouseholdAuthorizationState, HouseholdMembership, SessionFreshnessState,
};

fn trusted_parent_input(action: HouseholdAuthorityAction) -> HouseholdAuthorityInput {
    HouseholdAuthorityInput {
        actor_role: FamilyActorRole::Parent,
        actor_account_state: ActorAccountState::Active,
        household_membership: HouseholdMembership::Member,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        device_trust_state: DeviceTrustState::Trusted,
        session_freshness_state: SessionFreshnessState::Fresh,
        capability_granted: true,
        action,
    }
}

#[test]
fn parent_can_manage_billing_for_member_household() {
    let decision = authorize_household_action(trusted_parent_input(
        HouseholdAuthorityAction::ManageBilling,
    ));

    assert_eq!(
        decision.authorization_state,
        HouseholdAuthorizationState::Authorized
    );
    assert_eq!(
        decision.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        decision.elevated_confirmation_state,
        ElevatedConfirmationState::Required
    );
    assert_eq!(decision.failure_reason, None);
}

#[test]
fn guardian_cannot_manage_billing() {
    let decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: FamilyActorRole::Guardian,
        ..trusted_parent_input(HouseholdAuthorityAction::ManageBilling)
    });

    assert_eq!(
        decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::RoleNotAuthorized)
    );
}

#[test]
fn observer_can_view_child_status_but_cannot_change_policy() {
    let view_decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: FamilyActorRole::Observer,
        capability_granted: false,
        action: HouseholdAuthorityAction::ViewChildStatus,
        ..trusted_parent_input(HouseholdAuthorityAction::ViewChildStatus)
    });

    assert_eq!(
        view_decision.authorization_state,
        HouseholdAuthorizationState::Authorized
    );
    assert_eq!(
        view_decision.audit_requirement_state,
        AuditRequirementState::NotRequired
    );

    let policy_decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: FamilyActorRole::Observer,
        action: HouseholdAuthorityAction::ChangePolicy,
        ..trusted_parent_input(HouseholdAuthorityAction::ChangePolicy)
    });

    assert_eq!(
        policy_decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        policy_decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::RoleNotAuthorized)
    );
}

#[test]
fn remote_view_requires_capability_grant() {
    let decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: FamilyActorRole::Guardian,
        capability_granted: false,
        action: HouseholdAuthorityAction::StartRemoteView,
        ..trusted_parent_input(HouseholdAuthorityAction::StartRemoteView)
    });

    assert_eq!(
        decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::MissingCapabilityGrant)
    );
}

#[test]
fn stale_session_blocks_remote_control() {
    let decision = authorize_household_action(HouseholdAuthorityInput {
        session_freshness_state: SessionFreshnessState::Stale,
        action: HouseholdAuthorityAction::StartRemoteControl,
        ..trusted_parent_input(HouseholdAuthorityAction::StartRemoteControl)
    });

    assert_eq!(
        decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::SessionNotFresh)
    );
}

#[test]
fn revoked_or_untrusted_device_is_denied_even_for_parent() {
    let decision = authorize_household_action(HouseholdAuthorityInput {
        device_trust_state: DeviceTrustState::Revoked,
        action: HouseholdAuthorityAction::ViewChildStatus,
        ..trusted_parent_input(HouseholdAuthorityAction::ViewChildStatus)
    });

    assert_eq!(
        decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::DeviceNotTrusted)
    );
}

#[test]
fn external_household_and_wrong_device_scope_are_denied() {
    let external_household = authorize_household_action(HouseholdAuthorityInput {
        household_membership: HouseholdMembership::External,
        action: HouseholdAuthorityAction::PairChildDevice,
        ..trusted_parent_input(HouseholdAuthorityAction::PairChildDevice)
    });
    assert_eq!(
        external_household.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ExternalHousehold)
    );

    let wrong_scope = authorize_household_action(HouseholdAuthorityInput {
        device_ownership_scope: DeviceOwnershipScope::OtherDevice,
        action: HouseholdAuthorityAction::PairChildDevice,
        ..trusted_parent_input(HouseholdAuthorityAction::PairChildDevice)
    });
    assert_eq!(
        wrong_scope.failure_reason,
        Some(HouseholdAuthorizationFailureReason::WrongDeviceScope)
    );
}
