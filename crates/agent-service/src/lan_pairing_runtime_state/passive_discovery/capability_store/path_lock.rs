use std::fs::{File, OpenOptions};
use std::thread;
use std::time::Duration;

use super::LanPassiveDiscoveryCapabilityPath;

const LOCK_RETRY_COUNT: usize = 100;
const LOCK_RETRY_DELAY_MILLIS: u64 = 5;
const CAPABILITY_LOCK_EXTENSION: &str = "capability";

pub(super) struct CapabilityPathLock {
    _file: File,
}

pub(super) fn acquire(path: &LanPassiveDiscoveryCapabilityPath) -> Option<CapabilityPathLock> {
    #[cfg(target_os = "windows")]
    {
        windows_acquire(path)
    }
    #[cfg(not(target_os = "windows"))]
    non_windows_acquire(path)
}

#[cfg(target_os = "windows")]
fn windows_acquire(path: &LanPassiveDiscoveryCapabilityPath) -> Option<CapabilityPathLock> {
    use std::os::windows::fs::OpenOptionsExt;

    let lock_path = path.0.with_extension(CAPABILITY_LOCK_EXTENSION);
    for _ in 0..LOCK_RETRY_COUNT {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .share_mode(0)
            .open(&lock_path)
        {
            Ok(file) => return Some(CapabilityPathLock { _file: file }),
            Err(error) if matches!(error.raw_os_error(), Some(32 | 33)) => {
                thread::sleep(Duration::from_millis(LOCK_RETRY_DELAY_MILLIS));
            }
            Err(_) => return None,
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn non_windows_acquire(path: &LanPassiveDiscoveryCapabilityPath) -> Option<CapabilityPathLock> {
    let lock_path = path.0.with_extension(CAPABILITY_LOCK_EXTENSION);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .ok()?;
    for _ in 0..LOCK_RETRY_COUNT {
        match file.try_lock() {
            Ok(()) => return Some(CapabilityPathLock { _file: file }),
            Err(std::fs::TryLockError::WouldBlock) => {
                thread::sleep(Duration::from_millis(LOCK_RETRY_DELAY_MILLIS));
            }
            Err(_) => return None,
        }
    }
    None
}
