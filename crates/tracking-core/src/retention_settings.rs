use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    TrackingConfigEffectiveState, TrackingConfigUpdateRequest,
};
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::{
    TrackingDeleteAfterAlertResolutionState, TrackingDurableSettingsPersistenceState,
    TrackingParentExportState, TrackingRemoteAiState, TrackingRemoteSyncState,
    TrackingRetentionSettingsWriteRequest,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    default_tracking_runtime_config, TrackingAiBoundaryMode, TrackingNotificationMode,
    TrackingRuntimeEnabledState, TrackingRuntimeMode,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[derive(Debug)]
struct LocalRetentionSettingsState {
    revision: u64,
    tracking_enabled_state: TrackingRuntimeEnabledState,
    tracking_mode: TrackingRuntimeMode,
    ai_boundary_mode: TrackingAiBoundaryMode,
    notification_mode: TrackingNotificationMode,
    retention_window_hours: Option<u16>,
    delete_after_alert_resolution_state: TrackingDeleteAfterAlertResolutionState,
    parent_export_state: TrackingParentExportState,
    remote_sync_state: TrackingRemoteSyncState,
    remote_ai_state: TrackingRemoteAiState,
}

impl Default for LocalRetentionSettingsState {
    fn default() -> Self {
        let runtime_config = default_tracking_runtime_config();
        Self {
            revision: 0,
            tracking_enabled_state: runtime_config.tracking_enabled_state,
            tracking_mode: runtime_config.tracking_mode,
            ai_boundary_mode: runtime_config.ai_boundary_mode,
            notification_mode: runtime_config.notification_mode,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingConfigUpdateAppliedState {
    pub local_service_state_revision: u64,
    pub durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
    pub effective_tracking_state: TrackingConfigEffectiveState,
}

#[derive(Debug, Serialize)]
struct LocalRetentionSettingsDurableRecord {
    revision: u64,
    tracking_enabled_state: TrackingRuntimeEnabledState,
    tracking_mode: TrackingRuntimeMode,
    ai_boundary_mode: TrackingAiBoundaryMode,
    notification_mode: TrackingNotificationMode,
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
    let applied_state = apply_tracking_config_update(&TrackingConfigUpdateRequest {
        command_id: request.command_id.clone(),
        runtime_config: default_tracking_runtime_config(),
        retention_settings: request.clone(),
    });

    TrackingRetentionSettingsWriteAppliedState {
        local_service_state_revision: applied_state.local_service_state_revision,
        durable_settings_persistence_state: applied_state.durable_settings_persistence_state,
    }
}

pub fn apply_tracking_config_update(
    request: &TrackingConfigUpdateRequest,
) -> TrackingConfigUpdateAppliedState {
    let local_service_state_revision = apply_local_tracking_config_state(request);
    let durable_settings_persistence_state = persist_local_retention_settings_state();

    TrackingConfigUpdateAppliedState {
        local_service_state_revision,
        effective_tracking_state: effective_tracking_state_for_request(
            request,
            durable_settings_persistence_state,
        ),
        durable_settings_persistence_state,
    }
}

pub fn tracking_retention_settings_durable_store_path() -> PathBuf {
    std::env::temp_dir()
        .join(constants::tracking_retention_settings_write::DURABLE_SETTINGS_STORE_FILE_NAME)
}

fn apply_local_tracking_config_state(request: &TrackingConfigUpdateRequest) -> u64 {
    let state = LOCAL_RETENTION_SETTINGS_STATE
        .get_or_init(|| Mutex::new(LocalRetentionSettingsState::default()));
    let mut guard = state.lock().unwrap_or_else(|poison| poison.into_inner());
    guard.revision += 1;
    guard.tracking_enabled_state = request.runtime_config.tracking_enabled_state.clone();
    guard.tracking_mode = request.runtime_config.tracking_mode.clone();
    guard.ai_boundary_mode = request.runtime_config.ai_boundary_mode.clone();
    guard.notification_mode = request.runtime_config.notification_mode.clone();
    guard.retention_window_hours = request.retention_settings.requested_retention_window_hours;
    guard.delete_after_alert_resolution_state = request
        .retention_settings
        .requested_delete_after_alert_resolution_state;
    guard.parent_export_state = request.retention_settings.requested_parent_export_state;
    guard.remote_sync_state = request.retention_settings.requested_remote_sync_state;
    guard.remote_ai_state = request.retention_settings.requested_remote_ai_state;
    guard.revision
}

fn persist_local_retention_settings_state() -> TrackingDurableSettingsPersistenceState {
    let Some(state) = LOCAL_RETENTION_SETTINGS_STATE.get() else {
        return TrackingDurableSettingsPersistenceState::NotPersisted;
    };
    let guard = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let record = LocalRetentionSettingsDurableRecord {
        revision: guard.revision,
        tracking_enabled_state: guard.tracking_enabled_state.clone(),
        tracking_mode: guard.tracking_mode.clone(),
        ai_boundary_mode: guard.ai_boundary_mode.clone(),
        notification_mode: guard.notification_mode.clone(),
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

fn effective_tracking_state_for_request(
    request: &TrackingConfigUpdateRequest,
    durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
) -> TrackingConfigEffectiveState {
    if durable_settings_persistence_state != TrackingDurableSettingsPersistenceState::Persisted {
        TrackingConfigEffectiveState::Degraded
    } else if request.runtime_config.tracking_enabled_state == TrackingRuntimeEnabledState::Disabled
    {
        TrackingConfigEffectiveState::Disabled
    } else {
        TrackingConfigEffectiveState::Enabled
    }
}
