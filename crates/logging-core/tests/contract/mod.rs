use std::{error::Error, fs, path::PathBuf};

use ocentra_parent_logging_core::{
    event::{ParentLogEvent, LOG_SCHEMA_VERSION},
    level::LogLevel,
    local_ndjson_log_typescript::local_ndjson_log_typescript,
    source::LogSource,
};
use ocentra_schema::logging_contracts::{LogEntryId, LogLaneId, LogRunId};

#[test]
fn parent_log_event_serializes_expected_level_and_source() {
    let result = parent_log_event_serializes_expected_level_and_source_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn typescript_fixture_deserializes_into_parent_log_event() {
    let result = typescript_fixture_deserializes_into_parent_log_event_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn generated_local_ndjson_log_helper_stays_checked_in() {
    let checked_in = include_str!("../../../../packages/logging-domain/src/local-test-log.ts");

    assert_eq!(checked_in, local_ndjson_log_typescript());
    assert_eq!(
        checked_in.lines().next(),
        Some("/* generated from crates/logging-core/src/local_ndjson_log.rs */")
    );
}

fn parent_log_event_serializes_expected_level_and_source_impl() -> Result<(), Box<dyn Error>> {
    let event = ParentLogEvent {
        schema_version: LOG_SCHEMA_VERSION,
        entry_id: LogEntryId::parse("log-1").ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "valid log entry id")
        })?,
        timestamp: "2026-06-15T00:00:00.000Z".to_owned(),
        level: LogLevel::Info,
        source: LogSource::AgentService,
        message: "Agent service dev runtime started.".to_owned(),
        fields: Default::default(),
        run_id: LogRunId::parse("run-1"),
        lane_id: LogLaneId::parse("lane-1"),
        command_id: None,
        correlation_id: None,
        file: None,
        line: None,
        column: None,
    };

    let value = serde_json::to_value(&event)?;
    assert_eq!(value["level"], "info");
    assert_eq!(value["source"], "agent-service");
    Ok(())
}

fn typescript_fixture_deserializes_into_parent_log_event_impl() -> Result<(), Box<dyn Error>> {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/logging-domain/fixtures/dev-log-entry.json");
    let payload = fs::read_to_string(fixture)?;
    let event: ParentLogEvent = serde_json::from_str(&payload)?;
    assert_eq!(event.source, LogSource::AgentService);
    assert_eq!(event.level, LogLevel::Info);
    Ok(())
}
