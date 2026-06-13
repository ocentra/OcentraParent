use ocentra_child_enforcement_core::{
    EnforcementActionInput, EnforcementActionMode, EnforcementAdapterExecutionState,
    EnforcementAdapterState, EnforcementIdempotencyState, EnforcementRollbackState,
};
use ocentra_entitlement_core::{
    EntitlementCapability, EntitlementCapabilityInput, EntitlementCapabilityScope,
    EntitlementPolicyState, FamilySetupState, OfflineGraceState, SubscriptionState,
};
use ocentra_eventing::DomainEvent;
use ocentra_family_identity_core::{
    ActorAccountState, ChildDisclosureState, ChildProfileBindingState, DeviceOwnershipScope,
    DeviceScopeInput, FamilyActorRole, HouseholdMembership,
};
use ocentra_policy_control_core::ParentAuthorityState;
use ocentra_provisioning_core::{
    PairingTokenState, ParentDeviceRegistrationState, ProvisioningReadinessInput, RecoveryState,
    RequiredPermissionState,
};
use ocentra_remote_access_core::{
    RemoteAccessInputAuthorityState, RemoteAccessRelayState, RemoteAccessReplayState,
    RemoteAccessSessionAuthorizationState, RemoteAccessSessionRequest,
};
use ocentra_storage_custody_core::{
    ParentExportState, RemoteSyncState, RetentionWindowState, StorageCustodyInput,
    StorageCustodyLocation,
};

#[test]
fn child_runtime_preflight_allows_start_when_identity_setup_entitlement_and_storage_are_valid() {
    let decision = ocentra_child_runtime::evaluate_child_runtime_preflight(
        valid_child_runtime_preflight_input(),
    );

    assert_eq!(
        decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Allowed
    );
    assert_eq!(
        decision.manual_review_state,
        ocentra_child_runtime::ChildRuntimeManualReviewState::NotRequired
    );
    assert_eq!(
        ocentra_child_runtime::child_runtime_remote_upload_allowed(&decision),
        true
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
        ocentra_child_runtime::ChildRuntimeManualReviewState::Required
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
        .expect("child runtime aggregate"),
        request_id: ocentra_child_runtime::ChildRuntimePreflightRequestId::parse(
            "child-runtime-preflight-default",
        )
        .expect("child runtime preflight request"),
        input: valid_child_runtime_preflight_input(),
    };

    let decision = ocentra_child_runtime::record_child_runtime_preflight_decision(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_request_id, request.request_id);
    assert_eq!(
        decision.decision.runtime_start_state,
        ocentra_child_runtime::ChildRuntimeStartState::Allowed
    );
    assert_eq!(
        request
            .contract()
            .expect("child runtime preflight request contract")
            .event_type
            .as_str(),
        ocentra_child_runtime::CHILD_RUNTIME_PREFLIGHT_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        decision
            .contract()
            .expect("child runtime preflight decision contract")
            .event_type
            .as_str(),
        ocentra_child_runtime::CHILD_RUNTIME_PREFLIGHT_DECISION_RECORDED_EVENT_TYPE
    );
}

fn valid_child_runtime_preflight_input() -> ocentra_child_runtime::ChildRuntimePreflightInput {
    ocentra_child_runtime::ChildRuntimePreflightInput {
        device_scope_input: DeviceScopeInput {
            actor_role: FamilyActorRole::Parent,
            actor_account_state: ActorAccountState::Active,
            household_membership: HouseholdMembership::Member,
            child_profile_binding_state: ChildProfileBindingState::Bound,
            device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        },
        provisioning_input: ProvisioningReadinessInput {
            household_membership: HouseholdMembership::Member,
            parent_device_registration_state: ParentDeviceRegistrationState::Registered,
            child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
            required_permission_state: RequiredPermissionState::Granted,
            pairing_token_state: PairingTokenState::Valid,
            recovery_state: RecoveryState::Normal,
        },
        entitlement_input: EntitlementCapabilityInput {
            capability: EntitlementCapability::Tracking,
            subscription_state: SubscriptionState::Active,
            offline_grace_state: OfflineGraceState::Inactive,
            family_setup_state: FamilySetupState::Complete,
            policy_state: EntitlementPolicyState::Clean,
            capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        },
        storage_custody_input: StorageCustodyInput {
            location: StorageCustodyLocation::ParentOwnedRemote,
            retention_window_state: RetentionWindowState::Active,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Enabled,
        },
    }
}
