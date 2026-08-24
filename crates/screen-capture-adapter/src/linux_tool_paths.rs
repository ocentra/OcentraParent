use std::path::{Path, PathBuf};

use super::tool_security::{trusted_directory, trusted_executable};

const TRUSTED_TOOL_DIRECTORIES: [&str; 3] = ["/usr/bin", "/bin", "/usr/local/bin"];

pub(super) fn executable_path(name: &str) -> Option<PathBuf> {
    if !matches!(name, "xprop" | "xdotool" | "xwd" | "convert") {
        return None;
    }
    TRUSTED_TOOL_DIRECTORIES
        .iter()
        .find_map(|directory| trusted_tool(Path::new(directory), name))
}

pub(super) fn trusted_executable_path(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    if !matches!(name, "xprop" | "xdotool" | "xwd" | "convert") {
        return false;
    }
    TRUSTED_TOOL_DIRECTORIES.iter().any(|directory| {
        let Ok(canonical_root) = std::fs::canonicalize(directory) else {
            return false;
        };
        path.parent() == Some(canonical_root.as_path())
            && trusted_executable(path, &canonical_root)
            && std::fs::canonicalize(path).ok().as_deref() == Some(path)
    })
}

fn trusted_tool(directory: &Path, name: &str) -> Option<PathBuf> {
    let canonical_root = std::fs::canonicalize(directory).ok()?;
    if !trusted_directory(&canonical_root) {
        return None;
    }
    let candidate = directory.join(name);
    if !trusted_executable(&candidate, &canonical_root) {
        return None;
    }
    std::fs::canonicalize(candidate).ok()
}
