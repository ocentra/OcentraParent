use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;
use ocentra_eventing::envelope::StoredEventEnvelope;
use serde::Deserialize;

use super::{
    StorageCustodyActionPlannedEvent, StorageCustodyEffectKind, StorageCustodyEffectRecord,
    StorageCustodyEffectStatus, StorageCustodyInput,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireStorageCustodyEffectRecord {
    schema_version: u16,
    operation_ref: String,
    effect_kind: StorageCustodyEffectKind,
    effect_ref: String,
    relative_path: Option<String>,
    household_id: String,
    child_profile_id: String,
    target_device_id: String,
    authority_generation: u64,
    session_generation: u64,
    custody_input: StorageCustodyInput,
    action: StorageCustodyActionPlannedEvent,
    envelope: StoredEventEnvelope,
    status: StorageCustodyEffectStatus,
    manual_required_reason: Option<String>,
    #[serde(default)]
    apply_lease_id: Option<String>,
}

impl WireStorageCustodyEffectRecord {
    fn into_record(self) -> StorageCustodyEffectRecord {
        StorageCustodyEffectRecord {
            schema_version: self.schema_version,
            operation_ref: self.operation_ref,
            effect_kind: self.effect_kind,
            effect_ref: self.effect_ref,
            relative_path: self.relative_path,
            household_id: self.household_id,
            child_profile_id: self.child_profile_id,
            target_device_id: self.target_device_id,
            authority_generation: self.authority_generation,
            session_generation: self.session_generation,
            custody_input: self.custody_input,
            action: self.action,
            envelope: self.envelope,
            status: self.status,
            manual_required_reason: self.manual_required_reason,
            apply_lease_id: self.apply_lease_id,
        }
    }
}

pub(super) fn reject_symlink(path: &Path) -> io::Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "custody effect path must not be a symlink",
        ));
    }
    Ok(())
}

pub(super) fn read_records(path: &Path) -> io::Result<Vec<StorageCustodyEffectRecord>> {
    reject_symlink(path)?;
    let records = match fs::read(path) {
        Ok(bytes) => serde_json::from_slice::<Vec<WireStorageCustodyEffectRecord>>(&bytes)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?
            .into_iter()
            .map(WireStorageCustodyEffectRecord::into_record)
            .collect(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(error),
    };
    let mut operation_refs = HashSet::with_capacity(records.len());
    for record in &records {
        record.validate_loaded()?;
        if !operation_refs.insert(record.operation_ref.as_str()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custody effect ledger contains a duplicate operation reference",
            ));
        }
    }
    Ok(records)
}

pub(super) fn write_records(path: &Path, records: &[StorageCustodyEffectRecord]) -> io::Result<()> {
    reject_symlink(path)?;
    AtomicFile::new(path, AllowOverwrite)
        .write(|file| {
            serde_json::to_writer(&mut *file, records).map_err(io::Error::other)?;
            file.sync_all()
        })
        .map_err(|error| io::Error::other(error.to_string()))?;
    sync_parent_directory(path)
}

pub(super) fn lock(path: &PathBuf) -> io::Result<std::fs::File> {
    let lock_path = path.with_extension("lock");
    reject_symlink(&lock_path)?;
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(lock_path)
}

pub(super) fn open_instance_lock(directory: &Path) -> io::Result<std::fs::File> {
    let lock_path = directory.join("storage-custody-effects.instance.lock");
    reject_symlink(&lock_path)?;
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(lock_path)?;
    file.try_lock_exclusive().map_err(|error| {
        if error.kind() == io::ErrorKind::WouldBlock {
            io::Error::new(
                io::ErrorKind::WouldBlock,
                "child storage custody service instance is already running",
            )
        } else {
            error
        }
    })?;
    Ok(file)
}

#[cfg(not(windows))]
fn sync_parent_directory(path: &Path) -> io::Result<()> {
    fs::File::open(path.parent().unwrap_or_else(|| Path::new(".")))?.sync_all()
}

#[cfg(windows)]
fn sync_parent_directory(_path: &Path) -> io::Result<()> {
    Ok(())
}

pub(super) fn unlock(file: &std::fs::File) -> io::Result<()> {
    FileExt::unlock(file)
}
