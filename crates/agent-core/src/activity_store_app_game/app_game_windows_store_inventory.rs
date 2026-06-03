use ocentra_parent_agent_protocol::{
    ActivityEvidenceRef, AppGameInventoryCategoryCandidate, AppGameInventoryEvidenceRow,
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_STALE, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_STALE, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_CATEGORY_GAME, APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE,
    APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE, APP_GAME_INVENTORY_STATE_ADAPTER_ERROR,
    APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED, APP_GAME_INVENTORY_STATE_STALE,
    APP_GAME_INVENTORY_STATE_UNAVAILABLE, APP_GAME_PRODUCT_NATIVE_APP,
    APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION,
};

pub struct WindowsStorePackageInventoryRecord {
    pub observed_at: String,
    pub source_ref: String,
    pub display_label: String,
    pub package_id: Option<String>,
    pub bundle_id: Option<String>,
    pub app_user_model_id: Option<String>,
    pub store_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub category_kind: Option<String>,
    pub inventory_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub struct WindowsRuntimePackageIdentity {
    pub package_id: Option<String>,
    pub app_user_model_id: Option<String>,
    pub executable_path_ref: Option<String>,
    pub display_label: Option<String>,
}

pub fn windows_store_inventory_rows_from_records(
    records: &[WindowsStorePackageInventoryRecord],
) -> Vec<AppGameInventoryEvidenceRow> {
    records.iter().map(row_from_record).collect()
}

pub fn store_package_matches_runtime_identity(
    record: &WindowsStorePackageInventoryRecord,
    runtime_identity: &WindowsRuntimePackageIdentity,
) -> bool {
    record.package_id.is_some() && record.package_id == runtime_identity.package_id
        || record.app_user_model_id.is_some()
            && record.app_user_model_id == runtime_identity.app_user_model_id
}

pub fn store_app_user_model_policy_target(
    record: &WindowsStorePackageInventoryRecord,
) -> Option<String> {
    record.app_user_model_id.clone()
}

fn row_from_record(record: &WindowsStorePackageInventoryRecord) -> AppGameInventoryEvidenceRow {
    let product_kind = product_kind_for_record(record);
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: record.source_ref.clone(),
        observed_at: record.observed_at.clone(),
        source_kind: APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE.to_string(),
        source_ref: record.source_ref.clone(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE.to_string(),
        product_kind: product_kind.clone(),
        display_label: record.display_label.clone(),
        identity_id: None,
        package_id: record.package_id.clone(),
        bundle_id: record.bundle_id.clone(),
        app_user_model_id: record.app_user_model_id.clone(),
        desktop_entry_id: None,
        executable_path_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
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

fn product_kind_for_record(record: &WindowsStorePackageInventoryRecord) -> String {
    if record.category_kind.as_deref() == Some(APP_GAME_INVENTORY_CATEGORY_GAME) {
        APP_GAME_PRODUCT_NATIVE_GAME.to_string()
    } else {
        APP_GAME_PRODUCT_NATIVE_APP.to_string()
    }
}

fn classification_state_for_record(
    record: &WindowsStorePackageInventoryRecord,
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
    } else {
        APP_GAME_CLASSIFICATION_KNOWN_APP.to_string()
    }
}

fn catalog_ready_state_for_record(record: &WindowsStorePackageInventoryRecord) -> String {
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

fn capability_status_for_record(record: &WindowsStorePackageInventoryRecord) -> String {
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
    record: &WindowsStorePackageInventoryRecord,
    product_kind: &str,
) -> Vec<AppGameInventoryCategoryCandidate> {
    if product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
        vec![AppGameInventoryCategoryCandidate {
            category_kind: APP_GAME_INVENTORY_CATEGORY_GAME.to_string(),
            confidence: record.confidence,
            catalog_ref: record.catalog_ref.clone(),
            evidence: record.evidence.clone(),
        }]
    } else {
        Vec::new()
    }
}
