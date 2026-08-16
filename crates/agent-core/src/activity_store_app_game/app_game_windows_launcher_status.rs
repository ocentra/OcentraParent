use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_UNAVAILABLE,
};

use crate::activity_store_app_game::app_game_windows_launcher::WindowsLauncherEvidenceRecord;

pub(crate) fn catalog_ready_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CATALOG_PERMISSION_LIMITED.to_string();
    }
    if record.catalog_ref.is_some() {
        return APP_GAME_CATALOG_READY.to_string();
    }
    APP_GAME_CATALOG_UNAVAILABLE.to_string()
}

pub(crate) fn capability_status_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string()
    } else if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR.to_string()
    } else {
        APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string()
    }
}

pub(crate) fn has_launcher_reference(record: &WindowsLauncherEvidenceRecord) -> bool {
    !record.launcher_ref.is_empty()
}
