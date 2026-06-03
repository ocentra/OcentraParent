use std::collections::HashSet;

use ocentra_parent_agent_protocol::{
    ActivityEvidenceRef, AppGameInventoryCategoryCandidate, AppGameInventoryEvidenceRow,
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_STALE, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
    APP_GAME_CLASSIFICATION_PERMISSION_LIMITED, APP_GAME_CLASSIFICATION_STALE,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_CATEGORY_GAME, APP_GAME_INVENTORY_CATEGORY_LAUNCHER,
    APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST, APP_GAME_INVENTORY_SOURCE_UNKNOWN,
    APP_GAME_INVENTORY_STATE_ADAPTER_ERROR, APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED,
    APP_GAME_INVENTORY_STATE_STALE, APP_GAME_INVENTORY_STATE_UNAVAILABLE,
    APP_GAME_PRODUCT_LAUNCHER, APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME,
    APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION,
};

pub struct WindowsInstalledAppInventoryRecord {
    pub observed_at: String,
    pub source_kind: String,
    pub source_ref: String,
    pub custody_state: String,
    pub display_label: String,
    pub identity_id: Option<String>,
    pub package_id: Option<String>,
    pub bundle_id: Option<String>,
    pub app_user_model_id: Option<String>,
    pub desktop_entry_id: Option<String>,
    pub executable_path_ref: Option<String>,
    pub launcher_ref: Option<String>,
    pub launcher_app_id: Option<String>,
    pub launcher_manifest_id: Option<String>,
    pub store_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub inventory_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_installed_inventory_rows_from_records(
    records: &[WindowsInstalledAppInventoryRecord],
) -> Vec<AppGameInventoryEvidenceRow> {
    let mut strong_identities = HashSet::new();
    let mut rows = Vec::new();
    for record in records {
        if strong_identity_seen(record, &mut strong_identities) {
            continue;
        }
        rows.push(row_from_record(record));
    }
    rows
}

fn strong_identity_seen(
    record: &WindowsInstalledAppInventoryRecord,
    strong_identities: &mut HashSet<(u8, String)>,
) -> bool {
    let keys = strong_identity_keys(record);
    if keys.is_empty() {
        return false;
    }
    if keys.iter().any(|key| strong_identities.contains(key)) {
        return true;
    }
    strong_identities.extend(keys);
    false
}

fn strong_identity_keys(record: &WindowsInstalledAppInventoryRecord) -> Vec<(u8, String)> {
    let mut keys = Vec::new();
    push_identity_key(&mut keys, 1, &record.identity_id);
    push_identity_key(&mut keys, 2, &record.package_id);
    push_identity_key(&mut keys, 3, &record.bundle_id);
    push_identity_key(&mut keys, 4, &record.app_user_model_id);
    push_identity_key(&mut keys, 5, &record.executable_path_ref);
    push_identity_key(&mut keys, 6, &record.launcher_app_id);
    push_identity_key(&mut keys, 7, &record.launcher_manifest_id);
    push_identity_key(&mut keys, 8, &record.store_id);
    push_identity_key(&mut keys, 9, &record.catalog_ref);
    push_identity_key(&mut keys, 10, &record.desktop_entry_id);
    keys
}

fn push_identity_key(keys: &mut Vec<(u8, String)>, rank: u8, value: &Option<String>) {
    if let Some(value) = value {
        keys.push((rank, value.clone()));
    }
}

fn row_from_record(record: &WindowsInstalledAppInventoryRecord) -> AppGameInventoryEvidenceRow {
    let product_kind = product_kind_for_record(record);
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: record.source_ref.clone(),
        observed_at: record.observed_at.clone(),
        source_kind: record.source_kind.clone(),
        source_ref: record.source_ref.clone(),
        custody_state: record.custody_state.clone(),
        product_kind: product_kind.clone(),
        display_label: record.display_label.clone(),
        identity_id: record.identity_id.clone(),
        package_id: record.package_id.clone(),
        bundle_id: record.bundle_id.clone(),
        app_user_model_id: record.app_user_model_id.clone(),
        desktop_entry_id: record.desktop_entry_id.clone(),
        executable_path_ref: record.executable_path_ref.clone(),
        launcher_ref: record.launcher_ref.clone(),
        launcher_app_id: record.launcher_app_id.clone(),
        launcher_manifest_id: record.launcher_manifest_id.clone(),
        store_id: record.store_id.clone(),
        catalog_ref: record.catalog_ref.clone(),
        inventory_state: record.inventory_state.clone(),
        classification_state: classification_state_for_record(record, &product_kind),
        catalog_ready_state: catalog_ready_state_for_record(record),
        capability_status: capability_status_for_record(record),
        confidence: record.confidence,
        category_candidates: category_candidates_for_record(record, &product_kind),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: record.evidence.clone(),
    }
}

fn product_kind_for_record(record: &WindowsInstalledAppInventoryRecord) -> String {
    if record.source_kind == APP_GAME_INVENTORY_SOURCE_UNKNOWN {
        APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE.to_string()
    } else if record.source_kind == APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST
        || record.launcher_app_id.is_some()
        || record.launcher_manifest_id.is_some()
    {
        APP_GAME_PRODUCT_NATIVE_GAME.to_string()
    } else if record.launcher_ref.is_some() {
        APP_GAME_PRODUCT_LAUNCHER.to_string()
    } else {
        APP_GAME_PRODUCT_NATIVE_APP.to_string()
    }
}

fn classification_state_for_record(
    record: &WindowsInstalledAppInventoryRecord,
    product_kind: &str,
) -> String {
    if record.inventory_state == APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED {
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED.to_string()
    } else if record.inventory_state == APP_GAME_INVENTORY_STATE_ADAPTER_ERROR {
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR.to_string()
    } else if record.inventory_state == APP_GAME_INVENTORY_STATE_STALE {
        APP_GAME_CLASSIFICATION_STALE.to_string()
    } else if product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
        APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string()
    } else if product_kind == APP_GAME_PRODUCT_LAUNCHER {
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string()
    } else if product_kind == APP_GAME_PRODUCT_NATIVE_APP {
        APP_GAME_CLASSIFICATION_KNOWN_APP.to_string()
    } else {
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string()
    }
}

fn catalog_ready_state_for_record(record: &WindowsInstalledAppInventoryRecord) -> String {
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

fn capability_status_for_record(record: &WindowsInstalledAppInventoryRecord) -> String {
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

fn category_candidates_for_record(
    record: &WindowsInstalledAppInventoryRecord,
    product_kind: &str,
) -> Vec<AppGameInventoryCategoryCandidate> {
    if product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
        vec![category_candidate(record, APP_GAME_INVENTORY_CATEGORY_GAME)]
    } else if product_kind == APP_GAME_PRODUCT_LAUNCHER {
        vec![category_candidate(
            record,
            APP_GAME_INVENTORY_CATEGORY_LAUNCHER,
        )]
    } else {
        Vec::new()
    }
}

fn category_candidate(
    record: &WindowsInstalledAppInventoryRecord,
    category_kind: &str,
) -> AppGameInventoryCategoryCandidate {
    AppGameInventoryCategoryCandidate {
        category_kind: category_kind.to_string(),
        confidence: record.confidence,
        catalog_ref: record.catalog_ref.clone(),
        evidence: record.evidence.clone(),
    }
}
