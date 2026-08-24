use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::sync::{Mutex, OnceLock};

use atomicwrites::{AllowOverwrite, AtomicFile};

use super::{LanPassiveDiscoveryCapabilityPath, LanPassiveDiscoveryRuntimeCapability};

static FAILED_CAPABILITY_PATHS: OnceLock<Mutex<HashSet<LanPassiveDiscoveryCapabilityPath>>> =
    OnceLock::new();

pub(super) fn save(
    path: &LanPassiveDiscoveryCapabilityPath,
    capability: &LanPassiveDiscoveryRuntimeCapability,
) -> bool {
    if let Some(parent) = path.0.parent() {
        if fs::create_dir_all(parent).is_err() {
            record_failure(path);
            return false;
        }
    }
    if write(path, capability).is_ok() {
        clear_failure(path);
        return true;
    }
    record_failure(path);
    false
}

pub(super) fn failed(path: &LanPassiveDiscoveryCapabilityPath) -> bool {
    failed_paths()
        .lock()
        .map(|failed_paths| failed_paths.contains(path))
        .unwrap_or(true)
}

fn record_failure(path: &LanPassiveDiscoveryCapabilityPath) {
    if let Ok(mut failed_paths) = failed_paths().lock() {
        failed_paths.insert(path.clone());
    }
    let _removed = fs::remove_file(&path.0);
}

fn clear_failure(path: &LanPassiveDiscoveryCapabilityPath) {
    if let Ok(mut failed_paths) = failed_paths().lock() {
        failed_paths.remove(path);
    }
}

fn failed_paths() -> &'static Mutex<HashSet<LanPassiveDiscoveryCapabilityPath>> {
    FAILED_CAPABILITY_PATHS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn write(
    path: &LanPassiveDiscoveryCapabilityPath,
    capability: &LanPassiveDiscoveryRuntimeCapability,
) -> io::Result<()> {
    let json = serde_json::to_vec_pretty(capability).map_err(io::Error::other)?;
    AtomicFile::new(&path.0, AllowOverwrite)
        .write(|file| {
            file.write_all(&json)?;
            file.sync_all()
        })
        .map_err(|error| io::Error::other(error.to_string()))
}
