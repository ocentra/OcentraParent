use ocentra_parent_agent_protocol::{constants, TrackingRetentionSettingsWriteRequest};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Default)]
struct LocalRetentionSettingsState {
    revision: u64,
    retention_window_hours: Option<u16>,
    delete_after_alert_resolved: bool,
    parent_export_prepared: bool,
    remote_sync_enabled: bool,
    remote_ai_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionSettingsWriteAppliedState {
    pub local_service_state_revision: u64,
    pub durable_settings_persisted: bool,
}

#[derive(Debug, Serialize)]
struct LocalRetentionSettingsDurableRecord {
    revision: u64,
    retention_window_hours: Option<u16>,
    delete_after_alert_resolved: bool,
    parent_export_prepared: bool,
    remote_sync_enabled: bool,
    remote_ai_enabled: bool,
}

static LOCAL_RETENTION_SETTINGS_STATE: OnceLock<Mutex<LocalRetentionSettingsState>> =
    OnceLock::new();

pub fn apply_tracking_retention_settings_write(
    request: &TrackingRetentionSettingsWriteRequest,
) -> TrackingRetentionSettingsWriteAppliedState {
    let local_service_state_revision = apply_local_retention_settings_state(request);
    let durable_settings_persisted = persist_local_retention_settings_state();

    TrackingRetentionSettingsWriteAppliedState {
        local_service_state_revision,
        durable_settings_persisted,
    }
}

pub fn tracking_retention_settings_durable_store_path() -> PathBuf {
    std::env::temp_dir()
        .join(constants::tracking_retention_settings_write::DURABLE_SETTINGS_STORE_FILE_NAME)
}

fn apply_local_retention_settings_state(request: &TrackingRetentionSettingsWriteRequest) -> u64 {
    let state = LOCAL_RETENTION_SETTINGS_STATE
        .get_or_init(|| Mutex::new(LocalRetentionSettingsState::default()));
    let mut guard = state
        .lock()
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    guard.revision += 1;
    guard.retention_window_hours = request.requested_retention_window_hours;
    guard.delete_after_alert_resolved = request.requested_delete_after_alert_resolved;
    guard.parent_export_prepared = request.requested_parent_export;
    guard.remote_sync_enabled = false;
    guard.remote_ai_enabled = false;
    guard.revision
}

fn persist_local_retention_settings_state() -> bool {
    let Some(state) = LOCAL_RETENTION_SETTINGS_STATE.get() else {
        return false;
    };
    let guard = state
        .lock()
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let record = LocalRetentionSettingsDurableRecord {
        revision: guard.revision,
        retention_window_hours: guard.retention_window_hours,
        delete_after_alert_resolved: guard.delete_after_alert_resolved,
        parent_export_prepared: guard.parent_export_prepared,
        remote_sync_enabled: guard.remote_sync_enabled,
        remote_ai_enabled: guard.remote_ai_enabled,
    };
    let Ok(serialized) = serde_json::to_vec_pretty(&record) else {
        return false;
    };
    fs::write(tracking_retention_settings_durable_store_path(), serialized).is_ok()
}
