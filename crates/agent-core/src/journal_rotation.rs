use std::{
    fs::{rename, File, OpenOptions},
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ACTIVITY_JOURNAL_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::journal::{
    ActivityJournalRotationPolicy, ActivityJournalStatus,
};

use crate::journal_error::JournalError;

pub fn default_rotation_policy() -> ActivityJournalRotationPolicy {
    ActivityJournalRotationPolicy {
        max_segment_bytes: constants::journal::DEFAULT_MAX_SEGMENT_BYTES,
    }
}

pub fn rotate_if_needed(
    path: &Path,
    policy: &ActivityJournalRotationPolicy,
) -> Result<(), JournalError> {
    let bytes = File::open(path)?.metadata()?.len();
    if bytes == 0 || bytes < policy.max_segment_bytes {
        return Ok(());
    }

    let rotated_path = rotated_segment_path(path, rotated_segment_count(path)? + 1);
    rename(path, rotated_path)?;
    OpenOptions::new().create(true).append(true).open(path)?;
    Ok(())
}

pub fn status_from_path(
    path: &Path,
    policy: &ActivityJournalRotationPolicy,
    entries_written: u64,
    last_entry_id: Option<String>,
) -> Result<ActivityJournalStatus, JournalError> {
    let paths = segment_paths(path)?;
    Ok(ActivityJournalStatus {
        schema_version: ACTIVITY_JOURNAL_SCHEMA_VERSION,
        encrypted: true,
        entries_written,
        bytes_written: bytes_from_paths(&paths)?,
        active_segment_id: active_segment_id(path)?,
        segment_count: paths.len() as u64,
        rotation_max_bytes: policy.max_segment_bytes,
        last_entry_id,
    })
}

pub fn segment_paths(path: &Path) -> Result<Vec<PathBuf>, JournalError> {
    let rotated_count = rotated_segment_count(path)?;
    let mut paths = Vec::new();
    for index in 1..=rotated_count {
        paths.push(rotated_segment_path(path, index));
    }
    paths.push(path.to_path_buf());
    Ok(paths)
}

pub fn active_segment_id(path: &Path) -> Result<String, JournalError> {
    Ok(segment_id_from_index(rotated_segment_count(path)? + 1))
}

fn rotated_segment_count(path: &Path) -> Result<u64, JournalError> {
    let mut count = 0;
    loop {
        let next = count + 1;
        if rotated_segment_path(path, next).try_exists()? {
            count = next;
        } else {
            return Ok(count);
        }
    }
}

fn rotated_segment_path(path: &Path, index: u64) -> PathBuf {
    let mut rotated_path = path.to_path_buf();
    let mut extension = index.to_string();
    extension.push(constants::delimiter::DOT);
    extension.push_str(constants::journal::FILE_EXTENSION);
    rotated_path.set_extension(extension);
    rotated_path
}

fn bytes_from_paths(paths: &[PathBuf]) -> Result<u64, JournalError> {
    let mut bytes = 0;
    for path in paths {
        bytes += File::open(path)?.metadata()?.len();
    }
    Ok(bytes)
}

fn segment_id_from_index(index: u64) -> String {
    let mut segment_id = String::from(constants::journal::SEGMENT_ID_PREFIX);
    segment_id.push_str(&index.to_string());
    segment_id
}
