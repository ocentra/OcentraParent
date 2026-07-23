use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, OnceLock},
};

use crate::ndjson_operation_compaction_bloom::CommitBloom;

const MAX_CACHED_PATHS: usize = 8;
const MAX_HOT_MARKERS: usize = 4 * 1024;

#[derive(Default)]
pub(crate) struct CachedCommitIndex {
    pub(crate) markers: HashMap<String, String>,
    membership: CommitBloom,
    pub(crate) scanned_len: u64,
    #[cfg(feature = "test-support")]
    pub(crate) scanned_bytes: u64,
}

impl CachedCommitIndex {
    pub(crate) fn clear(&mut self) {
        self.markers.clear();
        self.membership.clear();
        self.scanned_len = 0;
    }

    pub(crate) fn record_marker(&mut self, key: String, marker: String) {
        self.membership.insert(&key);
        if self.markers.len() >= MAX_HOT_MARKERS && !self.markers.contains_key(&key) {
            if let Some(evicted) = self.markers.keys().next().cloned() {
                self.markers.remove(&evicted);
            }
        }
        self.markers.insert(key, marker);
    }

    pub(crate) fn might_contain(&self, key: &str) -> bool {
        self.membership.might_contain(key)
    }
}

#[derive(Default)]
struct CommitCaches(HashMap<PathBuf, CachedCommitIndex>);

static COMMIT_CACHES: OnceLock<Mutex<CommitCaches>> = OnceLock::new();

pub(crate) fn with_commit_index<T>(
    path: &Path,
    action: impl FnOnce(&mut CachedCommitIndex) -> io::Result<T>,
) -> io::Result<T> {
    let mut caches = lock_caches()?;
    if !caches.0.contains_key(path) && caches.0.len() >= MAX_CACHED_PATHS {
        if let Some(evicted) = caches.0.keys().next().cloned() {
            caches.0.remove(&evicted);
        }
    }
    action(caches.0.entry(path.to_owned()).or_default())
}

pub(crate) fn forget_commit_index(path: &Path) -> io::Result<()> {
    lock_caches()?.0.remove(path);
    Ok(())
}

#[cfg(feature = "test-support")]
pub(crate) fn scanned_bytes(path: &Path) -> io::Result<u64> {
    with_commit_index(path, |index| Ok(index.scanned_bytes))
}

#[cfg(feature = "test-support")]
pub(crate) fn cache_counts(path: &Path) -> io::Result<(usize, usize)> {
    let caches = lock_caches()?;
    Ok((
        caches.0.len(),
        caches
            .0
            .get(path)
            .map(|index| index.markers.len())
            .unwrap_or(0),
    ))
}

fn lock_caches() -> io::Result<MutexGuard<'static, CommitCaches>> {
    COMMIT_CACHES
        .get_or_init(|| Mutex::new(CommitCaches::default()))
        .lock()
        .map_err(|_poisoned| io::Error::other("operation commit cache lock poisoned"))
}
