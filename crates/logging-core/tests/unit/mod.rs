use std::{collections::BTreeMap, error::Error, fs, path::PathBuf};

use ocentra_parent_logging_core::{
    artifact::{ArtifactKind, ArtifactWriter},
    field::{LogFieldValue, LogFields},
    local_ndjson_log::{
        build_manifest, classify_manifest_changes, default_local_log_root,
        group_test_entries_by_file_path, logs_tree_key, matches_wipe_entry, run_dir_path,
        sanitize_path_segment, sanitize_test_name_for_ndjson, select_prune_candidates,
        IngestManifest, LogsTreeScope, ManifestEntry, ObservedFileState, PrunableFile,
        TestLogEntry, WipeScopeOptions,
    },
    ndjson_writer::NdjsonWriter,
    path::path_string,
    redaction::{redact_fields, REDACTED_VALUE},
};
use serde_json::json;

#[macro_use]
#[path = "../support/mod.rs"]
mod support;

mod artifact_subprocess;
mod concurrency_artifact;
#[cfg(feature = "test-support")]
mod ndjson_failure_recovery;
mod ndjson_operation_custody;
#[cfg(feature = "test-support")]
mod ndjson_operation_state;

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
fn path_string_preserves_windows_unc_and_normalizes_extended_drive_paths() {
    assert_eq!(
        path_string(std::path::Path::new(
            r"\\?\UNC\server\share\logs\artifact.log"
        )),
        "//server/share/logs/artifact.log"
    );
    assert_eq!(
        path_string(std::path::Path::new(r"\\server\share\logs\artifact.log")),
        "//server/share/logs/artifact.log"
    );
    assert_eq!(
        path_string(std::path::Path::new(r"\\?\C:\logs\artifact.log")),
        "C:/logs/artifact.log"
    );
    assert_eq!(
        path_string(std::path::Path::new(
            "//?/UNC/server/share/logs/artifact.log"
        )),
        "//server/share/logs/artifact.log"
    );
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
fn redaction_normalizes_secret_key_variants_without_hiding_safe_context() {
    let fields = [
        ("X-API-Key", "one"),
        ("credential.id", "two"),
        ("PRIVATE_key", "three"),
        ("nested/private-key", "four"),
        ("request-id", "safe"),
        ("attemptCount", "7"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_owned(), LogFieldValue::String(value.to_owned())))
    .collect::<LogFields>();

    let redacted = redact_fields(&fields);

    for key in [
        "X-API-Key",
        "credential.id",
        "PRIVATE_key",
        "nested/private-key",
    ] {
        assert_eq!(
            redacted.get(key),
            Some(&LogFieldValue::String(REDACTED_VALUE.to_owned()))
        );
    }
    assert_eq!(
        redacted.get("request-id"),
        Some(&LogFieldValue::String("safe".to_owned()))
    );
    assert_eq!(
        redacted.get("attemptCount"),
        Some(&LogFieldValue::String("7".to_owned()))
    );
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
    let root = temp_dir!();
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

fn ndjson_writer_appends_json_lines_in_order_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = NdjsonWriter::new(&root);

    let first = writer.append_event("parent-test", "dev-log", &json!({ "order": 1 }))?;
    let second = writer.append_event("parent-test", "dev-log", &json!({ "order": 2 }))?;

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
    let root = temp_dir!();
    let writer = NdjsonWriter::new(&root);
    let error = match writer.append_event("../bad", "dev-log", &json!({ "order": 1 })) {
        Ok(path) => {
            return Err(std::io::Error::other(format!(
                "invalid segments must not create log file: {}",
                path.display()
            ))
            .into());
        }
        Err(error) => error,
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(error.to_string(), "invalid log path segment");
    Ok(())
}

fn artifact_writer_writes_text_and_hashes_content_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!().join("fresh").join("nested");
    assert!(!root.exists());
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
    assert!(root.join("parent-codex").join("artifacts").is_dir());
    Ok(())
}
