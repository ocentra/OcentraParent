use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

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
    values: BTreeMap<String, u64>,
}

impl ExternalLifecycleAuthority {
    pub(crate) fn open(database_path: &Path) -> Result<Self, DeviceTrustLifecycleError> {
        let path = database_path.with_extension("authority.json");
        let values = if path.exists() {
            let json = fs::read_to_string(&path)
                .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
            serde_json::from_str(&json).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        } else if database_path.exists() {
            return Err(DeviceTrustLifecycleError::Unavailable);
        } else {
            BTreeMap::new()
        };
        let authority = Self { path, values };
        if !authority.path.exists() {
            authority.persist()?;
        }
        Ok(authority)
    }

    pub(crate) fn set(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        generation: u64,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.values
            .insert(Self::key(family_id, trust_subject, device_ref), generation);
        self.persist()
    }

    pub(crate) fn matches(
        &self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        generation: u64,
    ) -> bool {
        self.read_values().and_then(|values| {
            values
                .get(&Self::key(family_id, trust_subject, device_ref))
                .copied()
        }) == Some(generation)
    }

    fn read_values(&self) -> Option<BTreeMap<String, u64>> {
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
    }

    fn persist(&self) -> Result<(), DeviceTrustLifecycleError> {
        let json = serde_json::to_vec(&self.values)
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        fs::write(&self.path, json).map_err(|_error| DeviceTrustLifecycleError::Unavailable)
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
