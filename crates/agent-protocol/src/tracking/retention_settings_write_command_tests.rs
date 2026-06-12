use crate::{
    constants, AgentCommandName, AgentEventName, TrackingRetentionSettingsWriteRequest,
    TrackingRetentionSettingsWriteResult, AGENT_PROTOCOL_SCHEMA_VERSION,
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
        command_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        settings_kind: constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
            .to_string(),
        requested_retention_window_hours: Some(168),
        requested_delete_after_alert_resolved: false,
        requested_parent_export: false,
        requested_remote_sync_enabled: false,
        requested_remote_ai_enabled: false,
        source_writer_intent_refs: vec![
            constants::tracking_retention_settings_write::WRITER_INTENT_REF.to_string(),
        ],
        source_read_model_proof_refs: vec![
            constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF.to_string(),
            constants::tracking_retention_settings_write::JOURNAL_READ_MODEL_PROOF_REF.to_string(),
        ],
    };

    let serialized = serde_json::to_value(request).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], AGENT_PROTOCOL_SCHEMA_VERSION);
    assert_eq!(serialized["requestedRetentionWindowHours"], 168);
    assert_eq!(serialized["requestedRemoteSyncEnabled"], false);
    assert_eq!(serialized["requestedRemoteAiEnabled"], false);
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
        command_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        settings_kind: constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
            .to_string(),
        write_state: constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED.to_string(),
        accepted_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source_writer_intent_refs: vec![
            constants::tracking_retention_settings_write::WRITER_INTENT_REF.to_string(),
        ],
        source_read_model_proof_refs: vec![
            constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF.to_string(),
        ],
        source_mutation_proof_refs: vec![
            constants::tracking_retention_settings_write::MUTATION_PROOF_REF.to_string(),
        ],
        applied_retention_window_hours: Some(168),
        applied_delete_after_alert_resolved: false,
        parent_export_prepared: false,
        remote_sync_enabled: false,
        remote_ai_enabled: false,
        local_service_state_revision: Some(1),
        local_service_state_snapshot_ref:
            constants::tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF
                .to_string(),
        durable_settings_store_ref:
            constants::tracking_retention_settings_write::DURABLE_SETTINGS_STORE_REF.to_string(),
        durable_settings_persisted: true,
        child_config_response_state: Some(
            constants::tracking_config_update::RESPONSE_STATE_APPLIED.to_string(),
        ),
        effective_tracking_state: Some(
            constants::tracking_config_update::EFFECTIVE_STATE_ENABLED.to_string(),
        ),
        child_config_ack_received: true,
        command_transport_claimed: true,
        service_write_preflight_claimed: true,
        service_mutation_executed: true,
        portal_writable_ui_claimed: false,
        platform_runtime_claimed: false,
        child_device_delivery_claimed: false,
        provider_delivery_claimed: false,
        notification_receipt_claimed: false,
        physical_device_claimed: false,
        authority_claimed: false,
        product_claim_ready: false,
    };

    let serialized = serde_json::to_value(result).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], AGENT_PROTOCOL_SCHEMA_VERSION);
    assert_eq!(
        serialized["sourceMutationProofRefs"][0],
        constants::tracking_retention_settings_write::MUTATION_PROOF_REF
    );
    assert_eq!(serialized["appliedRetentionWindowHours"], 168);
    assert_eq!(serialized["remoteSyncEnabled"], false);
    assert_eq!(serialized["remoteAiEnabled"], false);
    assert_eq!(serialized["localServiceStateRevision"], 1);
    assert_eq!(
        serialized["localServiceStateSnapshotRef"],
        constants::tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF
    );
    assert_eq!(
        serialized["durableSettingsStoreRef"],
        constants::tracking_retention_settings_write::DURABLE_SETTINGS_STORE_REF
    );
    assert_eq!(serialized["durableSettingsPersisted"], true);
    assert_eq!(
        serialized["childConfigResponseState"],
        constants::tracking_config_update::RESPONSE_STATE_APPLIED
    );
    assert_eq!(
        serialized["effectiveTrackingState"],
        constants::tracking_config_update::EFFECTIVE_STATE_ENABLED
    );
    assert_eq!(serialized["childConfigAckReceived"], true);
    assert_eq!(serialized["commandTransportClaimed"], true);
    assert_eq!(serialized["serviceWritePreflightClaimed"], true);
    assert_eq!(serialized["serviceMutationExecuted"], true);
    assert_eq!(serialized["portalWritableUiClaimed"], false);
    assert_eq!(serialized["productClaimReady"], false);
}
