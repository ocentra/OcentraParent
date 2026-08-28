use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};
use std::os::windows::fs::MetadataExt;

use super::{
    active_record,
    record::{binding, hex, Record},
    Error,
};

// Recovery inspection is fail-closed and bounded so a hostile custody root cannot
// turn startup into an unbounded directory walk or allocation.
const MAX_ACTIVE_RECORD_SCAN_ENTRIES: usize = 1024;
const MAX_ACTIVE_RECORD_BYTES: u64 = 1024 * 1024;

pub(super) fn any_present(root: &Path, generation: &str) -> Result<bool, Error> {
    let entries = fs::read_dir(root).map_err(|_error| Error::Io)?;
    let mut scanned_entries = 0_usize;
    for entry in entries {
        scanned_entries = scanned_entries
            .checked_add(1)
            .ok_or(Error::Platform)?;
        if scanned_entries > MAX_ACTIVE_RECORD_SCAN_ENTRIES {
            return Err(Error::Platform);
        }
        let entry = entry.map_err(|_error| Error::Io)?;
        let path = entry.path();
        if path
            .extension()
            .is_none_or(|extension| extension != "sealed")
        {
            continue;
        }
        let metadata = fs::symlink_metadata(&path).map_err(|_error| Error::Io)?;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
        {
            return Err(Error::Platform);
        }
        let mut encoded = Vec::new();
        let read_result = fs::File::open(&path).and_then(|mut file| {
            file.by_ref()
                .take(MAX_ACTIVE_RECORD_BYTES.saturating_add(1))
                .read_to_end(&mut encoded)
        });
        let encoded = match read_result {
            Ok(_) if encoded.len() as u64 > MAX_ACTIVE_RECORD_BYTES => {
                return Err(Error::Platform)
            }
            Ok(_) => encoded,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_error) => return Err(Error::Io),
        };
        let record = match serde_json::from_slice::<Record>(&encoded) {
            Ok(record) => record,
            Err(_error) => continue,
        };
        let binding = binding([&record.family, &record.account, &record.device, generation])?;
        let expected_name = format!("{}.sealed", hex(Sha256::digest(&binding)));
        if path.file_name().and_then(|name| name.to_str()) != Some(expected_name.as_str()) {
            continue;
        }
        if active_record::record_is_active(&record, &binding)? {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::{any_present, MAX_ACTIVE_RECORD_SCAN_ENTRIES};
    use std::fs;

    #[test]
    fn over_bound_startup_scan_fails_closed() -> Result<(), String> {
        let root = std::env::temp_dir().join(format!(
            "ocentra-wp02-record-scan-bound-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).map_err(|error| format!("create scan root: {error}"))?;
        for index in 0..=MAX_ACTIVE_RECORD_SCAN_ENTRIES {
            fs::write(root.join(format!("{index}.sealed")), b"{}").map_err(|error| {
                format!("write bounded-scan fixture {index}: {error}")
            })?;
        }

        assert_eq!(
            any_present(&root, "generation"),
            Err(super::Error::Platform)
        );

        let _cleanup = fs::remove_dir_all(root);
        Ok(())
    }
}
