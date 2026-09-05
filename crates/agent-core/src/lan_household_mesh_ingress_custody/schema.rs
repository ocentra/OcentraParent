use std::path::Path;
use std::time::Duration;

use rusqlite::{Connection, OptionalExtension};

use super::{LanHouseholdMeshIngressCustodyError, LanHouseholdMeshIngressReceiptStore};

#[path = "schema_validation.rs"]
mod schema_validation;

const TABLE: &str = "lan_household_mesh_ingress_receipts_v2";
const MESSAGE_INDEX: &str = "lan_household_mesh_message_uq_v2";
const IDEMPOTENCY_INDEX: &str = "lan_household_mesh_idempotency_uq_v2";
const REPLAY_INDEX: &str = "lan_household_mesh_replay_uq_v2";
const USER_VERSION: i64 = 3;
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

const CREATE_TABLE: &str = "CREATE TABLE lan_household_mesh_ingress_receipts_v2 (receipt_id TEXT PRIMARY KEY NOT NULL, family_hash TEXT NOT NULL, child_device_id TEXT NOT NULL, target_device_id TEXT NOT NULL, parent_device_id TEXT NOT NULL, signer_public_key_id TEXT NOT NULL, signer_public_key_sha256 TEXT NOT NULL, message_kind TEXT NOT NULL CHECK(message_kind IN ('hello','heartbeat')), local_event_ref TEXT NOT NULL, lan_message_type TEXT NOT NULL, message_id TEXT NOT NULL, idempotency_key TEXT NOT NULL, route_id TEXT NOT NULL, nonce TEXT NOT NULL, sequence INTEGER NOT NULL CHECK(sequence > 0), payload_digest TEXT NOT NULL, install_id TEXT NOT NULL, pairing_id TEXT NOT NULL, registry_proof_digest TEXT NOT NULL, authority_generation INTEGER NOT NULL CHECK(authority_generation > 0), issued_at TEXT NOT NULL, expires_at TEXT NOT NULL, reserved_at TEXT NOT NULL) STRICT";
const CREATE_MESSAGE_INDEX: &str = "CREATE UNIQUE INDEX lan_household_mesh_message_uq_v2 ON lan_household_mesh_ingress_receipts_v2 (family_hash, child_device_id, message_id)";
const CREATE_IDEMPOTENCY_INDEX: &str = "CREATE UNIQUE INDEX lan_household_mesh_idempotency_uq_v2 ON lan_household_mesh_ingress_receipts_v2 (family_hash, child_device_id, idempotency_key)";
const CREATE_REPLAY_INDEX: &str = "CREATE UNIQUE INDEX lan_household_mesh_replay_uq_v2 ON lan_household_mesh_ingress_receipts_v2 (family_hash, child_device_id, pairing_id, authority_generation, signer_public_key_sha256, route_id, nonce, sequence)";

pub(super) fn create_store(
    path: &Path,
) -> Result<LanHouseholdMeshIngressReceiptStore, LanHouseholdMeshIngressCustodyError> {
    if path.exists() {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    let connection = Connection::open(path).map_err(storage_error)?;
    configure(&connection)?;
    connection
        .execute_batch("BEGIN IMMEDIATE;\n")
        .map_err(storage_error)?;
    connection
        .execute(CREATE_TABLE, [])
        .map_err(storage_error)?;
    connection
        .execute(CREATE_MESSAGE_INDEX, [])
        .map_err(storage_error)?;
    connection
        .execute(CREATE_IDEMPOTENCY_INDEX, [])
        .map_err(storage_error)?;
    connection
        .execute(CREATE_REPLAY_INDEX, [])
        .map_err(storage_error)?;
    super::rejection::create_schema(&connection)?;
    connection
        .pragma_update(None, "user_version", USER_VERSION)
        .map_err(storage_error)?;
    connection.execute_batch("COMMIT;").map_err(storage_error)?;
    validate(&connection)?;
    Ok(LanHouseholdMeshIngressReceiptStore { connection })
}

pub(super) fn open_store(
    path: &Path,
) -> Result<LanHouseholdMeshIngressReceiptStore, LanHouseholdMeshIngressCustodyError> {
    if !path.is_file() {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    let connection = Connection::open(path).map_err(storage_error)?;
    configure(&connection)?;
    validate(&connection)?;
    Ok(LanHouseholdMeshIngressReceiptStore { connection })
}

fn configure(connection: &Connection) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    connection
        .busy_timeout(BUSY_TIMEOUT)
        .map_err(storage_error)?;
    connection
        .execute_batch(
            "PRAGMA foreign_keys = ON; PRAGMA synchronous = FULL; PRAGMA journal_mode = WAL;",
        )
        .map_err(storage_error)
}

fn validate(connection: &Connection) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    validate_user_version(connection)?;
    schema_validation::validate_owned_schema_objects(connection)?;
    validate_table(connection)?;
    super::rejection::validate_schema(connection)?;
    validate_indexes(connection)?;
    validate_integrity(connection)
}

fn validate_user_version(
    connection: &Connection,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let user_version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::SchemaRejected)?;
    if user_version != USER_VERSION {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    Ok(())
}

fn validate_table(connection: &Connection) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let table_exists: Option<String> = connection
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name=?1",
            [TABLE],
            |row| row.get(0),
        )
        .optional()
        .map_err(storage_error)?;
    if table_exists.as_deref() != Some(TABLE) {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    let table_sql: String = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name=?1",
            [TABLE],
            |row| row.get(0),
        )
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::SchemaRejected)?;
    if normalize_sql(&table_sql) != normalize_sql(CREATE_TABLE) {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }

    let mut columns = connection
        .prepare("SELECT name, type, notnull, pk FROM pragma_table_info(?1) ORDER BY cid")
        .map_err(storage_error)?;
    let actual = columns
        .query_map([TABLE], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .map_err(storage_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(storage_error)?;
    let expected = [
        ("receipt_id", "TEXT", 1),
        ("family_hash", "TEXT", 0),
        ("child_device_id", "TEXT", 0),
        ("target_device_id", "TEXT", 0),
        ("parent_device_id", "TEXT", 0),
        ("signer_public_key_id", "TEXT", 0),
        ("signer_public_key_sha256", "TEXT", 0),
        ("message_kind", "TEXT", 0),
        ("local_event_ref", "TEXT", 0),
        ("lan_message_type", "TEXT", 0),
        ("message_id", "TEXT", 0),
        ("idempotency_key", "TEXT", 0),
        ("route_id", "TEXT", 0),
        ("nonce", "TEXT", 0),
        ("sequence", "INTEGER", 0),
        ("payload_digest", "TEXT", 0),
        ("install_id", "TEXT", 0),
        ("pairing_id", "TEXT", 0),
        ("registry_proof_digest", "TEXT", 0),
        ("authority_generation", "INTEGER", 0),
        ("issued_at", "TEXT", 0),
        ("expires_at", "TEXT", 0),
        ("reserved_at", "TEXT", 0),
    ];
    if actual.len() != expected.len()
        || actual.iter().zip(expected).any(
            |((name, kind, notnull, pk), (expected_name, expected_kind, expected_pk))| {
                name != expected_name
                    || kind != expected_kind
                    || *notnull != 1
                    || *pk != expected_pk
            },
        )
    {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    Ok(())
}

fn validate_indexes(connection: &Connection) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    for (index, create_sql, columns) in [
        (
            MESSAGE_INDEX,
            CREATE_MESSAGE_INDEX,
            &["family_hash", "child_device_id", "message_id"][..],
        ),
        (
            IDEMPOTENCY_INDEX,
            CREATE_IDEMPOTENCY_INDEX,
            &["family_hash", "child_device_id", "idempotency_key"][..],
        ),
        (
            REPLAY_INDEX,
            CREATE_REPLAY_INDEX,
            &[
                "family_hash",
                "child_device_id",
                "pairing_id",
                "authority_generation",
                "signer_public_key_sha256",
                "route_id",
                "nonce",
                "sequence",
            ][..],
        ),
    ] {
        validate_index(connection, index, create_sql, columns)?;
    }
    Ok(())
}

fn validate_index(
    connection: &Connection,
    index: &str,
    create_sql: &str,
    columns: &[&str],
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let index_sql: Option<String> = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type='index' AND name=?1",
            [index],
            |row| row.get(0),
        )
        .optional()
        .map_err(storage_error)?;
    if index_sql
        .as_deref()
        .is_none_or(|sql| normalize_sql(sql) != normalize_sql(create_sql))
    {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    let index_metadata: Option<(i64, String, i64)> = connection
        .query_row(
            "SELECT il.[unique], il.origin, il.partial FROM pragma_index_list(?1) AS il WHERE il.name=?2",
            rusqlite::params![TABLE, index],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()
        .map_err(storage_error)?;
    if index_metadata != Some((1, "c".to_string(), 0)) {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    validate_index_columns(connection, index, columns)
}

fn validate_index_columns(
    connection: &Connection,
    index: &str,
    columns: &[&str],
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_xinfo('{index}')"))
        .map_err(storage_error)?;
    let actual = statement
        .query_map([], |row| {
            let key: i64 = row.get(5)?;
            Ok((
                key,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(storage_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(storage_error)?;
    let actual_key_columns = actual
        .into_iter()
        .filter(|(key, _name, _collation)| *key == 1)
        .map(|(_key, name, collation)| (name, collation))
        .collect::<Vec<_>>();
    if actual_key_columns.len() != columns.len()
        || actual_key_columns.iter().zip(columns).any(
            |((actual_name, collation), expected_name)| {
                actual_name.as_deref() != Some(expected_name) || collation != "BINARY"
            },
        )
    {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    Ok(())
}

fn normalize_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

fn validate_integrity(connection: &Connection) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let integrity: String = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get(0))
        .map_err(|_error| LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
    if integrity != "ok" {
        return Err(LanHouseholdMeshIngressCustodyError::IntegrityRejected);
    }
    let invalid_rows: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM lan_household_mesh_ingress_receipts_v2 WHERE trim(receipt_id)='' OR trim(family_hash)='' OR trim(child_device_id)='' OR trim(target_device_id)='' OR trim(parent_device_id)='' OR length(signer_public_key_id) != 32 OR signer_public_key_id GLOB '*[^0-9a-f]*' OR length(signer_public_key_sha256) != 64 OR signer_public_key_sha256 GLOB '*[^0-9a-f]*' OR message_kind NOT IN ('hello','heartbeat') OR trim(local_event_ref)='' OR trim(lan_message_type)='' OR trim(message_id)='' OR trim(idempotency_key)='' OR trim(route_id)='' OR trim(nonce)='' OR sequence <= 0 OR length(payload_digest) != 64 OR payload_digest GLOB '*[^0-9a-f]*' OR trim(install_id)='' OR trim(pairing_id)='' OR trim(registry_proof_digest)='' OR authority_generation <= 0 OR datetime(issued_at) IS NULL OR datetime(expires_at) IS NULL OR datetime(reserved_at) IS NULL OR datetime(expires_at) <= datetime(issued_at) OR datetime(reserved_at) < datetime(issued_at) OR datetime(reserved_at) >= datetime(expires_at) OR trim(reserved_at)=''",
            [],
            |row| row.get(0),
        )
        .map_err(storage_error)?;
    if invalid_rows != 0 {
        return Err(LanHouseholdMeshIngressCustodyError::IntegrityRejected);
    }
    schema_validation::validate_timestamp_and_message_semantics(connection)?;
    super::rejection::validate_integrity(connection)
}

fn storage_error(_error: rusqlite::Error) -> LanHouseholdMeshIngressCustodyError {
    LanHouseholdMeshIngressCustodyError::StorageUnavailable
}
