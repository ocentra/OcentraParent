use serde::de::{Deserializer, Error as DeError};
use serde::{Deserialize, Serialize};

use crate::{
    activity::ActivityEvidenceKind, ActivityEvidenceRef, AppGameAiClassifierResult,
    AppGameControlActionResult, AppGameControlApprovalAuthority, AppGamePlatformAuthorityMatrix,
};

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
pub const APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN: &str = "inventoryScan";
pub const APP_GAME_OBSERVATION_MODE_LAUNCHER_MANIFEST: &str = "launcherManifest";
pub const APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY: &str = "inventory";
pub const APP_GAME_EVIDENCE_CLAIM_KIND_RUNTIME: &str = "runtime";
pub const APP_GAME_EVIDENCE_CLAIM_KIND_FOREGROUND: &str = "foreground";
pub const APP_GAME_EVIDENCE_CLAIM_KIND_LAUNCHER: &str = "launcher";
pub const APP_GAME_EVIDENCE_CLAIM_KIND_SESSION: &str = "session";
pub const APP_GAME_EVIDENCE_CLAIM_KIND_CATALOG: &str = "catalog";
pub const APP_GAME_EVIDENCE_CLAIM_KIND_AI_DIGEST: &str = "aiDigest";
pub const APP_GAME_IDENTITY_STRENGTH_DISPLAY_NAME_ONLY: &str = "displayNameOnly";
pub const APP_GAME_IDENTITY_STRENGTH_WEAK: &str = "weak";
pub const APP_GAME_IDENTITY_STRENGTH_OBSERVED_PROCESS: &str = "observedProcess";
pub const APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED: &str = "catalogMatched";
pub const APP_GAME_IDENTITY_STRENGTH_LAUNCHER_CLAIMED: &str = "launcherClaimed";
pub const APP_GAME_IDENTITY_STRENGTH_PLATFORM_MANAGED: &str = "platformManaged";
pub const APP_GAME_IDENTITY_STRENGTH_CHILD_GAME_PROOF: &str = "childGameProof";
pub const APP_GAME_IDENTITY_CONFIDENCE_WEAK: &str = "weak";
pub const APP_GAME_IDENTITY_CONFIDENCE_CANDIDATE: &str = "candidate";
pub const APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC: &str = "deterministic";
pub const APP_GAME_IDENTITY_CONFIDENCE_PARENT_LABELED: &str = "parentLabeled";
pub const APP_GAME_IDENTITY_CONFIDENCE_AI_ASSISTED: &str = "aiAssisted";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_PACKAGE_ID: &str = "packageId";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_BUNDLE_ID: &str = "bundleId";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_APP_USER_MODEL_ID: &str = "appUserModelId";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_DESKTOP_ENTRY_ID: &str = "desktopEntryId";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_APPLICATION_TOKEN_REF: &str = "applicationTokenRef";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_EXECUTABLE_PATH_REF: &str = "executablePathRef";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_PUBLISHER_SIGNATURE_REF: &str =
    "publisherSignatureRef";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF: &str = "fileHashRef";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_APP_ID: &str = "launcherAppId";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_MANIFEST_ID: &str = "launcherManifestId";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID: &str = "storeId";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_CATALOG_REF: &str = "catalogRef";
pub const APP_GAME_IDENTITY_DETERMINISTIC_REF_CHILD_GAME_EVIDENCE_CLAIM_ID: &str =
    "childGameEvidenceClaimId";
pub const APP_GAME_AI_ACTION_HINT_CLASSIFY_ONLY: &str = "classifyOnly";
pub const APP_GAME_AI_ACTION_HINT_SUMMARIZE_EVIDENCE: &str = "summarizeEvidence";
pub const APP_GAME_AI_ACTION_HINT_PARENT_REVIEW: &str = "parentReview";
pub const APP_GAME_AI_ACTION_HINT_POLICY_DRAFT_PREVIEW: &str = "policyDraftPreview";
pub const APP_GAME_AI_ACTION_HINT_ASK_PARENT_PREVIEW: &str = "askParentPreview";
pub const APP_GAME_AI_ACTION_HINT_MARK_UNAVAILABLE: &str = "markUnavailable";
pub const APP_GAME_SESSION_END_REASON_PROCESS_EXIT: &str = "processExit";
pub const APP_GAME_SESSION_END_REASON_TIMEOUT_INFERRED: &str = "timeoutInferred";
pub const APP_GAME_SESSION_END_REASON_DEVICE_SHUTDOWN: &str = "deviceShutdown";
pub const APP_GAME_SESSION_END_REASON_AGENT_RESTART: &str = "agentRestart";
pub const APP_GAME_SESSION_END_REASON_UNKNOWN: &str = "unknown";
pub const APP_GAME_LAUNCHER_KIND_STEAM: &str = "steam";
pub const APP_GAME_LAUNCHER_KIND_EPIC: &str = "epic";
pub const APP_GAME_LAUNCHER_KIND_XBOX: &str = "xbox";
pub const APP_GAME_LAUNCHER_KIND_RIOT: &str = "riot";
pub const APP_GAME_LAUNCHER_KIND_BATTLE_NET: &str = "battleNet";
pub const APP_GAME_LAUNCHER_KIND_EA: &str = "ea";
pub const APP_GAME_LAUNCHER_KIND_UBISOFT: &str = "ubisoft";
pub const APP_GAME_LAUNCHER_KIND_GOG: &str = "gog";
pub const APP_GAME_LAUNCHER_KIND_ROBLOX: &str = "roblox";
pub const APP_GAME_LAUNCHER_KIND_MINECRAFT: &str = "minecraft";
pub const APP_GAME_LAUNCHER_KIND_ITCH_IO: &str = "itchIo";
pub const APP_GAME_LAUNCHER_KIND_UNKNOWN: &str = "unknownLauncher";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_STEAM: &str = "steam";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_EPIC: &str = "epicgameslauncher";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_XBOX: &str = "xboxapp";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_GAMING_SERVICES: &str = "gamingservices";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_RIOT: &str = "riotclientservices";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_RIOT_UI: &str = "riotclientux";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_BATTLE_NET: &str = "battle.net";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_EA_DESKTOP: &str = "eadesktop";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_ORIGIN: &str = "origin";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT: &str = "upc";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT_CONNECT: &str = "ubisoftconnect";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_GALAXY: &str = "galaxyclient";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX: &str = "robloxplayerbeta";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX_PLAYER: &str = "robloxplayer";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_MINECRAFT: &str = "minecraftlauncher";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_ITCH: &str = "itch";
pub const APP_GAME_LAUNCHER_PROCESS_NAME_ITCH_IO: &str = "itchio";
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
pub const APP_GAME_INVENTORY_ENTRY_ID_PREFIX: &str = "inventory-source-ref-sha256-";
pub const APP_GAME_RUNTIME_EVIDENCE_ID_PREFIX: &str = "runtime-evidence-process-";
pub const APP_GAME_FOREGROUND_EVIDENCE_ID_PREFIX: &str = "foreground-evidence-window-";
pub const APP_GAME_LAUNCHER_EVIDENCE_ID_PREFIX: &str = "launcher-evidence-process-";
pub const APP_GAME_LAUNCHER_REF_PREFIX: &str = "launcher-ref-sha256-";
pub const APP_GAME_DESKTOP_ENTRY_ID_PREFIX: &str = "desktop-entry-ref-sha256-";
pub const APP_GAME_EXECUTABLE_PATH_REF_PREFIX: &str = "path-ref-sha256-";
pub const APP_GAME_WINDOW_REF_PREFIX: &str = "window-ref-sha256-";
pub const APP_GAME_WINDOW_TITLE_REF_PREFIX: &str = "title-ref-sha256-";
pub const APP_GAME_WINDOWS_SHORTCUT_EXTENSION: &str = "lnk";
pub const APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME: &str = "AppxManifest.xml";
pub const APP_GAME_WINDOWS_PATH_WINDOWS_APPS: &str = "WindowsApps";
pub const APP_GAME_WINDOWS_PATH_MICROSOFT: &str = "Microsoft";
pub const APP_GAME_WINDOWS_PATH_WINDOWS: &str = "Windows";
pub const APP_GAME_WINDOWS_PATH_START_MENU: &str = "Start Menu";
pub const APP_GAME_WINDOWS_PATH_PROGRAMS: &str = "Programs";
pub const APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION: &str = "reg";
pub const APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE: &str = "HKEY_LOCAL_MACHINE";
pub const APP_GAME_WINDOWS_REGISTRY_CURRENT_USER_HIVE: &str = "HKEY_CURRENT_USER";
pub const APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH: &str =
    "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
pub const APP_GAME_WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH: &str =
    "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
pub const APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE: &str = "DisplayName";
pub const APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE: &str = "InstallLocation";
pub const APP_GAME_WINDOWS_REGISTRY_DISPLAY_ICON_VALUE: &str = "DisplayIcon";
pub const APP_GAME_WINDOWS_REGISTRY_UNINSTALL_STRING_VALUE: &str = "UninstallString";
pub const APP_GAME_WINDOWS_REGISTRY_QUIET_UNINSTALL_STRING_VALUE: &str = "QuietUninstallString";
pub const APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE: &str = "SystemComponent";
pub const APP_GAME_WINDOWS_REGISTRY_EXPORT_HEADER: &str = "Windows Registry Editor Version 5.00";
pub const APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX: &str = "dword:";
pub const APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_VALUE: &str = "00000001";
pub const APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_TEXT: &str = "1";
pub const APP_GAME_APPX_ELEMENT_APPLICATION: &str = "Application";
pub const APP_GAME_APPX_ELEMENT_DISPLAY_NAME: &str = "DisplayName";
pub const APP_GAME_APPX_ELEMENT_IDENTITY: &str = "Identity";
pub const APP_GAME_APPX_ELEMENT_VISUAL_ELEMENTS: &str = "VisualElements";
pub const APP_GAME_APPX_ATTRIBUTE_DISPLAY_NAME: &str = "DisplayName";
pub const APP_GAME_APPX_ATTRIBUTE_ID: &str = "Id";
pub const APP_GAME_APPX_ATTRIBUTE_NAME: &str = "Name";
pub const APP_GAME_APPX_ATTRIBUTE_PUBLISHER: &str = "Publisher";
pub const APP_GAME_JOURNAL_FIELD_ROW_KIND: &str = "appGameRowKind";
pub const APP_GAME_JOURNAL_FIELD_ROW_JSON: &str = "appGameRowJson";
pub const APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL: &str = "appGameCustodyLabel";
pub const APP_GAME_JOURNAL_FIELD_REPLAY_STATE: &str = "appGameReplayState";
pub const APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE: &str = "appGameClassificationState";
pub const APP_GAME_JOURNAL_ROW_KIND_INVENTORY: &str = "inventory";
pub const APP_GAME_JOURNAL_ROW_KIND_RUNTIME: &str = "runtime";
pub const APP_GAME_JOURNAL_ROW_KIND_FOREGROUND: &str = "foreground";
pub const APP_GAME_JOURNAL_ROW_KIND_LAUNCHER: &str = "launcher";
pub const APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM: &str = "evidenceClaim";
pub const APP_GAME_JOURNAL_ROW_KIND_IDENTITY: &str = "identity";
pub const APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY: &str = "approvalAuthority";
pub const APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT: &str = "approvalActionResult";
pub const APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX: &str = "platformAuthorityMatrix";
pub const APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT: &str = "aiClassifierResult";
pub const APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL: &str = "localJournal";
pub const APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE: &str = "localSqlite";
pub const APP_GAME_JOURNAL_REPLAY_STATE_STORED: &str = "stored";
pub const APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED: &str = "replayed";
pub const APP_GAME_JOURNAL_SOURCE_ID: &str = "app-game-journal-ingest";
pub const APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID: &str = "app-game-inventory";
pub const APP_GAME_JOURNAL_LAUNCHER_SUBJECT_ID: &str = "app-game-launcher";
pub const APP_GAME_JOURNAL_EVIDENCE_CLAIM_SUBJECT_ID: &str = "app-game-evidence-claim";
pub const APP_GAME_JOURNAL_IDENTITY_SUBJECT_ID: &str = "app-game-identity";
pub const APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID: &str = "app-game-authority";
pub const APP_GAME_JOURNAL_CLASSIFIER_SUBJECT_ID: &str = "app-game-classifier";
pub const APP_GAME_CONFIDENCE_UNKNOWN: f64 = 0.0;
pub const APP_GAME_CONFIDENCE_OS_INSTALLED_RECORD: f64 = 0.84;
pub const APP_GAME_CONFIDENCE_SHORTCUT_INVENTORY: f64 = 0.8;
pub const APP_GAME_CONFIDENCE_STORE_PACKAGE_MANIFEST: f64 = 0.86;
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
pub const APP_GAME_TEST_STORE_APP_APPLICATION_ID: &str = "App";
pub const APP_GAME_TEST_STORE_PACKAGE_MANIFEST_USER_MODEL_ID: &str =
    "package-ref-ocentra-store-app!App";
pub const APP_GAME_TEST_STORE_PACKAGE_MANIFEST_XML: &str = r#"<Package xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10">
  <Identity Name="package-ref-ocentra-store-app" Publisher="bundle-ref-ocentra-store-app" Version="1.0.0.0" />
  <Properties>
    <DisplayName>Ocentra Store App Fixture</DisplayName>
  </Properties>
  <Applications>
    <Application Id="App">
      <uap:VisualElements DisplayName="Ocentra Store App Fixture" />
    </Application>
  </Applications>
</Package>"#;
pub const APP_GAME_TEST_STORE_APP_STORE_ID: &str = "store-ref-ocentra-store-app";
pub const APP_GAME_TEST_STORE_GAME_STORE_ID: &str = "store-ref-ocentra-store-game";
pub const APP_GAME_TEST_STORE_APP_CATALOG_REF: &str = "catalog-ref-ocentra-store-app";
pub const APP_GAME_TEST_STORE_GAME_CATALOG_REF: &str = "catalog-ref-ocentra-store-game";
pub const APP_GAME_TEST_SHORTCUT_FILE_NAME: &str = "Ocentra Inventory Fixture.lnk";
pub const APP_GAME_TEST_SECOND_SHORTCUT_FILE_NAME: &str = "Ocentra Game Fixture.lnk";
pub const APP_GAME_TEST_LIVE_INVENTORY_SUFFIX: &str = "live-inventory";
pub const APP_GAME_TEST_IDENTITY_ID: &str = "identity-ocentra-game";
pub const APP_GAME_TEST_SECOND_IDENTITY_ID: &str = "identity-ocentra-game-second";
pub const APP_GAME_TEST_MERGE_ID: &str = "identity-merge-ocentra-game";
pub const APP_GAME_TEST_EVIDENCE_CLAIM_ID: &str = "claim-ocentra-inventory";
pub const APP_GAME_TEST_AI_DIGEST_REF: &str = "digest-ref-app-game";
pub const APP_GAME_TEST_AI_DIGEST: &str = "sha256:app-game-digest";
pub const APP_GAME_TEST_UNAVAILABLE_REASON: &str = "provider-unavailable";
pub const APP_GAME_TEST_PARENT_LABEL: &str = "Parent labeled game fixture";
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

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameIdentity {
    pub schema_version: u16,
    pub identity_id: String,
    pub product_kind: String,
    pub display_label: String,
    pub parent_label: Option<String>,
    pub confidence: String,
    pub classification_state: String,
    pub package_id: Option<String>,
    pub bundle_id: Option<String>,
    pub app_user_model_id: Option<String>,
    pub desktop_entry_id: Option<String>,
    pub application_token_ref: Option<String>,
    pub executable_path_ref: Option<String>,
    pub publisher_signature_ref: Option<String>,
    pub file_hash_ref: Option<String>,
    pub launcher_ref: Option<String>,
    pub launcher_app_id: Option<String>,
    pub launcher_manifest_id: Option<String>,
    pub store_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub child_game_evidence_claim_id: Option<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameIdentityMergeProof {
    pub schema_version: u16,
    pub merge_id: String,
    pub target_identity: AppGameIdentity,
    pub source_identity_ids: Vec<String>,
    pub merge_confidence: f64,
    pub display_label_matched: bool,
    pub parent_label_changed: bool,
    pub conflicting_file_hash_refs: bool,
    pub shared_deterministic_refs: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AppGameEvidenceRefWire {
    evidence_id: String,
    kind: ActivityEvidenceKind,
    digest: Option<String>,
    uri: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AppGameIdentityWire {
    schema_version: u16,
    identity_id: String,
    product_kind: String,
    display_label: String,
    parent_label: Option<String>,
    confidence: String,
    classification_state: String,
    package_id: Option<String>,
    bundle_id: Option<String>,
    app_user_model_id: Option<String>,
    desktop_entry_id: Option<String>,
    application_token_ref: Option<String>,
    executable_path_ref: Option<String>,
    publisher_signature_ref: Option<String>,
    file_hash_ref: Option<String>,
    launcher_ref: Option<String>,
    launcher_app_id: Option<String>,
    launcher_manifest_id: Option<String>,
    store_id: Option<String>,
    catalog_ref: Option<String>,
    child_game_evidence_claim_id: Option<String>,
    evidence: Vec<AppGameEvidenceRefWire>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AppGameIdentityMergeProofWire {
    schema_version: u16,
    merge_id: String,
    target_identity: AppGameIdentity,
    source_identity_ids: Vec<String>,
    merge_confidence: f64,
    display_label_matched: bool,
    parent_label_changed: bool,
    conflicting_file_hash_refs: bool,
    shared_deterministic_refs: Vec<String>,
    evidence: Vec<AppGameEvidenceRefWire>,
}

impl<'de> Deserialize<'de> for AppGameIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = AppGameIdentityWire::deserialize(deserializer)?;
        let identity: Self = wire.into();
        identity.validate().map_err(D::Error::custom)?;
        Ok(identity)
    }
}

impl<'de> Deserialize<'de> for AppGameIdentityMergeProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = AppGameIdentityMergeProofWire::deserialize(deserializer)?;
        let merge = Self {
            schema_version: wire.schema_version,
            merge_id: wire.merge_id,
            target_identity: wire.target_identity,
            source_identity_ids: wire.source_identity_ids,
            merge_confidence: wire.merge_confidence,
            display_label_matched: wire.display_label_matched,
            parent_label_changed: wire.parent_label_changed,
            conflicting_file_hash_refs: wire.conflicting_file_hash_refs,
            shared_deterministic_refs: wire.shared_deterministic_refs,
            evidence: wire.evidence.into_iter().map(Into::into).collect(),
        };
        merge.validate().map_err(D::Error::custom)?;
        Ok(merge)
    }
}

impl AppGameIdentity {
    fn validate(&self) -> Result<(), &'static str> {
        if self.schema_version != APP_GAME_SCHEMA_VERSION {
            return Err("app game identity schema version is unsupported");
        }
        if is_blank(&self.identity_id) {
            return Err("app game identity id must not be empty");
        }
        if is_blank(&self.display_label) {
            return Err("app game identity display label must not be empty");
        }
        if ![
            APP_GAME_PRODUCT_NATIVE_APP,
            APP_GAME_PRODUCT_NATIVE_GAME,
            APP_GAME_PRODUCT_LAUNCHER,
            APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE,
        ]
        .contains(&self.product_kind.as_str())
        {
            return Err("app game identity product kind is unsupported");
        }
        if ![
            APP_GAME_IDENTITY_CONFIDENCE_WEAK,
            APP_GAME_IDENTITY_CONFIDENCE_CANDIDATE,
            APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC,
            APP_GAME_IDENTITY_CONFIDENCE_PARENT_LABELED,
            APP_GAME_IDENTITY_CONFIDENCE_AI_ASSISTED,
        ]
        .contains(&self.confidence.as_str())
        {
            return Err("app game identity confidence is unsupported");
        }
        if ![
            APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
            APP_GAME_CLASSIFICATION_KNOWN_APP,
            APP_GAME_CLASSIFICATION_KNOWN_GAME,
            APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
            APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
            APP_GAME_CLASSIFICATION_POSSIBLY_GAME,
            APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
            APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM,
            APP_GAME_CLASSIFICATION_STALE,
            APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
        ]
        .contains(&self.classification_state.as_str())
        {
            return Err("app game identity classification state is unsupported");
        }

        for (field, value) in [
            ("parentLabel", self.parent_label.as_deref()),
            ("packageId", self.package_id.as_deref()),
            ("bundleId", self.bundle_id.as_deref()),
            ("appUserModelId", self.app_user_model_id.as_deref()),
            ("desktopEntryId", self.desktop_entry_id.as_deref()),
            ("applicationTokenRef", self.application_token_ref.as_deref()),
            ("executablePathRef", self.executable_path_ref.as_deref()),
            (
                "publisherSignatureRef",
                self.publisher_signature_ref.as_deref(),
            ),
            ("fileHashRef", self.file_hash_ref.as_deref()),
            ("launcherRef", self.launcher_ref.as_deref()),
            ("launcherAppId", self.launcher_app_id.as_deref()),
            ("launcherManifestId", self.launcher_manifest_id.as_deref()),
            ("storeId", self.store_id.as_deref()),
            ("catalogRef", self.catalog_ref.as_deref()),
            (
                "childGameEvidenceClaimId",
                self.child_game_evidence_claim_id.as_deref(),
            ),
        ] {
            if value.is_some_and(is_blank) {
                return Err(match field {
                    "parentLabel" => "app game identity parent label must not be empty",
                    _ => "app game identity reference must not be empty",
                });
            }
        }

        validate_evidence_refs(
            &self.evidence,
            "app game identity must cite at least one evidence ref",
        )?;

        let has_raw_reference = app_game_identity_has_raw_reference(self);
        if !has_raw_reference
            && !(self.confidence == APP_GAME_IDENTITY_CONFIDENCE_WEAK
                && self.classification_state == APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
                && self.product_kind == APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE)
        {
            return Err("display-name-only app game identity must remain weak and unknown");
        }

        if (self.confidence == APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC
            || self.confidence == APP_GAME_IDENTITY_CONFIDENCE_PARENT_LABELED)
            && !app_game_identity_has_deterministic_reference(self)
            && !(self.product_kind == APP_GAME_PRODUCT_LAUNCHER && has_raw_reference)
        {
            return Err("deterministic app game identity must include an identity reference");
        }

        if app_game_identity_has_only_launcher_references(self)
            && (self.product_kind != APP_GAME_PRODUCT_LAUNCHER
                || self.classification_state == APP_GAME_CLASSIFICATION_KNOWN_GAME)
        {
            return Err("launcher-only app game identity cannot claim a known game");
        }

        Ok(())
    }
}

impl AppGameIdentityMergeProof {
    fn validate(&self) -> Result<(), &'static str> {
        if self.schema_version != APP_GAME_SCHEMA_VERSION {
            return Err("app game identity merge schema version is unsupported");
        }
        if is_blank(&self.merge_id) {
            return Err("app game identity merge id must not be empty");
        }
        if self.source_identity_ids.len() < 2 {
            return Err("app game identity merge must cite source identities");
        }
        if self.source_identity_ids.iter().any(|id| is_blank(id)) {
            return Err("app game identity merge source id must not be empty");
        }
        for (index, source_id) in self.source_identity_ids.iter().enumerate() {
            if self.source_identity_ids[..index].contains(source_id) {
                return Err("app game identity merge source ids must be distinct");
            }
            if source_id == &self.target_identity.identity_id {
                return Err("app game identity merge sources must not include the target");
            }
        }
        if !self.merge_confidence.is_finite()
            || self.merge_confidence < 0.0
            || self.merge_confidence > 1.0
        {
            return Err("app game identity merge confidence must be between zero and one");
        }
        if self.conflicting_file_hash_refs {
            return Err("conflicting file hashes must block app game identity merge");
        }
        if self.merge_confidence > 0.3 && self.shared_deterministic_refs.is_empty() {
            return Err("non-weak app game identity merge must share deterministic refs");
        }
        if self.parent_label_changed
            && (self.target_identity.parent_label.is_none()
                || self.shared_deterministic_refs.is_empty())
        {
            return Err("parent labels must not create an app game identity merge");
        }
        if self
            .shared_deterministic_refs
            .iter()
            .any(|kind| is_blank(kind) || !app_game_identity_deterministic_ref_kind_is_known(kind))
        {
            return Err("app game identity merge contains an unsupported deterministic ref kind");
        }
        validate_evidence_refs(
            &self.evidence,
            "app game identity merge must cite at least one evidence ref",
        )?;
        self.target_identity.validate()
    }
}

fn validate_evidence_refs(
    evidence: &[ActivityEvidenceRef],
    empty_message: &'static str,
) -> Result<(), &'static str> {
    if evidence.is_empty() {
        return Err(empty_message);
    }
    if evidence
        .iter()
        .any(|reference| is_blank(&reference.evidence_id))
    {
        return Err("app game evidence ref id must not be empty");
    }
    Ok(())
}

fn is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

fn app_game_identity_has_raw_reference(identity: &AppGameIdentity) -> bool {
    app_game_identity_has_deterministic_reference(identity)
        || identity.launcher_ref.is_some()
        || identity.launcher_app_id.is_some()
        || identity.launcher_manifest_id.is_some()
}

fn app_game_identity_has_deterministic_reference(identity: &AppGameIdentity) -> bool {
    identity.package_id.is_some()
        || identity.bundle_id.is_some()
        || identity.app_user_model_id.is_some()
        || identity.desktop_entry_id.is_some()
        || identity.application_token_ref.is_some()
        || identity.executable_path_ref.is_some()
        || identity.publisher_signature_ref.is_some()
        || identity.file_hash_ref.is_some()
        || identity.store_id.is_some()
        || identity.catalog_ref.is_some()
        || identity.child_game_evidence_claim_id.is_some()
}

fn app_game_identity_has_only_launcher_references(identity: &AppGameIdentity) -> bool {
    !app_game_identity_has_deterministic_reference(identity)
        && (identity.launcher_ref.is_some()
            || identity.launcher_app_id.is_some()
            || identity.launcher_manifest_id.is_some())
}

fn app_game_identity_deterministic_ref_kind_is_known(kind: &str) -> bool {
    [
        APP_GAME_IDENTITY_DETERMINISTIC_REF_PACKAGE_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_BUNDLE_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_APP_USER_MODEL_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_DESKTOP_ENTRY_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_APPLICATION_TOKEN_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_EXECUTABLE_PATH_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_PUBLISHER_SIGNATURE_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_APP_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_LAUNCHER_MANIFEST_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_CATALOG_REF,
        APP_GAME_IDENTITY_DETERMINISTIC_REF_CHILD_GAME_EVIDENCE_CLAIM_ID,
    ]
    .contains(&kind)
}

impl From<AppGameIdentityWire> for AppGameIdentity {
    fn from(wire: AppGameIdentityWire) -> Self {
        Self {
            schema_version: wire.schema_version,
            identity_id: wire.identity_id,
            product_kind: wire.product_kind,
            display_label: wire.display_label,
            parent_label: wire.parent_label,
            confidence: wire.confidence,
            classification_state: wire.classification_state,
            package_id: wire.package_id,
            bundle_id: wire.bundle_id,
            app_user_model_id: wire.app_user_model_id,
            desktop_entry_id: wire.desktop_entry_id,
            application_token_ref: wire.application_token_ref,
            executable_path_ref: wire.executable_path_ref,
            publisher_signature_ref: wire.publisher_signature_ref,
            file_hash_ref: wire.file_hash_ref,
            launcher_ref: wire.launcher_ref,
            launcher_app_id: wire.launcher_app_id,
            launcher_manifest_id: wire.launcher_manifest_id,
            store_id: wire.store_id,
            catalog_ref: wire.catalog_ref,
            child_game_evidence_claim_id: wire.child_game_evidence_claim_id,
            evidence: wire.evidence.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<AppGameEvidenceRefWire> for ActivityEvidenceRef {
    fn from(wire: AppGameEvidenceRefWire) -> Self {
        Self {
            evidence_id: wire.evidence_id,
            kind: wire.kind,
            digest: wire.digest,
            uri: wire.uri,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameEvidenceClaim {
    pub schema_version: u16,
    pub claim_id: String,
    pub observed_at: String,
    pub claim_kind: String,
    pub observation_mode: String,
    pub display_name: String,
    pub identity_strength: String,
    pub classification_state: String,
    pub catalog_ready_state: String,
    pub runtime_state: String,
    pub foreground_state: String,
    pub inventory_entry_id: Option<String>,
    pub process_identity: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameProcessObservation {
    pub schema_version: u16,
    pub observed_at: String,
    pub process_identity: String,
    pub process_id: u64,
    pub process_name: String,
    pub executable_path: Option<String>,
    pub foreground_state: String,
    pub observation_mode: String,
    pub classification_state: String,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAiDigestReference {
    pub schema_version: u16,
    pub digest_ref: String,
    pub digest: Option<String>,
    pub generated_at: String,
    pub confidence: f64,
    pub source_evidence_ids: Vec<String>,
    pub source_session_ids: Vec<String>,
    pub unavailable_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAiClassificationDigest {
    pub schema_version: u16,
    pub digest_ref: String,
    pub digest: Option<String>,
    pub generated_at: String,
    pub classification_state: String,
    pub confidence: f64,
    pub action_hints: Vec<String>,
    pub source_evidence_ids: Vec<String>,
    pub source_session_ids: Vec<String>,
    pub unavailable_reason: Option<String>,
}

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
    pub end_reason: Option<String>,
    pub running_duration_ms: u64,
    pub foreground_duration_ms: u64,
    pub background_duration_ms: u64,
    pub last_foreground_at: Option<String>,
    pub last_background_at: Option<String>,
    pub observation_gap_ms: u64,
    pub observation_count: u64,
    pub evidence_count: u64,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub ai_digest_ref: Option<String>,
    pub confidence: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameSessionDailyRollup {
    pub schema_version: u16,
    pub rollup_date: String,
    pub classification_state: String,
    pub session_count: u64,
    pub running_duration_ms: u64,
    pub foreground_duration_ms: u64,
    pub background_duration_ms: u64,
    pub evidence_count: u64,
    pub session_ids: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
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
pub struct AppGameServiceReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub limit: u64,
    pub custody_label: String,
    pub replay_state: String,
    pub capability_status: String,
    pub inventory_returned: u64,
    pub running_now_returned: u64,
    pub foreground_now_returned: u64,
    pub launcher_returned: u64,
    pub daily_rollup_returned: u64,
    pub evidence_claim_returned: u64,
    pub identity_returned: u64,
    pub approval_authority_returned: u64,
    pub approval_action_result_returned: u64,
    pub platform_authority_matrix_returned: u64,
    pub ai_classifier_result_returned: u64,
    pub inventory_rows: Vec<AppGameInventoryEvidenceRow>,
    pub running_now_rows: Vec<AppGameRuntimeEvidenceRow>,
    pub foreground_now_rows: Vec<AppGameForegroundEvidenceRow>,
    pub launcher_rows: Vec<AppGameLauncherEvidenceRow>,
    pub daily_rollups: Vec<AppGameSessionDailyRollup>,
    pub evidence_claim_rows: Vec<AppGameEvidenceClaim>,
    pub identity_rows: Vec<AppGameIdentity>,
    pub approval_authority_rows: Vec<AppGameControlApprovalAuthority>,
    pub approval_action_result_rows: Vec<AppGameControlActionResult>,
    pub platform_authority_matrices: Vec<AppGamePlatformAuthorityMatrix>,
    pub ai_classifier_result_rows: Vec<AppGameAiClassifierResult>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppGameRiskCategoryKind {
    VpnProxy,
    RemoteDesktop,
    DownloadTorrent,
    InstallerUpdater,
    AiChatbot,
    SocialVideoMessaging,
    UnknownRisk,
}

impl AppGameRiskCategoryKind {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "vpnProxy" => Some(Self::VpnProxy),
            "remoteDesktop" => Some(Self::RemoteDesktop),
            "downloadTorrent" => Some(Self::DownloadTorrent),
            "installerUpdater" => Some(Self::InstallerUpdater),
            "aiChatbot" => Some(Self::AiChatbot),
            "socialVideoMessaging" => Some(Self::SocialVideoMessaging),
            "unknownRisk" => Some(Self::UnknownRisk),
            _ => None,
        }
    }
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
