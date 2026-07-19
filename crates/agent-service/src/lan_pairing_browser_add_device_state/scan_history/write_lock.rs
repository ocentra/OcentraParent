use std::fs::{File, OpenOptions};
use std::thread;
use std::time::Duration;

use super::LanScanHistoryPath;

const LOCK_EXTENSION: &str = "lock";
const LOCK_RETRY_COUNT: usize = 100;
const LOCK_RETRY_DELAY_MILLIS: u64 = 5;

pub(crate) struct ScanHistoryWriteLock {
    _file: File,
}

pub(crate) fn scan_history_write_lock(path: &LanScanHistoryPath) -> Option<ScanHistoryWriteLock> {
    #[cfg(target_os = "windows")]
    {
        windows_scan_history_write_lock(path)
    }
    #[cfg(not(target_os = "windows"))]
    non_windows_scan_history_write_lock(path)
}

#[cfg(target_os = "windows")]
fn windows_scan_history_write_lock(path: &LanScanHistoryPath) -> Option<ScanHistoryWriteLock> {
    use std::os::windows::fs::OpenOptionsExt;

    let lock_path = path.0.with_extension(LOCK_EXTENSION);
    for _ in 0..LOCK_RETRY_COUNT {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .share_mode(0)
            .open(&lock_path)
        {
            Ok(file) => return Some(ScanHistoryWriteLock { _file: file }),
            Err(error) if matches!(error.raw_os_error(), Some(32 | 33)) => {
                thread::sleep(Duration::from_millis(LOCK_RETRY_DELAY_MILLIS));
            }
            Err(_) => return None,
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn non_windows_scan_history_write_lock(path: &LanScanHistoryPath) -> Option<ScanHistoryWriteLock> {
    let lock_path = path.0.with_extension(LOCK_EXTENSION);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .ok()?;
    for _ in 0..LOCK_RETRY_COUNT {
        match file.try_lock() {
            Ok(()) => return Some(ScanHistoryWriteLock { _file: file }),
            Err(std::fs::TryLockError::WouldBlock) => {
                thread::sleep(Duration::from_millis(LOCK_RETRY_DELAY_MILLIS));
            }
            Err(_) => return None,
        }
    }
    None
}
