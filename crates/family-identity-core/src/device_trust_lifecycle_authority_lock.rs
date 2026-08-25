use std::{
    fs::File,
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};

use fs2::FileExt;

use crate::{
    device_trust_lifecycle::DeviceTrustLifecycleError,
    device_trust_lifecycle_authority_intent,
    device_trust_lifecycle_authority_store::{load_values, open_lock},
};

const AUTHORITY_LOCK_WAIT: Duration = Duration::from_secs(10);

/// A bounded shared owner fence. The file is intentionally owned by this
/// guard so the sidecar generation and pending-intent checks remain stable
/// until the caller's SQLite transaction has committed or rolled back.
pub(crate) struct AuthorityReadFence {
    lock: Option<File>,
    values_path: PathBuf,
    intent_path: PathBuf,
}

impl AuthorityReadFence {
    pub(crate) fn matches(
        &self,
        key: &str,
        generation: u64,
    ) -> Result<bool, DeviceTrustLifecycleError> {
        let has_pending_intent =
            device_trust_lifecycle_authority_intent::load(&self.intent_path)?.is_some();
        let generation_matches =
            load_values(&self.values_path, true)?.get(key).copied() == Some(generation);
        Ok(!has_pending_intent && generation_matches)
    }

    fn release_lock(&mut self) -> Result<(), DeviceTrustLifecycleError> {
        let Some(lock) = self.lock.take() else {
            return Ok(());
        };
        FileExt::unlock(&lock).map_err(|_error| DeviceTrustLifecycleError::Unavailable)
    }
}

impl Drop for AuthorityReadFence {
    fn drop(&mut self) {
        let _unlock_result = self.release_lock();
    }
}

pub(crate) fn read_fence(
    values_path: &Path,
    intent_path: &Path,
    lock_path: &Path,
) -> Result<AuthorityReadFence, DeviceTrustLifecycleError> {
    let lock = open_lock(lock_path)?;
    lock_shared_bounded(&lock)?;
    Ok(AuthorityReadFence {
        lock: Some(lock),
        values_path: values_path.to_owned(),
        intent_path: intent_path.to_owned(),
    })
}

pub(crate) fn matches(
    values_path: &Path,
    intent_path: &Path,
    lock_path: &Path,
    key: &str,
    generation: u64,
) -> Result<bool, DeviceTrustLifecycleError> {
    let fence = read_fence(values_path, intent_path, lock_path)?;
    fence.matches(key, generation)
}

pub(crate) fn lock_exclusive_bounded(lock: &File) -> Result<(), DeviceTrustLifecycleError> {
    bounded_lock(|| FileExt::try_lock_exclusive(lock))
}

fn lock_shared_bounded(lock: &File) -> Result<(), DeviceTrustLifecycleError> {
    bounded_lock(|| FileExt::try_lock_shared(lock))
}

fn bounded_lock<F>(mut attempt: F) -> Result<(), DeviceTrustLifecycleError>
where
    F: FnMut() -> std::io::Result<()>,
{
    let deadline = Instant::now() + AUTHORITY_LOCK_WAIT;
    loop {
        match attempt() {
            Ok(()) => return Ok(()),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
                ) && Instant::now() < deadline =>
            {
                thread::sleep(Duration::from_millis(10));
            }
            Err(_error) => return Err(DeviceTrustLifecycleError::Unavailable),
        }
    }
}
