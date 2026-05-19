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
    pub const CAPTURE_ENABLED: &str = "captureEnabled";
    pub const ENTRIES: &str = "entries";
    pub const MODE: &str = "mode";
    pub const NETWORK_MODE: &str = "networkMode";
    pub const NOTE: &str = "note";
    pub const ONLINE: &str = "online";
    pub const PID: &str = "pid";
    pub const POLICY_ENGINE_ENABLED: &str = "policyEngineEnabled";
    pub const REASON: &str = "reason";
    pub const REMOTE_SYNC: &str = "remoteSync";
    pub const TRANSPORT: &str = "transport";
}

pub mod value {
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

pub mod error {
    pub const AGENT_ADDR_SOCKET_ADDRESS: &str =
        "OCENTRA_PARENT_AGENT_ADDR must be a socket address";
    pub const AGENT_ORIGIN_HEADER_VALID: &str = "agent origin header is valid";
    pub const LAN_BIND_REQUIRES_FLAG: &str =
        "non-loopback bind requires OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED=true";
    pub const LOCALHOST_BIND_SUCCEEDS: &str = "agent service localhost bind succeeds";
    pub const AGENT_SERVICE_RUNS: &str = "agent service runs";
    pub const AGENT_EVENT_SERIALIZES: &str = "agent event serializes";
}

pub mod delimiter {
    pub const LIST: char = ',';
}
