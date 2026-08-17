use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io,
    path::Path,
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;

use super::{RetentionDeleteOutboxRecord, RetentionDeleteTombstoneStore};

pub(super) fn records(
    store: &RetentionDeleteTombstoneStore,
) -> io::Result<Vec<RetentionDeleteOutboxRecord>> {
    super::path::reject_symlink(&store.path, "custody tombstone record")?;
    match fs::read(&store.path) {
        Ok(bytes) => {
            let values: Vec<serde_json::Value> = serde_json::from_slice(&bytes)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            let records = values
                .into_iter()
                .map(RetentionDeleteOutboxRecord::decode)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            validate_records(records)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error),
    }
}

fn validate_records(
    records: Vec<RetentionDeleteOutboxRecord>,
) -> io::Result<Vec<RetentionDeleteOutboxRecord>> {
    let mut deletion_refs = HashSet::with_capacity(records.len());
    for record in &records {
        record.validate_loaded()?;
        if !deletion_refs.insert(record.deletion_ref.as_str()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custody tombstone store contains a duplicate deletion reference",
            ));
        }
    }
    Ok(records)
}

pub(super) fn write(
    store: &RetentionDeleteTombstoneStore,
    records: &[RetentionDeleteOutboxRecord],
) -> io::Result<()> {
    super::path::reject_symlink(&store.path, "custody tombstone record")?;
    let encoded = records
        .iter()
        .map(RetentionDeleteOutboxRecord::encode)
        .collect::<Result<Vec<_>, _>>()
        .map_err(io::Error::other)?;
    AtomicFile::new(&store.path, AllowOverwrite)
        .write(|file| {
            serde_json::to_writer(&mut *file, &encoded).map_err(io::Error::other)?;
            file.sync_all()
        })
        .map_err(|error| io::Error::other(error.to_string()))?;
    sync_parent_directory(&store.path)
}

pub(super) fn lock(store: &RetentionDeleteTombstoneStore) -> io::Result<std::fs::File> {
    let lock_path = store.path.with_extension("lock");
    super::path::reject_symlink(&lock_path, "custody tombstone lock")?;
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(lock_path)
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
