use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::local_ndjson_log_typescript::{
    is_ascii_alpha_numeric, is_ascii_lowercase_alpha_numeric, matches_file,
    sanitize_with_collapsed_dashes, suite_segment,
};

pub const LOCAL_LOG_ROOT_ENV: &str = "OCENTRA_PARENT_LOG_DIR";
pub const TEST_LOG_DIR: &str = "test-logs";
pub const APP_LOG_DIR: &str = "app-logs";
pub const DB_DIR: &str = "db";
pub const MANIFEST_DIR: &str = "manifests";
const DEFAULT_SEGMENT: &str = "default";
const DEFAULT_TEST_NAME: &str = "unnamed-test";
const TEST_NAME_LIMIT: usize = 100;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogsTreeScope<'a> {
    pub scope: &'a str,
    pub run_type: &'a str,
    pub suite_type: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestLogEntry<'a> {
    pub scope: &'a str,
    pub run_type: &'a str,
    pub run_id: &'a str,
    pub suite_type: Option<&'a str>,
    pub file: Option<&'a str>,
    pub file_path: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WipeScopeOptions<'a> {
    pub scope: &'a str,
    pub run_type: Option<&'a str>,
    pub suite_type: Option<&'a str>,
    pub run_id: Option<&'a str>,
    pub file_path: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrunableFile<'a> {
    pub file_path: &'a str,
    pub modified_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestEntry {
    pub size: u64,
    pub modified_ms: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedFileState {
    pub resolved_path: String,
    pub size: u64,
    pub modified_ms: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IngestManifest {
    pub scope: String,
    pub updated_at: u64,
    pub files: BTreeMap<String, ManifestEntry>,
}

pub fn sanitize_path_segment(value: &str) -> String {
    let sanitized = sanitize_with_collapsed_dashes(value, |character| {
        is_ascii_alpha_numeric(character) || matches!(character, '.' | '_' | '-')
    });
    if sanitized.is_empty() {
        DEFAULT_SEGMENT.to_owned()
    } else {
        sanitized
    }
}

pub fn sanitize_test_name_for_ndjson(test_name: &str) -> String {
    let lower = test_name.to_ascii_lowercase();
    let sanitized = sanitize_with_collapsed_dashes(&lower, is_ascii_lowercase_alpha_numeric);
    let truncated = sanitized.chars().take(TEST_NAME_LIMIT).collect::<String>();
    if truncated.is_empty() {
        DEFAULT_TEST_NAME.to_owned()
    } else {
        truncated
    }
}

pub fn default_local_log_root(workspace_root: &Path) -> PathBuf {
    workspace_root.join("output").join("logging-domain")
}

pub fn test_log_scope_dir(root: &Path, scope: &str) -> PathBuf {
    root.join(TEST_LOG_DIR).join(scope)
}

pub fn run_ndjson_file_path(
    root: &Path,
    scope: &str,
    run_type: &str,
    run_id: &str,
    suite_type: Option<&str>,
) -> PathBuf {
    test_log_scope_dir(root, scope)
        .join(run_type)
        .join(suite_segment(suite_type))
        .join(format!("{}.ndjson", sanitize_path_segment(run_id)))
}

pub fn app_log_scope_dir(root: &Path, scope: &str) -> PathBuf {
    root.join(APP_LOG_DIR).join(scope)
}

pub fn app_session_file_path(root: &Path, scope: &str, session_id: &str) -> PathBuf {
    app_log_scope_dir(root, scope).join(format!("{}.ndjson", sanitize_path_segment(session_id)))
}

pub fn db_dir(root: &Path) -> PathBuf {
    root.join(DB_DIR)
}

pub fn manifest_dir(root: &Path) -> PathBuf {
    root.join(MANIFEST_DIR)
}

pub fn manifest_path(root: &Path, scope: &str) -> PathBuf {
    manifest_dir(root).join(format!("{scope}-ingest-manifest.json"))
}

pub fn logs_tree_key(
    scope: &str,
    run_type: &str,
    suite_type: Option<&str>,
    file_key: &str,
) -> String {
    format!(
        "{scope}\0{run_type}\0{}\0{file_key}",
        suite_segment(suite_type)
    )
}

pub fn run_dir_path(root: &Path, scope: &LogsTreeScope<'_>, file_key: &str) -> PathBuf {
    let run_file_path = run_ndjson_file_path(
        root,
        scope.scope,
        scope.run_type,
        file_key,
        scope.suite_type,
    );
    run_file_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or(run_file_path)
}

pub fn group_test_entries_by_file_path(
    root: &Path,
    entries: &[TestLogEntry<'_>],
) -> BTreeMap<String, Vec<usize>> {
    let mut grouped: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, entry) in entries.iter().enumerate() {
        let file_path = run_ndjson_file_path(
            root,
            entry.scope,
            entry.run_type,
            entry.run_id,
            entry.suite_type,
        )
        .to_string_lossy()
        .to_string();
        grouped.entry(file_path).or_default().push(index);
    }
    grouped
}

pub fn non_empty_ndjson_lines(content: &str) -> Vec<&str> {
    content
        .trim()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn matches_wipe_entry(entry: &TestLogEntry<'_>, options: &WipeScopeOptions<'_>) -> bool {
    entry.scope == options.scope
        && options
            .run_type
            .is_none_or(|run_type| entry.run_type == run_type)
        && options
            .suite_type
            .is_none_or(|suite_type| entry.suite_type == Some(suite_type))
        && options.run_id.is_none_or(|run_id| entry.run_id == run_id)
        && options
            .file_path
            .is_none_or(|file_path| matches_file(entry, file_path))
}

pub fn select_prune_candidates(files: &[PrunableFile<'_>], keep_newest: isize) -> Vec<String> {
    let mut sorted = files.to_vec();
    sorted.sort_by(|left, right| {
        right
            .modified_ms
            .cmp(&left.modified_ms)
            .then_with(|| left.file_path.cmp(right.file_path))
    });
    let keep = keep_newest.max(0) as usize;
    sorted
        .into_iter()
        .skip(keep)
        .map(|file| file.file_path.to_owned())
        .collect()
}

pub fn classify_manifest_changes(
    manifest: &IngestManifest,
    observed_files: &[ObservedFileState],
) -> (Vec<String>, Vec<String>) {
    observed_files.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut new_files, mut changed_files), observed| {
            match manifest.files.get(&observed.resolved_path) {
                None => new_files.push(observed.resolved_path.clone()),
                Some(existing)
                    if existing.size != observed.size
                        || existing.modified_ms != observed.modified_ms
                        || existing.sha256 != observed.sha256 =>
                {
                    changed_files.push(observed.resolved_path.clone());
                }
                Some(_) => {}
            }
            (new_files, changed_files)
        },
    )
}

pub fn build_manifest(
    scope: &str,
    updated_at: u64,
    observed_files: &[ObservedFileState],
) -> IngestManifest {
    IngestManifest {
        scope: scope.to_owned(),
        updated_at,
        files: observed_files
            .iter()
            .map(|observed| {
                (
                    observed.resolved_path.clone(),
                    ManifestEntry {
                        size: observed.size,
                        modified_ms: observed.modified_ms,
                        sha256: observed.sha256.clone(),
                    },
                )
            })
            .collect(),
    }
}
