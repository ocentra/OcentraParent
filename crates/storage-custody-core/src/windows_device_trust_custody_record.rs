use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

use super::{platform, Error};

#[derive(Serialize, Deserialize)]
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

pub(super) fn install_generation(
    root: &Path,
    root_was_absent: bool,
    sealed_content_present: bool,
) -> Result<String, Error> {
    let lock = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(root.join("device-trust-install-generation.lock"))
        .map_err(|_error| Error::Io)?;
    fs2::FileExt::lock_exclusive(&lock).map_err(|_error| Error::Io)?;
    platform::load_or_rotate_install_generation(root, root_was_absent, sealed_content_present)
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
