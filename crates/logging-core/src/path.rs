use std::{
    env, io,
    path::{Path, PathBuf},
};

use chrono::{SecondsFormat, Utc};

pub const LOG_ROOT_ENV: &str = "OCENTRA_PARENT_LOG_ROOT";
pub const LOG_SCOPE_ENV: &str = "OCENTRA_PARENT_LOG_SCOPE";
pub const DEV_LOG_DIR_ENV: &str = "OCENTRA_PARENT_DEV_LOG_DIR";
pub const CODEX_RUN_ID_ENV: &str = "OCENTRA_PARENT_CODEX_RUN_ID";
pub const CODEX_LANE_ID_ENV: &str = "OCENTRA_PARENT_CODEX_LANE_ID";
pub const DEFAULT_SCOPE: &str = "parent-agent";

pub fn resolve_log_root() -> io::Result<PathBuf> {
    if let Some(path) = env_path(LOG_ROOT_ENV) {
        return Ok(path);
    }
    if let Some(repo_root) = find_repo_root()? {
        return Ok(repo_root.join(".logs"));
    }
    Ok(env::current_dir()?.join(".logs"))
}

pub fn resolve_log_scope() -> String {
    match env::var(LOG_SCOPE_ENV) {
        Ok(value) => sanitize_segment(&value).unwrap_or_else(|_| DEFAULT_SCOPE.to_owned()),
        Err(_) => DEFAULT_SCOPE.to_owned(),
    }
}

pub fn sanitize_segment(segment: &str) -> io::Result<String> {
    let trimmed = segment.trim();
    let has_forbidden_separator =
        trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains(':');
    if trimmed.is_empty() || trimmed == "." || trimmed == ".." || has_forbidden_separator {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid log path segment",
        ));
    }
    Ok(trimmed.to_owned())
}

pub fn path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn timestamp_now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn date_stamp_now() -> String {
    Utc::now().format("%F").to_string()
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn find_repo_root() -> io::Result<Option<PathBuf>> {
    let mut current = env::current_dir()?;
    loop {
        if has_repo_markers(&current) {
            return Ok(Some(current));
        }
        if !current.pop() {
            break;
        }
    }
    Ok(None)
}

fn has_repo_markers(path: &Path) -> bool {
    path.join(".git").exists()
        || path.join("Cargo.toml").exists()
        || path.join("package.json").exists()
}
