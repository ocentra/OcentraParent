use ::ocentra_child_runtime::runtime_gate_tombstone;
use ocentra_child_enforcement_core::enforcement_action::{
    EnforcementActionInput, EnforcementActionMode, EnforcementAdapterExecutionState,
    EnforcementAdapterState, EnforcementIdempotencyState, EnforcementRollbackState,
};
use ocentra_child_runtime::runtime_gate as ocentra_child_runtime;
use ocentra_entitlement_core::entitlement_access::{
    EntitlementCapability, EntitlementCapabilityInput, EntitlementCapabilityRejectionReason,
    EntitlementCapabilityScope, EntitlementPolicyState, FamilySetupState, OfflineGraceState,
    SubscriptionState,
};
use ocentra_entitlement_core::entitlement_snapshot::EntitlementSnapshotContext;
use ocentra_entitlement_core::entitlement_snapshot_values::{
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
    EntitlementPackageBuildState, EntitlementSnapshotBindingState,
    EntitlementSnapshotFreshnessState, EntitlementSnapshotSignatureState,
};
use ocentra_eventing::envelope::{DomainEvent, EventEnvelope, EventMetadata, EventSource};
use ocentra_eventing::ids::{
    CorrelationId, EventCustody, RuntimeInstanceId, RuntimeRole, SourceComponent, SourceService,
};
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
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
use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    ParentExportState, RemoteSyncState, RemoteUploadState, RetentionWindowState,
    StorageCustodyAggregateId, StorageCustodyDecisionId, StorageCustodyInput,
    StorageCustodyLocation,
};

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
        decision.storage_custody_decision.remote_upload_state,
        RemoteUploadState::Allowed
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
        Some(EntitlementCapabilityRejectionReason::ParentPortalOnlyScope)
    );
}

#[test]
fn child_runtime_preflight_blocks_when_entitlement_snapshot_household_binding_is_wrong() {
    let mut input = valid_child_runtime_preflight_input();
    input
        .entitlement_input
        .snapshot_context
        .household_binding_state = EntitlementSnapshotBindingState::Mismatched;

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
        Some(EntitlementCapabilityRejectionReason::WrongHousehold)
    );
}

#[test]
fn child_runtime_preflight_blocks_when_entitlement_snapshot_device_binding_is_wrong() {
    let mut input = valid_child_runtime_preflight_input();
    input
        .entitlement_input
        .snapshot_context
        .device_binding_state = EntitlementSnapshotBindingState::Mismatched;

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
        Some(EntitlementCapabilityRejectionReason::WrongDevice)
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
        ocentra_child_runtime::ChildRuntimeStartState::Allowed
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

#[tokio::test]
async fn child_runtime_persists_and_recovers_typed_tombstone_action_before_acknowledgement(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-tombstone-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-retention-family")?,
        StorageCustodyDecisionId::parse("child-runtime-retention-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    std::fs::create_dir_all(&directory)?;
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;

    let append = runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &journal, &store, &envelope, &action,
    )
    .await?;
    assert_eq!(append.sequence, 1);

    let recovered = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(recovered.len(), 1);
    assert_eq!(
        recovered[0].deletion_ref,
        "storage-custody-delete:child-runtime-retention-decision"
    );
    runtime_gate_tombstone::acknowledge_child_runtime_tombstone_publication(
        &store,
        &recovered[0].deletion_ref,
    )?;
    assert!(RetentionDeleteTombstoneStore::open(&directory)?
        .records()?
        .is_empty());
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_replays_a_durable_tombstone_obligation_after_journal_failure(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-journal-tombstone-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-journal-family")?,
        StorageCustodyDecisionId::parse("child-runtime-journal-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;
    journal.inject_next_sync_failure_for_debug();
    assert!(
        runtime_gate_tombstone::persist_child_runtime_tombstone_action(
            &journal, &store, &envelope, &action,
        )
        .await
        .is_err()
    );
    let recovered = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[0].proof_ref, action.action_plan_id.as_str());

    let restarted_journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let append = runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &restarted_journal,
        &RetentionDeleteTombstoneStore::open(&directory)?,
        &envelope,
        &action,
    )
    .await?;
    assert_eq!(append.sequence, 1);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_rejects_a_journal_envelope_for_a_different_custody_action(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-tombstone-mismatch-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let delete_action =
        storage_custody_action_planned_event(storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse("child-runtime-mismatch-family")?,
            StorageCustodyDecisionId::parse("child-runtime-mismatch-delete")?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state: RetentionWindowState::Expired,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ));
    let different_action =
        storage_custody_action_planned_event(storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse("child-runtime-mismatch-family")?,
            StorageCustodyDecisionId::parse("child-runtime-mismatch-retain")?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state: RetentionWindowState::Active,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ));
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(different_action, retention_delete_metadata()?)?.store()?;

    let error = runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &journal,
        &store,
        &envelope,
        &delete_action,
    )
    .await
    .expect_err("a journal envelope for another action must not create a tombstone intent");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(store.records()?.is_empty());
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

fn retention_delete_metadata() -> Result<EventMetadata, Box<dyn std::error::Error>> {
    Ok(EventMetadata::new(
        CorrelationId::parse("child-runtime-retention-delete-correlation")?,
        EventSource::new(
            EventCustody::parse("local-journal")?,
            RuntimeRole::parse("controller")?,
            SourceService::parse("child-runtime")?,
            SourceComponent::parse("retention-delete-runtime")?,
            RuntimeInstanceId::parse("child-runtime-test-instance")?,
        ),
    ))
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
        entitlement_input: EntitlementCapabilityInput {
            capability: EntitlementCapability::Tracking,
            subscription_state: SubscriptionState::Active,
            offline_grace_state: OfflineGraceState::Inactive,
            family_setup_state: FamilySetupState::Complete,
            policy_state: EntitlementPolicyState::Clean,
            capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
            snapshot_context: EntitlementSnapshotContext {
                signature_state: EntitlementSnapshotSignatureState::Trusted,
                freshness_state: EntitlementSnapshotFreshnessState::Fresh,
                household_binding_state: EntitlementSnapshotBindingState::Matched,
                device_binding_state: EntitlementSnapshotBindingState::Matched,
                device_trust_requirement_state: EntitlementDeviceTrustRequirementState::Required,
                device_trust_state: EntitlementDeviceTrustState::Present,
                package_build_state: EntitlementPackageBuildState::Valid,
            },
        },
        storage_custody_input: StorageCustodyInput {
            location: StorageCustodyLocation::ParentOwnedRemote,
            retention_window_state: RetentionWindowState::Active,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Enabled,
        },
    }
}
