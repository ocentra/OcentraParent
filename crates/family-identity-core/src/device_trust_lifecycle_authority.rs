use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;
use sha2::{Digest, Sha256};

use rusqlite::OptionalExtension;

use crate::{
    device_trust_lifecycle::{DeviceTrustLifecycleError, DeviceTrustLifecycleRepository},
    trust_bootstrap::current_authority::{
        CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError,
        CurrentParentDeviceTrustAuthoritySource,
    },
};

pub(crate) struct ExternalLifecycleAuthority {
    path: PathBuf,
    lock_path: PathBuf,
    values: BTreeMap<String, u64>,
}

impl ExternalLifecycleAuthority {
    pub(crate) fn open(database_path: &Path) -> Result<Self, DeviceTrustLifecycleError> {
        let path = database_path.with_extension("authority.json");
        let lock_path = database_path.with_extension("authority.lock");
        let lock = open_lock(&lock_path)?;
        lock.lock_exclusive()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let values = load_values(&path, database_path.exists());
        let values = match values {
            Ok(values) => values,
            Err(error) => {
                let _unlock_result = FileExt::unlock(&lock);
                return Err(error);
            }
        };
        let authority = Self {
            path,
            lock_path,
            values,
        };
        if !authority.path.exists() {
            if let Err(error) = authority.persist() {
                let _unlock_result = FileExt::unlock(&lock);
                return Err(error);
            }
        }
        FileExt::unlock(&lock).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Ok(authority)
    }

    pub(crate) fn set(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        generation: u64,
    ) -> Result<(), DeviceTrustLifecycleError> {
        let lock = open_lock(&self.lock_path)?;
        lock.lock_exclusive()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let mut values = match load_values(&self.path, true) {
            Ok(values) => values,
            Err(error) => {
                let _unlock_result = FileExt::unlock(&lock);
                return Err(error);
            }
        };
        values.insert(Self::key(family_id, trust_subject, device_ref), generation);
        let result = persist_values(&self.path, &values);
        if result.is_ok() {
            self.values = values;
        }
        let unlock_result = FileExt::unlock(&lock);
        result.and_then(|_| unlock_result.map_err(|_error| DeviceTrustLifecycleError::Unavailable))
    }

    pub(crate) fn matches(
        &self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        generation: u64,
    ) -> bool {
        let lock = open_lock(&self.lock_path).ok();
        let Some(lock) = lock else {
            return false;
        };
        if lock.lock_shared().is_err() {
            return false;
        }
        let matches = load_values(&self.path, true).ok().and_then(|values| {
            values
                .get(&Self::key(family_id, trust_subject, device_ref))
                .copied()
        }) == Some(generation);
        matches && FileExt::unlock(&lock).is_ok()
    }

    fn persist(&self) -> Result<(), DeviceTrustLifecycleError> {
        persist_values(&self.path, &self.values)
    }

    fn key(family_id: &str, trust_subject: &str, device_ref: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"ocentra-device-trust-lifecycle-authority-v2\0");
        hasher.update(family_id.as_bytes());
        hasher.update([0]);
        hasher.update(trust_subject.as_bytes());
        hasher.update([0]);
        hasher.update(device_ref.as_bytes());
        hex_encode(&hasher.finalize())
    }
}

fn open_lock(path: &Path) -> Result<fs::File, DeviceTrustLifecycleError> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(path)
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}

fn load_values(
    path: &Path,
    database_exists: bool,
) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    match fs::read_to_string(path) {
        Ok(json) => {
            serde_json::from_str(&json).map_err(|_error| DeviceTrustLifecycleError::Unavailable)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound && !database_exists => {
            Ok(BTreeMap::new())
        }
        Err(_error) => Err(DeviceTrustLifecycleError::Unavailable),
    }
}

fn persist_values(
    path: &Path,
    values: &BTreeMap<String, u64>,
) -> Result<(), DeviceTrustLifecycleError> {
    AtomicFile::new(path, AllowOverwrite)
        .write(|file| {
            serde_json::to_writer(&mut *file, values).map_err(io::Error::other)?;
            file.sync_all()
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    sync_parent_directory(path)
}

#[cfg(not(windows))]
fn sync_parent_directory(path: &Path) -> Result<(), DeviceTrustLifecycleError> {
    fs::File::open(path.parent().unwrap_or_else(|| Path::new(".")))
        .and_then(|directory| directory.sync_all())
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}

#[cfg(windows)]
fn sync_parent_directory(_path: &Path) -> Result<(), DeviceTrustLifecycleError> {
    Ok(())
}

pub(crate) fn redacted_binding(family_id: &str, trust_subject: &str, device_ref: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ocentra-device-trust-lifecycle-binding-v2\0");
    hasher.update(family_id.as_bytes());
    hasher.update([0]);
    hasher.update(trust_subject.as_bytes());
    hasher.update([0]);
    hasher.update(device_ref.as_bytes());
    hex_encode(&hasher.finalize())
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

impl CurrentParentDeviceTrustAuthoritySource for DeviceTrustLifecycleRepository {
    fn current_authorized_parent_device(
        &self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
    ) -> Result<CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError> {
        let row = self
            .connection
            .query_row(
                "SELECT lifecycle_state, lifecycle_generation, installation_binding_generation, authority_generation FROM device_trust_lifecycle WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
                rusqlite::params![family_id, trust_subject, device_ref],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, i64>(3)?,
                    ))
                },
            )
            .optional()
            .ok()
            .flatten();
        let Some((state, lifecycle_generation, installation_generation, authority_generation)) =
            row
        else {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        };
        if state != "trusted" {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        }
        let authority_generation = u64::try_from(authority_generation)
            .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?;
        if !self.external_authority.matches(
            family_id,
            trust_subject,
            device_ref,
            authority_generation,
        ) {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        }
        Ok(CurrentParentDeviceTrustAuthority {
            lifecycle_generation: u64::try_from(lifecycle_generation)
                .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?,
            installation_binding_generation: u64::try_from(installation_generation)
                .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?,
        })
    }
}
