use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

use atomicwrites::{AllowOverwrite, AtomicFile};

use super::{
    LanPassiveDiscoveryCapabilityPath, LanPassiveDiscoveryPipelineHealthSnapshot,
    LanPassiveDiscoveryRuntimeCapability, LanPassiveDiscoverySourceCapability,
};

static FAILED_CAPABILITY_PATHS: OnceLock<Mutex<HashSet<LanPassiveDiscoveryCapabilityPath>>> =
    OnceLock::new();
static CAPABILITY_PATH_LOCKS: OnceLock<
    Mutex<HashMap<LanPassiveDiscoveryCapabilityPath, Arc<Mutex<()>>>>,
> = OnceLock::new();

pub(super) fn load(
    path: &LanPassiveDiscoveryCapabilityPath,
) -> Option<LanPassiveDiscoveryRuntimeCapability> {
    with_path_lock(path, || load_unlocked(path)).flatten()
}

pub(super) fn save(
    path: &LanPassiveDiscoveryCapabilityPath,
    capability: &LanPassiveDiscoveryRuntimeCapability,
) -> bool {
    with_path_lock(path, || save_validated_unlocked(path, capability)).unwrap_or_else(|| {
        record_failure(path);
        false
    })
}

pub(super) fn save_pipeline_health(
    path: &LanPassiveDiscoveryCapabilityPath,
    pipeline_health: &LanPassiveDiscoveryPipelineHealthSnapshot,
) -> bool {
    with_path_lock(path, || {
        let sources = load_unlocked(path)
            .and_then(super::validation::validate_and_rederive)
            .map(|capability| capability.sources)
            .unwrap_or_else(super::pending_source_capabilities);
        let capability =
            LanPassiveDiscoveryRuntimeCapability::from_sources(sources, pipeline_health.clone());
        save_validated_unlocked(path, &capability)
    })
    .unwrap_or_else(|| {
        record_failure(path);
        false
    })
}

pub(super) fn save_sources(
    path: &LanPassiveDiscoveryCapabilityPath,
    sources: &[LanPassiveDiscoverySourceCapability],
    pipeline_health: &LanPassiveDiscoveryPipelineHealthSnapshot,
) -> bool {
    with_path_lock(path, || {
        let capability = LanPassiveDiscoveryRuntimeCapability::from_sources(
            sources.to_vec(),
            pipeline_health.clone(),
        );
        save_validated_unlocked(path, &capability)
    })
    .unwrap_or_else(|| {
        record_failure(path);
        false
    })
}

fn with_path_lock<T>(
    path: &LanPassiveDiscoveryCapabilityPath,
    operation: impl FnOnce() -> T,
) -> Option<T> {
    if let Some(parent) = path.0.parent() {
        fs::create_dir_all(parent).ok()?;
    }
    let process_lock = {
        let mut locks = capability_path_locks().lock().ok()?;
        Arc::clone(
            locks
                .entry(path.clone())
                .or_insert_with(|| Arc::new(Mutex::new(()))),
        )
    };
    let _process_guard = acquire_process_lock(&process_lock)?;
    let _cross_process_guard = super::path_lock::acquire(path)?;
    Some(operation())
}

fn acquire_process_lock(lock: &Mutex<()>) -> Option<MutexGuard<'_, ()>> {
    const RETRY_COUNT: usize = 100;
    const RETRY_DELAY: Duration = Duration::from_millis(5);
    for _ in 0..RETRY_COUNT {
        match lock.try_lock() {
            Ok(guard) => return Some(guard),
            Err(std::sync::TryLockError::WouldBlock) => thread::sleep(RETRY_DELAY),
            Err(std::sync::TryLockError::Poisoned(_)) => return None,
        }
    }
    None
}

fn load_unlocked(
    path: &LanPassiveDiscoveryCapabilityPath,
) -> Option<LanPassiveDiscoveryRuntimeCapability> {
    if failed_unlocked(path) {
        return None;
    }
    let json = fs::read_to_string(&path.0).ok()?;
    serde_json::from_str(&json).ok()
}

fn save_unlocked(
    path: &LanPassiveDiscoveryCapabilityPath,
    capability: &LanPassiveDiscoveryRuntimeCapability,
) -> bool {
    if write(path, capability).is_ok() {
        clear_failure(path);
        return true;
    }
    record_failure(path);
    false
}

fn save_validated_unlocked(
    path: &LanPassiveDiscoveryCapabilityPath,
    capability: &LanPassiveDiscoveryRuntimeCapability,
) -> bool {
    let Some(capability) = super::validation::validate_and_rederive(capability.clone()) else {
        record_failure(path);
        return false;
    };
    save_unlocked(path, &capability)
}

fn failed_unlocked(path: &LanPassiveDiscoveryCapabilityPath) -> bool {
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

fn capability_path_locks(
) -> &'static Mutex<HashMap<LanPassiveDiscoveryCapabilityPath, Arc<Mutex<()>>>> {
    CAPABILITY_PATH_LOCKS.get_or_init(|| Mutex::new(HashMap::new()))
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
