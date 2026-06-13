use ocentra_parent_agent_protocol::{
    tracking_read_model_proof_ref, tracking_retention_command_id,
    tracking_retention_settings_kind, tracking_writer_intent_ref,
    TrackingDeleteAfterAlertResolutionState, TrackingDurableSettingsPersistenceState,
    TrackingParentExportState, TrackingRemoteAiState, TrackingRemoteSyncState,
    TrackingRetentionSettingsWriteRequest, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use ocentra_tracking_core::{
    apply_tracking_retention_settings_write, tracking_retention_settings_durable_store_path,
};

#[test]
fn retention_settings_write_state_is_owned_by_agent_core_not_websocket_transport() {
    let applied = apply_tracking_retention_settings_write(&retention_write_request());

    assert!(applied.local_service_state_revision > 0);
    assert_eq!(
        applied.durable_settings_persistence_state,
        TrackingDurableSettingsPersistenceState::Persisted
    );
    assert!(tracking_retention_settings_durable_store_path().exists());
}

#[test]
fn retention_settings_write_persists_requested_remote_states() {
    let mut request = retention_write_request();
    request.requested_remote_sync_state = TrackingRemoteSyncState::Enabled;
    request.requested_remote_ai_state = TrackingRemoteAiState::Enabled;

    apply_tracking_retention_settings_write(&request);

    let durable_record = std::fs::read_to_string(tracking_retention_settings_durable_store_path())
        .expect("durable tracking settings record");
    let durable_record: serde_json::Value =
        serde_json::from_str(&durable_record).expect("json durable tracking settings record");

    assert_eq!(durable_record["remote_sync_state"], "enabled");
    assert_eq!(durable_record["remote_ai_state"], "enabled");
}

fn retention_write_request() -> TrackingRetentionSettingsWriteRequest {
    TrackingRetentionSettingsWriteRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: tracking_retention_command_id(),
        settings_kind: tracking_retention_settings_kind(),
        requested_retention_window_hours: Some(168),
        requested_delete_after_alert_resolution_state:
            TrackingDeleteAfterAlertResolutionState::DeleteAfterAlertResolved,
        requested_parent_export_state: TrackingParentExportState::Prepared,
        requested_remote_sync_state: TrackingRemoteSyncState::Disabled,
        requested_remote_ai_state: TrackingRemoteAiState::Disabled,
        source_writer_intent_refs: vec![tracking_writer_intent_ref()],
        source_read_model_proof_refs: vec![tracking_read_model_proof_ref(
            ocentra_parent_agent_protocol::constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF,
        )],
    }
}
