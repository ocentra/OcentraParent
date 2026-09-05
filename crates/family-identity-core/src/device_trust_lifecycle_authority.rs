use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use rusqlite::Connection;
use sha2::{Digest, Sha256};

use crate::{
    device_trust_lifecycle::DeviceTrustLifecycleError,
    device_trust_lifecycle_authority_fence::{self, AuthorityTransition},
    device_trust_lifecycle_authority_intent,
    device_trust_lifecycle_authority_lock::{self},
    device_trust_lifecycle_authority_reconciliation,
    device_trust_lifecycle_authority_store::{load_values, open_lock, persist_values},
};

pub(crate) struct ExternalLifecycleAuthority {
    path: PathBuf,
    intent_path: PathBuf,
    lock_path: PathBuf,
    values: BTreeMap<String, u64>,
}

impl ExternalLifecycleAuthority {
    pub(crate) fn open(database_path: &Path) -> Result<Self, DeviceTrustLifecycleError> {
        let path = database_path.with_extension("authority.json");
        let intent_path = database_path.with_extension("authority-intent.json");
        let lock_path = database_path.with_extension("authority.lock");
        let lock = open_lock(&lock_path)?;
        device_trust_lifecycle_authority_lock::lock_exclusive_bounded(&lock)?;
        let values = load_values(&path, database_path.exists()).and_then(|values| {
            device_trust_lifecycle_authority_intent::load(&intent_path)?;
            Ok(values)
        });
        let values = match values {
            Ok(values) => values,
            Err(error) => {
                let _unlock_result = fs2::FileExt::unlock(&lock);
                return Err(error);
            }
        };
        let authority = Self {
            path,
            intent_path,
            lock_path,
            values,
        };
        if !authority.path.exists() {
            if let Err(error) = authority.persist() {
                let _unlock_result = fs2::FileExt::unlock(&lock);
                return Err(error);
            }
        }
        fs2::FileExt::unlock(&lock).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Ok(authority)
    }

    pub(crate) fn begin_transition(
        &mut self,
        connection: &Connection,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        expected_generation: Option<u64>,
        target_generation: u64,
    ) -> Result<AuthorityTransition, DeviceTrustLifecycleError> {
        let (transition, values) = device_trust_lifecycle_authority_fence::begin(
            connection,
            &self.path,
            &self.intent_path,
            &self.lock_path,
            authority_key(family_id, trust_subject, device_ref),
            expected_generation,
            target_generation,
        )?;
        self.values = values;
        Ok(transition)
    }

    pub(crate) fn reconcile(
        &mut self,
        connection: &Connection,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.values = device_trust_lifecycle_authority_reconciliation::reconcile(
            connection,
            &self.path,
            &self.intent_path,
            &self.lock_path,
        )?;
        Ok(())
    }

    pub(crate) fn replace_values(&mut self, values: BTreeMap<String, u64>) {
        self.values = values;
    }

    pub(crate) fn matches(
        &self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        generation: u64,
    ) -> bool {
        device_trust_lifecycle_authority_fence::matches(
            &self.path,
            &self.intent_path,
            &self.lock_path,
            &authority_key(family_id, trust_subject, device_ref),
            generation,
        )
        .unwrap_or_default()
    }

    fn persist(&self) -> Result<(), DeviceTrustLifecycleError> {
        persist_values(&self.path, &self.values)
    }
}

pub(crate) fn authority_key(family_id: &str, trust_subject: &str, device_ref: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ocentra-device-trust-lifecycle-authority-v2\0");
    hasher.update(family_id.as_bytes());
    hasher.update([0]);
    hasher.update(trust_subject.as_bytes());
    hasher.update([0]);
    hasher.update(device_ref.as_bytes());
    hex_encode(&hasher.finalize())
}

pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}
