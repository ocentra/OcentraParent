use ocentra_eventing::envelope::DomainEvent;
use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    FamilyActorRole, HouseholdMembership, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorityInput, ParentControllerLeaseState,
};
use ocentra_family_identity_core::session_lifecycle::{
    SessionActivityState, SessionCredentialKind, SessionLifecycleAction, SessionTokenInput,
    TokenReplayState, TokenValidityWindowState,
};
use ocentra_family_identity_core::setup_lifecycle::{
    RecoveryIdentityProofState, RecoveryKind as FamilyRecoveryKind,
    RecoveryOperation as FamilyRecoveryOperation, RecoveryState as FamilyRecoveryState,
    RecoverySupportChannel, SetupInviteInput, SetupInvitePurpose, SetupInviteReplayState,
    SetupInviteState, SetupInviteTargetRole,
};
use ocentra_provisioning_core::provisioning_install::{
    derive_provisioning_readiness_input_from_family_context, evaluate_provisioning_readiness,
    plan_provisioning_actions, provisioning_action_planned_event,
    provisioning_readiness_evaluated_event, AccountReadinessState, ChildAppReadinessState,
    ChildInstallState, ChildRuntimeReadinessState, ChildServiceState, DataCustodySyncState,
    NetworkReachabilityState, PairingLifecycleState, ParentAppReadinessState,
    ParentDeviceRegistrationState, PermissionReadinessState, PolicyBaselineState,
    ProvisioningActionPlan, ProvisioningActionPlanId, ProvisioningAggregateId,
    ProvisioningAuditState, ProvisioningBlockerReason, ProvisioningChildRuntimeStartAction,
    ProvisioningFamilyContextInput, ProvisioningManualStepState, ProvisioningOverallState,
    ProvisioningReadinessDecision, ProvisioningReadinessEvaluationId, ProvisioningReadinessInput,
    ProvisioningRecoveryAction, RecoveryState,
};
use serde_json::Value;

const PROVISIONING_AGGREGATE_ID: &str = "provisioning-family-default";
const PROVISIONING_EVALUATION_ID: &str = "provisioning-readiness-default";
const PROVISIONING_READINESS_EVENT_TYPE: &str = "provisioning.readiness.evaluated";
const PROVISIONING_ACTION_EVENT_TYPE: &str = "provisioning.action.planned";

fn ready_input() -> ProvisioningReadinessInput {
    ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        account_readiness_state: AccountReadinessState::Ready,
        parent_app_readiness_state: ParentAppReadinessState::Ready,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_install_state: ChildInstallState::Installed,
        child_service_state: ChildServiceState::ServiceStarted,
        child_app_readiness_state: ChildAppReadinessState::Ready,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        device_trust_state: DeviceTrustState::Trusted,
        permission_readiness_state: PermissionReadinessState::Granted,
        pairing_lifecycle_state: PairingLifecycleState::Trusted,
        policy_baseline_state: PolicyBaselineState::Applied,
        data_custody_sync_state: DataCustodySyncState::Synced,
        network_reachability_state: NetworkReachabilityState::Reachable,
        recovery_state: RecoveryState::Normal,
    }
}

fn ready_family_context() -> ProvisioningFamilyContextInput {
    ProvisioningFamilyContextInput {
        account_matches_invite_target: true,
        setup_invite_input: SetupInviteInput {
            inviter_role: FamilyActorRole::Parent,
            household_membership: HouseholdMembership::Member,
            purpose: SetupInvitePurpose::ChildDevicePairing,
            target_role: SetupInviteTargetRole::ChildDeviceAgent,
            invite_state: SetupInviteState::Accepted,
            single_use: true,
            replay_state: SetupInviteReplayState::Fresh,
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
            actor_role: FamilyActorRole::Parent,
            actor_account_state: ActorAccountState::Active,
            household_membership: HouseholdMembership::Member,
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

fn evaluate_family_context(
    input: ProvisioningFamilyContextInput,
) -> (
    ProvisioningReadinessInput,
    ProvisioningReadinessDecision,
    ProvisioningActionPlan,
) {
    let projected_input = derive_provisioning_readiness_input_from_family_context(input);
    let readiness = evaluate_provisioning_readiness(projected_input);
    let actions = plan_provisioning_actions(projected_input);

    (projected_input, readiness, actions)
}

#[test]
fn ready_provisioning_starts_child_runtime_without_manual_step() {
    let input = ready_input();

    let readiness = evaluate_provisioning_readiness(input);
    let actions = plan_provisioning_actions(input);

    assert_eq!(
        readiness.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(
        readiness.manual_step_state,
        ProvisioningManualStepState::NotRequired
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Ready);
    assert_eq!(readiness.blocker_reason, None);
    assert_eq!(
        actions.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::Start
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::Continue
    );
    assert_eq!(actions.audit_state, ProvisioningAuditState::Record);
}

#[test]
fn custody_sync_pending_keeps_runtime_blocked_but_marks_state_degraded() {
    let input = ProvisioningReadinessInput {
        data_custody_sync_state: DataCustodySyncState::SyncPending,
        ..ready_input()
    };

    let readiness = evaluate_provisioning_readiness(input);
    let actions = plan_provisioning_actions(input);

    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::DataCustodySyncPending)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Degraded);
    assert_eq!(
        actions.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::DoNotStart
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RepairCustodySync
    );
}

#[test]
fn readiness_event_projects_typed_action_event() {
    let readiness_event = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse(PROVISIONING_AGGREGATE_ID)
            .expect("provisioning aggregate id"),
        ProvisioningReadinessEvaluationId::parse(PROVISIONING_EVALUATION_ID)
            .expect("provisioning evaluation id"),
        ready_input(),
    );

    let action_event = provisioning_action_planned_event(readiness_event.clone());

    assert_eq!(
        readiness_event
            .contract()
            .expect("provisioning readiness contract")
            .event_type
            .as_str(),
        PROVISIONING_READINESS_EVENT_TYPE
    );
    assert_eq!(
        action_event
            .contract()
            .expect("provisioning action contract")
            .event_type
            .as_str(),
        PROVISIONING_ACTION_EVENT_TYPE
    );
    assert_eq!(action_event.aggregate_id, readiness_event.aggregate_id);
    assert_eq!(
        action_event.source_evaluation_id,
        readiness_event.evaluation_id
    );
    assert!(
        ProvisioningActionPlanId::parse(action_event.action_plan_id.as_str()).is_ok(),
        "action plan id remains branded"
    );
}

#[test]
fn bootstrap_audit_events_omit_raw_pairing_session_fields() {
    let family_context = ProvisioningFamilyContextInput {
        pairing_session_input: SessionTokenInput {
            replay_state: TokenReplayState::ReplayDetected,
            ..ready_family_context().pairing_session_input
        },
        ..ready_family_context()
    };
    let projected_input = derive_provisioning_readiness_input_from_family_context(family_context);
    let readiness_event = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse(PROVISIONING_AGGREGATE_ID)
            .expect("provisioning aggregate id"),
        ProvisioningReadinessEvaluationId::parse(PROVISIONING_EVALUATION_ID)
            .expect("provisioning evaluation id"),
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

    let readiness_json =
        serde_json::to_value(&readiness_event).expect("readiness event serializes");
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

    let action_json = serde_json::to_value(&action_event).expect("action event serializes");
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
}

#[test]
fn family_context_projects_trusted_pairing_into_ready_provisioning() {
    let (projected_input, readiness, actions) = evaluate_family_context(ready_family_context());

    assert_eq!(
        projected_input.account_readiness_state,
        AccountReadinessState::Ready
    );
    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::Trusted
    );
    assert_eq!(projected_input.recovery_state, RecoveryState::Normal);
    assert_eq!(
        readiness.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::Continue
    );
}

#[test]
fn family_context_installed_not_started_projects_explicit_start_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            child_service_state: ChildServiceState::NotStarted,
            child_app_readiness_state: ChildAppReadinessState::Installed,
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.child_install_state,
        ChildInstallState::Installed
    );
    assert_eq!(
        projected_input.child_service_state,
        ChildServiceState::NotStarted
    );
    assert_eq!(
        projected_input.child_app_readiness_state,
        ChildAppReadinessState::Installed
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildServiceNotStarted)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::StartChildService
    );
}

#[test]
fn family_context_offline_service_projects_degraded_runtime_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            child_service_state: ChildServiceState::Offline,
            child_app_readiness_state: ChildAppReadinessState::Offline,
            network_reachability_state: NetworkReachabilityState::OfflineChild,
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.child_install_state,
        ChildInstallState::Installed
    );
    assert_eq!(
        projected_input.child_service_state,
        ChildServiceState::Offline
    );
    assert_eq!(
        projected_input.child_app_readiness_state,
        ChildAppReadinessState::Offline
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildAppOffline)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Degraded);
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::WaitForChildConnectivity
    );
}

#[test]
fn family_context_reinstall_required_projects_reinstall_recovery() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            child_install_state: ChildInstallState::ReinstallRequired,
            child_service_state: ChildServiceState::NotStarted,
            child_app_readiness_state: ChildAppReadinessState::ReinstallRequired,
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.child_install_state,
        ChildInstallState::ReinstallRequired
    );
    assert_eq!(
        projected_input.child_service_state,
        ChildServiceState::NotStarted
    );
    assert_eq!(
        projected_input.recovery_state,
        RecoveryState::ChildReinstall
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildAppReinstallRequired)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::ReinstallChildApp
    );
}

#[test]
fn family_context_replay_detection_reissues_pairing_code() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            pairing_session_input: SessionTokenInput {
                replay_state: TokenReplayState::ReplayDetected,
                ..ready_family_context().pairing_session_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::Replayed
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingReplayRejected)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::ReissuePairingCode
    );
}

#[test]
fn family_context_accepted_pairing_waits_for_parent_trust_confirmation() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            setup_invite_input: SetupInviteInput {
                invite_state: SetupInviteState::Accepted,
                ..ready_family_context().setup_invite_input
            },
            household_authority_input: HouseholdAuthorityInput {
                device_trust_state: DeviceTrustState::Pending,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::Accepted
    );
    assert_eq!(
        projected_input.device_trust_state,
        DeviceTrustState::Pending
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildDeviceTrustRequired)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::TrustChildDevice
    );
}

#[test]
fn family_context_wrong_household_surfaces_pairing_blocker() {
    let (projected_input, readiness, _) = evaluate_family_context(ProvisioningFamilyContextInput {
        setup_invite_input: SetupInviteInput {
            household_membership: HouseholdMembership::External,
            invite_state: SetupInviteState::Pending,
            ..ready_family_context().setup_invite_input
        },
        ..ready_family_context()
    });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::WrongHousehold
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingWrongHousehold)
    );
}

#[test]
fn family_context_wrong_device_scope_surfaces_explicit_pairing_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            household_authority_input: HouseholdAuthorityInput {
                device_ownership_scope: DeviceOwnershipScope::OtherDevice,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::WrongDevice
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingWrongDevice)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RePairChildDevice
    );
}

#[test]
fn family_context_unbound_child_profile_surfaces_anonymous_device_pairing_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            household_authority_input: HouseholdAuthorityInput {
                child_profile_binding_state: ChildProfileBindingState::Missing,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::AnonymousDevice
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingAnonymousDevice)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RePairChildDevice
    );
}

#[test]
fn family_context_parent_role_required_surfaces_explicit_pairing_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            household_authority_input: HouseholdAuthorityInput {
                actor_role: FamilyActorRole::Observer,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::ParentRoleRequired
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingParentRoleRequired)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::SwitchToCorrectAccount
    );
}

#[test]
fn family_context_stale_signed_hello_reissues_pairing_code() {
    for validity_window_state in [
        TokenValidityWindowState::Expired,
        TokenValidityWindowState::NotYetValid,
    ] {
        let (projected_input, readiness, actions) =
            evaluate_family_context(ProvisioningFamilyContextInput {
                pairing_session_input: SessionTokenInput {
                    validity_window_state,
                    ..ready_family_context().pairing_session_input
                },
                ..ready_family_context()
            });

        assert_eq!(
            projected_input.pairing_lifecycle_state,
            PairingLifecycleState::StaleSignedHello
        );
        assert_eq!(
            readiness.blocker_reason,
            Some(ProvisioningBlockerReason::PairingStaleSignedHello)
        );
        assert_eq!(
            actions.recovery_action,
            ProvisioningRecoveryAction::ReissuePairingCode
        );
    }
}

#[test]
fn family_context_support_recovery_handoff_blocks_custody_sync_first() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            recovery_operation: Some(FamilyRecoveryOperation {
                requester_role: FamilyActorRole::Parent,
                household_membership: HouseholdMembership::Member,
                kind: FamilyRecoveryKind::CompromisedAccount,
                state: FamilyRecoveryState::Approved,
                owner_approval_required: false,
                identity_proof_state: RecoveryIdentityProofState::Verified,
                support_channel: RecoverySupportChannel::SupportAssisted,
                delete_export_handoff_required: true,
            }),
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.data_custody_sync_state,
        DataCustodySyncState::Blocked
    );
    assert_eq!(
        projected_input.recovery_state,
        RecoveryState::PermissionLoss
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::DataCustodySyncBlocked)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RepairCustodySync
    );
}
