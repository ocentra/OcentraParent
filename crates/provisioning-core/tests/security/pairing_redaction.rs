use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorityInput, ParentControllerLeaseState,
};
use ocentra_family_identity_core::session_lifecycle::{
    SessionActivityState, SessionCredentialKind, SessionLifecycleAction, SessionTokenInput,
    TokenReplayState, TokenValidityWindowState,
};
use ocentra_family_identity_core::setup_lifecycle::{
    SetupInviteInput, SetupInvitePurpose, SetupInviteReplayState, SetupInviteState,
    SetupInviteTargetRole, SetupRecoveryAbuseState, SetupRecoveryResponseTimingState,
};
use ocentra_provisioning_core::provisioning_install::{
    derive_provisioning_readiness_input_from_family_context, provisioning_action_planned_event,
    provisioning_readiness_evaluated_event, ChildAppReadinessState, ChildInstallState,
    ChildServiceState, DataCustodySyncState, NetworkReachabilityState, PairingLifecycleState,
    ParentAppReadinessState, ParentDeviceRegistrationState, PermissionReadinessState,
    PolicyBaselineState, ProvisioningAggregateId, ProvisioningAuditState,
    ProvisioningBlockerReason, ProvisioningFamilyContextInput, ProvisioningReadinessEvaluationId,
};
use serde_json::Value;

const PROVISIONING_AGGREGATE_ID: &str = "provisioning-family-default";
const PROVISIONING_EVALUATION_ID: &str = "provisioning-readiness-default";

fn ready_family_context() -> ProvisioningFamilyContextInput {
    ProvisioningFamilyContextInput {
        account_matches_invite_target: true,
        setup_invite_input: SetupInviteInput {
            inviter_role: HouseholdRole::ParentOwner,
            same_family: true,
            purpose: SetupInvitePurpose::ChildDevicePairing,
            target_role: SetupInviteTargetRole::ChildDeviceAgent,
            invite_state: SetupInviteState::Accepted,
            single_use: true,
            replay_state: SetupInviteReplayState::Fresh,
            abuse_state: SetupRecoveryAbuseState::WithinLimit,
            response_timing_state: SetupRecoveryResponseTimingState::Uniform,
        },
        pairing_session_input: SessionTokenInput {
            credential_kind: SessionCredentialKind::PairingToken,
            action: SessionLifecycleAction::AcceptPairingToken,
            activity_state: SessionActivityState::Active,
            replay_state: TokenReplayState::Fresh,
            validity_window_state: TokenValidityWindowState::Valid,
            session_freshness_state: SessionFreshnessState::Fresh,
        },
        household_authority_input: HouseholdAuthorityInput {
            actor_role: HouseholdRole::ParentOwner,
            same_family: true,
            actor_account_state: ActorAccountState::Active,
            membership_state: HouseholdMembershipState::Active,
            child_profile_binding_state: ChildProfileBindingState::Bound,
            device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
            device_trust_state: DeviceTrustState::Trusted,
            session_freshness_state: SessionFreshnessState::Fresh,
            capability_granted: true,
            controller_lease_state: Some(ParentControllerLeaseState::Active),
            action: HouseholdAuthorityAction::PairChildDevice,
        },
        recovery_operation: None,
        parent_app_readiness_state: ParentAppReadinessState::Ready,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_install_state: ChildInstallState::Installed,
        child_service_state: ChildServiceState::ServiceStarted,
        child_app_readiness_state: ChildAppReadinessState::Ready,
        permission_readiness_state: PermissionReadinessState::Granted,
        policy_baseline_state: PolicyBaselineState::Applied,
        data_custody_sync_state: DataCustodySyncState::Synced,
        network_reachability_state: NetworkReachabilityState::Reachable,
    }
}

#[test]
fn bootstrap_audit_events_omit_raw_pairing_session_fields() -> Result<(), Box<dyn std::error::Error>>
{
    let family_context = ProvisioningFamilyContextInput {
        pairing_session_input: SessionTokenInput {
            replay_state: TokenReplayState::ReplayDetected,
            ..ready_family_context().pairing_session_input
        },
        ..ready_family_context()
    };
    let projected_input = derive_provisioning_readiness_input_from_family_context(family_context);
    let readiness_event = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse(PROVISIONING_AGGREGATE_ID)?,
        ProvisioningReadinessEvaluationId::parse(PROVISIONING_EVALUATION_ID)?,
        projected_input,
    );
    let action_event = provisioning_action_planned_event(readiness_event.clone());

    assert_eq!(
        readiness_event.input.pairing_lifecycle_state,
        PairingLifecycleState::Replayed
    );
    assert_eq!(
        readiness_event.decision.blocker_reason,
        Some(ProvisioningBlockerReason::PairingReplayRejected)
    );
    assert_eq!(
        action_event.action_plan.audit_state,
        ProvisioningAuditState::Record
    );

    let readiness_json = serde_json::to_value(&readiness_event)?;
    assert_eq!(
        readiness_json["input"]["pairing_lifecycle_state"],
        Value::String(String::from("replayed"))
    );
    assert_eq!(
        readiness_json["decision"]["blocker_reason"],
        Value::String(String::from("pairing-replay-rejected"))
    );
    assert!(readiness_json.get("pairing_session_input").is_none());
    assert!(readiness_json["input"].get("credential_kind").is_none());
    assert!(readiness_json["input"].get("replay_state").is_none());
    assert!(readiness_json["input"]
        .get("validity_window_state")
        .is_none());

    let action_json = serde_json::to_value(&action_event)?;
    assert_eq!(
        action_json["action_plan"]["audit_state"],
        Value::String(String::from("record"))
    );
    assert!(action_json.get("pairing_session_input").is_none());
    assert!(action_json["action_plan"].get("credential_kind").is_none());
    assert!(action_json["action_plan"].get("replay_state").is_none());
    assert!(action_json["action_plan"]
        .get("validity_window_state")
        .is_none());

    Ok(())
}

#[test]
fn accepted_pairing_replay_projects_a_redacted_rejection_without_invite_fields(
) -> Result<(), Box<dyn std::error::Error>> {
    let family_context = ProvisioningFamilyContextInput {
        setup_invite_input: SetupInviteInput {
            replay_state: SetupInviteReplayState::ReplayDetected,
            ..ready_family_context().setup_invite_input
        },
        ..ready_family_context()
    };
    let projected_input = derive_provisioning_readiness_input_from_family_context(family_context);
    let readiness_event = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse(PROVISIONING_AGGREGATE_ID)?,
        ProvisioningReadinessEvaluationId::parse(PROVISIONING_EVALUATION_ID)?,
        projected_input,
    );

    assert_eq!(
        readiness_event.input.pairing_lifecycle_state,
        PairingLifecycleState::Replayed
    );
    assert_eq!(
        readiness_event.decision.blocker_reason,
        Some(ProvisioningBlockerReason::PairingReplayRejected)
    );

    let readiness_json = serde_json::to_value(&readiness_event)?;
    assert_eq!(
        readiness_json["input"]["pairing_lifecycle_state"],
        Value::String(String::from("replayed"))
    );
    for field in ["purpose", "target_role", "invite_state", "replay_state"] {
        assert_eq!(readiness_json.get(field), None);
        assert_eq!(readiness_json["input"].get(field), None);
    }

    Ok(())
}
