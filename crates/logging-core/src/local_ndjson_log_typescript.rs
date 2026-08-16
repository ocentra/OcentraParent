use std::path::Path;

use crate::local_ndjson_log::TestLogEntry;

pub fn local_ndjson_log_typescript() -> &'static str {
    LOCAL_NDJSON_LOG_TYPESCRIPT
}

pub(crate) fn suite_segment(value: Option<&str>) -> &str {
    value.unwrap_or(UNSPECIFIED_SUITE)
}

pub(crate) fn is_ascii_alpha_numeric(character: char) -> bool {
    character.is_ascii_alphanumeric()
}

pub(crate) fn is_ascii_lowercase_alpha_numeric(character: char) -> bool {
    character.is_ascii_digit() || character.is_ascii_lowercase()
}

pub(crate) fn sanitize_with_collapsed_dashes(
    value: &str,
    is_allowed: impl Fn(char) -> bool,
) -> String {
    let mut sanitized = String::new();
    for character in value.chars() {
        if is_allowed(character) {
            sanitized.push(character);
            continue;
        }
        if sanitized.is_empty() || sanitized.ends_with('-') {
            continue;
        }
        sanitized.push('-');
    }
    trim_edge_dashes(&sanitized)
}

pub(crate) fn trim_edge_dashes(value: &str) -> String {
    value.trim_matches('-').to_owned()
}

pub(crate) fn matches_file(entry: &TestLogEntry<'_>, file_path: &str) -> bool {
    if entry.file_path == Some(file_path) {
        return true;
    }

    let basename = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str());
    match (entry.file, basename) {
        (Some(file), Some(basename)) => file == basename,
        _ => false,
    }
}

const UNSPECIFIED_SUITE: &str = "unspecified";

const LOCAL_NDJSON_LOG_TYPESCRIPT: &str =
    include_str!("../../../packages/logging-domain/src/local-test-log.ts");
