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
    local_ndjson_log::{
        build_manifest, classify_manifest_changes, default_local_log_root,
        group_test_entries_by_file_path, logs_tree_key, matches_wipe_entry, run_dir_path,
        sanitize_path_segment, sanitize_test_name_for_ndjson, select_prune_candidates,
        IngestManifest, LogsTreeScope, ManifestEntry, ObservedFileState, PrunableFile,
        TestLogEntry, WipeScopeOptions,
    },
    local_ndjson_log_typescript::local_ndjson_log_typescript,
    ndjson_writer::NdjsonWriter,
    path::{path_string, DEV_LOG_DIR_ENV, LANE_ID_ENV, LOG_ROOT_ENV, LOG_RUN_ID_ENV},
    redaction::{redact_fields, REDACTED_VALUE},
    source::LogSource,
};
use ocentra_schema::logging_contracts::{LogEntryId, LogLaneId, LogRunId};
use serde_json::json;

trait LoggingCoreTestOptionExt<T> {
    fn value_or_unreachable(self, context: &str) -> T;
}

impl<T> LoggingCoreTestOptionExt<T> for Option<T> {
    fn value_or_unreachable(self, context: &str) -> T {
        match self {
            Some(value) => value,
            None => unreachable!("{context}"),
        }
    }
}

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn temp_dir(scenario: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    env::temp_dir().join(format!("ocentra-parent-logging-core-{scenario}-{nanos}"))
}

#[test]
fn ndjson_writer_appends_json_lines_in_order() {
    let result = ndjson_writer_appends_json_lines_in_order_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_rejects_invalid_segments() {
    let result = ndjson_writer_rejects_invalid_segments_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_writer_writes_text_and_hashes_content() {
    let result = artifact_writer_writes_text_and_hashes_content_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
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
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn typescript_fixture_deserializes_into_parent_log_event() {
    let result = typescript_fixture_deserializes_into_parent_log_event_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn dev_logger_writes_legacy_file_when_compat_dir_is_set() {
    let result = dev_logger_writes_legacy_file_when_compat_dir_is_set_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn dev_logger_prefers_shared_runtime_env_names() {
    let result = dev_logger_prefers_shared_runtime_env_names_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn local_ndjson_log_sanitizes_and_builds_paths() {
    let root = default_local_log_root(PathBuf::from("workspace").as_path());
    assert_eq!(
        sanitize_path_segment("Run 42 / beta"),
        "Run-42-beta".to_string()
    );
    assert_eq!(
        sanitize_test_name_for_ndjson("Launch Portal (Smoke)!"),
        "launch-portal-smoke".to_string()
    );
    assert_eq!(
        logs_tree_key("parent-test", "single", Some("unit"), "sample"),
        "parent-test\0single\0unit\0sample".to_string()
    );

    let dir = run_dir_path(
        &root,
        &LogsTreeScope {
            scope: "parent-test",
            run_type: "single",
            suite_type: Some("unit"),
        },
        "run-1",
    );
    assert_eq!(
        dir,
        root.join("test-logs")
            .join("parent-test")
            .join("single")
            .join("unit")
    );
}

#[test]
fn local_ndjson_log_groups_entries_and_matches_wipe_scope() {
    let root = temp_dir("local-ndjson-group");
    let entries = vec![
        TestLogEntry {
            scope: "parent-test",
            run_type: "single",
            run_id: "run-a",
            suite_type: Some("unit"),
            file: Some("dev-logger.ts"),
            file_path: Some("apps/portal/src/dev-logger.ts"),
        },
        TestLogEntry {
            scope: "parent-test",
            run_type: "single",
            run_id: "run-a",
            suite_type: Some("unit"),
            file: Some("dev-logger.ts"),
            file_path: Some("apps/portal/src/dev-logger.ts"),
        },
        TestLogEntry {
            scope: "parent-test",
            run_type: "single",
            run_id: "run-b",
            suite_type: Some("unit"),
            file: Some("dev_log.rs"),
            file_path: Some("crates/agent-service/src/dev_log.rs"),
        },
    ];

    let grouped = group_test_entries_by_file_path(&root, &entries);
    assert_eq!(grouped.len(), 2);

    let keep_target = WipeScopeOptions {
        scope: "parent-test",
        run_type: Some("single"),
        suite_type: Some("unit"),
        run_id: Some("run-a"),
        file_path: Some("apps/portal/src/dev-logger.ts"),
    };
    assert!(matches_wipe_entry(&entries[0], &keep_target));
    assert!(!matches_wipe_entry(&entries[2], &keep_target));
}

#[test]
fn local_ndjson_log_selects_prune_candidates_and_builds_manifest() {
    let files = vec![
        PrunableFile {
            file_path: "new.ndjson",
            modified_ms: 20,
        },
        PrunableFile {
            file_path: "old.ndjson",
            modified_ms: 10,
        },
        PrunableFile {
            file_path: "mid.ndjson",
            modified_ms: 15,
        },
    ];
    assert_eq!(
        select_prune_candidates(&files, 1),
        vec!["mid.ndjson".to_string(), "old.ndjson".to_string()]
    );

    let manifest = IngestManifest {
        scope: "parent-test".to_string(),
        updated_at: 1,
        files: BTreeMap::from([(
            "a.ndjson".to_string(),
            ManifestEntry {
                size: 1,
                modified_ms: 1,
                sha256: "same".to_string(),
            },
        )]),
    };
    let observed = vec![
        ObservedFileState {
            resolved_path: "a.ndjson".to_string(),
            size: 1,
            modified_ms: 1,
            sha256: "same".to_string(),
        },
        ObservedFileState {
            resolved_path: "b.ndjson".to_string(),
            size: 2,
            modified_ms: 2,
            sha256: "new".to_string(),
        },
        ObservedFileState {
            resolved_path: "c.ndjson".to_string(),
            size: 3,
            modified_ms: 3,
            sha256: "changed".to_string(),
        },
    ];
    let (new_files, changed_files) = classify_manifest_changes(&manifest, &observed);
    assert_eq!(
        new_files,
        vec!["b.ndjson".to_string(), "c.ndjson".to_string()]
    );
    assert_eq!(changed_files, Vec::<String>::new());

    let rebuilt = build_manifest("parent-test", 5, &observed);
    assert_eq!(rebuilt.scope, "parent-test");
    assert_eq!(rebuilt.updated_at, 5);
    assert_eq!(rebuilt.files.len(), 3);
}

#[test]
fn generated_local_ndjson_log_helper_stays_checked_in() {
    let checked_in =
        include_str!("../../../packages/logging-domain/src/generated/local-test-log.ts");

    assert_eq!(checked_in, local_ndjson_log_typescript());
    assert_eq!(
        checked_in.lines().next(),
        Some("/* generated from crates/logging-core/src/local_ndjson_log.rs */")
    );
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
    let error = match writer.append_event("../bad", DEV_LOG_STREAM, &json!({ "order": 1 })) {
        Ok(path) => unreachable!(
            "invalid segments must not create log file: {}",
            path.display()
        ),
        Err(error) => error,
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(error.to_string(), "invalid log path segment");
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
    let expected_path = path_string(
        &root
            .join("parent-codex")
            .join("artifacts")
            .join("run-1")
            .join("cmd-1")
            .join("stdout.log"),
    );
    assert_eq!(artifact.artifact_path, expected_path);
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
        entry_id: LogEntryId::parse("log-1").value_or_unreachable("valid log entry id"),
        timestamp: "2026-06-15T00:00:00.000Z".to_owned(),
        level: LogLevel::Info,
        source: LogSource::AgentService,
        message: "Agent service dev runtime started.".to_owned(),
        fields: BTreeMap::new(),
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

fn dev_logger_writes_legacy_file_when_compat_dir_is_set_impl() -> Result<(), Box<dyn Error>> {
    let _guard = env_lock()
        .lock()
        .map_err(|error| std::io::Error::other(format!("failed to lock env mutex: {error:?}")))?;

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
    let timestamp = value
        .get("timestamp")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "missing timestamp field")
        })?;
    let expected_name = format!("agent-service-{}.ndjson", &timestamp[..10]);
    assert_eq!(path.parent(), Some(temp.as_path()));
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some(expected_name.as_str())
    );
    assert_eq!(value["message"], "Agent service dev runtime started.");
    Ok(())
}

fn dev_logger_prefers_shared_runtime_env_names_impl() -> Result<(), Box<dyn Error>> {
    let _guard = env_lock()
        .lock()
        .map_err(|error| std::io::Error::other(format!("failed to lock env mutex: {error:?}")))?;

    let temp = temp_dir("shared-runtime-env");
    env::set_var(DEV_LOG_DIR_ENV, &temp);
    env::set_var(LOG_RUN_ID_ENV, "shared-run-id");
    env::set_var("OCENTRA_PARENT_CODEX_RUN_ID", "legacy-run-id");
    env::set_var("LEDGER_LANE", "ledger-lane");
    env::set_var(LANE_ID_ENV, "shared-lane");
    env::set_var("OCENTRA_PARENT_CODEX_LANE_ID", "legacy-lane");

    let path = write_agent_info(
        LogSource::AgentService,
        "Agent service dev runtime started.",
        BTreeMap::new(),
    )?;

    env::remove_var(DEV_LOG_DIR_ENV);
    env::remove_var(LOG_RUN_ID_ENV);
    env::remove_var("OCENTRA_PARENT_CODEX_RUN_ID");
    env::remove_var("LEDGER_LANE");
    env::remove_var(LANE_ID_ENV);
    env::remove_var("OCENTRA_PARENT_CODEX_LANE_ID");

    let payload = fs::read_to_string(path)?;
    let line = payload.lines().next().unwrap_or_default();
    let value: serde_json::Value = serde_json::from_str(line)?;
    assert_eq!(value["runId"].as_str(), Some("shared-run-id"));
    assert_eq!(value["laneId"].as_str(), Some("ledger-lane"));
    Ok(())
}
