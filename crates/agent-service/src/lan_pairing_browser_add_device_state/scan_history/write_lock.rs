use std::fs::{File, OpenOptions};
use std::thread;
use std::time::Duration;

use super::LanScanHistoryPath;

const LOCK_EXTENSION: &str = "lock";
const SCAN_EXECUTION_LOCK_EXTENSION: &str = "scan-execution";
const LOCK_RETRY_COUNT: usize = 100;
const LOCK_RETRY_DELAY_MILLIS: u64 = 5;

pub(crate) enum ScanHistoryLockKind {
    Write,
    Execution,
}

pub(crate) struct ScanHistoryWriteLock {
    _lock: CrossProcessPathLock,
}

pub(crate) fn scan_history_write_lock(path: &LanScanHistoryPath) -> Option<ScanHistoryWriteLock> {
    cross_process_path_lock(path, &ScanHistoryLockKind::Write)
        .map(|lock| ScanHistoryWriteLock { _lock: lock })
}

pub(crate) struct CrossProcessPathLock {
    _file: File,
}

fn lock_path(path: &LanScanHistoryPath, lock_kind: &ScanHistoryLockKind) -> LanScanHistoryPath {
    let extension = if matches!(lock_kind, ScanHistoryLockKind::Write) {
        LOCK_EXTENSION
    } else {
        SCAN_EXECUTION_LOCK_EXTENSION
    };
    LanScanHistoryPath(path.0.with_extension(extension))
}

pub(crate) fn cross_process_path_lock(
    path: &LanScanHistoryPath,
    lock_kind: &ScanHistoryLockKind,
) -> Option<CrossProcessPathLock> {
    #[cfg(target_os = "windows")]
    {
        windows_cross_process_path_lock(path, lock_kind)
    }
    #[cfg(not(target_os = "windows"))]
    non_windows_cross_process_path_lock(path, lock_kind)
}

#[cfg(target_os = "windows")]
fn windows_cross_process_path_lock(
    path: &LanScanHistoryPath,
    lock_kind: &ScanHistoryLockKind,
) -> Option<CrossProcessPathLock> {
    use std::os::windows::fs::OpenOptionsExt;

    let lock_path = lock_path(path, lock_kind);
    for _ in 0..LOCK_RETRY_COUNT {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .share_mode(0)
            .open(&lock_path.0)
        {
            Ok(file) => return Some(CrossProcessPathLock { _file: file }),
            Err(error) if matches!(error.raw_os_error(), Some(32 | 33)) => {
                thread::sleep(Duration::from_millis(LOCK_RETRY_DELAY_MILLIS));
            }
            Err(_) => return None,
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn non_windows_cross_process_path_lock(
    path: &LanScanHistoryPath,
    lock_kind: &ScanHistoryLockKind,
) -> Option<CrossProcessPathLock> {
    let lock_path = lock_path(path, lock_kind);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path.0)
        .ok()?;
    for _ in 0..LOCK_RETRY_COUNT {
        match file.try_lock() {
            Ok(()) => return Some(CrossProcessPathLock { _file: file }),
            Err(std::fs::TryLockError::WouldBlock) => {
                thread::sleep(Duration::from_millis(LOCK_RETRY_DELAY_MILLIS));
            }
            Err(_) => return None,
        }
    }
    None
}
