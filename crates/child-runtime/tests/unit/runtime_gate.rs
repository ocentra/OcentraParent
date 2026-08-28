use ocentra_child_enforcement_core::enforcement_action::{
    EnforcementActionInput, EnforcementActionMode, EnforcementAdapterExecutionState,
    EnforcementAdapterState, EnforcementIdempotencyState, EnforcementRollbackState,
};
use ocentra_child_runtime::runtime_gate as ocentra_child_runtime;
use ocentra_entitlement_core::entitlement_access::{
    EntitlementCapabilityInput, EntitlementCapabilityRejectionReason, EntitlementCapabilityScope,
};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildDisclosureState, ChildProfileBindingState, DeviceOwnershipScope,
    DeviceScopeInput, DeviceTrustState, HouseholdMembershipState, HouseholdRole,
};
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use ocentra_provisioning_core::provisioning_install::{
    AccountReadinessState, ChildAppReadinessState, ChildInstallState, ChildServiceState,
    DataCustodySyncState, NetworkReachabilityState, PairingLifecycleState, ParentAppReadinessState,
    ParentDeviceRegistrationState, PermissionReadinessState, PolicyBaselineState,
    ProvisioningBlockerReason, ProvisioningOverallState, ProvisioningReadinessInput, RecoveryState,
};
use ocentra_remote_access_core::remote_access_session::{
    RemoteAccessInputAuthorityState, RemoteAccessRelayState, RemoteAccessReplayState,
    RemoteAccessSessionAuthorizationState, RemoteAccessSessionRequest,
};
use ocentra_storage_custody_core::storage_custody::{
    ParentExportState, RemoteSyncState, RemoteUploadState, RetentionWindowState,
    StorageCustodyInput, StorageCustodyLocation,
};
use serde_json::json;

#[path = "runtime_gate_tombstone_recovery.rs"]
mod runtime_gate_tombstone_recovery;

trait ResultRequiredExt<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T, E: std::fmt::Debug> ResultRequiredExt<T, E> for Result<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let _ = context;
        self.unwrap_or_else(|_| std::process::abort())
    }
}

#[test]
fn child_runtime_preflight_blocks_when_entitlement_snapshot_is_unverified() {
    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(
        valid_child_runtime_preflight_input(),
    );

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::NotRequired
    );
    assert_eq!(
        decision.storage_custody_decision.remote_upload_state,
        RemoteUploadState::Allowed
    );
    assert_eq!(
        decision.entitlement_decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
}

#[test]
fn child_runtime_preflight_blocks_when_entitlement_is_parent_portal_only() {
    let mut input = valid_child_runtime_preflight_input();
    input.entitlement_input.capability_scope = EntitlementCapabilityScope::ParentPortalOnly;

    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(input);

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::NotRequired
    );
    assert_eq!(
        decision.entitlement_decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
}

#[test]
fn child_runtime_preflight_blocks_when_entitlement_snapshot_is_unavailable() {
    let input = valid_child_runtime_preflight_input();

    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(input);

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::NotRequired
    );
    assert_eq!(
        decision.entitlement_decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
}

#[test]
fn child_runtime_preflight_rejects_caller_supplied_snapshot_trust_context() {
    let mut input = valid_child_runtime_preflight_input();
    input.entitlement_input = entitlement_input_with_snapshot_context(json!({
        "signature_state": "trusted",
        "freshness_state": "fresh",
        "household_binding_state": "matched",
        "device_binding_state": "matched",
        "device_trust_requirement_state": "required",
        "device_trust_state": "present",
        "package_build_state": "valid"
    }));

    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(input);

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::NotRequired
    );
    assert_eq!(
        decision.entitlement_decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
}

#[test]
fn child_runtime_preflight_keeps_offline_child_in_manual_review_state() {
    let mut input = valid_child_runtime_preflight_input();
    input.provisioning_input.child_install_state = ChildInstallState::Installed;
    input.provisioning_input.child_service_state = ChildServiceState::Offline;
    input.provisioning_input.child_app_readiness_state = ChildAppReadinessState::Offline;
    input.provisioning_input.network_reachability_state = NetworkReachabilityState::OfflineChild;

    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(input);

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::Required
    );
    assert_eq!(
        decision.provisioning_decision.overall_state,
        ProvisioningOverallState::Degraded
    );
    assert_eq!(
        decision.provisioning_decision.blocker_reason,
        Some(ProvisioningBlockerReason::ChildAppOffline)
    );
}

#[test]
fn child_runtime_preflight_blocks_installed_not_started_separately_from_offline() {
    let mut input = valid_child_runtime_preflight_input();
    input.provisioning_input.child_install_state = ChildInstallState::Installed;
    input.provisioning_input.child_service_state = ChildServiceState::NotStarted;
    input.provisioning_input.child_app_readiness_state = ChildAppReadinessState::Installed;

    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(input);

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::Required
    );
    assert_eq!(
        decision.provisioning_decision.overall_state,
        ProvisioningOverallState::Blocked
    );
    assert_eq!(
        decision.provisioning_decision.blocker_reason,
        Some(ProvisioningBlockerReason::ChildServiceNotStarted)
    );
}

#[test]
fn child_runtime_preflight_rejects_pairing_authority_failures_before_start() {
    for (pairing_lifecycle_state, blocker_reason) in [
        (
            PairingLifecycleState::Expired,
            ProvisioningBlockerReason::PairingExpired,
        ),
        (
            PairingLifecycleState::Replayed,
            ProvisioningBlockerReason::PairingReplayRejected,
        ),
        (
            PairingLifecycleState::WrongHousehold,
            ProvisioningBlockerReason::PairingWrongHousehold,
        ),
        (
            PairingLifecycleState::WrongDevice,
            ProvisioningBlockerReason::PairingWrongDevice,
        ),
        (
            PairingLifecycleState::Revoked,
            ProvisioningBlockerReason::PairingRevoked,
        ),
    ] {
        let mut input = valid_child_runtime_preflight_input();
        input.provisioning_input.pairing_lifecycle_state = pairing_lifecycle_state;

        let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(input);

        assert_eq!(
            decision.runtime_start_state,
            ocentra_child_runtime::ChildRuntimeStartState::Blocked
        );
        assert_eq!(
            decision.manual_review_state,
            ocentra_child_runtime::ChildRuntimeManualReviewState::Required
        );
        assert_eq!(
            decision.provisioning_decision.blocker_reason,
            Some(blocker_reason)
        );
    }
}

#[test]
fn child_runtime_preflight_rejects_untrusted_device_even_when_pairing_is_trusted() {
    let mut input = valid_child_runtime_preflight_input();
    input.provisioning_input.device_trust_state = DeviceTrustState::Revoked;

    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(input);

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::Required
    );
    assert_eq!(
        decision.provisioning_decision.blocker_reason,
        Some(ProvisioningBlockerReason::ChildDeviceTrustRequired)
    );
}

#[test]
fn child_runtime_remote_access_reuses_remote_session_gate() {
    let decision =
        ocentra_child_runtime::evaluate_child_runtime_remote_access(RemoteAccessSessionRequest {
            parent_authority_state: ParentAuthorityState::Authorized,
            child_disclosure_state: ChildDisclosureState::Disclosed,
            relay_state: RemoteAccessRelayState::Available,
            replay_state: RemoteAccessReplayState::Fresh,
            input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
            requested_minutes: 15,
            maximum_minutes: 30,
        });

    assert_eq!(
        decision.session_decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Allowed
    );
    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Allowed
    );
}

#[test]
fn child_runtime_enforcement_reuses_policy_authorized_adapter_gate() {
    let decision =
        ocentra_child_runtime::evaluate_child_runtime_enforcement(EnforcementActionInput {
            mode: EnforcementActionMode::Execute,
            policy_authority_state: ParentAuthorityState::Authorized,
            adapter_state: EnforcementAdapterState::Available,
            rollback_state: EnforcementRollbackState::Available,
            idempotency_state: EnforcementIdempotencyState::NewAction,
        });

    assert_eq!(
        decision.action_decision.adapter_execution_state,
        EnforcementAdapterExecutionState::Execute
    );
    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Allowed
    );
}

#[test]
fn child_runtime_preflight_request_records_typed_decision_event() {
    let request = ocentra_child_runtime::ChildRuntimePreflightRequestedEvent {
        aggregate_id: ocentra_child_runtime::ChildRuntimeAggregateId::parse(
            "child-runtime-device-default",
        )
        .required("child runtime aggregate"),
        request_id: ocentra_child_runtime::ChildRuntimePreflightRequestId::parse(
            "child-runtime-preflight-default",
        )
        .required("child runtime preflight request"),
        input: valid_child_runtime_preflight_input(),
    };

    let decision = ocentra_child_runtime::record_child_runtime_preflight_decision(&request)
        .required("child runtime preflight decision recorded");

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_request_id, request.request_id);
    assert_eq!(
        decision.decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Blocked
    );
    assert_eq!(
        request
            .contract()
            .required("child runtime preflight request contract")
            .event_type
            .as_str(),
        ocentra_child_runtime::CHILD_RUNTIME_PREFLIGHT_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        decision
            .contract()
            .required("child runtime preflight decision contract")
            .event_type
            .as_str(),
        ocentra_child_runtime::CHILD_RUNTIME_PREFLIGHT_DECISION_RECORDED_EVENT_TYPE
    );
}

fn valid_child_runtime_preflight_input() -> ocentra_child_runtime::ChildRuntimePreflightInput {
    ocentra_child_runtime::ChildRuntimePreflightInput {
        device_scope_input: DeviceScopeInput {
            actor_role: HouseholdRole::ParentOwner,
            same_family: true,
            actor_account_state: ActorAccountState::Active,
            membership_state: HouseholdMembershipState::Active,
            child_profile_binding_state: ChildProfileBindingState::Bound,
            device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        },
        provisioning_input: ProvisioningReadinessInput {
            membership_state: HouseholdMembershipState::Active,
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
        },
        entitlement_input: entitlement_input_with_snapshot_context(serde_json::Value::Null),
        storage_custody_input: StorageCustodyInput {
            location: StorageCustodyLocation::ParentOwnedRemote,
            retention_window_state: RetentionWindowState::Active,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Enabled,
        },
    }
}

fn entitlement_input_with_snapshot_context(
    snapshot_context: serde_json::Value,
) -> EntitlementCapabilityInput {
    serde_json::from_value(json!({
        "capability": "tracking",
        "subscription_state": "active",
        "offline_grace_state": "inactive",
        "family_setup_state": "complete",
        "policy_state": "clean",
        "capability_scope": "local-child-runtime",
        "snapshot_context": snapshot_context
    }))
    .required("untrusted entitlement input")
}
