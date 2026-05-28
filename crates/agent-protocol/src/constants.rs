pub mod endpoint {
    pub const HEALTH: &str = "/health";
    pub const DEV_LOG_SNAPSHOT: &str = "/api/dev/log-snapshot";
    pub const DEV_WS: &str = "/api/dev/ws";
}

pub mod env_var {
    pub const AGENT_ALLOWED_ORIGINS: &str = "OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS";
    pub const AGENT_ADDR: &str = "OCENTRA_PARENT_AGENT_ADDR";
    pub const AGENT_LAN_PAIRING_REGISTRY_PATH: &str =
        "OCENTRA_PARENT_AGENT_LAN_PAIRING_REGISTRY_PATH";
    pub const AGENT_LOCAL_NETWORK_ENABLED: &str = "OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED";
    pub const AGENT_ENFORCEMENT_TIMER_STATE_PATH: &str =
        "OCENTRA_PARENT_AGENT_ENFORCEMENT_TIMER_STATE_PATH";
    pub const ACTIVITY_DB_PATH: &str = "OCENTRA_PARENT_ACTIVITY_DB_PATH";
    pub const ACTIVITY_JOURNAL_KEY_PATH: &str = "OCENTRA_PARENT_ACTIVITY_JOURNAL_KEY_PATH";
    pub const ACTIVITY_JOURNAL_PATH: &str = "OCENTRA_PARENT_ACTIVITY_JOURNAL_PATH";
    pub const LOCAL_APP_DATA: &str = "LOCALAPPDATA";
    pub const MANAGED_BROWSER_BRIDGE_PORT: &str = "OCENTRA_PARENT_MANAGED_BROWSER_BRIDGE_PORT";
    pub const MANAGED_BROWSER_EXECUTABLE: &str = "OCENTRA_PARENT_MANAGED_BROWSER_EXECUTABLE";
    pub const MANAGED_BROWSER_PROFILE_DIR: &str = "OCENTRA_PARENT_MANAGED_BROWSER_PROFILE_DIR";
    pub const PROGRAM_FILES: &str = "PROGRAMFILES";
    pub const PROGRAM_FILES_X86: &str = "ProgramFiles(x86)";
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
    pub const DEV_LOG_DIR: &str = "OCENTRA_PARENT_DEV_LOG_DIR";
    pub const HOME: &str = "HOME";
    pub const HOSTNAME: &str = "HOSTNAME";
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
    pub const ACTIVITY_NETWORK_READ_MODEL_REPORTED: &str = "activity-network-read-model-reported";
    pub const BROWSER_EVIDENCE_RECENT_REPORTED: &str = "browser-evidence-recent-reported";
    pub const BROWSER_MANAGED_STATUS_REPORTED: &str = "browser-managed-status-reported";
    pub const NETWORK_FLOW_READ_MODEL_REPORTED: &str = "network-flow-read-model-reported";
    pub const LOCAL_AI_RUNTIME_STATUS_REPORTED: &str = "local-ai-runtime-status-reported";
    pub const LOCAL_AI_CHAT_GENERATION_REPORTED: &str = "local-ai-chat-generation-reported";
    pub const PARENT_ASSISTANT_ANSWER_REPORTED: &str = "parent-assistant-answer-reported";
    pub const POLICY_PREVIEW_READ_MODEL_REPORTED: &str = "policy-preview-read-model-reported";
    pub const BROWSER_POLICY_REPORTED: &str = "browser-policy-reported";
    pub const BROWSER_POLICY_PREVIEWED: &str = "browser-policy-previewed";
    pub const BROWSER_POLICY_PATCH_ACCEPTED: &str = "browser-policy-patch-accepted";
    pub const BROWSER_POLICY_PATCH_REJECTED: &str = "browser-policy-patch-rejected";
    pub const BROWSER_POLICY_REPLACE_ACCEPTED: &str = "browser-policy-replace-accepted";
    pub const BROWSER_POLICY_REPLACE_REJECTED: &str = "browser-policy-replace-rejected";
    pub const BROWSER_POLICY_ROLLBACK_ACCEPTED: &str = "browser-policy-rollback-accepted";
    pub const BROWSER_POLICY_ROLLBACK_REJECTED: &str = "browser-policy-rollback-rejected";
    pub const ENFORCEMENT_AUDIT_REPORTED: &str = "enforcement-audit-reported";
    pub const ENFORCEMENT_TIMER_REPORTED: &str = "enforcement-timer-reported";
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

#[path = "constants/field.rs"]
pub mod field;

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
    pub const TEST_REPLAY_SUFFIX: &str = "replay";
    pub const TEST_ROTATION_SUFFIX: &str = "rotation";
    pub const TEST_TAMPER_SUFFIX: &str = "tamper";
    pub const TEST_ROTATION_BYTES: u64 = 1;
    pub const XCHACHA20_NONCE_BYTES: usize = 24;
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
    pub const WRITES_TO_ENABLED: &str = "/browserPolicy/enabled";
    pub const UPDATE_KIND_GET: &str = "get";
    pub const UPDATE_KIND_PREVIEW: &str = "preview";
    pub const UPDATE_KIND_PATCH: &str = "patch";
    pub const UPDATE_KIND_REPLACE: &str = "replace";
    pub const UPDATE_KIND_ROLLBACK: &str = "rollback";
    pub const UPDATE_STATUS_ACCEPTED: &str = "accepted";
    pub const UPDATE_STATUS_REJECTED: &str = "rejected";
    pub const PATCH_OPERATION_REPLACE: &str = "replace";
    pub const REJECTION_SCAFFOLD_UNAVAILABLE: &str = "scaffold-unavailable";
    pub const SCAFFOLD_UNAVAILABLE_MESSAGE: &str =
        "Browser policy persistence and compiler are not implemented in this scaffold slice.";
}
#[path = "constants/enforcement.rs"]
pub mod enforcement;
#[path = "constants/lan_pairing.rs"]
pub mod lan_pairing;
#[path = "constants/network_flow.rs"]
pub mod network_flow;

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
    pub const BROWSER_BRIDGE_MAPS_TARGET: &str = "browser bridge maps target";
    pub const BROWSER_BRIDGE_REJECTS_INVALID_URL: &str = "browser bridge rejects invalid URL";
    pub const LOCAL_AI_RUNTIME_SPAWNS: &str = "local AI runtime process spawns";
    pub const LOCAL_AI_CHAT_REQUEST_PARSES: &str = "local AI chat request parses";
    pub const LOCAL_AI_CACHE_ROOT_EXISTS: &str = "local AI cache root exists";
    pub const UNEXPECTED_LAN_DISCOVERY_STATE: &str = "unexpected LAN discovery state";
}

#[path = "constants/local_ai_runtime.rs"]
pub mod local_ai_runtime;
#[path = "constants/parent_assistant.rs"]
pub mod parent_assistant;

pub mod delimiter {
    pub const CLOSE_BRACKET: char = ']';
    pub const COLON: char = ':';
    pub const DOT: char = '.';
    pub const HYPHEN: char = '-';
    pub const LIST: char = ',';
    pub const NEWLINE: char = '\n';
    pub const OPEN_BRACKET: char = '[';
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
