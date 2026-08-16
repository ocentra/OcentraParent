use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    default_tracking_config_update_request, TrackingConfigEffectiveState,
};
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::{
    default_tracking_retention_settings_write_request, TrackingDeleteAfterAlertResolutionState,
    TrackingDurableSettingsPersistenceState, TrackingParentExportState, TrackingRemoteAiState,
    TrackingRemoteSyncState, TrackingRetentionSettingsWriteRequest,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingRuntimeEnabledState;
use ocentra_tracking_core::retention_settings::{
    apply_tracking_config_update, apply_tracking_retention_settings_write,
    tracking_retention_settings_durable_store_path,
};
use std::sync::{Mutex, MutexGuard, OnceLock};

static RETENTION_SETTINGS_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn retention_settings_write_state_is_owned_by_agent_core_not_websocket_transport() {
    let _guard = lock_retention_settings_test_state();
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
    let _guard = lock_retention_settings_test_state();
    let mut request = retention_write_request();
    request.requested_remote_sync_state = TrackingRemoteSyncState::Enabled;
    request.requested_remote_ai_state = TrackingRemoteAiState::Enabled;

    apply_tracking_retention_settings_write(&request);

    let durable_record = std::fs::read_to_string(tracking_retention_settings_durable_store_path())
        .expect_value("durable tracking settings record");
    let durable_record: serde_json::Value =
        serde_json::from_str(&durable_record).expect_value("json durable tracking settings record");

    assert_eq!(durable_record["remote_sync_state"], "enabled");
    assert_eq!(durable_record["remote_ai_state"], "enabled");
}

#[test]
fn tracking_config_update_persists_runtime_state_and_can_disable_tracking() {
    let _guard = lock_retention_settings_test_state();
    let mut request = default_tracking_config_update_request();
    request.runtime_config.tracking_enabled_state = TrackingRuntimeEnabledState::Disabled;

    let applied = apply_tracking_config_update(&request);
    let durable_record = std::fs::read_to_string(tracking_retention_settings_durable_store_path())
        .expect_value("durable tracking settings record");
    let durable_record: serde_json::Value =
        serde_json::from_str(&durable_record).expect_value("json durable tracking settings record");

    assert_eq!(
        applied.effective_tracking_state,
        TrackingConfigEffectiveState::Disabled
    );
    assert_eq!(durable_record["tracking_enabled_state"], "disabled");
    assert_eq!(durable_record["tracking_mode"], "observe-only");
}

fn retention_write_request() -> TrackingRetentionSettingsWriteRequest {
    let mut request = default_tracking_retention_settings_write_request();
    request.requested_delete_after_alert_resolution_state =
        TrackingDeleteAfterAlertResolutionState::DeleteAfterAlertResolved;
    request.requested_parent_export_state = TrackingParentExportState::Prepared;
    request.requested_remote_sync_state = TrackingRemoteSyncState::Disabled;
    request.requested_remote_ai_state = TrackingRemoteAiState::Disabled;
    request.source_read_model_proof_refs.truncate(1);
    request
}

fn lock_retention_settings_test_state() -> MutexGuard<'static, ()> {
    RETENTION_SETTINGS_TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect_value("retention settings test lock")
}
