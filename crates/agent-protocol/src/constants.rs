pub mod endpoint {
    pub const HEALTH: &str = "/health";
    pub const DEV_LOG_SNAPSHOT: &str = "/api/dev/log-snapshot";
    pub const DEV_WS: &str = "/api/dev/ws";
}

pub mod env_var {
    pub const AGENT_ALLOWED_ORIGINS: &str = "OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS";
    pub const AGENT_ADDR: &str = "OCENTRA_PARENT_AGENT_ADDR";
    pub const AGENT_LOCAL_NETWORK_ENABLED: &str = "OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED";
    pub const COMPUTER_NAME: &str = "COMPUTERNAME";
    pub const DEV_LOG_DIR: &str = "OCENTRA_PARENT_DEV_LOG_DIR";
    pub const HOSTNAME: &str = "HOSTNAME";
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
    pub const DEV_LOCALHOST_API_READY: &str = "dev-localhost-api-ready";
    pub const UNKNOWN_COMMAND: &str = "unknown-command";
}

pub mod field {
    pub const AVAILABLE: &str = "available";
    pub const ACTIVITY_DIGEST: &str = "activityDigest";
    pub const CAPTURE_ENABLED: &str = "captureEnabled";
    pub const CIPHER: &str = "cipher";
    pub const CIPHERTEXT: &str = "ciphertext";
    pub const COMMAND: &str = "command";
    pub const ENTRIES: &str = "entries";
    pub const ENTRY_ID: &str = "entryId";
    pub const EVENT_ID: &str = "eventId";
    pub const MAX_SEGMENT_BYTES: &str = "maxSegmentBytes";
    pub const MODE: &str = "mode";
    pub const NETWORK_MODE: &str = "networkMode";
    pub const NONCE: &str = "nonce";
    pub const NOTE: &str = "note";
    pub const ONLINE: &str = "online";
    pub const PID: &str = "pid";
    pub const POLICY_ENGINE_ENABLED: &str = "policyEngineEnabled";
    pub const REASON: &str = "reason";
    pub const REMOTE_SYNC: &str = "remoteSync";
    pub const SCHEMA_VERSION: &str = "schemaVersion";
    pub const SEGMENT_ID: &str = "segmentId";
    pub const TRANSPORT: &str = "transport";
    pub const WRITTEN_AT: &str = "writtenAt";
}

pub mod value {
    pub const ACTIVITY_JOURNAL_CIPHER: &str = "xchacha20poly1305";
    pub const DEV_MODE: &str = "dev";
    pub const LOCAL_NETWORK_MODE: &str = "lan";
    pub const LOOPBACK_MODE: &str = "loopback";
    pub const TRANSPORT_WEBSOCKET: &str = "websocket";
    pub const TRUE: &str = "true";
    pub const UNKNOWN_HOST: &str = "unknown-host";
    pub const WATCHER_STATUS_ONLY: &str =
        "Watcher status endpoint is available; watcher runtime is not active.";
    pub const LOCALHOST_API_REACHABLE: &str = "Agent service localhost API is reachable.";
}

pub mod dev_log {
    pub const AGENT_FILE_PREFIX: &str = "agent-service";
    pub const DATE_CHARS: usize = 10;
    pub const DEFAULT_DIR: &str = ".logs/dev";
    pub const FILE_EXTENSION: &str = "ndjson";
    pub const ID_PREFIX: &str = "agent-log-";
}

pub mod dev_log_message {
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
}

pub mod delimiter {
    pub const DOT: char = '.';
    pub const HYPHEN: char = '-';
    pub const LIST: char = ',';
    pub const NEWLINE: char = '\n';
}

pub mod byte {
    pub const NEWLINE: u8 = b'\n';
}
