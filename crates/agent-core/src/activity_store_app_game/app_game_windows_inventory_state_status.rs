use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_STALE, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_INVENTORY_STATE_ADAPTER_ERROR, APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED,
    APP_GAME_INVENTORY_STATE_STALE, APP_GAME_INVENTORY_STATE_UNAVAILABLE,
};

use super::super::WindowsInstalledAppInventoryRecord;

pub(crate) fn catalog_ready_state_for_record(
    record: &WindowsInstalledAppInventoryRecord,
) -> String {
    if record.inventory_state == APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED {
        APP_GAME_CATALOG_PERMISSION_LIMITED.to_string()
    } else if record.inventory_state == APP_GAME_INVENTORY_STATE_STALE {
        APP_GAME_CATALOG_STALE.to_string()
    } else if record.catalog_ref.is_some() {
        APP_GAME_CATALOG_READY.to_string()
    } else {
        APP_GAME_CATALOG_UNAVAILABLE.to_string()
    }
}

pub(crate) fn capability_status_for_record(record: &WindowsInstalledAppInventoryRecord) -> String {
    match record.inventory_state.as_str() {
        APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED => {
            APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string()
        }
        APP_GAME_INVENTORY_STATE_STALE => APP_GAME_CAPABILITY_STATUS_STALE.to_string(),
        APP_GAME_INVENTORY_STATE_ADAPTER_ERROR => {
            APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR.to_string()
        }
        APP_GAME_INVENTORY_STATE_UNAVAILABLE => APP_GAME_CAPABILITY_STATUS_UNAVAILABLE.to_string(),
        _ => APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
    }
}
