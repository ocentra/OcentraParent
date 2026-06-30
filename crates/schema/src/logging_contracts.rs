use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

macro_rules! logging_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

logging_text_identifier!(AgentDeviceId);
logging_text_identifier!(AgentHostname);
logging_text_identifier!(AgentPlatform);
logging_text_identifier!(AgentServiceVersion);
logging_text_identifier!(LogEntryId);
logging_text_identifier!(LogTimestamp);
logging_text_identifier!(LogMessage);
logging_text_identifier!(LogRunId);
logging_text_identifier!(LogLaneId);
logging_text_identifier!(LogCommandId);
logging_text_identifier!(LogCorrelationId);
logging_text_identifier!(StackTrace);

pub const LOG_SCHEMA_VERSION: u16 = 1;
pub const LOG_SNAPSHOT_SCHEMA_VERSION: u16 = 1;

pub struct LoggerRuntimeEnvironment;

impl LoggerRuntimeEnvironment {
    pub const RUN_ID: &str = "OCENTRA_PARENT_LOG_RUN_ID";
    pub const TEST_NAME: &str = "OCENTRA_PARENT_LOG_TEST_NAME";
    pub const SCOPE: &str = "OCENTRA_PARENT_LOG_SCOPE";
    pub const RUN_TYPE: &str = "OCENTRA_PARENT_LOG_RUN_TYPE";
    pub const SUITE_TYPE: &str = "OCENTRA_PARENT_LOG_SUITE_TYPE";
    pub const ORIGIN: &str = "OCENTRA_PARENT_LOG_ORIGIN";
    pub const ENVIRONMENT: &str = "OCENTRA_PARENT_LOG_ENVIRONMENT";
}

pub struct LoggerRuntimeDefaults;

impl LoggerRuntimeDefaults {
    pub const GENERATED_RUN_ID_PREFIX: &str = "parent-log-run-";
    pub const TEST_NAME: &str = "parent-runtime-logger";
    pub const UNKNOWN_MODULE: &str = "UnknownModule";
    pub const MODULE_CONTEXT_SUFFIX: &str = "module";
}

pub struct DevLogEndpoint;

impl DevLogEndpoint {
    pub const WRITE: &str = "/__ocentra-parent-dev-log";
}

pub struct DevLogHttp;

impl DevLogHttp {
    pub const METHOD_POST: &str = "POST";
    pub const HEADER_CONTENT_TYPE: &str = "Content-Type";
    pub const CONTENT_TYPE_JSON: &str = "application/json";
    pub const CREDENTIALS_SAME_ORIGIN: &str = "same-origin";
}

pub struct DevLogEnvironment;

impl DevLogEnvironment {
    pub const DIRECTORY: &str = "OCENTRA_PARENT_DEV_LOG_DIR";
}

pub struct DevLogBridge;

impl DevLogBridge {
    pub const DEFAULT_URL: &str = "http://127.0.0.1:4479";
    pub const ENVIRONMENT_URL: &str = "VITE_OCENTRA_PARENT_LOG_BRIDGE_URL";
    pub const GLOBAL_URL_KEY: &str = "__OCENTRA_PARENT_LOG_BRIDGE_URL";
    pub const PORTAL_CONTEXT: &str = "portal-dev-observability";
    pub const PORTAL_ENVIRONMENT: &str = "dev";
    pub const PORTAL_TEST_NAME: &str = "portal-dev-runtime";
}

pub struct DevLogFile;

impl DevLogFile {
    pub const DIRECTORY_NAME: &str = "dev";
    pub const EXTENSION: &str = "ndjson";
    pub const AGENT_SERVICE_PREFIX: &str = "agent-service";
    pub const PORTAL_PREFIX: &str = "portal";
    pub const DEV_SERVER_PREFIX: &str = "dev-server";
}

pub struct DevLogField;

impl DevLogField {
    pub const AGENT_WEBSOCKET_URL: &str = "agentWebSocketUrl";
    pub const COMMAND: &str = "command";
    pub const CONNECTION_STATE: &str = "connectionState";
    pub const EVENT: &str = "event";
    pub const EVENTS_BUFFERED: &str = "eventsBuffered";
    pub const PORT: &str = "port";
}

pub struct DevLogIdPrefix;

impl DevLogIdPrefix {
    pub const PORTAL: &str = "portal-log-";
    pub const DEV_SERVER: &str = "dev-server-log-";
}

pub struct DevLogMessage;

impl DevLogMessage {
    pub const PORTAL_STARTED: &str = "Portal dev runtime started.";
    pub const PORTAL_COMMAND_SENT: &str = "Portal command sent.";
    pub const PORTAL_EVENT_RECEIVED: &str = "Portal host bridge event received.";
    pub const PORTAL_RESULT_COPIED: &str = "Portal command result copied.";
    pub const DEV_SERVER_STARTED: &str = "Vite dev server started.";
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogFieldValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null(()),
}

pub type LogFields = BTreeMap<String, LogFieldValue>;

impl From<&str> for LogFieldValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for LogFieldValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for LogFieldValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for LogFieldValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogSource {
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "dev-server")]
    DevServer,
    #[serde(rename = "local-api")]
    LocalApi,
    #[serde(rename = "portal")]
    Portal,
    #[serde(rename = "codex")]
    Codex,
    #[serde(rename = "validation")]
    Validation,
    #[serde(rename = "rust-test")]
    RustTest,
}

impl LogSource {
    pub const AGENT_SERVICE_FILE_PREFIX: &str = "agent-service";
    pub const DEV_SERVER_FILE_PREFIX: &str = "dev-server";
    pub const LOCAL_API_FILE_PREFIX: &str = "local-api";
    pub const PORTAL_FILE_PREFIX: &str = "portal";
    pub const CODEX_FILE_PREFIX: &str = "codex";
    pub const VALIDATION_FILE_PREFIX: &str = "validation";
    pub const RUST_TEST_FILE_PREFIX: &str = "rust-test";

    pub fn compat_file_prefix(&self) -> &str {
        match self {
            Self::AgentService => Self::AGENT_SERVICE_FILE_PREFIX,
            Self::DevServer => Self::DEV_SERVER_FILE_PREFIX,
            Self::LocalApi => Self::LOCAL_API_FILE_PREFIX,
            Self::Portal => Self::PORTAL_FILE_PREFIX,
            Self::Codex => Self::CODEX_FILE_PREFIX,
            Self::Validation => Self::VALIDATION_FILE_PREFIX,
            Self::RustTest => Self::RUST_TEST_FILE_PREFIX,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdentity {
    pub device_id: AgentDeviceId,
    pub hostname: AgentHostname,
    pub platform: AgentPlatform,
    pub service_version: AgentServiceVersion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentLogEntry {
    pub schema_version: u16,
    #[serde(rename = "id")]
    pub entry_id: LogEntryId,
    pub timestamp: LogTimestamp,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: LogMessage,
    pub fields: LogFields,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentLogSnapshot {
    pub schema_version: u16,
    pub agent: AgentIdentity,
    pub entries: Vec<AgentLogEntry>,
}

pub type DevLogEntry = AgentLogEntry;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLogEvent {
    pub schema_version: u16,
    #[serde(rename = "id")]
    pub entry_id: LogEntryId,
    pub timestamp: String,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: String,
    #[serde(default)]
    pub fields: LogFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<LogRunId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lane_id: Option<LogLaneId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<LogCommandId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<LogCorrelationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogSnapshot {
    pub schema_version: u16,
    #[serde(rename = "status")]
    pub snapshot_state: String,
    #[serde(default)]
    pub entries: Vec<ParentLogEvent>,
}
