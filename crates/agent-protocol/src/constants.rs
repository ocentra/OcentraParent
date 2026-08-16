pub mod endpoint {
    pub const BROWSER_INTERVENTION_PAGE: &str = "/api/browser/intervention/page";
    pub const HEALTH: &str = "/health";
    pub const DEV_LOG_SNAPSHOT: &str = "/api/dev/log-snapshot";
    pub const DEV_WS: &str = "/api/dev/ws";
}

pub mod http {
    pub const CACHE_CONTROL_NO_STORE: &str = "no-store";
    pub const CONTENT_TYPE_TEXT_HTML_UTF8: &str = "text/html; charset=utf-8";
}

pub mod env_var {
    pub const AGENT_ALLOWED_ORIGINS: &str = "OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS";
    pub const AGENT_ADDR: &str = "OCENTRA_PARENT_AGENT_ADDR";
    pub const AGENT_LAN_PAIRING_REGISTRY_PATH: &str =
        "OCENTRA_PARENT_AGENT_LAN_PAIRING_REGISTRY_PATH";
    pub const AGENT_LOCAL_NETWORK_ENABLED: &str = "OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED";
    pub const AGENT_ENFORCEMENT_TIMER_STATE_PATH: &str =
        "OCENTRA_PARENT_AGENT_ENFORCEMENT_TIMER_STATE_PATH";
    pub const AGENT_BROWSER_POLICY_STORE_PATH: &str = "OCENTRA_PARENT_BROWSER_POLICY_STORE_PATH";
    pub const AGENT_SCREEN_SETTINGS_STORE_PATH: &str = "OCENTRA_PARENT_SCREEN_SETTINGS_STORE_PATH";
    pub const ACTIVITY_CAPTURE_STARTUP_DISABLED: &str =
        "OCENTRA_PARENT_ACTIVITY_CAPTURE_STARTUP_DISABLED";
    pub const ACTIVITY_DB_PATH: &str = "OCENTRA_PARENT_ACTIVITY_DB_PATH";
    pub const ACTIVITY_JOURNAL_KEY_PATH: &str = "OCENTRA_PARENT_ACTIVITY_JOURNAL_KEY_PATH";
    pub const ACTIVITY_JOURNAL_PATH: &str = "OCENTRA_PARENT_ACTIVITY_JOURNAL_PATH";
    pub const APP_DATA: &str = "APPDATA";
    pub const LOCAL_APP_DATA: &str = "LOCALAPPDATA";
    pub const MANAGED_BROWSER_BRIDGE_PORT: &str = "OCENTRA_PARENT_MANAGED_BROWSER_BRIDGE_PORT";
    pub const MANAGED_BROWSER_EXECUTABLE: &str = "OCENTRA_PARENT_MANAGED_BROWSER_EXECUTABLE";
    pub const MANAGED_BROWSER_INTERVENTION_HTML_PATH: &str =
        "OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_HTML_PATH";
    pub const MANAGED_BROWSER_LAUNCH_ON_STATUS: &str =
        "OCENTRA_PARENT_MANAGED_BROWSER_LAUNCH_ON_STATUS";
    pub const MANAGED_BROWSER_PROFILE_DIR: &str = "OCENTRA_PARENT_MANAGED_BROWSER_PROFILE_DIR";
    pub const PROGRAM_FILES: &str = "PROGRAMFILES";
    pub const PROGRAM_FILES_X86: &str = "ProgramFiles(x86)";
    pub const PROGRAM_DATA: &str = "PROGRAMDATA";
    pub const LOCAL_AI_RUNTIME_BINARY: &str = "OCENTRA_PARENT_LOCAL_AI_RUNTIME_BINARY";
    pub const LOCAL_AI_MODEL_ID: &str = "OCENTRA_PARENT_LOCAL_AI_MODEL_ID";
    pub const LOCAL_AI_MODEL_FILE: &str = "OCENTRA_PARENT_LOCAL_AI_MODEL_FILE";
    pub const LOCAL_AI_MODEL_ARTIFACT_REF: &str = "OCENTRA_PARENT_LOCAL_AI_MODEL_ARTIFACT_REF";
    pub const LOCAL_AI_MODEL_MANIFEST_REF: &str = "OCENTRA_PARENT_LOCAL_AI_MODEL_MANIFEST_REF";
    pub const LOCAL_AI_EXECUTION_ENABLED: &str = "OCENTRA_PARENT_LOCAL_AI_EXECUTION_ENABLED";
    pub const LOCAL_AI_GENERATION_TIMEOUT_MS: &str =
        "OCENTRA_PARENT_LOCAL_AI_GENERATION_TIMEOUT_MS";
    pub const LOCAL_AI_GENERATION_MAX_TOKENS: &str =
        "OCENTRA_PARENT_LOCAL_AI_GENERATION_MAX_TOKENS";
    pub const LOCAL_AI_RUNTIME_DEVICE: &str = "OCENTRA_PARENT_LOCAL_AI_RUNTIME_DEVICE";
    pub const LOCAL_AI_GPU_LAYERS: &str = "OCENTRA_PARENT_LOCAL_AI_GPU_LAYERS";
    pub const LOCAL_AI_SPLIT_MODE: &str = "OCENTRA_PARENT_LOCAL_AI_SPLIT_MODE";
    pub const LOCAL_AI_TENSOR_SPLIT: &str = "OCENTRA_PARENT_LOCAL_AI_TENSOR_SPLIT";
    pub const LOCAL_AI_MAIN_GPU: &str = "OCENTRA_PARENT_LOCAL_AI_MAIN_GPU";
    pub const LOCAL_AI_FIT: &str = "OCENTRA_PARENT_LOCAL_AI_FIT";
    pub const LOCAL_AI_FIT_TARGET: &str = "OCENTRA_PARENT_LOCAL_AI_FIT_TARGET";
    pub const LOCAL_AI_OP_OFFLOAD: &str = "OCENTRA_PARENT_LOCAL_AI_OP_OFFLOAD";
    pub const LOCAL_AI_CPU_MOE: &str = "OCENTRA_PARENT_LOCAL_AI_CPU_MOE";
    pub const LOCAL_AI_CPU_MOE_LAYERS: &str = "OCENTRA_PARENT_LOCAL_AI_CPU_MOE_LAYERS";
    pub const LOCAL_AI_LLAMA_CPP_RELEASE_TAG: &str =
        "OCENTRA_PARENT_LOCAL_AI_LLAMA_CPP_RELEASE_TAG";
    pub const LOCAL_AI_RUNTIME_CACHE_DIR: &str = "OCENTRA_PARENT_LOCAL_AI_RUNTIME_CACHE_DIR";
    pub const COMPUTER_NAME: &str = "COMPUTERNAME";
    pub const DEV_NETWORK_MODE: &str = "OCENTRA_PARENT_DEV_NETWORK";
    pub const DEV_LOG_DIR: &str = "OCENTRA_PARENT_DEV_LOG_DIR";
    pub const HOME: &str = "HOME";
    pub const HOSTNAME: &str = "HOSTNAME";
    pub const PARENT_DEV_BRIDGE_PORT: &str = "OCENTRA_PARENT_PARENT_BRIDGE_PORT";
    pub const USERPROFILE: &str = "USERPROFILE";
}

pub mod bind {
    pub const DEFAULT_AGENT_ADDR: &str = "127.0.0.1:4477";
    pub const DEFAULT_ALLOWED_ORIGINS: &[&str] =
        &["http://127.0.0.1:4478", "http://localhost:4478"];
}

pub mod peer {
    pub const PORTAL_DEV: &str = "portal-dev";
    pub const LOCAL_DEV_AGENT: &str = "local-dev-agent";
}

pub mod eventing_source {
    pub const ERROR_EVENT_CUSTODY_CONSTANT_PARSES: &str = "protocol event custody constant parses";
    pub const ERROR_EVENT_CUSTODY_PARSES: &str = "event custody parses";
    pub const ERROR_RUNTIME_ROLE_CONSTANT_PARSES: &str = "protocol runtime role constant parses";
    pub const ERROR_RUNTIME_ROLE_PARSES: &str = "runtime role parses";
    pub const ROLE_CONTROLLER: &str = "controller";
    pub const ROLE_AGENT: &str = "agent";
    pub const ROLE_ANALYZER: &str = "analyzer";
    pub const ROLE_DECISION_ENGINE: &str = "decision-engine";
    pub const ROLE_SIDE_EFFECT_ADAPTER: &str = "side-effect-adapter";
    pub const ROLE_AUDIT_WRITER: &str = "audit-writer";
    pub const ROLE_READ_MODEL: &str = "read-model";
    pub const CUSTODY_LOCAL_ONLY: &str = "local-only";
    pub const CUSTODY_LOCAL_JOURNAL: &str = "local-journal";
    pub const CUSTODY_LOCAL_QUERY_STORE: &str = "local-query-store";
    pub const CUSTODY_COORDINATOR_CACHE: &str = "coordinator-cache";
    pub const CUSTODY_UNAVAILABLE: &str = "unavailable";
}

pub mod event_id {
    pub const CONNECTION_READY: &str = "connection-ready";
    pub const COMMAND_REJECTED: &str = "command-rejected";
    pub const HEALTH_REPORTED: &str = "health-reported";
    pub const LOG_SNAPSHOT_REPORTED: &str = "log-snapshot-reported";
    pub const DEV_ECHOED: &str = "dev-echoed";
    pub const WATCH_STATUS_REPORTED: &str = "watch-status-reported";
    pub const ACTIVITY_INGEST_STATUS_REPORTED: &str = "activity-ingest-status-reported";
    pub const ACTIVITY_RECENT_SUMMARY_REPORTED: &str = "activity-recent-summary-reported";
    pub const ACTIVITY_MEMORY_GRAPH_REPORTED: &str = "activity-memory-graph-reported";
    pub const ACTIVITY_REPORT_GENERATED: &str = "activity-report-generated";
    pub const ACTIVITY_REPORT_SAVED: &str = "activity-report-saved";
    pub const ACTIVITY_REPORT_HISTORY_REPORTED: &str = "activity-report-history-reported";
    pub const ACTIVITY_SCREEN_READ_MODEL_REPORTED: &str = "activity-screen-read-model-reported";
    pub const ACTIVITY_APP_USE_READ_MODEL_REPORTED: &str = "activity-app-use-read-model-reported";
    pub const ACTIVITY_BROWSER_READ_MODEL_REPORTED: &str = "activity-browser-read-model-reported";
    pub const ACTIVITY_GAMES_READ_MODEL_REPORTED: &str = "activity-games-read-model-reported";
    pub const ACTIVITY_APP_GAME_BOUNDARY_READ_MODEL_REPORTED: &str =
        "activity-app-game-boundary-read-model-reported";
    pub const ACTIVITY_APP_GAME_POLICY_READINESS_READ_MODEL_REPORTED: &str =
        "activity-app-game-policy-readiness-read-model-reported";
    pub const ACTIVITY_APP_GAME_NOTIFICATION_READINESS_READ_MODEL_REPORTED: &str =
        "activity-app-game-notification-readiness-read-model-reported";
    pub const ACTIVITY_APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_REPORTED: &str =
        "activity-app-game-adapter-execution-readiness-read-model-reported";
    pub const ACTIVITY_APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_REPORTED: &str =
        "activity-app-game-platform-proof-status-read-model-reported";
    pub const ACTIVITY_APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_REPORTED: &str =
        "activity-app-game-child-runtime-transport-receipt-read-model-reported";
    pub const ACTIVITY_APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_REPORTED: &str =
        "activity-app-game-adapter-dispatch-preflight-read-model-reported";
    pub const ACTIVITY_APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_REPORTED: &str =
        "activity-app-game-adapter-dispatch-result-read-model-reported";
    pub const ACTIVITY_APP_GAME_ADAPTER_DISPATCH_EXECUTED: &str =
        "activity-app-game-adapter-dispatch-executed";
    pub const ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED: &str =
        "activity-app-game-timer-parent-surface-read-model-reported";
    pub const ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED: &str =
        "activity-app-game-timer-parent-preference-setup-requested";
    pub const BROWSER_SOCIAL_DASHBOARD_READ_MODEL_REPORTED: &str =
        "browser-social-dashboard-read-model-reported";
    pub const BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL_REPORTED: &str =
        "browser-social-audit-explanation-read-model-reported";
    pub const BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL_REPORTED: &str =
        "browser-social-alert-report-read-model-reported";
    pub const BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL_REPORTED: &str =
        "browser-social-alert-report-parent-surface-read-model-reported";
    pub const BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL_REPORTED: &str =
        "browser-social-parent-notification-delivery-read-model-reported";
    pub const BROWSER_SOCIAL_SOURCE_CUSTODY_MUTATION_APPLIED: &str =
        "browser-social-source-custody-mutation-applied";
    pub const ACTIVITY_NETWORK_READ_MODEL_REPORTED: &str = "activity-network-read-model-reported";
    pub const ACTIVITY_TRACKING_READ_MODEL_REPORTED: &str = "activity-tracking-read-model-reported";
    pub const BROWSER_INVENTORY_READ_MODEL_REPORTED: &str = "browser-inventory-read-model-reported";
    pub const BROWSER_EVIDENCE_RECENT_REPORTED: &str = "browser-evidence-recent-reported";
    pub const BROWSER_MANAGED_STATUS_REPORTED: &str = "browser-managed-status-reported";
    pub const BROWSER_RUNTIME_EVENT_CHAIN_STREAM_REPORTED: &str =
        "browser-runtime-event-chain-stream-reported";
    pub const NETWORK_FLOW_READ_MODEL_REPORTED: &str = "network-flow-read-model-reported";
    pub const NETWORK_RUNTIME_EVENT_CHAIN_STREAM_REPORTED: &str =
        "network-runtime-event-chain-stream-reported";
    pub const NETWORK_REMOTE_DELIVERY_STATUS_REPORTED: &str =
        "network-remote-delivery-status-reported";
    pub const NETWORK_LIVE_CAPTURE_STATUS_REPORTED: &str = "network-live-capture-status-reported";
    pub const NETWORK_LINUX_NFTABLES_LAB_STATUS_REPORTED: &str =
        "network-linux-nftables-lab-status-reported";
    pub const NETWORK_WINDOWS_FIREWALL_LAB_STATUS_REPORTED: &str =
        "network-windows-firewall-lab-status-reported";
    pub const NETWORK_WINDOWS_WFP_GATE_STATUS_REPORTED: &str =
        "network-windows-wfp-gate-status-reported";
    pub const NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS_REPORTED: &str =
        "network-android-vpn-service-gate-status-reported";
    pub const NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS_REPORTED: &str =
        "network-apple-network-extension-gate-status-reported";
    pub const LOCAL_AI_RUNTIME_STATUS_REPORTED: &str = "local-ai-runtime-status-reported";
    pub const LOCAL_AI_CHAT_GENERATION_REPORTED: &str = "local-ai-chat-generation-reported";
    pub const PARENT_ASSISTANT_ANSWER_REPORTED: &str = "parent-assistant-answer-reported";
    pub const POLICY_PREVIEW_READ_MODEL_REPORTED: &str = "policy-preview-read-model-reported";
    pub const POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REPORTED: &str =
        "policy-request-assistant-preview-confirm-reported";
    pub const BROWSER_POLICY_REPORTED: &str = "browser-policy-reported";
    pub const BROWSER_POLICY_PREVIEWED: &str = "browser-policy-previewed";
    pub const BROWSER_POLICY_PATCH_ACCEPTED: &str = "browser-policy-patch-accepted";
    pub const BROWSER_POLICY_PATCH_REJECTED: &str = "browser-policy-patch-rejected";
    pub const BROWSER_POLICY_REPLACE_ACCEPTED: &str = "browser-policy-replace-accepted";
    pub const BROWSER_POLICY_REPLACE_REJECTED: &str = "browser-policy-replace-rejected";
    pub const BROWSER_POLICY_ROLLBACK_ACCEPTED: &str = "browser-policy-rollback-accepted";
    pub const BROWSER_POLICY_ROLLBACK_REJECTED: &str = "browser-policy-rollback-rejected";
    pub const SCREEN_SETTINGS_REPORTED: &str = "screen-settings-reported";
    pub const SCREEN_SETTINGS_REPLACE_ACCEPTED: &str = "screen-settings-replace-accepted";
    pub const SCREEN_SETTINGS_REPLACE_REJECTED: &str = "screen-settings-replace-rejected";
    pub const ENFORCEMENT_AUDIT_REPORTED: &str = "enforcement-audit-reported";
    pub const ENFORCEMENT_TIMER_REPORTED: &str = "enforcement-timer-reported";
    pub const ENFORCEMENT_PRODUCT_CONTROL_SPINE_REPORTED: &str =
        "enforcement-product-control-spine-reported";
    pub const ENFORCEMENT_POLICY_DISPATCH_REPORTED: &str = "enforcement-policy-dispatch-reported";
    pub const ENFORCEMENT_BROAD_ADAPTER_PROOF_REPORTED: &str =
        "enforcement-broad-adapter-proof-reported";
    pub const ENFORCEMENT_SUPPORTED_ADAPTER_RUNTIME_PROOF_REPORTED: &str =
        "enforcement-supported-adapter-runtime-proof-reported";
    pub const PARENT_ASSISTANT_THREAD_UPDATED: &str = "parent-assistant-thread-updated";
    pub const PARENT_ASSISTANT_MESSAGE_ACCEPTED: &str = "parent-assistant-message-accepted";
    pub const PARENT_ASSISTANT_RUN_STARTED: &str = "parent-assistant-run-started";
    pub const PARENT_ASSISTANT_MESSAGE_DELTA: &str = "parent-assistant-message-delta";
    pub const PARENT_ASSISTANT_MESSAGE_COMPLETED: &str = "parent-assistant-message-completed";
    pub const PARENT_ASSISTANT_ACTION_PREVIEWED: &str = "parent-assistant-action-previewed";
    pub const PARENT_ASSISTANT_ACTION_CONFIRMED: &str = "parent-assistant-action-confirmed";
    pub const PARENT_ASSISTANT_PROVIDER_DEGRADED: &str = "parent-assistant-provider-degraded";
    pub const PARENT_ASSISTANT_ERROR_REPORTED: &str = "parent-assistant-error-reported";
    pub const DEV_LOCALHOST_API_READY: &str = "dev-localhost-api-ready";
    pub const UNKNOWN_COMMAND: &str = "unknown-command";
}

#[path = "constants/app_game_android_usage_events.rs"]
pub mod app_game_android_usage_events;
#[path = "constants/field.rs"]
pub mod field;
#[path = "constants/policy_control.rs"]
pub mod policy_control;
#[path = "constants/tracking_config_update.rs"]
pub mod tracking_config_update;
#[path = "constants/tracking_retention_settings_write.rs"]
pub mod tracking_retention_settings_write;
#[path = "constants/tracking_runtime.rs"]
pub mod tracking_runtime;

pub mod dev_log {
    pub const AGENT_FILE_PREFIX: &str = "agent-service";
    pub const DATE_CHARS: usize = 10;
    pub const DEFAULT_DIR: &str = ".logs/dev";
    pub const FILE_EXTENSION: &str = "ndjson";
    pub const ID_PREFIX: &str = "agent-log-";
}

pub mod dev_log_message {
    pub const ACTIVITY_CAPTURE_FAILED: &str = "Agent activity capture failed.";
    pub const AGENT_SERVICE_STARTED: &str = "Agent service dev runtime started.";
    pub const AGENT_HEALTH_REQUESTED: &str = "Agent health endpoint requested.";
}

pub mod journal {
    pub const DEFAULT_MAX_SEGMENT_BYTES: u64 = 1048576;
    pub const ENTRY_ID_PREFIX: &str = "journal-entry-";
    pub const FILE_EXTENSION: &str = "ndjson";
    pub const SEGMENT_ID_PREFIX: &str = "journal-segment-";
    pub const TEST_FILE_PREFIX: &str = "ocentra-parent-journal-test-";
    pub const TEST_APPEND_SUFFIX: &str = "append";
    pub const TEST_PROTOCOL_ROWS_SUFFIX: &str = "-protocol-rows";
    pub const TEST_LIVE_PROCESS_SUFFIX: &str = "live-process";
    pub const TEST_LIVE_FOREGROUND_SUFFIX: &str = "live-foreground";
    pub const TEST_REPLAY_SUFFIX: &str = "replay";
    pub const ROTATED_EXTENSION_SEPARATOR: &str = ".";
    pub const TEST_ROTATION_SUFFIX: &str = "rotation";
    pub const TEST_TAMPER_SUFFIX: &str = "tamper";
    pub const TEST_ROTATION_BYTES: u64 = 1;
    pub const XCHACHA20_NONCE_BYTES: usize = 24;
}

pub mod network_raw_artifact {
    pub const ARTIFACT_FILE_PREFIX: &str = "network-raw-artifact-";
    pub const EXPORT_FILE_PREFIX: &str = "network-raw-export-";
    pub const FILE_EXTENSION: &str = "pcap";
    pub const HASH_ALGORITHM_SHA256: &str = "sha256";
    pub const STATE_ACTIVE: &str = "active";
    pub const STATE_DELETED: &str = "deleted";
    pub const TEST_ARTIFACT_ID: &str = "network-capture-artifact-1";
    pub const TEST_CAPTURED_AT: &str = "2026-06-08T22:20:00Z";
    pub const TEST_CUSTODY_LABEL: &str = "local-raw-capture-custody";
    pub const TEST_DIR_PREFIX: &str = "ocentra-parent-network-raw-artifact-test-";
    pub const TEST_EMPTY_PAYLOAD_SUFFIX: &str = "empty-payload";
    pub const TEST_EXPORT_DIR_SUFFIX: &str = "export";
    pub const TEST_OVERSIZED_ARTIFACT_ID: &str = "network-capture-artifact-oversized";
    pub const TEST_QUOTA_SUFFIX: &str = "quota";
    pub const TEST_REJECT_IDS_SUFFIX: &str = "reject-ids";
    pub const TEST_SOURCE_EVENT_ID: &str = "network-live-capture-event-1";
    pub const TEST_WRITE_READ_SUFFIX: &str = "write-read";
}

#[path = "constants/activity_capture.rs"]
pub mod activity_capture;
#[path = "constants/activity_event_kind.rs"]
pub mod activity_event_kind;
#[path = "constants/activity_observer.rs"]
pub mod activity_observer;
#[path = "constants/activity_store.rs"]
pub mod activity_store;
#[path = "constants/activity_subject_kind.rs"]
pub mod activity_subject_kind;
#[path = "constants/activity_surface.rs"]
pub mod activity_surface;
#[path = "constants/browser.rs"]
pub mod browser;
#[path = "constants/child_agent.rs"]
pub mod child_agent;
#[path = "constants/child_domain_runtime.rs"]
pub mod child_domain_runtime;
#[path = "constants/enforcement_broad_adapter_proof.rs"]
pub mod enforcement_broad_adapter_proof;
#[path = "constants/host_identity.rs"]
pub mod host_identity;
#[path = "constants/v08_browser_domain_adapter_proof.rs"]
pub mod v08_browser_domain_adapter_proof;
#[path = "constants/v08_cross_platform_enforcement_capability_proof.rs"]
pub mod v08_cross_platform_enforcement_capability_proof;
#[path = "constants/v08_enforcement_integrity_runtime_audit.rs"]
pub mod v08_enforcement_integrity_runtime_audit;
#[path = "constants/v08_enforcement_policy_dispatch.rs"]
pub mod v08_enforcement_policy_dispatch;
#[path = "constants/v08_enforcement_product_control_spine.rs"]
pub mod v08_enforcement_product_control_spine;
#[path = "constants/v08_integrity_alert_status_bridge.rs"]
pub mod v08_integrity_alert_status_bridge;
#[path = "constants/v08_notification_provider_status_boundary.rs"]
pub mod v08_notification_provider_status_boundary;
#[path = "constants/v08_os_adapter_product_proof.rs"]
pub mod v08_os_adapter_product_proof;
#[path = "constants/v08_supported_adapter_runtime_proof.rs"]
pub mod v08_supported_adapter_runtime_proof;
#[path = "constants/windows_adapter_artifact_gate.rs"]
pub mod windows_adapter_artifact_gate;
#[path = "constants/windows_adapter_artifact_ingestion.rs"]
pub mod windows_adapter_artifact_ingestion;
#[path = "constants/windows_adapter_capability.rs"]
pub mod windows_adapter_capability;

pub mod browser_policy {
    pub const COMMAND_GET: &str = "agent.browser-policy.get";
    pub const COMMAND_PREVIEW: &str = "agent.browser-policy.preview";
    pub const COMMAND_PATCH: &str = "agent.browser-policy.patch";
    pub const COMMAND_REPLACE: &str = "agent.browser-policy.replace";
    pub const COMMAND_ROLLBACK: &str = "agent.browser-policy.rollback";
    pub const EVENT_REPORTED: &str = "agent.browser-policy.reported";
    pub const EVENT_PREVIEWED: &str = "agent.browser-policy.previewed";
    pub const EVENT_PATCH_ACCEPTED: &str = "agent.browser-policy.patch.accepted";
    pub const EVENT_PATCH_REJECTED: &str = "agent.browser-policy.patch.rejected";
    pub const EVENT_REPLACE_ACCEPTED: &str = "agent.browser-policy.replace.accepted";
    pub const EVENT_REPLACE_REJECTED: &str = "agent.browser-policy.replace.rejected";
    pub const EVENT_ROLLBACK_ACCEPTED: &str = "agent.browser-policy.rollback.accepted";
    pub const EVENT_ROLLBACK_REJECTED: &str = "agent.browser-policy.rollback.rejected";
    pub const REQUEST_ID: &str = "browser-control-request-1";
    pub const COMMAND_MESSAGE_ID: &str = "cmd-browser-policy";
    pub const TEST_SENT_AT: &str = "2026-05-28T17:35:00Z";
    pub const POLICY_ID: &str = "browser-policy-child-1";
    pub const REVISION_ID: &str = "browser-policy-revision-1";
    pub const AUDIT_EVENT_ID: &str = "browser-policy-audit-1";
    pub const FIELD_ID_ENABLED: &str = "browser.enabled";
    pub const FIELD_ID_EXECUTION_MODE: &str = "browser.executionMode";
    pub const FIELD_ID_DEFAULT_POSTURE: &str = "browser.defaultPosture";
    pub const FIELD_ID_MANAGEMENT_MODE: &str = "browser.managementMode";
    pub const FIELD_ID_DISCOVERY_SCAN_INSTALLED_BROWSERS: &str = "discovery.scanInstalledBrowsers";
    pub const FIELD_ID_DISCOVERY_SCAN_RUNNING_BROWSERS: &str = "discovery.scanRunningBrowsers";
    pub const FIELD_ID_DISCOVERY_DETECT_UNMANAGED_BROWSERS: &str =
        "discovery.detectUnmanagedBrowsers";
    pub const FIELD_ID_MANAGED_BROWSER_MODE: &str = "managedBrowser.mode";
    pub const FIELD_ID_MANAGED_BROWSER_ALLOWED_FAMILIES: &str = "managedBrowser.allowedFamilies";
    pub const FIELD_ID_MANAGED_BROWSER_LAUNCH_MODE: &str = "managedBrowser.launchMode";
    pub const FIELD_ID_MANAGED_BROWSER_PROFILE_MODE: &str = "managedBrowser.profileMode";
    pub const FIELD_ID_MANAGED_BROWSER_BRIDGE_REQUIREMENTS: &str =
        "managedBrowser.bridgeRequirements";
    pub const FIELD_ID_MANAGED_BROWSER_INTEGRATION_MECHANISMS: &str =
        "managedBrowser.integrationMechanisms";
    pub const FIELD_ID_MANAGED_BROWSER_POLICY_WRITER_CONTROLS: &str =
        "managedBrowser.policyWriterControls";
    pub const FIELD_ID_MANAGED_BROWSER_POLICY_WRITER_FALLBACK: &str =
        "managedBrowser.policyWriterFallback";
    pub const FIELD_ID_UNMANAGED_BROWSER_MODE: &str = "unmanagedBrowser.mode";
    pub const FIELD_ID_UNMANAGED_BROWSER_GRACE_SECONDS: &str = "unmanagedBrowser.graceSeconds";
    pub const FIELD_ID_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL: &str =
        "unmanagedBrowser.allowRecoverLaunchUrl";
    pub const FIELD_ID_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS: &str =
        "unmanagedBrowser.classificationTargets";
    pub const FIELD_ID_EVIDENCE_URL_SCOPE: &str = "evidence.urlScope";
    pub const FIELD_ID_REQUIRED_PROOF: &str = "evidence.requiredProof";
    pub const FIELD_ID_PROOF_FALLBACK: &str = "evidence.proofFallback";
    pub const FIELD_ID_WHEN_PROOF_UNAVAILABLE: &str = "evidence.whenProofUnavailable";
    pub const FIELD_ID_EVIDENCE_NEVER_COLLECT: &str = "evidence.neverCollect";
    pub const FIELD_ID_ALLOWED_TARGET_TYPES: &str = "rules.allowedTargetTypes";
    pub const FIELD_ID_ALLOWED_ACTIONS: &str = "rules.allowedActions";
    pub const FIELD_ID_RULE_ITEMS: &str = "rules.items";
    pub const FIELD_ID_URL_ALLOW_LIST: &str = "rules.urlAllowList";
    pub const FIELD_ID_URL_BLOCK_LIST: &str = "rules.urlBlockList";
    pub const FIELD_ID_BUDGETS_ENABLED: &str = "budgets.enabled";
    pub const FIELD_ID_DAILY_BUDGET_MINUTES: &str = "budgets.defaultDailyMinutes";
    pub const FIELD_ID_BUDGET_COUNTING_MODE: &str = "budgets.countingMode";
    pub const FIELD_ID_BROWSER_GAME_EDUCATIONAL_MODE: &str = "browserGames.educationalGameMode";
    pub const FIELD_ID_BROWSER_GAME_UNKNOWN_MODE: &str = "browserGames.unknownGameMode";
    pub const FIELD_ID_BROWSER_GAME_CLOUD_GAMING_APPROVAL: &str =
        "browserGames.cloudGamingApproval";
    pub const FIELD_ID_BROWSER_GAME_PURCHASE_ACCOUNT_APPROVAL: &str =
        "browserGames.purchaseAccountApproval";
    pub const FIELD_ID_BROWSER_GAME_UNBLOCKED_PORTAL_MODE: &str =
        "browserGames.unblockedPortalMode";
    pub const FIELD_ID_BROWSER_GAME_WEBGL_CANVAS_MODE: &str = "browserGames.webglCanvasMode";
    pub const FIELD_ID_BROWSER_GAME_DAILY_BUDGET_MINUTES: &str = "browserGames.defaultDailyMinutes";
    pub const FIELD_ID_DOWNLOAD_MODE: &str = "downloads.mode";
    pub const FIELD_ID_DOWNLOAD_BLOCKED_TYPES: &str = "downloads.blockedTypes";
    pub const FIELD_ID_DOWNLOAD_STATE: &str = "downloads.state";
    pub const FIELD_ID_APPROVAL_REQUIRED_FOR: &str = "approvals.requiredFor";
    pub const FIELD_ID_APPROVAL_UNANSWERED_DEFAULT: &str = "approvals.unansweredDefault";
    pub const FIELD_ID_APPROVAL_STATE: &str = "approvals.state";
    pub const FIELD_ID_REPORT_VISIBLE_FIELDS: &str = "reports.visibleFields";
    pub const FIELD_ID_REPORT_STATE: &str = "reports.state";
    pub const FIELD_ID_RETENTION_EXACT_URL: &str = "retention.exactUrl";
    pub const FIELD_ID_AUDIT_STATE: &str = "audit.state";
    pub const FIELD_ID_CUSTODY_ALLOWED_USES: &str = "custody.allowedUses";
    pub const FIELD_ID_AUDIT_REQUIRED_FIELDS: &str = "audit.requiredFields";
    pub const FIELD_ID_RETENTION_STATE: &str = "retention.state";
    pub const WRITES_TO_ENABLED: &str = "/browserPolicy/enabled";
    pub const WRITES_TO_EXECUTION_MODE: &str = "/browserPolicy/executionMode";
    pub const WRITES_TO_DEFAULT_POSTURE: &str = "/browserPolicy/defaultPosture";
    pub const WRITES_TO_MANAGEMENT_MODE: &str = "/browserPolicy/managementMode";
    pub const WRITES_TO_DISCOVERY_SCAN_INSTALLED_BROWSERS: &str =
        "/browserPolicy/discovery/scanInstalledBrowsers";
    pub const WRITES_TO_DISCOVERY_SCAN_RUNNING_BROWSERS: &str =
        "/browserPolicy/discovery/scanRunningBrowsers";
    pub const WRITES_TO_DISCOVERY_DETECT_UNMANAGED_BROWSERS: &str =
        "/browserPolicy/discovery/detectUnmanagedBrowsers";
    pub const WRITES_TO_MANAGED_BROWSER_MODE: &str = "/browserPolicy/managedBrowser/mode";
    pub const WRITES_TO_MANAGED_BROWSER_ALLOWED_FAMILIES: &str =
        "/browserPolicy/managedBrowser/allowedFamilies";
    pub const WRITES_TO_MANAGED_BROWSER_LAUNCH_MODE: &str =
        "/browserPolicy/managedBrowser/launchMode";
    pub const WRITES_TO_MANAGED_BROWSER_PROFILE_MODE: &str =
        "/browserPolicy/managedBrowser/profileMode";
    pub const WRITES_TO_MANAGED_BROWSER_BRIDGE_REQUIREMENTS: &str =
        "/browserPolicy/managedBrowser/bridgeRequirements";
    pub const WRITES_TO_MANAGED_BROWSER_INTEGRATION_MECHANISMS: &str =
        "/browserPolicy/managedBrowser/integrationMechanisms";
    pub const WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_CONTROLS: &str =
        "/browserPolicy/managedBrowser/policyWriterControls";
    pub const WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_FALLBACK: &str =
        "/browserPolicy/managedBrowser/policyWriterFallback";
    pub const WRITES_TO_UNMANAGED_BROWSER_MODE: &str = "/browserPolicy/unmanagedBrowser/mode";
    pub const WRITES_TO_UNMANAGED_BROWSER_GRACE_SECONDS: &str =
        "/browserPolicy/unmanagedBrowser/graceSeconds";
    pub const WRITES_TO_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL: &str =
        "/browserPolicy/unmanagedBrowser/allowRecoverLaunchUrl";
    pub const WRITES_TO_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS: &str =
        "/browserPolicy/unmanagedBrowser/classificationTargets";
    pub const WRITES_TO_EVIDENCE_URL_SCOPE: &str = "/browserPolicy/evidence/urlScope";
    pub const WRITES_TO_REQUIRED_PROOF: &str = "/browserPolicy/evidence/requiredProof";
    pub const WRITES_TO_PROOF_FALLBACK: &str = "/browserPolicy/evidence/proofFallback";
    pub const WRITES_TO_WHEN_PROOF_UNAVAILABLE: &str =
        "/browserPolicy/evidence/whenProofUnavailable";
    pub const WRITES_TO_EVIDENCE_NEVER_COLLECT: &str = "/browserPolicy/evidence/neverCollect";
    pub const WRITES_TO_ALLOWED_TARGET_TYPES: &str = "/browserPolicy/rules/allowedTargetTypes";
    pub const WRITES_TO_ALLOWED_ACTIONS: &str = "/browserPolicy/rules/allowedActions";
    pub const WRITES_TO_RULE_ITEMS: &str = "/browserPolicy/rules/items";
    pub const WRITES_TO_URL_ALLOW_LIST: &str = "/browserPolicy/rules/urlAllowList";
    pub const WRITES_TO_URL_BLOCK_LIST: &str = "/browserPolicy/rules/urlBlockList";
    pub const WRITES_TO_BUDGETS_ENABLED: &str = "/browserPolicy/budgets/enabled";
    pub const WRITES_TO_DAILY_BUDGET_MINUTES: &str = "/browserPolicy/budgets/defaultDailyMinutes";
    pub const WRITES_TO_BUDGET_COUNTING_MODE: &str = "/browserPolicy/budgets/countingMode";
    pub const WRITES_TO_BROWSER_GAME_EDUCATIONAL_MODE: &str =
        "/browserPolicy/browserGames/educationalGameMode";
    pub const WRITES_TO_BROWSER_GAME_UNKNOWN_MODE: &str =
        "/browserPolicy/browserGames/unknownGameMode";
    pub const WRITES_TO_BROWSER_GAME_CLOUD_GAMING_APPROVAL: &str =
        "/browserPolicy/browserGames/cloudGamingApproval";
    pub const WRITES_TO_BROWSER_GAME_PURCHASE_ACCOUNT_APPROVAL: &str =
        "/browserPolicy/browserGames/purchaseAccountApproval";
    pub const WRITES_TO_BROWSER_GAME_UNBLOCKED_PORTAL_MODE: &str =
        "/browserPolicy/browserGames/unblockedPortalMode";
    pub const WRITES_TO_BROWSER_GAME_WEBGL_CANVAS_MODE: &str =
        "/browserPolicy/browserGames/webglCanvasMode";
    pub const WRITES_TO_BROWSER_GAME_DAILY_BUDGET_MINUTES: &str =
        "/browserPolicy/browserGames/defaultDailyMinutes";
    pub const WRITES_TO_DOWNLOAD_MODE: &str = "/browserPolicy/downloads/mode";
    pub const WRITES_TO_DOWNLOAD_BLOCKED_TYPES: &str = "/browserPolicy/downloads/blockedTypes";
    pub const WRITES_TO_DOWNLOAD_STATE: &str = "/browserPolicy/downloads/state";
    pub const WRITES_TO_APPROVAL_REQUIRED_FOR: &str = "/browserPolicy/approvals/requiredFor";
    pub const WRITES_TO_APPROVAL_UNANSWERED_DEFAULT: &str =
        "/browserPolicy/approvals/unansweredDefault";
    pub const WRITES_TO_APPROVAL_STATE: &str = "/browserPolicy/approvals/state";
    pub const WRITES_TO_REPORT_VISIBLE_FIELDS: &str = "/browserPolicy/reports/visibleFields";
    pub const WRITES_TO_REPORT_STATE: &str = "/browserPolicy/reports/state";
    pub const WRITES_TO_RETENTION_EXACT_URL: &str = "/browserPolicy/retention/exactUrl";
    pub const WRITES_TO_AUDIT_STATE: &str = "/browserPolicy/audit/state";
    pub const WRITES_TO_CUSTODY_ALLOWED_USES: &str = "/browserPolicy/custody/allowedUses";
    pub const WRITES_TO_AUDIT_REQUIRED_FIELDS: &str = "/browserPolicy/audit/requiredFields";
    pub const WRITES_TO_RETENTION_STATE: &str = "/browserPolicy/retention/state";
    pub const UPDATE_KIND_GET: &str = "get";
    pub const UPDATE_KIND_PREVIEW: &str = "preview";
    pub const UPDATE_KIND_PATCH: &str = "patch";
    pub const UPDATE_KIND_REPLACE: &str = "replace";
    pub const UPDATE_KIND_ROLLBACK: &str = "rollback";
    pub const UPDATE_STATUS_ACCEPTED: &str = "accepted";
    pub const UPDATE_STATUS_REJECTED: &str = "rejected";
    pub const PATCH_OPERATION_REPLACE: &str = "replace";
    pub const STORE_FILE_NAME: &str = "ocentra-parent-browser-policy-state.json";
    pub const STORE_FILE_EXTENSION: &str = "json";
    pub const TEST_STORE_FILE_PREFIX: &str = "ocentra-parent-browser-policy-test";
    pub const DEFAULT_POLICY_ID: &str = "browser-policy-default";
    pub const DEFAULT_RULE_ID: &str = "browser-rule-default-domain";
    pub const DEFAULT_TARGET_VALUE: &str = "example.test";
    pub const DEFAULT_RULE_MATCH_MODE: &str = "origin";
    pub const DEFAULT_RULE_REASON_CODE: &str = "school-domain";
    pub const DEFAULT_RULE_SCHEDULE_ID: &str = "always";
    pub const DEFAULT_RULE_AUDIT_LEVEL: &str = "decision";
    pub const DEFAULT_CAPABILITY_ID: &str = "managed-browser-active-tab-proof";
    pub const DEFAULT_CAPABILITY_LABEL: &str = "Managed browser active tab proof";
    pub const DEFAULT_CAPABILITY_REASON: &str =
        "Compiler scaffold does not claim managed browser proof is installed.";
    pub const POLICY_WRITER_CAPABILITY_ID: &str = "managed-browser-policy-writer";
    pub const POLICY_WRITER_CAPABILITY_LABEL: &str = "Managed browser policy writer";
    pub const POLICY_WRITER_CAPABILITY_REASON: &str =
        "Managed browser policy writer stays manual-required until adapter proof exists.";
    pub const CAPABILITY_STATE_READY: &str = "ready";
    pub const CAPABILITY_STATE_SUPPORTED: &str = "supported";
    pub const DOMAIN_CAPABILITY_ID: &str = "browser-domain-proof";
    pub const DOMAIN_CAPABILITY_LABEL: &str = "Browser domain proof";
    pub const CLASSIFIER_CAPABILITY_ID: &str = "browser-category-classifier-proof";
    pub const CLASSIFIER_CAPABILITY_LABEL: &str = "Browser category classifier proof";
    pub const SOCIAL_CAPABILITY_ID: &str = "browser-social-route-proof";
    pub const SOCIAL_CAPABILITY_LABEL: &str = "Social route evidence proof";
    pub const GAME_CAPABILITY_ID: &str = "browser-game-runtime-proof";
    pub const GAME_CAPABILITY_LABEL: &str = "Browser game runtime proof";
    pub const ACTION_ADAPTER_CAPABILITY_ID: &str = "browser-action-adapter-proof";
    pub const ACTION_ADAPTER_CAPABILITY_LABEL: &str = "Browser action adapter proof";
    pub const PROCESS_CAPABILITY_ID: &str = "unmanaged-browser-process-proof";
    pub const PROCESS_CAPABILITY_LABEL: &str = "Unmanaged browser process proof";
    pub const COMPILE_NOTE_PARENT_POLICY: &str =
        "Compiled from deterministic parent policy; AI output is candidate evidence only.";
    pub const COMPILE_NOTE_MANAGED_EXACT_URL: &str =
        "Exact URL target requires fresh managed active-tab proof.";
    pub const COMPILE_NOTE_DOMAIN_OR_MANAGED: &str =
        "Domain target accepts managed URL evidence or network-domain proof.";
    pub const COMPILE_NOTE_CLASSIFIER_REQUIRED: &str =
        "Category target remains manual-required until classifier proof exists.";
    pub const COMPILE_NOTE_URL_METADATA_REQUIRED: &str =
        "Search or video target remains manual-required until URL metadata proof exists.";
    pub const COMPILE_NOTE_SOCIAL_REQUIRED: &str =
        "Social target remains manual-required until social route evidence exists.";
    pub const COMPILE_NOTE_GAME_REQUIRED: &str =
        "Browser game target remains manual-required until game runtime evidence exists.";
    pub const COMPILE_NOTE_POLICY_WRITER_REQUIRED: &str =
        "Managed browser policy output remains manual-required until writer adapter proof exists.";
    pub const COMPILE_NOTE_PROCESS_REQUIRED: &str =
        "Unmanaged browser target requires process detection proof.";
    pub const COMPILE_NOTE_ACTION_ADAPTER_REQUIRED: &str =
        "Enforcement action remains manual-required until adapter proof exists.";
    pub const COMPILE_NOTE_OBSERVE_DRY_RUN: &str =
        "Observe and dry-run modes do not execute browser adapters.";
    pub const REVISION_PREFIX: &str = "browser-policy-revision-";
    pub const COMPILED_HASH_PREFIX: &str = "browser-policy-compiled-";
    pub const AUDIT_PREFIX: &str = "browser-policy-audit-";
    pub const REJECTION_INVALID_REQUEST: &str = "invalid-request";
    pub const REJECTION_UNKNOWN_WRITES_TO: &str = "unknown-writes-to";
    pub const REJECTION_UNKNOWN_FIELD: &str = "unknown-field";
    pub const REJECTION_INVALID_ENUM_VALUE: &str = "invalid-enum-value";
    pub const REJECTION_MISSING_BUDGET_OR_FALLBACK: &str = "missing-budget-or-fallback";
    pub const REJECTION_MISSING_MANAGED_PROOF_OR_FALLBACK: &str =
        "missing-managed-proof-or-fallback";
    pub const REJECTION_CAPABILITY_UNAVAILABLE: &str = "capability-unavailable";
    pub const REJECTION_SCAFFOLD_UNAVAILABLE: &str = "scaffold-unavailable";
    pub const REJECTION_STORAGE_UNAVAILABLE: &str = "storage-unavailable";
    pub const REJECTION_STALE_REVISION: &str = "stale-revision";
    pub const REJECTION_REVISION_NOT_FOUND: &str = "revision-not-found";
    pub const SCAFFOLD_UNAVAILABLE_MESSAGE: &str =
        "Browser policy runtime returned a legacy scaffold-unavailable rejection.";
    pub const MESSAGE_ACCEPTED: &str = "Browser policy update accepted.";
    pub const MESSAGE_PREVIEWED: &str =
        "Browser policy preview compiled without persisting a revision.";
    pub const MESSAGE_REPORTED: &str = "Browser policy state reported.";
    pub const MESSAGE_ROLLBACK_ACCEPTED: &str = "Browser policy rollback accepted.";
    pub const MESSAGE_INVALID_REQUEST: &str = "Browser policy request is invalid.";
    pub const MESSAGE_STORAGE_UNAVAILABLE: &str = "Browser policy storage is unavailable.";
    pub const MESSAGE_STALE_REVISION: &str = "Browser policy base revision is stale.";
    pub const MESSAGE_REVISION_NOT_FOUND: &str = "Browser policy revision was not found.";
    pub const MESSAGE_INVALID_POLICY: &str = "Browser policy value is inconsistent.";
}

pub mod screen_settings {
    pub const UPDATE_KIND_GET: &str = "get";
    pub const UPDATE_KIND_REPLACE: &str = "replace";
    pub const UPDATE_STATUS_ACCEPTED: &str = "accepted";
    pub const UPDATE_STATUS_REJECTED: &str = "rejected";
    pub const REJECTION_STORAGE_UNAVAILABLE: &str = "storage-unavailable";
    pub const REJECTION_INVALID_SETTING: &str = "invalid-setting";
    pub const REJECTION_STALE_REVISION: &str = "stale-revision";
    pub const REJECTION_RAW_RETENTION_FORBIDDEN: &str = "raw-retention-forbidden";
    pub const REJECTION_DISABLED_SETTING_INCONSISTENT: &str = "disabled-setting-inconsistent";
    pub const REJECTION_POLICY_MODE_INCONSISTENT: &str = "policy-mode-inconsistent";
    pub const REJECTION_STRICT_MODE_INCONSISTENT: &str = "strict-mode-inconsistent";
    pub const REJECTION_TRIGGER_MODE_INCONSISTENT: &str = "trigger-mode-inconsistent";
    pub const REJECTION_OCR_MODE_INCONSISTENT: &str = "ocr-mode-inconsistent";
    pub const STORE_FILE_NAME: &str = "ocentra-parent-screen-settings-state.json";
    pub const TEST_STORE_FILE_PREFIX: &str = "ocentra-parent-screen-settings-test";
    pub const REQUEST_ID_GET: &str = "screen-settings-request-get";
    pub const REQUEST_ID_REPLACE: &str = "screen-settings-request-replace";
    pub const DEFAULT_PARENT_SETTING_REF: &str = "screen-parent-setting-default";
    pub const DEFAULT_CHANGED_BY_PARENT_REF: &str = "screen-parent-local-settings";
    pub const DEFAULT_CHANGED_AT: &str = "2026-06-07T04:20:00Z";
    pub const DEFAULT_REASON: &str = "screen-settings-service-default-disabled";
    pub const STRICT_REASON: &str = "parent-enabled-strict-dry-run";
    pub const RAW_RETENTION_LOCAL_TTL_REASON: &str = "parent-approved-local-ttl-raw-retention";
    pub const REVISION_PREFIX: &str = "screen-setting-revision-";
    pub const AUDIT_PREFIX: &str = "screen-setting-audit-";
    pub const TEST_AUDIT_EVENT_ID_1: &str = "screen-setting-audit-1";
    pub const TEST_PATH_SUFFIX_PERSISTENCE: &str = "persists-parent-opt-in";
    pub const TEST_PATH_SUFFIX_COMMAND: &str = "service-command-path";
    pub const TEST_JSON_EXTENSION: &str = "json";
    pub const COMMAND_NAME_GET: &str = "agent.screen-settings.get";
    pub const COMMAND_NAME_REPLACE: &str = "agent.screen-settings.replace";
    pub const EVENT_NAME_REPORTED: &str = "agent.screen-settings.reported";
    pub const EVENT_NAME_REPLACE_ACCEPTED: &str = "agent.screen-settings.replace.accepted";
    pub const EVENT_NAME_REPLACE_REJECTED: &str = "agent.screen-settings.replace.rejected";
    pub const COMMAND_MESSAGE_ID: &str = "screen-settings-command-message";
    pub const TEST_SENT_AT: &str = "2026-06-07T04:55:00Z";
    pub const TEST_SETTING_RETURNED: &str = "screen setting returned";
    pub const TEST_PERSISTED_SETTING_RETURNED: &str = "persisted screen setting returned";
    pub const TEST_STORE_READABLE: &str = "screen settings store is readable";
    pub const MESSAGE_REPORTED: &str = "Screen settings state reported.";
    pub const MESSAGE_ACCEPTED: &str = "Screen settings update accepted.";
    pub const MESSAGE_STORAGE_UNAVAILABLE: &str = "Screen settings storage is unavailable.";
    pub const MESSAGE_STALE_REVISION: &str = "Screen settings base revision is stale.";
    pub const MESSAGE_INVALID_SETTING: &str = "Screen settings value is inconsistent.";
    pub const ANALYSIS_MODE_OBSERVE_ONLY: &str = "observeOnly";
    pub const ANALYSIS_MODE_POLICY_DRY_RUN: &str = "policyDryRun";
    pub const ANALYSIS_MODE_ENFORCEMENT_ELIGIBLE: &str = "enforcementEligible";
    pub const CAPTURE_SCOPE_ACTIVE_WINDOW: &str = "activeWindow";
    pub const CAPTURE_TRIGGER_TIMED_CADENCE: &str = "timedCadence";
    pub const CAPTURE_TRIGGER_NATIVE_APP_FOREGROUND: &str = "nativeAppForegroundStart";
    pub const REDACTION_MODE_DISABLED: &str = "disabled";
    pub const REDACTION_MODE_LOCAL_SENSITIVE_TEXT: &str = "localSensitiveText";
    pub const OCR_TEXT_RETENTION_DISABLED: &str = "disabled";
    pub const OCR_TEXT_RETENTION_REDACTED_SNIPPETS: &str = "redactedSnippets";
    pub const MIN_CADENCE_SECONDS: u64 = 60;
    pub const MAX_CADENCE_SECONDS: u64 = 3600;
    pub const DEFAULT_DISABLED_CADENCE_SECONDS: u64 = 300;
    pub const STRICT_CADENCE_SECONDS: u64 = 60;
    pub const MIN_TTL_SECONDS: u64 = 60;
    pub const MAX_TTL_SECONDS: u64 = 1800;
    pub const DEFAULT_TTL_SECONDS: u64 = 300;
    pub const RAW_RETENTION_MAX_TTL_SECONDS: u64 = 120;
    pub const MAX_RETRY_COUNT: u64 = 5;
    pub const DEFAULT_RETRY_COUNT: u64 = 2;
    pub const MAX_OCR_SNIPPET_LIMIT: u64 = 20;
}
#[path = "constants/enforcement.rs"]
pub mod enforcement;
#[path = "constants/household_mesh.rs"]
pub mod household_mesh;
#[path = "constants/lan_pairing.rs"]
pub mod lan_pairing;
#[path = "constants/network_flow.rs"]
pub mod network_flow;
#[path = "constants/screen_flow.rs"]
pub mod screen_flow;

#[path = "constants/sqlite.rs"]
pub mod sqlite;
#[path = "constants/value.rs"]
pub mod value;

pub mod error {
    pub const AGENT_ADDR_SOCKET_ADDRESS: &str =
        "OCENTRA_PARENT_AGENT_ADDR must be a socket address";
    pub const AGENT_ORIGIN_HEADER_VALID: &str = "agent origin header is valid";
    pub const LAN_BIND_REQUIRES_FLAG: &str =
        "non-loopback bind requires OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED=true";
    pub const LOCALHOST_BIND_SUCCEEDS: &str = "agent service localhost bind succeeds";
    pub const AGENT_SERVICE_RUNS: &str = "agent service runs";
    pub const PARENT_DEV_BRIDGE_BINDS: &str = "parent dev bridge localhost bind succeeds";
    pub const PARENT_DEV_BRIDGE_RUNS: &str = "parent dev bridge runs";
    pub const DEV_LOG_SERIALIZES: &str = "dev log serializes";
    pub const AGENT_EVENT_SERIALIZES: &str = "agent event serializes";
    pub const JOURNAL_APPENDS: &str = "activity journal appends";
    pub const JOURNAL_DECRYPTS: &str = "activity journal decrypts";
    pub const JOURNAL_OPENS: &str = "activity journal opens";
    pub const JOURNAL_READS: &str = "activity journal reads";
    pub const ACTIVITY_STORE_OPENS: &str = "activity SQLite store opens";
    pub const ACTIVITY_STORE_INGESTS: &str = "activity SQLite store ingests";
    pub const ACTIVITY_STORE_QUERIES: &str = "activity SQLite store queries";
    pub const ACTIVITY_CAPTURE_RECORDS: &str = "activity capture records";
    pub const ACTIVITY_CAPTURE_REJECTS_INVALID_KEY: &str =
        "activity capture rejects invalid journal key";
    pub const NETWORK_CAPTURE_OBSERVES_SOCKET: &str =
        "network capture observes the current process socket";
    pub const NETWORK_RAW_ARTIFACT_DELETES: &str = "network raw artifact deletes";
    pub const NETWORK_RAW_ARTIFACT_EXPORTS: &str = "network raw artifact exports";
    pub const NETWORK_RAW_ARTIFACT_OPENS: &str = "network raw artifact store opens";
    pub const NETWORK_RAW_ARTIFACT_READS: &str = "network raw artifact reads";
    pub const NETWORK_RAW_ARTIFACT_WRITES: &str = "network raw artifact writes";
    pub const BROWSER_BRIDGE_MAPS_TARGET: &str = "browser bridge maps target";
    pub const BROWSER_BRIDGE_REJECTS_INVALID_URL: &str = "browser bridge rejects invalid URL";
    pub const LOCAL_AI_RUNTIME_SPAWNS: &str = "local AI runtime process spawns";
    pub const LOCAL_AI_CHAT_REQUEST_PARSES: &str = "local AI chat request parses";
    pub const LOCAL_AI_CACHE_ROOT_EXISTS: &str = "local AI cache root exists";
    pub const UNEXPECTED_LAN_DISCOVERY_STATE: &str = "unexpected LAN discovery state";
}

#[path = "constants/local_ai_runtime.rs"]
pub mod local_ai_runtime;
#[path = "constants/local_ai_runtime_provider_proof.rs"]
pub mod local_ai_runtime_provider_proof;
#[path = "constants/parent_assistant.rs"]
pub mod parent_assistant;
#[path = "constants/parent_controller.rs"]
pub mod parent_controller;

pub mod delimiter {
    pub const BANG: char = '!';
    pub const BACKSLASH: char = '\\';
    pub const CLOSE_BRACKET: char = ']';
    pub const COLON: char = ':';
    pub const DOT: char = '.';
    pub const EQUALS: char = '=';
    pub const HYPHEN: char = '-';
    pub const LIST: char = ',';
    pub const NEWLINE: char = '\n';
    pub const OPEN_BRACKET: char = '[';
    pub const QUOTE: char = '"';
    pub const SLASH: char = '/';
    pub const AT: char = '@';
    pub const UNDERSCORE: char = '_';
}

pub mod test_network {
    pub const LOOPBACK_ANY_PORT: &str = "127.0.0.1:0";
    pub const LOOPBACK_IP: &str = "127.0.0.1";
    pub const NETSTAT_TCP_ESTABLISHED_ROW: &str =
        "TCP    127.0.0.1:4242        127.0.0.1:443          ESTABLISHED     4242";
    pub const SUBJECT_ID: &str = "network-destination-127.0.0.1-443";
}

pub mod byte {
    pub const NEWLINE: u8 = b'\n';
}
