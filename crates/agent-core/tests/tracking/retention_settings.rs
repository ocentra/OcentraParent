use ocentra_parent_agent_core::{
    apply_tracking_retention_settings_write, tracking_retention_settings_durable_store_path,
};
use ocentra_parent_agent_protocol::{
    constants, TrackingRetentionSettingsWriteRequest, AGENT_PROTOCOL_SCHEMA_VERSION,
};

#[test]
fn retention_settings_write_state_is_owned_by_agent_core_not_websocket_transport() {
    let applied = apply_tracking_retention_settings_write(&retention_write_request());

    assert!(applied.local_service_state_revision > 0);
    assert!(applied.durable_settings_persisted);
    assert!(tracking_retention_settings_durable_store_path().exists());
}

fn retention_write_request() -> TrackingRetentionSettingsWriteRequest {
    TrackingRetentionSettingsWriteRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        settings_kind: constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
            .to_string(),
        requested_retention_window_hours: Some(168),
        requested_delete_after_alert_resolved: true,
        requested_parent_export: true,
        requested_remote_sync_enabled: false,
        requested_remote_ai_enabled: false,
        source_writer_intent_refs: vec![
            constants::tracking_retention_settings_write::WRITER_INTENT_REF.to_string(),
        ],
        source_read_model_proof_refs: vec![
            constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF.to_string(),
        ],
    }
}
