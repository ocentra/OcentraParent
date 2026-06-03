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
pub const APP_GAME_FOREGROUND_FOREGROUND: &str = "foreground";
pub const APP_GAME_FOREGROUND_BACKGROUND: &str = "background";
pub const APP_GAME_FOREGROUND_UNKNOWN: &str = "unknown";
pub const APP_GAME_FOREGROUND_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_FOREGROUND_DEGRADED: &str = "degraded";
pub const APP_GAME_FOREGROUND_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_FOREGROUND_NOT_CLAIMED: &str = "notClaimed";
pub const APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED: &str = "notClaimed";
pub const APP_GAME_TITLE_CAPTURE_TITLE_REF: &str = "titleRef";
pub const APP_GAME_TITLE_CAPTURE_TITLE_OMITTED: &str = "titleOmitted";
pub const APP_GAME_TITLE_CAPTURE_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_TITLE_CAPTURE_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_TITLE_CAPTURE_NOT_CLAIMED: &str = "notClaimed";
pub const APP_GAME_RUNTIME_RUNNING: &str = "running";
pub const APP_GAME_RUNTIME_NOT_RUNNING: &str = "notRunning";
pub const APP_GAME_RUNTIME_NOT_CLAIMED: &str = "notClaimed";
pub const APP_GAME_RUNTIME_UNKNOWN: &str = "unknown";
pub const APP_GAME_RUNTIME_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_RUNTIME_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_RUNTIME_DEGRADED: &str = "degraded";
pub const APP_GAME_RUNTIME_STALE: &str = "stale";
pub const APP_GAME_RUNTIME_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW: &str = "foregroundWindow";
pub const APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT: &str = "processSnapshot";
pub const APP_GAME_OBSERVATION_MODE_PROCESS_START: &str = "processStart";
pub const APP_GAME_OBSERVATION_MODE_PROCESS_EXIT: &str = "processExit";
pub const APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST: &str = "launcherManifest";
pub const APP_GAME_LAUNCHER_KIND_STEAM: &str = "steam";
pub const APP_GAME_LAUNCHER_KIND_UNKNOWN: &str = "unknownLauncher";
pub const APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY: &str = "launcherOnly";
pub const APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE: &str = "launcherManifestCandidate";
pub const APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE: &str = "childProcessCandidate";
pub const APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME: &str = "deterministicChildGame";
pub const APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME: &str = "classifierBackedChildGame";
pub const APP_GAME_LAUNCHER_PROOF_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_LAUNCHER_PROOF_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_LAUNCHER_PROOF_NOT_CLAIMED: &str = "notClaimed";
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
pub const APP_GAME_TEST_STORE_APP_SOURCE_REF: &str = "source-store-package-app";
pub const APP_GAME_TEST_STORE_GAME_SOURCE_REF: &str = "source-store-package-game";
pub const APP_GAME_TEST_STORE_APP_DISPLAY_LABEL: &str = "Ocentra Store App Fixture";
pub const APP_GAME_TEST_STORE_GAME_DISPLAY_LABEL: &str = "Ocentra Store Game Fixture";
pub const APP_GAME_TEST_STORE_APP_PACKAGE_ID: &str = "package-ref-ocentra-store-app";
pub const APP_GAME_TEST_STORE_GAME_PACKAGE_ID: &str = "package-ref-ocentra-store-game";
pub const APP_GAME_TEST_STORE_APP_BUNDLE_ID: &str = "bundle-ref-ocentra-store-app";
pub const APP_GAME_TEST_STORE_GAME_BUNDLE_ID: &str = "bundle-ref-ocentra-store-game";
pub const APP_GAME_TEST_STORE_APP_USER_MODEL_ID: &str = "aumid-ref-ocentra-store-app";
pub const APP_GAME_TEST_STORE_GAME_USER_MODEL_ID: &str = "aumid-ref-ocentra-store-game";
pub const APP_GAME_TEST_STORE_APP_STORE_ID: &str = "store-ref-ocentra-store-app";
pub const APP_GAME_TEST_STORE_GAME_STORE_ID: &str = "store-ref-ocentra-store-game";
pub const APP_GAME_TEST_STORE_APP_CATALOG_REF: &str = "catalog-ref-ocentra-store-app";
pub const APP_GAME_TEST_STORE_GAME_CATALOG_REF: &str = "catalog-ref-ocentra-store-game";
pub const APP_GAME_TEST_RUNTIME_EVIDENCE_ID: &str = "runtime-evidence-process-4242";
pub const APP_GAME_TEST_RUNTIME_EXIT_EVIDENCE_ID: &str = "runtime-evidence-process-4242-exit";
pub const APP_GAME_TEST_RUNTIME_LAUNCHER_EVIDENCE_ID: &str = "runtime-evidence-launcher-5150";
pub const APP_GAME_TEST_RUNTIME_PERMISSION_EVIDENCE_ID: &str = "runtime-evidence-private-6161";
pub const APP_GAME_TEST_FOREGROUND_EVIDENCE_ID: &str = "foreground-evidence-window-4242";
pub const APP_GAME_TEST_FOREGROUND_CLOSED_EVIDENCE_ID: &str =
    "foreground-evidence-window-4242-closed";
pub const APP_GAME_TEST_FOREGROUND_PERMISSION_EVIDENCE_ID: &str =
    "foreground-evidence-window-permission-limited";
pub const APP_GAME_TEST_LAUNCHER_EVIDENCE_ID: &str = "launcher-evidence-steam-only";
pub const APP_GAME_TEST_LAUNCHER_CANDIDATE_EVIDENCE_ID: &str = "launcher-evidence-steam-candidate";
pub const APP_GAME_TEST_LAUNCHER_KNOWN_GAME_EVIDENCE_ID: &str =
    "launcher-evidence-steam-known-game";
pub const APP_GAME_TEST_LAUNCHER_CHILD_PROCESS_IDENTITY: &str = "process-cs2-candidate";
pub const APP_GAME_TEST_LAUNCHER_CHILD_INVENTORY_ENTRY_ID: &str = "inventory-cs2";
pub const APP_GAME_TEST_LAUNCHER_CHILD_GAME_CLAIM_ID: &str = "claim-cs2-child-game";
pub const APP_GAME_TEST_WINDOW_REF: &str = "window-ref-4242";
pub const APP_GAME_TEST_WINDOW_TITLE_REF: &str = "title-ref-4242";
pub const APP_GAME_TEST_PROCESS_ID: u64 = 4242;
pub const APP_GAME_TEST_PARENT_PROCESS_ID: u64 = 1000;
pub const APP_GAME_TEST_LAUNCHER_PROCESS_ID: u64 = 5150;
pub const APP_GAME_TEST_PERMISSION_PROCESS_ID: u64 = 6161;
pub const APP_GAME_TEST_PROCESS_IDENTITY: &str = "process-4242";
pub const APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY: &str = "process-5150";
pub const APP_GAME_TEST_PERMISSION_PROCESS_IDENTITY: &str = "process-6161";
pub const APP_GAME_TEST_PROCESS_NAME: &str = "ocentra-fixture.exe";
pub const APP_GAME_TEST_LAUNCHER_PROCESS_NAME: &str = "steam.exe";
pub const APP_GAME_TEST_PERMISSION_PROCESS_NAME: &str = "private-process.exe";
pub const APP_GAME_TEST_PUBLISHER_SIGNATURE_REF: &str = "signature-ref-ocentra-fixture";
pub const APP_GAME_TEST_FILE_HASH_REF: &str = "hash-ref-ocentra-fixture";

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
pub struct AppGameRuntimeEvidenceRow {
    pub schema_version: u16,
    pub runtime_evidence_id: String,
    pub observed_at: String,
    pub process_identity: String,
    pub process_id: u64,
    pub parent_process_id: Option<u64>,
    pub process_name: String,
    pub executable_path_ref: Option<String>,
    pub publisher_signature_ref: Option<String>,
    pub file_hash_ref: Option<String>,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub started_at: Option<String>,
    pub exited_at: Option<String>,
    pub running_duration_ms: u64,
    pub runtime_state: String,
    pub foreground_state: String,
    pub observation_mode: String,
    pub classification_state: String,
    pub catalog_ready_state: String,
    pub capability_status: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameForegroundEvidenceRow {
    pub schema_version: u16,
    pub foreground_evidence_id: String,
    pub observed_at: String,
    pub process_identity: String,
    pub process_id: u64,
    pub process_name: String,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub window_ref: Option<String>,
    pub window_title_ref: Option<String>,
    pub title_capture_state: String,
    pub foreground_started_at: Option<String>,
    pub foreground_ended_at: Option<String>,
    pub foreground_duration_ms: u64,
    pub runtime_state: String,
    pub foreground_state: String,
    pub observation_mode: String,
    pub classification_state: String,
    pub catalog_ready_state: String,
    pub capability_status: String,
    pub content_knowledge_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameLauncherEvidenceRow {
    pub schema_version: u16,
    pub launcher_evidence_id: String,
    pub observed_at: String,
    pub launcher_kind: String,
    pub launcher_ref: String,
    pub launcher_inventory_entry_id: Option<String>,
    pub launcher_manifest_id: Option<String>,
    pub launcher_app_id: Option<String>,
    pub launcher_process_identity: Option<String>,
    pub launcher_process_id: Option<u64>,
    pub launcher_process_name: Option<String>,
    pub child_process_identity: Option<String>,
    pub child_inventory_entry_id: Option<String>,
    pub child_game_evidence_claim_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub runtime_state: String,
    pub foreground_state: String,
    pub observation_mode: String,
    pub classification_state: String,
    pub catalog_ready_state: String,
    pub capability_status: String,
    pub game_proof_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
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
