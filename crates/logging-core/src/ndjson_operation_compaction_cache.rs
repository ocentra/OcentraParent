use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, OnceLock},
};

#[derive(Default)]
pub(crate) struct CachedCommitIndex {
    pub(crate) markers: HashMap<String, String>,
    pub(crate) scanned_len: u64,
    #[cfg(feature = "test-support")]
    pub(crate) scanned_bytes: u64,
}

#[derive(Default)]
struct CommitCaches(HashMap<PathBuf, CachedCommitIndex>);

static COMMIT_CACHES: OnceLock<Mutex<CommitCaches>> = OnceLock::new();

pub(crate) fn with_commit_index<T>(
    path: &Path,
    action: impl FnOnce(&mut CachedCommitIndex) -> io::Result<T>,
) -> io::Result<T> {
    let mut caches = lock_caches()?;
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

fn lock_caches() -> io::Result<MutexGuard<'static, CommitCaches>> {
    COMMIT_CACHES
        .get_or_init(|| Mutex::new(CommitCaches::default()))
        .lock()
        .map_err(|_poisoned| io::Error::other("operation commit cache lock poisoned"))
}
