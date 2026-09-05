use std::{fs, path::Path};

use super::Error;

#[path = "windows_device_trust_custody_active_record_scan/entry.rs"]
mod entry;

// Recovery inspection is fail-closed and bounded so a hostile custody root cannot
// turn startup into an unbounded directory walk or allocation.
const MAX_ACTIVE_RECORD_SCAN_ENTRIES: usize = 1024;
const MAX_ACTIVE_RECORD_BYTES: u64 = 1024 * 1024;

pub(super) fn any_present(root: &Path, generation: &str) -> Result<bool, Error> {
    let entries = fs::read_dir(root).map_err(|_error| Error::Io)?;
    let mut scanned_entries = 0_usize;
    for entry in entries {
        scanned_entries = scanned_entries.checked_add(1).ok_or(Error::Platform)?;
        if scanned_entries > MAX_ACTIVE_RECORD_SCAN_ENTRIES {
            return Err(Error::Platform);
        }
        let path = entry.map_err(|_error| Error::Io)?.path();
        if entry::is_active(&path, generation)? {
            return Ok(true);
        }
    }
    Ok(false)
}
