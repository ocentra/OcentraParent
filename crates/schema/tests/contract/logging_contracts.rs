use crate::support::ValueOrUnreachable as _;
use ocentra_schema::logging_contracts::{
    AgentDeviceId, AgentHostname, AgentIdentity, AgentLogEntry, AgentLogSnapshot, AgentPlatform,
    AgentServiceVersion, DevLogBridge, DevLogEndpoint, DevLogField, DevLogHttp, DevLogIdPrefix,
    DevLogMessage, LogCorrelationId, LogEntryId, LogFieldValue, LogFields, LogLevel, LogMessage,
    LogRunId, LogSnapshot, LogSource, LogTimestamp, LoggerRuntimeDefaults,
    LoggerRuntimeEnvironment, ParentLogEvent, LOG_SCHEMA_VERSION, LOG_SNAPSHOT_SCHEMA_VERSION,
};
use ocentra_schema::logging_contracts_ts::logging_contracts_typescript;
use serde_json::json;

fn log_fields() -> LogFields {
    let mut fields = LogFields::new();
    fields.insert("attempt".to_string(), LogFieldValue::Number(2.0));
    fields.insert("allowed".to_string(), LogFieldValue::Boolean(true));
    fields.insert("token".to_string(), LogFieldValue::Null(()));
    fields
}

#[test]
fn parent_log_event_preserves_rust_owned_encoded_shape() {
    let mut fields = log_fields();
    fields.insert(
        "scope".to_string(),
        LogFieldValue::String("portal".to_string()),
    );

    let event = ParentLogEvent {
        schema_version: LOG_SCHEMA_VERSION,
        entry_id: LogEntryId::parse("parent-log-1")
            .value_or_unreachable(crate::assert_context!("entry id")),
        timestamp: "2026-06-26T07:30:00Z".to_string(),
        level: LogLevel::Info,
        source: LogSource::AgentService,
        message: "parent runtime started".to_string(),
        fields,
        run_id: Some(
            LogRunId::parse("run-1").value_or_unreachable(crate::assert_context!("run id")),
        ),
        lane_id: None,
        command_id: None,
        correlation_id: Some(
            LogCorrelationId::parse("corr-1")
                .value_or_unreachable(crate::assert_context!("correlation id")),
        ),
        file: None,
        line: Some(42),
        column: None,
    };

    let encoded = serde_json::to_value(&event)
        .value_or_unreachable(crate::assert_context!("log event must serialize"));

    assert_eq!(encoded["schemaVersion"], json!(LOG_SCHEMA_VERSION));
    assert_eq!(encoded["id"], json!("parent-log-1"));
    assert_eq!(encoded["level"], json!("info"));
    assert_eq!(encoded["source"], json!("agent-service"));
    assert_eq!(encoded["fields"]["attempt"], json!(2.0));
    assert_eq!(encoded["fields"]["allowed"], json!(true));
    assert_eq!(encoded["fields"]["token"], json!(null));
    assert_eq!(encoded["runId"], json!("run-1"));
    assert_eq!(encoded["correlationId"], json!("corr-1"));
    assert_eq!(encoded["line"], json!(42));
    assert!(encoded.get("schema_version").is_none());
    assert!(encoded.get("entry_id").is_none());
    assert!(encoded.get("laneId").is_none());

    let decoded: ParentLogEvent = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("encoded event must deserialize"));
    assert_eq!(decoded, event);
}

#[test]
fn agent_log_snapshot_matches_ts_logging_contract_family_shape() {
    let entry = AgentLogEntry {
        schema_version: LOG_SCHEMA_VERSION,
        entry_id: LogEntryId::parse("agent-log-1")
            .value_or_unreachable(crate::assert_context!("entry id")),
        timestamp: LogTimestamp::parse("2026-06-26T07:31:00Z")
            .value_or_unreachable(crate::assert_context!("timestamp")),
        level: LogLevel::Warn,
        source: LogSource::Portal,
        message: LogMessage::parse("portal command retried")
            .value_or_unreachable(crate::assert_context!("message")),
        fields: log_fields(),
    };
    let snapshot = AgentLogSnapshot {
        schema_version: LOG_SCHEMA_VERSION,
        agent: AgentIdentity {
            device_id: AgentDeviceId::parse("device-alpha")
                .value_or_unreachable(crate::assert_context!("device id")),
            hostname: AgentHostname::parse("parent-host")
                .value_or_unreachable(crate::assert_context!("hostname")),
            platform: AgentPlatform::parse("windows")
                .value_or_unreachable(crate::assert_context!("platform")),
            service_version: AgentServiceVersion::parse("0.1.1")
                .value_or_unreachable(crate::assert_context!("version")),
        },
        entries: vec![entry],
    };

    let encoded = serde_json::to_value(&snapshot)
        .value_or_unreachable(crate::assert_context!("snapshot must serialize"));

    assert_eq!(encoded["schemaVersion"], json!(LOG_SCHEMA_VERSION));
    assert_eq!(encoded["agent"]["deviceId"], json!("device-alpha"));
    assert_eq!(encoded["agent"]["serviceVersion"], json!("0.1.1"));
    assert_eq!(encoded["entries"][0]["id"], json!("agent-log-1"));
    assert_eq!(encoded["entries"][0]["source"], json!("portal"));
    assert_eq!(encoded["entries"][0]["level"], json!("warn"));
    assert!(encoded["entries"][0].get("entryId").is_none());

    let decoded: AgentLogSnapshot = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("snapshot must deserialize"));
    assert_eq!(decoded, snapshot);
}

#[test]
fn log_snapshot_preserves_status_alias_and_defaults_entries() {
    let encoded = json!({
        "schemaVersion": LOG_SNAPSHOT_SCHEMA_VERSION,
        "status": "ready"
    });

    let decoded: LogSnapshot = serde_json::from_value(encoded).value_or_unreachable(
        crate::assert_context!("snapshot must deserialize without entries"),
    );
    assert_eq!(decoded.schema_version, LOG_SNAPSHOT_SCHEMA_VERSION);
    assert_eq!(decoded.snapshot_state, "ready");
    assert!(decoded.entries.is_empty());

    let reencoded = serde_json::to_value(&decoded)
        .value_or_unreachable(crate::assert_context!("snapshot must serialize"));
    assert_eq!(reencoded["status"], json!("ready"));
    assert_eq!(reencoded["entries"], json!([]));
}

#[test]
fn generated_typescript_logging_contracts_stay_checked_in() {
    let checked_in =
        include_str!("../../../../packages/schema-domain/src/generated-logging-contracts.ts");
    let generated = logging_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_logger_runtime_environment_constants();
    assert_logger_runtime_defaults();
    assert_dev_log_bridge_constants();
    assert_logging_enum_serialization();
    assert_logging_identifier_parsers();
}

fn assert_logger_runtime_environment_constants() {
    assert_eq!(
        LoggerRuntimeEnvironment::RUN_ID,
        "OCENTRA_PARENT_LOG_RUN_ID"
    );
    assert_eq!(
        LoggerRuntimeEnvironment::TEST_NAME,
        "OCENTRA_PARENT_LOG_TEST_NAME"
    );
    assert_eq!(LoggerRuntimeEnvironment::SCOPE, "OCENTRA_PARENT_LOG_SCOPE");
    assert_eq!(
        LoggerRuntimeEnvironment::RUN_TYPE,
        "OCENTRA_PARENT_LOG_RUN_TYPE"
    );
    assert_eq!(
        LoggerRuntimeEnvironment::SUITE_TYPE,
        "OCENTRA_PARENT_LOG_SUITE_TYPE"
    );
    assert_eq!(
        LoggerRuntimeEnvironment::ORIGIN,
        "OCENTRA_PARENT_LOG_ORIGIN"
    );
    assert_eq!(
        LoggerRuntimeEnvironment::ENVIRONMENT,
        "OCENTRA_PARENT_LOG_ENVIRONMENT"
    );
}

fn assert_logger_runtime_defaults() {
    assert_eq!(
        LoggerRuntimeDefaults::GENERATED_RUN_ID_PREFIX,
        "parent-log-run-"
    );
    assert_eq!(LoggerRuntimeDefaults::TEST_NAME, "parent-runtime-logger");
    assert_eq!(LoggerRuntimeDefaults::UNKNOWN_MODULE, "UnknownModule");
    assert_eq!(LoggerRuntimeDefaults::MODULE_CONTEXT_SUFFIX, "module");
}

fn assert_dev_log_bridge_constants() {
    assert_eq!(DevLogEndpoint::WRITE, "/__ocentra-parent-dev-log");
    assert_eq!(DevLogHttp::METHOD_POST, "POST");
    assert_eq!(DevLogHttp::HEADER_CONTENT_TYPE, "Content-Type");
    assert_eq!(DevLogHttp::CONTENT_TYPE_JSON, "application/json");
    assert_eq!(DevLogHttp::CREDENTIALS_SAME_ORIGIN, "same-origin");
    assert_eq!(DevLogBridge::DEFAULT_URL, "http://127.0.0.1:4479");
    assert_eq!(
        DevLogBridge::ENVIRONMENT_URL,
        "VITE_OCENTRA_PARENT_LOG_BRIDGE_URL"
    );
    assert_eq!(
        DevLogBridge::GLOBAL_URL_KEY,
        "__OCENTRA_PARENT_LOG_BRIDGE_URL"
    );
    assert_eq!(DevLogBridge::PORTAL_CONTEXT, "portal-dev-observability");
    assert_eq!(DevLogBridge::PORTAL_ENVIRONMENT, "dev");
    assert_eq!(DevLogBridge::PORTAL_TEST_NAME, "portal-dev-runtime");

    assert_eq!(DevLogField::AGENT_WEBSOCKET_URL, "agentWebSocketUrl");
    assert_eq!(DevLogField::COMMAND, "command");
    assert_eq!(DevLogField::CONNECTION_STATE, "connectionState");
    assert_eq!(DevLogField::EVENT, "event");
    assert_eq!(DevLogField::EVENTS_BUFFERED, "eventsBuffered");
    assert_eq!(DevLogField::PORT, "port");

    assert_eq!(DevLogIdPrefix::PORTAL, "portal-log-");
    assert_eq!(DevLogIdPrefix::DEV_SERVER, "dev-server-log-");

    assert_eq!(DevLogMessage::PORTAL_STARTED, "Portal dev runtime started.");
    assert_eq!(DevLogMessage::PORTAL_COMMAND_SENT, "Portal command sent.");
    assert_eq!(
        DevLogMessage::PORTAL_EVENT_RECEIVED,
        "Portal host bridge event received."
    );
    assert_eq!(
        DevLogMessage::PORTAL_RESULT_COPIED,
        "Portal command result copied."
    );
    assert_eq!(
        DevLogMessage::DEV_SERVER_STARTED,
        "Vite dev server started."
    );
}

fn assert_logging_enum_serialization() {
    let levels = [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ];
    assert_eq!(
        serde_json::to_value(levels)
            .value_or_unreachable(crate::assert_context!("levels must serialize")),
        json!(["trace", "debug", "info", "warn", "error"])
    );

    let sources = [
        LogSource::AgentService,
        LogSource::DevServer,
        LogSource::LocalApi,
        LogSource::Portal,
        LogSource::Codex,
        LogSource::Validation,
        LogSource::RustTest,
    ];
    assert_eq!(
        serde_json::to_value(sources)
            .value_or_unreachable(crate::assert_context!("sources must serialize")),
        json!([
            "agent-service",
            "dev-server",
            "local-api",
            "portal",
            "codex",
            "validation",
            "rust-test"
        ])
    );
}

fn assert_logging_identifier_parsers() {
    assert_eq!(
        LogEntryId::parse("agent-log-1")
            .value_or_unreachable(crate::assert_context!("entry id"))
            .as_str(),
        "agent-log-1"
    );
    assert_eq!(
        LogTimestamp::parse("2026-06-26T07:31:00Z")
            .value_or_unreachable(crate::assert_context!("timestamp"))
            .as_str(),
        "2026-06-26T07:31:00Z"
    );
    assert_eq!(
        LogMessage::parse(DevLogMessage::PORTAL_STARTED)
            .value_or_unreachable(crate::assert_context!("message"))
            .as_str(),
        DevLogMessage::PORTAL_STARTED
    );
    assert_eq!(
        AgentDeviceId::parse("device-alpha")
            .value_or_unreachable(crate::assert_context!("device id"))
            .as_str(),
        "device-alpha"
    );
    assert_eq!(
        AgentHostname::parse("parent-host")
            .value_or_unreachable(crate::assert_context!("hostname"))
            .as_str(),
        "parent-host"
    );
    assert_eq!(
        AgentPlatform::parse("windows")
            .value_or_unreachable(crate::assert_context!("platform"))
            .as_str(),
        "windows"
    );
    assert_eq!(
        AgentServiceVersion::parse("0.1.1")
            .value_or_unreachable(crate::assert_context!("version"))
            .as_str(),
        "0.1.1"
    );
}

#[test]
fn logging_contracts_adapter_stays_thin_and_generated_backed() {
    let adapter = include_str!("../../../../packages/logging-domain/src/logging-contracts.ts");

    assert_eq!(
        adapter.lines().next(),
        Some("/* thin adapter over Rust-generated logging contracts */")
    );
    assert_eq!(
        adapter
            .lines()
            .find(|line| *line == "} from './generated-logging-contracts';"),
        Some("} from './generated-logging-contracts';")
    );
    assert_eq!(adapter.lines().last(), Some("} as const;"));
}
