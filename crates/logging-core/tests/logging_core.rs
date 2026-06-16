use std::{
    collections::BTreeMap,
    env,
    error::Error,
    fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_logging_core::{
    artifact::{ArtifactKind, ArtifactWriter},
    dev_log::{write_agent_info, DEV_LOG_STREAM},
    event::{ParentLogEvent, LOG_SCHEMA_VERSION},
    field::{LogFieldValue, LogFields},
    level::LogLevel,
    ndjson_writer::NdjsonWriter,
    path::{path_string, DEV_LOG_DIR_ENV, LOG_ROOT_ENV},
    redaction::{redact_fields, REDACTED_VALUE},
    source::LogSource,
};
use serde_json::json;

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    env::temp_dir().join(format!("ocentra-parent-logging-core-{label}-{nanos}"))
}

#[test]
fn ndjson_writer_appends_json_lines_in_order() {
    let result = ndjson_writer_appends_json_lines_in_order_impl();
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn ndjson_writer_rejects_invalid_segments() {
    let result = ndjson_writer_rejects_invalid_segments_impl();
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn artifact_writer_writes_text_and_hashes_content() {
    let result = artifact_writer_writes_text_and_hashes_content_impl();
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn redaction_replaces_secret_like_fields() {
    let mut fields = LogFields::new();
    fields.insert(
        "apiToken".to_owned(),
        LogFieldValue::String("top-secret".to_owned()),
    );
    fields.insert(
        "safe".to_owned(),
        LogFieldValue::String("visible".to_owned()),
    );

    let redacted = redact_fields(&fields);

    assert_eq!(
        redacted.get("apiToken"),
        Some(&LogFieldValue::String(REDACTED_VALUE.to_owned()))
    );
    assert_eq!(
        redacted.get("safe"),
        Some(&LogFieldValue::String("visible".to_owned()))
    );
}

#[test]
fn parent_log_event_serializes_expected_level_and_source() {
    let result = parent_log_event_serializes_expected_level_and_source_impl();
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn typescript_fixture_deserializes_into_parent_log_event() {
    let result = typescript_fixture_deserializes_into_parent_log_event_impl();
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn dev_logger_writes_legacy_file_when_compat_dir_is_set() {
    let result = dev_logger_writes_legacy_file_when_compat_dir_is_set_impl();
    assert!(result.is_ok(), "{result:?}");
}

fn ndjson_writer_appends_json_lines_in_order_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir("ndjson-order");
    let writer = NdjsonWriter::new(&root);

    let first = writer.append_event("parent-test", DEV_LOG_STREAM, &json!({ "order": 1 }))?;
    let second = writer.append_event("parent-test", DEV_LOG_STREAM, &json!({ "order": 2 }))?;

    assert_eq!(first, second);
    let payload = fs::read_to_string(&first)?;
    let lines: Vec<&str> = payload.lines().collect();
    assert_eq!(lines.len(), 2);

    let first_value: serde_json::Value = serde_json::from_str(lines[0])?;
    let second_value: serde_json::Value = serde_json::from_str(lines[1])?;
    assert_eq!(first_value["order"], 1);
    assert_eq!(second_value["order"], 2);
    Ok(())
}

fn ndjson_writer_rejects_invalid_segments_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir("ndjson-invalid");
    let writer = NdjsonWriter::new(&root);
    let append = writer.append_event("../bad", DEV_LOG_STREAM, &json!({ "order": 1 }));
    assert!(append.is_err());
    Ok(())
}

fn artifact_writer_writes_text_and_hashes_content_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir("artifact");
    let writer = ArtifactWriter::new(&root);
    let artifact = writer.write_text_artifact(
        "parent-codex",
        "run-1",
        "cmd-1",
        ArtifactKind::Stdout,
        "alpha\nbeta\n",
    )?;

    assert_eq!(artifact.kind, ArtifactKind::Stdout);
    assert_eq!(artifact.byte_length, 11);
    assert_eq!(artifact.line_count, 2);
    assert_eq!(artifact.sha256.len(), 64);
    assert!(artifact.path.ends_with("stdout.log"));
    let file_text = fs::read_to_string(
        root.join("parent-codex")
            .join("artifacts")
            .join("run-1")
            .join("cmd-1")
            .join("stdout.log"),
    )?;
    assert_eq!(file_text, "alpha\nbeta\n");
    Ok(())
}

fn parent_log_event_serializes_expected_level_and_source_impl() -> Result<(), Box<dyn Error>> {
    let event = ParentLogEvent {
        schema_version: LOG_SCHEMA_VERSION,
        id: "log-1".to_owned(),
        timestamp: "2026-06-15T00:00:00.000Z".to_owned(),
        level: LogLevel::Info,
        source: LogSource::AgentService,
        message: "Agent service dev runtime started.".to_owned(),
        fields: BTreeMap::new(),
        run_id: None,
        lane_id: None,
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

fn dev_logger_writes_legacy_file_when_compat_dir_is_set_impl() -> Result<(), Box<dyn Error>> {
    let guard = env_lock().lock();
    assert!(guard.is_ok());
    let _guard = guard.ok();

    let temp = temp_dir("legacy");
    env::remove_var(LOG_ROOT_ENV);
    env::set_var(DEV_LOG_DIR_ENV, &temp);

    let path = write_agent_info(
        LogSource::AgentService,
        "Agent service dev runtime started.",
        BTreeMap::new(),
    )?;

    env::remove_var(DEV_LOG_DIR_ENV);

    let payload = fs::read_to_string(&path)?;
    let line = payload.lines().next().unwrap_or_default();
    let value: serde_json::Value = serde_json::from_str(line)?;
    assert!(path_string(&path).contains("agent-service-"));
    assert_eq!(value["message"], "Agent service dev runtime started.");
    Ok(())
}
