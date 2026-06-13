use ocentra_parent_agent_protocol::{
    constants, TrackingDeleteAfterAlertResolutionState, TrackingDurableSettingsPersistenceState,
    TrackingParentExportState, TrackingRemoteAiState, TrackingRemoteSyncState,
    TrackingRetentionSettingsWriteRequest,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[derive(Debug)]
struct LocalRetentionSettingsState {
    revision: u64,
    retention_window_hours: Option<u16>,
    delete_after_alert_resolution_state: TrackingDeleteAfterAlertResolutionState,
    parent_export_state: TrackingParentExportState,
    remote_sync_state: TrackingRemoteSyncState,
    remote_ai_state: TrackingRemoteAiState,
}

impl Default for LocalRetentionSettingsState {
    fn default() -> Self {
        Self {
            revision: 0,
            retention_window_hours: None,
            delete_after_alert_resolution_state:
                TrackingDeleteAfterAlertResolutionState::RetainAfterAlertResolved,
            parent_export_state: TrackingParentExportState::NotPrepared,
            remote_sync_state: TrackingRemoteSyncState::Disabled,
            remote_ai_state: TrackingRemoteAiState::Disabled,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionSettingsWriteAppliedState {
    pub local_service_state_revision: u64,
    pub durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
}

#[derive(Debug, Serialize)]
struct LocalRetentionSettingsDurableRecord {
    revision: u64,
    retention_window_hours: Option<u16>,
    delete_after_alert_resolution_state: TrackingDeleteAfterAlertResolutionState,
    parent_export_state: TrackingParentExportState,
    remote_sync_state: TrackingRemoteSyncState,
    remote_ai_state: TrackingRemoteAiState,
}

static LOCAL_RETENTION_SETTINGS_STATE: OnceLock<Mutex<LocalRetentionSettingsState>> =
    OnceLock::new();

pub fn apply_tracking_retention_settings_write(
    request: &TrackingRetentionSettingsWriteRequest,
) -> TrackingRetentionSettingsWriteAppliedState {
    let local_service_state_revision = apply_local_retention_settings_state(request);
    let durable_settings_persistence_state = persist_local_retention_settings_state();

    TrackingRetentionSettingsWriteAppliedState {
        local_service_state_revision,
        durable_settings_persistence_state,
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
    guard.delete_after_alert_resolution_state =
        request.requested_delete_after_alert_resolution_state;
    guard.parent_export_state = request.requested_parent_export_state;
    guard.remote_sync_state = TrackingRemoteSyncState::Disabled;
    guard.remote_ai_state = TrackingRemoteAiState::Disabled;
    guard.revision
}

fn persist_local_retention_settings_state() -> TrackingDurableSettingsPersistenceState {
    let Some(state) = LOCAL_RETENTION_SETTINGS_STATE.get() else {
        return TrackingDurableSettingsPersistenceState::NotPersisted;
    };
    let guard = state
        .lock()
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let record = LocalRetentionSettingsDurableRecord {
        revision: guard.revision,
        retention_window_hours: guard.retention_window_hours,
        delete_after_alert_resolution_state: guard.delete_after_alert_resolution_state,
        parent_export_state: guard.parent_export_state,
        remote_sync_state: guard.remote_sync_state,
        remote_ai_state: guard.remote_ai_state,
    };
    let Ok(serialized) = serde_json::to_vec_pretty(&record) else {
        return TrackingDurableSettingsPersistenceState::NotPersisted;
    };
    if fs::write(tracking_retention_settings_durable_store_path(), serialized).is_ok() {
        TrackingDurableSettingsPersistenceState::Persisted
    } else {
        TrackingDurableSettingsPersistenceState::NotPersisted
    }
}
