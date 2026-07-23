use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    authorize_household_action, requires_parent_step_up, validate_parent_step_up_assertion,
    AuditRequirementState, ElevatedConfirmationState, HouseholdAuthorityAction,
    HouseholdAuthorityInput, HouseholdAuthorizationFailureReason, HouseholdAuthorizationState,
    ParentControllerLeaseState, ParentStepUpAssertionSnapshot, ParentStepUpValidationDecision,
    ParentStepUpValidationFailureReason, ParentStepUpValidationInput,
};

fn trusted_parent_input(action: HouseholdAuthorityAction) -> HouseholdAuthorityInput {
    HouseholdAuthorityInput {
        actor_role: HouseholdRole::ParentOwner,
        same_family: true,
        actor_account_state: ActorAccountState::Active,
        membership_state: HouseholdMembershipState::Active,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        device_trust_state: DeviceTrustState::Trusted,
        session_freshness_state: SessionFreshnessState::Fresh,
        capability_granted: true,
        controller_lease_state: None,
        recovery_repair_authorized: false,
        action,
    }
}

const PARENT_ACTION_DEVICE_ID: &str = "device-parent-1";
const TARGET_CHILD_PROFILE_ID: &str = "child-1";

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
        actor_role: HouseholdRole::CoParentGuardian,
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
fn export_delete_requires_parent_owner_authority() {
    let owner_decision = authorize_household_action(trusted_parent_input(
        HouseholdAuthorityAction::ExportDeleteData,
    ));

    assert_eq!(
        owner_decision.authorization_state,
        HouseholdAuthorizationState::Authorized
    );
    assert_eq!(
        owner_decision.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        owner_decision.elevated_confirmation_state,
        ElevatedConfirmationState::Required
    );
    assert_eq!(owner_decision.failure_reason, None);

    let guardian_decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::CoParentGuardian,
        ..trusted_parent_input(HouseholdAuthorityAction::ExportDeleteData)
    });

    assert_eq!(
        guardian_decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        guardian_decision.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        guardian_decision.elevated_confirmation_state,
        ElevatedConfirmationState::Required
    );
    assert_eq!(
        guardian_decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::RoleNotAuthorized)
    );
}

#[test]
fn observer_can_view_child_status_but_cannot_change_policy() {
    let view_decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::Observer,
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
        actor_role: HouseholdRole::Observer,
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
fn child_device_agent_cannot_use_parent_controller_authority() {
    let remote_view = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::ChildDeviceAgent,
        action: HouseholdAuthorityAction::StartRemoteView,
        ..trusted_parent_input(HouseholdAuthorityAction::StartRemoteView)
    });

    assert_eq!(
        remote_view.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        remote_view.failure_reason,
        Some(HouseholdAuthorizationFailureReason::RoleNotAuthorized)
    );

    let policy_change = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::ChildDeviceAgent,
        action: HouseholdAuthorityAction::ChangePolicy,
        ..trusted_parent_input(HouseholdAuthorityAction::ChangePolicy)
    });

    assert_eq!(
        policy_change.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        policy_change.failure_reason,
        Some(HouseholdAuthorizationFailureReason::RoleNotAuthorized)
    );
}

#[test]
fn remote_view_requires_capability_grant() {
    let decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::CoParentGuardian,
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
fn active_controller_lease_allows_remote_control() {
    let decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::CoParentGuardian,
        controller_lease_state: Some(ParentControllerLeaseState::Active),
        action: HouseholdAuthorityAction::StartRemoteControl,
        ..trusted_parent_input(HouseholdAuthorityAction::StartRemoteControl)
    });

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
fn missing_controller_lease_blocks_remote_view() {
    let decision = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::CoParentGuardian,
        action: HouseholdAuthorityAction::StartRemoteView,
        ..trusted_parent_input(HouseholdAuthorityAction::StartRemoteView)
    });

    assert_eq!(
        decision.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        decision.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        decision.elevated_confirmation_state,
        ElevatedConfirmationState::NotRequired
    );
    assert_eq!(
        decision.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ControllerLeaseRequired)
    );
}

#[test]
fn expired_or_revoked_controller_lease_is_denied() {
    let expired_lease = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::CoParentGuardian,
        controller_lease_state: Some(ParentControllerLeaseState::Expired),
        action: HouseholdAuthorityAction::StartRemoteView,
        ..trusted_parent_input(HouseholdAuthorityAction::StartRemoteView)
    });

    assert_eq!(
        expired_lease.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        expired_lease.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        expired_lease.elevated_confirmation_state,
        ElevatedConfirmationState::NotRequired
    );
    assert_eq!(
        expired_lease.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ControllerLeaseExpired)
    );

    let revoked_lease = authorize_household_action(HouseholdAuthorityInput {
        actor_role: HouseholdRole::CoParentGuardian,
        controller_lease_state: Some(ParentControllerLeaseState::Revoked),
        action: HouseholdAuthorityAction::StartRemoteControl,
        ..trusted_parent_input(HouseholdAuthorityAction::StartRemoteControl)
    });

    assert_eq!(
        revoked_lease.authorization_state,
        HouseholdAuthorizationState::Rejected
    );
    assert_eq!(
        revoked_lease.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        revoked_lease.elevated_confirmation_state,
        ElevatedConfirmationState::Required
    );
    assert_eq!(
        revoked_lease.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ControllerLeaseRevoked)
    );
}

#[test]
fn revoked_or_untrusted_device_is_denied_even_for_parent() {
    for device_trust_state in [
        DeviceTrustState::Pending,
        DeviceTrustState::ResetRequired,
        DeviceTrustState::Revoked,
        DeviceTrustState::Disabled,
    ] {
        let decision = authorize_household_action(HouseholdAuthorityInput {
            device_trust_state,
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
}

#[test]
fn external_household_membership_drift_and_wrong_device_scope_are_denied() {
    let external_household = authorize_household_action(HouseholdAuthorityInput {
        same_family: false,
        action: HouseholdAuthorityAction::PairChildDevice,
        ..trusted_parent_input(HouseholdAuthorityAction::PairChildDevice)
    });
    assert_eq!(
        external_household.failure_reason,
        Some(HouseholdAuthorizationFailureReason::ExternalHousehold)
    );

    let inactive_membership = authorize_household_action(HouseholdAuthorityInput {
        membership_state: HouseholdMembershipState::Invited,
        action: HouseholdAuthorityAction::PairChildDevice,
        ..trusted_parent_input(HouseholdAuthorityAction::PairChildDevice)
    });
    assert_eq!(
        inactive_membership.failure_reason,
        Some(HouseholdAuthorizationFailureReason::MembershipNotActive)
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

#[test]
fn parent_step_up_required_actions_are_explicit() {
    assert!(requires_parent_step_up(
        HouseholdAuthorityAction::PairChildDevice
    ));
    assert!(requires_parent_step_up(
        HouseholdAuthorityAction::ChangePolicy
    ));
    assert!(!requires_parent_step_up(
        HouseholdAuthorityAction::ViewChildStatus
    ));
}

#[test]
fn validates_parent_step_up_assertions_as_action_device_and_target_bound() {
    let decision = validate_parent_step_up_assertion(&ParentStepUpValidationInput {
        assertion: Some(ParentStepUpAssertionSnapshot {
            family_id: "family-main".to_owned(),
            parent_account_id: "parent-account-1".to_owned(),
            action_device_id: PARENT_ACTION_DEVICE_ID.to_owned(),
            action_device_child_profile_id: None,
            target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
            target_child_device_id: None,
            action: HouseholdAuthorityAction::PairChildDevice,
            nonce: "step-up-nonce-1".to_owned(),
            expires_at: "2026-06-13T16:01:00.000Z".to_owned(),
        }),
        family_id: "family-main".to_owned(),
        parent_account_id: "parent-account-1".to_owned(),
        action_device_id: PARENT_ACTION_DEVICE_ID.to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
        target_child_device_id: None,
        action: HouseholdAuthorityAction::PairChildDevice,
        observed_at: "2026-06-13T15:58:00.000Z".to_owned(),
        expected_nonce: Some("step-up-nonce-1".to_owned()),
    });

    assert_eq!(
        decision,
        ParentStepUpValidationDecision {
            valid: true,
            failure_reason: None,
        }
    );
}

#[test]
fn rejects_expired_or_replayed_parent_step_up_assertions() {
    let expired = validate_parent_step_up_assertion(&ParentStepUpValidationInput {
        assertion: Some(ParentStepUpAssertionSnapshot {
            family_id: "family-main".to_owned(),
            parent_account_id: "parent-account-1".to_owned(),
            action_device_id: PARENT_ACTION_DEVICE_ID.to_owned(),
            action_device_child_profile_id: None,
            target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
            target_child_device_id: None,
            action: HouseholdAuthorityAction::PairChildDevice,
            nonce: "step-up-nonce-1".to_owned(),
            expires_at: "2026-06-13T16:01:00.000Z".to_owned(),
        }),
        family_id: "family-main".to_owned(),
        parent_account_id: "parent-account-1".to_owned(),
        action_device_id: PARENT_ACTION_DEVICE_ID.to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
        target_child_device_id: None,
        action: HouseholdAuthorityAction::PairChildDevice,
        observed_at: "2026-06-13T16:02:00.000Z".to_owned(),
        expected_nonce: Some("step-up-nonce-1".to_owned()),
    });

    assert_eq!(
        expired.failure_reason,
        Some(ParentStepUpValidationFailureReason::Expired)
    );

    let replayed = validate_parent_step_up_assertion(&ParentStepUpValidationInput {
        assertion: Some(ParentStepUpAssertionSnapshot {
            family_id: "family-main".to_owned(),
            parent_account_id: "parent-account-1".to_owned(),
            action_device_id: PARENT_ACTION_DEVICE_ID.to_owned(),
            action_device_child_profile_id: None,
            target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
            target_child_device_id: None,
            action: HouseholdAuthorityAction::PairChildDevice,
            nonce: "step-up-nonce-1".to_owned(),
            expires_at: "2026-06-13T16:01:00.000Z".to_owned(),
        }),
        family_id: "family-main".to_owned(),
        parent_account_id: "parent-account-1".to_owned(),
        action_device_id: PARENT_ACTION_DEVICE_ID.to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
        target_child_device_id: None,
        action: HouseholdAuthorityAction::PairChildDevice,
        observed_at: "2026-06-13T15:58:00.000Z".to_owned(),
        expected_nonce: Some("different-nonce".to_owned()),
    });

    assert_eq!(
        replayed.failure_reason,
        Some(ParentStepUpValidationFailureReason::ReplayRejected)
    );
}

#[test]
fn parent_step_up_assertion_debug_redacts_identity_device_and_nonce_material() {
    let assertion = ParentStepUpAssertionSnapshot {
        family_id: "family-sensitive".to_owned(),
        parent_account_id: "parent-sensitive".to_owned(),
        action_device_id: "parent-device-sensitive".to_owned(),
        action_device_child_profile_id: Some("parent-child-sensitive".to_owned()),
        target_child_profile_id: Some("target-profile-sensitive".to_owned()),
        target_child_device_id: Some("target-device-sensitive".to_owned()),
        action: HouseholdAuthorityAction::PairChildDevice,
        nonce: "step-up-nonce-sensitive".to_owned(),
        expires_at: "2099-01-01T00:00:00.000Z".to_owned(),
    };

    let debug = format!("{assertion:?}");
    assert_eq!(
        debug,
        "ParentStepUpAssertionSnapshot { family_id: \"[redacted]\", parent_account_id: \"[redacted]\", action_device_id: \"[redacted]\", action_device_child_profile_id: \"[redacted]\", target_child_profile_id: \"[redacted]\", target_child_device_id: \"[redacted]\", action: \"[redacted]\", nonce: \"[redacted]\", expires_at: \"[redacted]\" }"
    );
}
