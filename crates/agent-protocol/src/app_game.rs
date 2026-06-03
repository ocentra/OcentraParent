use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const APP_GAME_SCHEMA_VERSION: u16 = 1;
pub const APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS: &str = "unknownProcess";
pub const APP_GAME_CLASSIFICATION_KNOWN_APP: &str = "knownApp";
pub const APP_GAME_CLASSIFICATION_KNOWN_GAME: &str = "knownGame";
pub const APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER: &str = "knownLauncher";
pub const APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE: &str = "launcherGameCandidate";
pub const APP_GAME_CLASSIFICATION_POSSIBLY_GAME: &str = "possiblyGame";
pub const APP_GAME_CLASSIFICATION_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM: &str = "unsupportedPlatform";
pub const APP_GAME_CLASSIFICATION_STALE: &str = "stale";
pub const APP_GAME_CLASSIFICATION_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_CATALOG_READY: &str = "catalogReady";
pub const APP_GAME_CATALOG_UNAVAILABLE: &str = "catalogUnavailable";
pub const APP_GAME_CATALOG_NOT_LOADED: &str = "catalogNotLoaded";
pub const APP_GAME_CATALOG_STALE: &str = "catalogStale";
pub const APP_GAME_CATALOG_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_CAPABILITY_STATUS_AVAILABLE: &str = "available";
pub const APP_GAME_CAPABILITY_STATUS_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM: &str = "unsupportedPlatform";
pub const APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_CAPABILITY_STATUS_STALE: &str = "stale";
pub const APP_GAME_CAPABILITY_STATUS_DEGRADED: &str = "degraded";
pub const APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED: &str = "manualRequired";
pub const APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED: &str = "notClaimed";
pub const APP_GAME_FOREGROUND_NOT_CLAIMED: &str = "notClaimed";
pub const APP_GAME_RUNTIME_NOT_CLAIMED: &str = "notClaimed";
pub const APP_GAME_PRODUCT_NATIVE_APP: &str = "nativeApp";
pub const APP_GAME_PRODUCT_NATIVE_GAME: &str = "nativeGame";
pub const APP_GAME_PRODUCT_LAUNCHER: &str = "launcher";
pub const APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE: &str = "unknownExecutable";
pub const APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD: &str = "osInstalledRecord";
pub const APP_GAME_INVENTORY_SOURCE_SHORTCUT: &str = "shortcut";
pub const APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE: &str = "storePackage";
pub const APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST: &str = "launcherManifest";
pub const APP_GAME_INVENTORY_SOURCE_PARENT_CATALOG: &str = "parentCatalog";
pub const APP_GAME_INVENTORY_SOURCE_MANAGED_DEVICE: &str = "managedDevice";
pub const APP_GAME_INVENTORY_SOURCE_PORTABLE_APP: &str = "portableApp";
pub const APP_GAME_INVENTORY_SOURCE_UNKNOWN: &str = "unknownSource";
pub const APP_GAME_INVENTORY_STATE_INSTALLED: &str = "installed";
pub const APP_GAME_INVENTORY_STATE_DETECTABLE: &str = "detectable";
pub const APP_GAME_INVENTORY_STATE_MISSING: &str = "missing";
pub const APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_INVENTORY_STATE_STALE: &str = "stale";
pub const APP_GAME_INVENTORY_STATE_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_INVENTORY_STATE_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT: &str = "localAgent";
pub const APP_GAME_INVENTORY_CUSTODY_LAUNCHER_MANIFEST: &str = "launcherManifest";
pub const APP_GAME_INVENTORY_CUSTODY_PARENT_CATALOG: &str = "parentCatalog";
pub const APP_GAME_INVENTORY_CUSTODY_MANAGED_DEVICE: &str = "managedDevice";
pub const APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE: &str = "storePackage";
pub const APP_GAME_INVENTORY_CUSTODY_UNKNOWN: &str = "unknown";
pub const APP_GAME_INVENTORY_CATEGORY_GAME: &str = "game";
pub const APP_GAME_INVENTORY_CATEGORY_LAUNCHER: &str = "launcher";
pub const APP_GAME_INVENTORY_CATEGORY_UNKNOWN: &str = "unknown";
pub const APP_GAME_SESSION_ID_PREFIX: &str = "app-game-session-";
pub const APP_GAME_CONFIDENCE_UNKNOWN: f64 = 0.0;
pub const APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE: f64 = 0.25;
pub const APP_GAME_TEST_REGISTRY_SOURCE_REF: &str = "source-registry-native-app";
pub const APP_GAME_TEST_SHORTCUT_SOURCE_REF: &str = "source-start-menu-shortcut";
pub const APP_GAME_TEST_SECOND_SHORTCUT_SOURCE_REF: &str = "source-second-start-menu-shortcut";
pub const APP_GAME_TEST_LAUNCHER_SOURCE_REF: &str = "source-launcher-manifest-game";
pub const APP_GAME_TEST_UNKNOWN_SOURCE_REF: &str = "source-display-only-unknown";
pub const APP_GAME_TEST_SECOND_UNKNOWN_SOURCE_REF: &str = "source-display-only-second";
pub const APP_GAME_TEST_DISPLAY_LABEL: &str = "Ocentra Inventory Fixture";
pub const APP_GAME_TEST_GAME_DISPLAY_LABEL: &str = "Ocentra Game Fixture";
pub const APP_GAME_TEST_EXECUTABLE_PATH_REF: &str = "path-ref-ocentra-fixture";
pub const APP_GAME_TEST_SECOND_EXECUTABLE_PATH_REF: &str = "path-ref-ocentra-second-fixture";
pub const APP_GAME_TEST_PACKAGE_ID: &str = "package-ref-ocentra-fixture";
pub const APP_GAME_TEST_APP_USER_MODEL_ID: &str = "aumid-ref-ocentra-fixture";
pub const APP_GAME_TEST_DESKTOP_ENTRY_ID: &str = "desktop-entry-ref-ocentra-fixture";
pub const APP_GAME_TEST_SECOND_DESKTOP_ENTRY_ID: &str = "desktop-entry-ref-ocentra-second";
pub const APP_GAME_TEST_LAUNCHER_REF: &str = "launcher-ref-ocentra";
pub const APP_GAME_TEST_LAUNCHER_APP_ID: &str = "launcher-app-ref-ocentra-game";
pub const APP_GAME_TEST_LAUNCHER_MANIFEST_ID: &str = "launcher-manifest-ref-ocentra-game";
pub const APP_GAME_TEST_STORE_ID: &str = "store-ref-ocentra-game";
pub const APP_GAME_TEST_CATALOG_REF: &str = "catalog-ref-ocentra-game";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameSessionSummary {
    pub schema_version: u16,
    pub session_id: String,
    pub primary_process_identity: String,
    pub display_name: String,
    pub classification_state: String,
    pub catalog_ready_state: String,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub started_at: String,
    pub last_observed_at: String,
    pub ended_at: Option<String>,
    pub running_duration_ms: u64,
    pub foreground_duration_ms: u64,
    pub background_duration_ms: u64,
    pub observation_count: u64,
    pub evidence_count: u64,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub ai_digest_ref: Option<String>,
    pub confidence: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameSessionReport {
    pub schema_version: u16,
    pub limit: u64,
    pub returned: u64,
    pub catalog_ready_state: String,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub most_recent_session_id: Option<String>,
    pub most_recent_classification_state: Option<String>,
    pub most_recent_process_identity: Option<String>,
    pub most_recent_display_name: Option<String>,
    pub most_recent_running_duration_ms: Option<u64>,
    pub most_recent_foreground_duration_ms: Option<u64>,
    pub most_recent_evidence_count: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameInventoryCategoryCandidate {
    pub category_kind: String,
    pub confidence: f64,
    pub catalog_ref: Option<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameInventoryEvidenceRow {
    pub schema_version: u16,
    pub inventory_entry_id: String,
    pub observed_at: String,
    pub source_kind: String,
    pub source_ref: String,
    pub custody_state: String,
    pub product_kind: String,
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
    pub classification_state: String,
    pub catalog_ready_state: String,
    pub capability_status: String,
    pub confidence: f64,
    pub category_candidates: Vec<AppGameInventoryCategoryCandidate>,
    pub runtime_state: String,
    pub foreground_state: String,
    pub running_duration_ms: u64,
    pub foreground_duration_ms: u64,
    pub evidence: Vec<ActivityEvidenceRef>,
}
