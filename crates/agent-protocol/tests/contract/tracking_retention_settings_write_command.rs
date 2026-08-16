use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::request::{EventResponseContract, RequestEvent};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    TrackingConfigEffectiveState, TrackingConfigUpdateResponseState,
};
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingCheckInId, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingPolicyViolationId, TrackingReasonCode, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::{
    tracking_durable_settings_store_ref, tracking_local_service_state_snapshot_ref,
    tracking_mutation_proof_ref, tracking_read_model_proof_ref, tracking_retention_accepted_at,
    tracking_retention_command_id, tracking_retention_settings_kind,
    tracking_retention_write_state_accepted, tracking_writer_intent_ref, TrackingConfigAckState,
    TrackingDeleteAfterAlertResolutionState, TrackingDurableSettingsPersistenceState,
    TrackingExecutionClaimState, TrackingParentExportState, TrackingRemoteAiState,
    TrackingRemoteSyncState, TrackingRetentionSettingsWriteRequest,
    TrackingRetentionSettingsWriteResult, TRACKING_RETENTION_SETTINGS_WRITE_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingChildCheckInDeliveryState, TrackingChildCheckInRequestReceipt,
    TrackingChildCheckInRequestState, TrackingChildCheckInRequestedEvent,
    TRACKING_RUNTIME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::transport::{AgentCommandName, AgentEventName};

#[test]
fn retention_settings_write_command_and_event_names_serialize_to_contract_shape() {
    let command =
        serde_json::to_value(AgentCommandName::AgentActivityTrackingRetentionSettingsWrite)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let event =
        serde_json::to_value(AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(command, "agent.activity.tracking.retention-settings.write");
    assert_eq!(
        event,
        "agent.activity.tracking.retention-settings.write.reported"
    );
}

#[test]
fn retention_settings_write_request_serializes_without_remote_overclaims() {
    let request = TrackingRetentionSettingsWriteRequest {
        schema_version: TRACKING_RETENTION_SETTINGS_WRITE_SCHEMA_VERSION,
        command_id: tracking_retention_command_id(),
        settings_kind: tracking_retention_settings_kind(),
        requested_retention_window_hours: Some(168),
        requested_delete_after_alert_resolution_state:
            TrackingDeleteAfterAlertResolutionState::RetainAfterAlertResolved,
        requested_parent_export_state: TrackingParentExportState::NotPrepared,
        requested_remote_sync_state: TrackingRemoteSyncState::Disabled,
        requested_remote_ai_state: TrackingRemoteAiState::Disabled,
        source_writer_intent_refs: vec![tracking_writer_intent_ref()],
        source_read_model_proof_refs: vec![
            tracking_read_model_proof_ref(
                constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF,
            ),
            tracking_read_model_proof_ref(
                constants::tracking_retention_settings_write::JOURNAL_READ_MODEL_PROOF_REF,
            ),
        ],
    };

    let serialized =
        serde_json::to_value(request).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["schemaVersion"],
        TRACKING_RETENTION_SETTINGS_WRITE_SCHEMA_VERSION
    );
    assert_eq!(serialized["requestedRetentionWindowHours"], 168);
    assert_eq!(serialized["requestedRemoteSyncState"], "disabled");
    assert_eq!(serialized["requestedRemoteAiState"], "disabled");
    assert_eq!(
        serialized["sourceWriterIntentRefs"][0],
        constants::tracking_retention_settings_write::WRITER_INTENT_REF
    );
    assert_eq!(
        serialized["sourceReadModelProofRefs"][0],
        constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF
    );
    assert_eq!(
        serialized["sourceReadModelProofRefs"][1],
        constants::tracking_retention_settings_write::JOURNAL_READ_MODEL_PROOF_REF
    );
}

#[test]
fn retention_settings_write_result_serializes_local_execution_without_product_overclaims() {
    let result = TrackingRetentionSettingsWriteResult {
        schema_version: TRACKING_RETENTION_SETTINGS_WRITE_SCHEMA_VERSION,
        command_id: tracking_retention_command_id(),
        settings_kind: tracking_retention_settings_kind(),
        write_state: tracking_retention_write_state_accepted(),
        accepted_at: tracking_retention_accepted_at(),
        source_writer_intent_refs: vec![tracking_writer_intent_ref()],
        source_read_model_proof_refs: vec![tracking_read_model_proof_ref(
            constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF,
        )],
        source_mutation_proof_refs: vec![tracking_mutation_proof_ref()],
        applied_retention_window_hours: Some(168),
        applied_delete_after_alert_resolution_state:
            TrackingDeleteAfterAlertResolutionState::RetainAfterAlertResolved,
        parent_export_state: TrackingParentExportState::NotPrepared,
        remote_sync_state: TrackingRemoteSyncState::Disabled,
        remote_ai_state: TrackingRemoteAiState::Disabled,
        local_service_state_revision: Some(1),
        local_service_state_snapshot_ref: tracking_local_service_state_snapshot_ref(),
        durable_settings_store_ref: tracking_durable_settings_store_ref(),
        durable_settings_persistence_state: TrackingDurableSettingsPersistenceState::Persisted,
        child_config_response_state: Some(TrackingConfigUpdateResponseState::Applied),
        effective_tracking_state: Some(TrackingConfigEffectiveState::Enabled),
        child_config_ack_state: TrackingConfigAckState::Received,
        command_transport_claim_state: TrackingExecutionClaimState::Claimed,
        service_write_preflight_claim_state: TrackingExecutionClaimState::Claimed,
        service_mutation_execution_state: TrackingExecutionClaimState::Claimed,
        portal_writable_ui_claim_state: TrackingExecutionClaimState::Unclaimed,
        platform_runtime_claim_state: TrackingExecutionClaimState::Unclaimed,
        child_device_delivery_claim_state: TrackingExecutionClaimState::Unclaimed,
        provider_delivery_claim_state: TrackingExecutionClaimState::Unclaimed,
        notification_receipt_claim_state: TrackingExecutionClaimState::Unclaimed,
        physical_device_claim_state: TrackingExecutionClaimState::Unclaimed,
        authority_claim_state: TrackingExecutionClaimState::Unclaimed,
        product_claim_state: TrackingExecutionClaimState::Unclaimed,
    };

    let serialized =
        serde_json::to_value(result).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["schemaVersion"],
        TRACKING_RETENTION_SETTINGS_WRITE_SCHEMA_VERSION
    );
    assert_eq!(
        serialized["sourceMutationProofRefs"][0],
        constants::tracking_retention_settings_write::MUTATION_PROOF_REF
    );
    assert_eq!(serialized["appliedRetentionWindowHours"], 168);
    assert_eq!(serialized["remoteSyncState"], "disabled");
    assert_eq!(serialized["remoteAiState"], "disabled");
    assert_eq!(serialized["localServiceStateRevision"], 1);
    assert_eq!(
        serialized["localServiceStateSnapshotRef"],
        constants::tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF
    );
    assert_eq!(
        serialized["durableSettingsStoreRef"],
        constants::tracking_retention_settings_write::DURABLE_SETTINGS_STORE_REF
    );
    assert_eq!(serialized["durableSettingsPersistenceState"], "persisted");
    assert_eq!(
        serialized["childConfigResponseState"],
        constants::tracking_config_update::RESPONSE_STATE_APPLIED
    );
    assert_eq!(
        serialized["effectiveTrackingState"],
        constants::tracking_config_update::EFFECTIVE_STATE_ENABLED
    );
    assert_eq!(serialized["childConfigAckState"], "received");
    assert_eq!(serialized["commandTransportClaimState"], "claimed");
    assert_eq!(serialized["serviceWritePreflightClaimState"], "claimed");
    assert_eq!(serialized["serviceMutationExecutionState"], "claimed");
    assert_eq!(serialized["portalWritableUiClaimState"], "unclaimed");
    assert_eq!(serialized["productClaimState"], "unclaimed");
}

#[test]
fn child_check_in_request_contract_serializes_with_check_in_id_as_request_id() {
    let request = TrackingChildCheckInRequestedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        check_in_id: TrackingCheckInId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        requested_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        request_state: TrackingChildCheckInRequestState::Pending,
        delivery_state: TrackingChildCheckInDeliveryState::Queued,
        related_alert_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        include_location_if_permitted: true,
        expires_at: TrackingTimestamp::parse("2026-06-12T12:05:00Z")
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES)],
        audit_refs: vec![String::from("audit.tracking.child-check-in.request")],
    };

    let serialized =
        serde_json::to_value(&request).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        request
            .contract()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        request
            .request_id()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .as_str(),
        constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID
    );
    assert_eq!(
        serialized["requestState"],
        constants::tracking_runtime::CHILD_CHECK_IN_REQUEST_STATE_PENDING
    );
    assert_eq!(
        serialized["deliveryState"],
        constants::tracking_runtime::CHILD_CHECK_IN_DELIVERY_STATE_QUEUED
    );
    assert_eq!(
        serialized["relatedAlertId"],
        constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID
    );
}

#[test]
fn child_check_in_request_receipt_serializes_delivery_receipt_state_and_reason() {
    let receipt = TrackingChildCheckInRequestReceipt {
        schema_version: TRACKING_RUNTIME_SCHEMA_VERSION,
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        check_in_id: TrackingCheckInId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        related_alert_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        request_state: TrackingChildCheckInRequestState::Pending,
        delivery_state: TrackingChildCheckInDeliveryState::Duplicate,
        receipt_recorded_at: TrackingTimestamp::parse(
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        reason_code: Some(
            TrackingReasonCode::parse(
                constants::tracking_runtime::REASON_DUPLICATE_CHECK_IN_REQUEST,
            )
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    };

    let serialized =
        serde_json::to_value(&receipt).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    receipt
        .validate()
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(serialized["schemaVersion"], TRACKING_RUNTIME_SCHEMA_VERSION);
    assert_eq!(
        serialized["deliveryState"],
        constants::tracking_runtime::CHILD_CHECK_IN_DELIVERY_STATE_DUPLICATE
    );
    assert_eq!(
        serialized["reasonCode"],
        constants::tracking_runtime::REASON_DUPLICATE_CHECK_IN_REQUEST
    );
}
