use std::{
    collections::BTreeMap,
    fmt,
    fs::{self, OpenOptions},
    io,
    path::Path,
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::device_trust_lifecycle::DeviceTrustLifecycleError;

pub(crate) fn open_lock(path: &Path) -> Result<fs::File, DeviceTrustLifecycleError> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(path)
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}

pub(crate) fn load_values(
    path: &Path,
    database_exists: bool,
) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    match fs::read_to_string(path) {
        Ok(json) => decode_values(&json),
        Err(error) if error.kind() == io::ErrorKind::NotFound && !database_exists => {
            Ok(BTreeMap::new())
        }
        Err(_error) => Err(DeviceTrustLifecycleError::Unavailable),
    }
}

pub(crate) fn persist_values(
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

fn decode_values(json: &str) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let values = UniqueAuthorityMap::deserialize(&mut deserializer)
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    deserializer
        .end()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    validate_values(values.0)
}

struct UniqueAuthorityMap(BTreeMap<String, u64>);

impl<'de> Deserialize<'de> for UniqueAuthorityMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(UniqueAuthorityMapVisitor)
    }
}

struct UniqueAuthorityMapVisitor;

impl<'de> Visitor<'de> for UniqueAuthorityMapVisitor {
    type Value = UniqueAuthorityMap;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an authority-generation object with unique keys")
    }

    fn visit_map<A>(self, mut entries: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = BTreeMap::new();
        while let Some((key, generation)) = entries.next_entry::<String, u64>()? {
            if values.insert(key, generation).is_some() {
                return Err(serde::de::Error::custom("duplicate authority key"));
            }
        }
        Ok(UniqueAuthorityMap(values))
    }
}

fn validate_values(
    values: BTreeMap<String, u64>,
) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    values
        .iter()
        .all(|(key, generation)| is_lower_hex(key, 64) && *generation > 0)
        .then_some(values)
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

pub(crate) fn is_lower_hex(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(not(windows))]
pub(crate) fn sync_parent_directory(path: &Path) -> Result<(), DeviceTrustLifecycleError> {
    fs::File::open(path.parent().unwrap_or_else(|| Path::new(".")))
        .and_then(|directory| directory.sync_all())
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}

#[cfg(windows)]
pub(crate) fn sync_parent_directory(path: &Path) -> Result<(), DeviceTrustLifecycleError> {
    use std::{fs::OpenOptions, os::windows::fs::OpenOptionsExt};

    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_WRITE_THROUGH: u32 = 0x8000_0000;
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_WRITE_THROUGH)
        .open(path.parent().unwrap_or_else(|| Path::new(".")))
        .and_then(|directory| directory.sync_all())
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}
