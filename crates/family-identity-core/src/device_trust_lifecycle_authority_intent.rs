use std::{collections::BTreeMap, fs, io, path::Path};

use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::{Deserialize, Serialize};

use crate::{
    device_trust_lifecycle::DeviceTrustLifecycleError,
    device_trust_lifecycle_authority_store::{
        is_lower_hex, load_values, persist_values, sync_parent_directory,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AuthorityIntent {
    pub(crate) authority_key: String,
    pub(crate) operation_id: String,
    pub(crate) expected_generation: Option<u64>,
    pub(crate) target_generation: u64,
}

impl AuthorityIntent {
    pub(crate) fn new(
        authority_key: String,
        operation_id: String,
        expected_generation: Option<u64>,
        target_generation: u64,
    ) -> Result<Self, DeviceTrustLifecycleError> {
        let intent = Self {
            authority_key,
            operation_id,
            expected_generation,
            target_generation,
        };
        intent.validate()?;
        Ok(intent)
    }

    fn validate(&self) -> Result<(), DeviceTrustLifecycleError> {
        let generation_is_next = match self.expected_generation {
            None => self.target_generation == 1,
            Some(expected) => {
                expected > 0 && expected.checked_add(1) == Some(self.target_generation)
            }
        };
        (is_lower_hex(&self.authority_key, 64)
            && is_lower_hex(&self.operation_id, 64)
            && generation_is_next)
            .then_some(())
            .ok_or(DeviceTrustLifecycleError::Unavailable)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AuthorityIntentDocument {
    pending: Option<AuthorityIntent>,
}

pub(crate) fn load(path: &Path) -> Result<Option<AuthorityIntent>, DeviceTrustLifecycleError> {
    let json = match fs::read_to_string(path) {
        Ok(json) => json,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_error) => return Err(DeviceTrustLifecycleError::Unavailable),
    };
    let mut deserializer = serde_json::Deserializer::from_str(&json);
    let document = AuthorityIntentDocument::deserialize(&mut deserializer)
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    deserializer
        .end()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    document
        .pending
        .as_ref()
        .map(AuthorityIntent::validate)
        .transpose()?;
    Ok(document.pending)
}

pub(crate) fn persist(
    path: &Path,
    intent: Option<&AuthorityIntent>,
) -> Result<(), DeviceTrustLifecycleError> {
    let document = AuthorityIntentDocument {
        pending: intent.cloned(),
    };
    AtomicFile::new(path, AllowOverwrite)
        .write(|file| {
            serde_json::to_writer(&mut *file, &document).map_err(io::Error::other)?;
            file.sync_all()
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    sync_parent_directory(path)
}

pub(crate) fn complete(
    values_path: &Path,
    intent_path: &Path,
    intent: &AuthorityIntent,
) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    if load(intent_path)?.as_ref() != Some(intent) {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    let mut values = load_values(values_path, true)?;
    finalize(values_path, intent_path, &mut values, intent)?;
    Ok(values)
}

pub(crate) fn finalize(
    values_path: &Path,
    intent_path: &Path,
    values: &mut BTreeMap<String, u64>,
    intent: &AuthorityIntent,
) -> Result<(), DeviceTrustLifecycleError> {
    match values.get(&intent.authority_key).copied() {
        current if current == intent.expected_generation => {
            values.insert(intent.authority_key.clone(), intent.target_generation);
            persist_values(values_path, values)?;
        }
        Some(current) if current == intent.target_generation => {}
        _ => return Err(DeviceTrustLifecycleError::Unavailable),
    }
    persist(intent_path, None)
}
