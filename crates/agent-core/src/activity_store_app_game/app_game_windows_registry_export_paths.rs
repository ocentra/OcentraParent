use std::fs;
use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION;

pub(crate) fn registry_export_paths_from_roots(roots: &[PathBuf], limit: usize) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in roots {
        collect_registry_export_paths(root, limit, &mut paths);
        if paths.len() >= limit {
            break;
        }
    }
    paths
}

fn collect_registry_export_paths(root: &Path, limit: usize, paths: &mut Vec<PathBuf>) {
    if paths.len() >= limit {
        return;
    }
    if is_registry_export_path(root) {
        paths.push(root.to_path_buf());
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            collect_registry_export_paths(&path, limit, paths);
        } else if is_registry_export_path(&path) {
            paths.push(path);
        }
        if paths.len() >= limit {
            break;
        }
    }
}

fn is_registry_export_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        extension
            .to_string_lossy()
            .eq_ignore_ascii_case(APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION)
    })
}
