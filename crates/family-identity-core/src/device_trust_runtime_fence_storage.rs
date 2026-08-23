use rusqlite::{Connection, OptionalExtension};

use crate::{
    device_trust_lifecycle_schema::is_lower_hex,
    device_trust_signer_registration_validation::validate_canonical_identity,
};

use super::{
    target, DeviceTrustRuntimeFenceError, DeviceTrustRuntimeFenceTarget, StoredReservation,
};

const TABLE: &str = "device_trust_runtime_fence_reservation";
const TABLE_SQL: &str = concat!(
    "CREATETABLEDEVICE_TRUST_RUNTIME_FENCE_RESERVATION(",
    "OPERATION_IDTEXTNOTNULLPRIMARYKEY,",
    "RESERVATION_REFTEXTNOTNULLUNIQUECHECK(LENGTH(RESERVATION_REF)=64),",
    "ACTION_CODEINTEGERNOTNULLCHECK(ACTION_CODEBETWEEN0AND10),",
    "FAMILY_IDTEXTNOTNULL,",
    "TRUST_SUBJECTTEXTNOTNULL,",
    "PARENT_DEVICE_IDTEXTNOTNULL,",
    "CHILD_DEVICE_IDTEXTNOTNULL,",
    "INSTALLATION_IDTEXTNOTNULL,",
    "SIGNER_KEY_IDTEXTNOTNULLCHECK(LENGTH(SIGNER_KEY_ID)=32),",
    "SIGNER_KEY_SHA256TEXTNOTNULLCHECK(LENGTH(SIGNER_KEY_SHA256)=64),",
    "LIFECYCLE_GENERATIONINTEGERNOTNULLCHECK(LIFECYCLE_GENERATION>0),",
    "INSTALLATION_BINDING_GENERATIONINTEGERNOTNULLCHECK(INSTALLATION_BINDING_GENERATION>0),",
    "AUTHORITY_GENERATIONINTEGERNOTNULLCHECK(AUTHORITY_GENERATION>0),",
    "RESERVATION_STATETEXTNOTNULLCHECK(RESERVATION_STATEIN('PREPARED','COMMITTED','ABORTED')),",
    "OUTCOME_DIGESTTEXTCHECK(OUTCOME_DIGESTISNULLORLENGTH(OUTCOME_DIGEST)=64)",
    ")STRICT"
);

pub(super) fn read_reservation(
    connection: &Connection,
    operation_id: &str,
) -> Result<Option<StoredReservation>, DeviceTrustRuntimeFenceError> {
    connection
        .query_row(
            "SELECT operation_id, reservation_ref, action_code, family_id, trust_subject,
                    parent_device_id, child_device_id, installation_id, signer_key_id,
                    signer_key_sha256, lifecycle_generation, installation_binding_generation,
                    authority_generation, reservation_state, outcome_digest
             FROM device_trust_runtime_fence_reservation WHERE operation_id = ?1",
            [operation_id],
            read_stored_reservation,
        )
        .optional()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)
}

fn read_stored_reservation(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredReservation> {
    Ok(StoredReservation {
        operation_id: row.get(0)?,
        reservation_ref: row.get(1)?,
        action_code: row.get(2)?,
        family_id: row.get(3)?,
        trust_subject: row.get(4)?,
        parent_device_id: row.get(5)?,
        child_device_id: row.get(6)?,
        installation_id: row.get(7)?,
        signer_key_id: row.get(8)?,
        signer_key_sha256: row.get(9)?,
        lifecycle_generation: row.get(10)?,
        installation_binding_generation: row.get(11)?,
        authority_generation: row.get(12)?,
        state: row.get(13)?,
        outcome_digest: row.get(14)?,
    })
}

pub(super) fn ensure_schema(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS device_trust_runtime_fence_reservation (
                operation_id TEXT NOT NULL PRIMARY KEY,
                reservation_ref TEXT NOT NULL UNIQUE CHECK (length(reservation_ref) = 64),
                action_code INTEGER NOT NULL CHECK (action_code BETWEEN 0 AND 10),
                family_id TEXT NOT NULL,
                trust_subject TEXT NOT NULL,
                parent_device_id TEXT NOT NULL,
                child_device_id TEXT NOT NULL,
                installation_id TEXT NOT NULL,
                signer_key_id TEXT NOT NULL CHECK (length(signer_key_id) = 32),
                signer_key_sha256 TEXT NOT NULL CHECK (length(signer_key_sha256) = 64),
                lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
                installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
                authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
                reservation_state TEXT NOT NULL CHECK (reservation_state IN ('prepared', 'committed', 'aborted')),
                outcome_digest TEXT CHECK (outcome_digest IS NULL OR length(outcome_digest) = 64)
            ) STRICT;",
        )
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    validate_table_sql(connection)?;
    validate_rows(connection)
}

fn validate_table_sql(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    let sql = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1 AND tbl_name = ?1",
            [TABLE],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)?;
    let compact = compact_sql(&sql);
    let with_if_not_exists = TABLE_SQL.replacen("CREATETABLE", "CREATETABLEIFNOTEXISTS", 1);
    (compact == TABLE_SQL || compact == with_if_not_exists)
        .then_some(())
        .ok_or(DeviceTrustRuntimeFenceError::Unavailable)
}

fn validate_rows(connection: &Connection) -> Result<(), DeviceTrustRuntimeFenceError> {
    let mut statement = connection
        .prepare(
            "SELECT operation_id, reservation_ref, action_code, family_id, trust_subject,
                    parent_device_id, child_device_id, installation_id, signer_key_id,
                    signer_key_sha256, lifecycle_generation, installation_binding_generation,
                    authority_generation, reservation_state, outcome_digest
             FROM device_trust_runtime_fence_reservation ORDER BY operation_id",
        )
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    let rows = statement
        .query_map([], read_stored_reservation)
        .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    for row in rows {
        let stored = row.map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
        validate_stored(&stored)?;
        target_from_stored(&stored)?;
    }
    Ok(())
}

fn validate_stored(stored: &StoredReservation) -> Result<(), DeviceTrustRuntimeFenceError> {
    validate_operation_id(&stored.operation_id)?;
    if !is_lower_hex(&stored.reservation_ref, 64)
        || !is_lower_hex(&stored.signer_key_id, 32)
        || !is_lower_hex(&stored.signer_key_sha256, 64)
        || stored.lifecycle_generation <= 0
        || stored.installation_binding_generation <= 0
        || stored.authority_generation <= 0
        || !(0..=10).contains(&stored.action_code)
        || !matches!(stored.state.as_str(), "prepared" | "committed" | "aborted")
    {
        return Err(DeviceTrustRuntimeFenceError::Unavailable);
    }
    if stored.state == "committed" {
        if stored
            .outcome_digest
            .as_deref()
            .is_none_or(|value| !is_lower_hex(value, 64))
        {
            return Err(DeviceTrustRuntimeFenceError::Unavailable);
        }
    } else if stored.outcome_digest.is_some() {
        return Err(DeviceTrustRuntimeFenceError::Unavailable);
    }
    for identity in [
        stored.family_id.as_str(),
        stored.trust_subject.as_str(),
        stored.parent_device_id.as_str(),
        stored.child_device_id.as_str(),
        stored.installation_id.as_str(),
    ] {
        validate_canonical_identity(identity)
            .map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)?;
    }
    Ok(())
}

fn target_from_stored(
    stored: &StoredReservation,
) -> Result<DeviceTrustRuntimeFenceTarget, DeviceTrustRuntimeFenceError> {
    target::from_stored(
        stored.action_code,
        &stored.family_id,
        &stored.trust_subject,
        &stored.parent_device_id,
        &stored.child_device_id,
        &stored.installation_id,
        &stored.signer_key_id,
        &stored.signer_key_sha256,
        from_sql_generation(stored.lifecycle_generation)?,
        from_sql_generation(stored.installation_binding_generation)?,
        from_sql_generation(stored.authority_generation)?,
    )
}

pub(super) fn target(
    stored: &StoredReservation,
) -> Result<DeviceTrustRuntimeFenceTarget, DeviceTrustRuntimeFenceError> {
    target_from_stored(stored)
}

fn validate_operation_id(operation_id: &str) -> Result<(), DeviceTrustRuntimeFenceError> {
    validate_canonical_identity(operation_id)
        .map_err(|_| DeviceTrustRuntimeFenceError::InvalidOperation)
}

pub(super) fn validate_operation(operation_id: &str) -> Result<(), DeviceTrustRuntimeFenceError> {
    validate_operation_id(operation_id)
}

pub(super) fn to_sql_generation(value: u64) -> Result<i64, DeviceTrustRuntimeFenceError> {
    i64::try_from(value).map_err(|_| DeviceTrustRuntimeFenceError::GenerationMismatch)
}

fn from_sql_generation(value: i64) -> Result<u64, DeviceTrustRuntimeFenceError> {
    u64::try_from(value).map_err(|_| DeviceTrustRuntimeFenceError::Unavailable)
}

fn compact_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}
