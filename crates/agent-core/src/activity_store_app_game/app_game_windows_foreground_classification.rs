use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_UNAVAILABLE, APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CONFIDENCE_UNKNOWN,
};

use super::WindowsForegroundWindowRecord;

pub(super) fn classification_state_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CLASSIFICATION_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        return APP_GAME_CLASSIFICATION_ADAPTER_ERROR.to_string();
    }
    if record.classification_state == APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
        && record.launcher_ref.is_some()
    {
        return APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string();
    }
    if has_deterministic_foreground_ref(record) {
        return record.classification_state.clone();
    }
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string()
}

pub(super) fn catalog_ready_state_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CATALOG_PERMISSION_LIMITED.to_string();
    }
    if record.catalog_ref.is_some() {
        return APP_GAME_CATALOG_READY.to_string();
    }
    APP_GAME_CATALOG_UNAVAILABLE.to_string()
}

pub(super) fn capability_status_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string()
    } else if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR.to_string()
    } else {
        APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string()
    }
}

pub(super) fn confidence_for_record(record: &WindowsForegroundWindowRecord) -> f64 {
    if has_deterministic_foreground_ref(record) {
        record.confidence
    } else {
        APP_GAME_CONFIDENCE_UNKNOWN
    }
}

fn has_deterministic_foreground_ref(record: &WindowsForegroundWindowRecord) -> bool {
    record.inventory_entry_id.is_some()
        || record.launcher_ref.is_some()
        || record.catalog_ref.is_some()
        || record.window_ref.is_some()
}
