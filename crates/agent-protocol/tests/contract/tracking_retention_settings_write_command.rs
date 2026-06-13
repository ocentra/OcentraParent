use ocentra_parent_agent_protocol::{
    constants, tracking_durable_settings_store_ref, tracking_local_service_state_snapshot_ref,
    tracking_mutation_proof_ref, tracking_read_model_proof_ref, tracking_retention_accepted_at,
    tracking_retention_command_id, tracking_retention_settings_kind,
    tracking_retention_write_state_accepted, tracking_writer_intent_ref, AgentCommandName,
    AgentEventName, TrackingConfigAckState, TrackingConfigEffectiveState,
    TrackingConfigUpdateResponseState, TrackingDeleteAfterAlertResolutionState,
    TrackingDurableSettingsPersistenceState, TrackingExecutionClaimState,
    TrackingParentExportState, TrackingRemoteAiState, TrackingRemoteSyncState,
    TrackingRetentionSettingsWriteRequest, TrackingRetentionSettingsWriteResult,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

#[test]
fn retention_settings_write_command_and_event_names_serialize_to_contract_shape() {
    let command =
        serde_json::to_value(AgentCommandName::AgentActivityTrackingRetentionSettingsWrite)
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event =
        serde_json::to_value(AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported)
            .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(command, "agent.activity.tracking.retention-settings.write");
    assert_eq!(
        event,
        "agent.activity.tracking.retention-settings.write.reported"
    );
}

#[test]
fn retention_settings_write_request_serializes_without_remote_overclaims() {
    let request = TrackingRetentionSettingsWriteRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
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

    let serialized = serde_json::to_value(request).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], AGENT_PROTOCOL_SCHEMA_VERSION);
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
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
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

    let serialized = serde_json::to_value(result).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], AGENT_PROTOCOL_SCHEMA_VERSION);
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
