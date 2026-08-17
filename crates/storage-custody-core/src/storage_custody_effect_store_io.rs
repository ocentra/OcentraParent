use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;

use super::StorageCustodyEffectRecord;

pub(super) fn reject_symlink(path: &Path) -> io::Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "custody effect directory must not be a symlink",
        ));
    }
    Ok(())
}

pub(super) fn read_records(path: &Path) -> io::Result<Vec<StorageCustodyEffectRecord>> {
    reject_symlink(path)?;
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error),
    }
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
