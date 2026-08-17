use std::{collections::BTreeMap, path::Path};

use fs2::FileExt;
use rusqlite::{Connection, OptionalExtension};

use crate::{
    device_trust_lifecycle::DeviceTrustLifecycleError,
    device_trust_lifecycle_authority::authority_key,
    device_trust_lifecycle_authority_intent::{self, AuthorityIntent},
    device_trust_lifecycle_authority_store::{load_values, open_lock},
};

pub(crate) fn reconcile(
    connection: &Connection,
    values_path: &Path,
    intent_path: &Path,
    lock_path: &Path,
) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    let lock = open_lock(lock_path)?;
    lock.lock_exclusive()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let result = reconcile_locked(connection, values_path, intent_path);
    let unlock_result =
        FileExt::unlock(&lock).map_err(|_error| DeviceTrustLifecycleError::Unavailable);
    result.and_then(|values| unlock_result.map(|()| values))
}

pub(crate) fn reconcile_locked(
    connection: &Connection,
    values_path: &Path,
    intent_path: &Path,
) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    let mut values = load_values(values_path, true)?;
    if let Some(intent) = device_trust_lifecycle_authority_intent::load(intent_path)? {
        recover_pending(connection, values_path, intent_path, &mut values, &intent)?;
    }
    verify_database_matches(connection, &values)?;
    Ok(values)
}

pub(crate) fn require_expected_generation(
    values: &BTreeMap<String, u64>,
    key: &str,
    expected_generation: Option<u64>,
) -> Result<(), DeviceTrustLifecycleError> {
    (values.get(key).copied() == expected_generation)
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn recover_pending(
    connection: &Connection,
    values_path: &Path,
    intent_path: &Path,
    values: &mut BTreeMap<String, u64>,
    intent: &AuthorityIntent,
) -> Result<(), DeviceTrustLifecycleError> {
    let database_generation = load_database_generations(connection)?
        .get(&intent.authority_key)
        .copied();
    let journal = load_journal_intent(connection, &intent.authority_key)?;
    match recovery_decision(intent, journal.as_ref(), database_generation)? {
        RecoveryDecision::Complete => device_trust_lifecycle_authority_intent::finalize(
            values_path,
            intent_path,
            values,
            intent,
        ),
        RecoveryDecision::Abort => abort_pending(intent_path, values, intent),
    }
}

fn recovery_decision(
    intent: &AuthorityIntent,
    journal: Option<&AuthorityIntent>,
    database_generation: Option<u64>,
) -> Result<RecoveryDecision, DeviceTrustLifecycleError> {
    match (journal == Some(intent), database_generation) {
        (true, Some(generation)) if generation == intent.target_generation => {
            Ok(RecoveryDecision::Complete)
        }
        (false, generation) if generation == intent.expected_generation => {
            Ok(RecoveryDecision::Abort)
        }
        _ => Err(DeviceTrustLifecycleError::Unavailable),
    }
}

fn abort_pending(
    intent_path: &Path,
    values: &BTreeMap<String, u64>,
    intent: &AuthorityIntent,
) -> Result<(), DeviceTrustLifecycleError> {
    require_expected_generation(values, &intent.authority_key, intent.expected_generation)?;
    device_trust_lifecycle_authority_intent::persist(intent_path, None)
}

fn verify_database_matches(
    connection: &Connection,
    values: &BTreeMap<String, u64>,
) -> Result<(), DeviceTrustLifecycleError> {
    load_database_generations(connection)?
        .eq(values)
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}

fn load_database_generations(
    connection: &Connection,
) -> Result<BTreeMap<String, u64>, DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT family_id, trust_subject, device_ref, authority_generation
             FROM device_trust_lifecycle
             ORDER BY family_id, trust_subject, device_ref",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let mut generations = BTreeMap::new();
    for row in rows {
        let (family_id, trust_subject, device_ref, generation) =
            row.map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let generation =
            u64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let prior = generations.insert(
            authority_key(&family_id, &trust_subject, &device_ref),
            generation,
        );
        if generation == 0 || prior.is_some() {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
    }
    Ok(generations)
}

fn load_journal_intent(
    connection: &Connection,
    key: &str,
) -> Result<Option<AuthorityIntent>, DeviceTrustLifecycleError> {
    let row = connection
        .query_row(
            "SELECT operation_id, from_generation, to_generation
             FROM device_trust_authority_transition
             WHERE authority_key = ?1",
            [key],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<i64>>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    row.map(|(operation_id, expected, target)| {
        AuthorityIntent::new(
            key.to_owned(),
            operation_id,
            expected.map(from_sql_generation).transpose()?,
            from_sql_generation(target)?,
        )
    })
    .transpose()
}

fn from_sql_generation(generation: i64) -> Result<u64, DeviceTrustLifecycleError> {
    u64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}

enum RecoveryDecision {
    Complete,
    Abort,
}
