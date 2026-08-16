use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, io, path::Path};

use super::{platform, Error};

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Record {
    pub(super) family: String,
    pub(super) account: String,
    pub(super) device: String,
    pub(super) epoch_hash: String,
    pub(super) ciphertext: Vec<u8>,
}

pub(super) fn binding(parts: [&str; 4]) -> Result<Vec<u8>, Error> {
    if parts.iter().any(|part| part.trim().is_empty()) {
        return Err(Error::Invalid);
    }
    let mut output = Vec::new();
    for part in parts {
        let bytes = part.as_bytes();
        output.extend_from_slice(
            &u64::try_from(bytes.len())
                .map_err(|_error| Error::Invalid)?
                .to_be_bytes(),
        );
        output.extend_from_slice(bytes);
    }
    Ok(output)
}

pub(super) fn install_generation(root: &Path, root_was_absent: bool) -> Result<String, Error> {
    platform::load_or_rotate_install_generation(root, root_was_absent)
}

pub(super) fn install_generation_fence(root: &Path) -> Result<fs::File, Error> {
    let parent = root.parent().ok_or(Error::Invalid)?;
    fs::create_dir_all(parent).map_err(|_error| Error::Io)?;
    let canonical_parent = parent.canonicalize().map_err(|_error| Error::Io)?;
    let root_name = root.file_name().ok_or(Error::Invalid)?;
    let canonical_root = canonical_parent.join(root_name);
    let root_key = hex(Sha256::digest(canonical_root.to_string_lossy().as_bytes()));
    let lock = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(canonical_parent.join(format!(".device-trust-install-generation-{root_key}.lock")))
        .map_err(|_error| Error::Io)?;
    fs2::FileExt::lock_exclusive(&lock).map_err(|_error| Error::Io)?;
    Ok(lock)
}

pub(super) fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
pub(super) fn write(path: &Path, record: &Record) -> Result<(), Error> {
    AtomicFile::new(path, AllowOverwrite)
        .write(|file| {
            serde_json::to_writer(&mut *file, record).map_err(io::Error::other)?;
            file.sync_all()
        })
        .map_err(|_error| Error::Io)
}

pub(super) fn remove(path: &Path) -> Result<(), Error> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_error) => Err(Error::Io),
    }
}
