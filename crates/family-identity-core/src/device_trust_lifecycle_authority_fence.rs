use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use fs2::FileExt;
use rusqlite::{params, Connection, Transaction};

use crate::{
    device_trust_lifecycle::DeviceTrustLifecycleError,
    device_trust_lifecycle_authority_intent::{self, AuthorityIntent},
    device_trust_lifecycle_authority_lock,
    device_trust_lifecycle_authority_reconciliation::{
        reconcile_locked, require_expected_generation,
    },
    device_trust_lifecycle_authority_store::open_lock,
    device_trust_signer_registration_validation::random_receipt,
};

pub(crate) struct AuthorityTransition {
    lock: Option<File>,
    values_path: PathBuf,
    intent_path: PathBuf,
    intent: AuthorityIntent,
}

impl AuthorityTransition {
    pub(crate) fn record(
        &self,
        transaction: &Transaction<'_>,
    ) -> Result<(), DeviceTrustLifecycleError> {
        let expected_generation = self
            .intent
            .expected_generation
            .map(to_sql_generation)
            .transpose()?;
        let changed = transaction
            .execute(
                "INSERT INTO device_trust_authority_transition
                 (authority_key, operation_id, from_generation, to_generation)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(authority_key) DO UPDATE SET
                    operation_id = excluded.operation_id,
                    from_generation = excluded.from_generation,
                    to_generation = excluded.to_generation",
                params![
                    self.intent.authority_key,
                    self.intent.operation_id,
                    expected_generation,
                    to_sql_generation(self.intent.target_generation)?,
                ],
            )
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        (changed == 1)
            .then_some(())
            .ok_or(DeviceTrustLifecycleError::Unavailable)
    }

    pub(crate) fn complete(mut self) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
        let result = device_trust_lifecycle_authority_intent::complete(
            &self.values_path,
            &self.intent_path,
            &self.intent,
        );
        self.finish_with_unlock(result)
    }

    pub(crate) fn reconcile_after_database_error(
        mut self,
        connection: &Connection,
    ) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
        let result = reconcile_locked(connection, &self.values_path, &self.intent_path);
        self.finish_with_unlock(result)
    }

    fn finish_with_unlock<T>(
        &mut self,
        result: Result<T, DeviceTrustLifecycleError>,
    ) -> Result<T, DeviceTrustLifecycleError> {
        let unlock_result = self.release_lock();
        match (result, unlock_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (Ok(_value), Err(error)) => Err(error),
        }
    }

    fn release_lock(&mut self) -> Result<(), DeviceTrustLifecycleError> {
        let Some(lock) = self.lock.take() else {
            return Ok(());
        };
        FileExt::unlock(&lock).map_err(|_error| DeviceTrustLifecycleError::Unavailable)
    }
}

impl Drop for AuthorityTransition {
    fn drop(&mut self) {
        let _unlock_result = self.release_lock();
    }
}

pub(crate) fn begin(
    connection: &Connection,
    values_path: &Path,
    intent_path: &Path,
    lock_path: &Path,
    key: String,
    expected_generation: Option<u64>,
    target_generation: u64,
) -> Result<(AuthorityTransition, BTreeMap<String, u64>), DeviceTrustLifecycleError> {
    let intent = AuthorityIntent::new(
        key,
        random_receipt()?,
        expected_generation,
        target_generation,
    )?;
    let lock = open_lock(lock_path)?;
    device_trust_lifecycle_authority_lock::lock_exclusive_bounded(&lock)?;
    let preparation = (|| {
        let values = reconcile_locked(connection, values_path, intent_path)?;
        require_expected_generation(&values, &intent.authority_key, intent.expected_generation)?;
        device_trust_lifecycle_authority_intent::persist(intent_path, Some(&intent))?;
        Ok(values)
    })();
    match preparation {
        Ok(values) => Ok((
            AuthorityTransition {
                lock: Some(lock),
                values_path: values_path.to_owned(),
                intent_path: intent_path.to_owned(),
                intent,
            },
            values,
        )),
        Err(error) => {
            let _unlock_result = FileExt::unlock(&lock);
            Err(error)
        }
    }
}

pub(crate) fn matches(
    values_path: &Path,
    intent_path: &Path,
    lock_path: &Path,
    key: &str,
    generation: u64,
) -> Result<bool, DeviceTrustLifecycleError> {
    device_trust_lifecycle_authority_lock::matches(
        values_path,
        intent_path,
        lock_path,
        key,
        generation,
    )
}

fn to_sql_generation(generation: u64) -> Result<i64, DeviceTrustLifecycleError> {
    i64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}
