use std::path::Path;
use std::time::Duration;

use rusqlite::{Connection, OpenFlags};

use crate::parent_presence_store::ParentPresenceStoreError;
use crate::parent_presence_store_file::{open_store_file_guard, StoreFileGuard};
use crate::parent_presence_store_file_creation::publish_initialized_store_if_absent;
use crate::parent_presence_store_schema_objects::{
    validate_foreign_key_rows, validate_foreign_keys_enabled, validate_schema_objects,
};
use crate::parent_presence_store_sql_shape::{
    challenge_table_is_canonical, decision_outbox_table_is_canonical, receipt_table_is_canonical,
};

#[path = "parent_presence_store_schema_runtime.rs"]
mod runtime;
#[path = "parent_presence_store_schema_step_up.rs"]
mod step_up;

pub(crate) const CHALLENGE_TABLE: &str = "parent_presence_challenges";
const DECISION_OUTBOX_TABLE: &str = "parent_presence_decision_outbox";
const RECEIPT_TABLE: &str = "parent_presence_receipts";
pub(crate) const INTENT_TABLE: &str = "parent_step_up_intents";
const NONCE_IDENTITY_INDEX: &str = "parent_presence_nonce_identity";

const INITIALIZE_PARENT_PRESENCE_STORE: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS parent_presence_challenges (
    challenge_ref TEXT PRIMARY KEY NOT NULL,
    challenge_json TEXT NOT NULL,
    privileged_action_json TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    nonce_ref TEXT NOT NULL UNIQUE,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('issued', 'consumed')
    )
) STRICT;

CREATE TABLE IF NOT EXISTS parent_presence_receipts (
    receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_ref TEXT NOT NULL UNIQUE,
    receipt_ref TEXT NOT NULL UNIQUE,
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
        ON DELETE RESTRICT
) STRICT;

CREATE TABLE IF NOT EXISTS parent_presence_decision_outbox (
    decision_id TEXT PRIMARY KEY NOT NULL,
    envelope_json TEXT NOT NULL,
    delivery_claim TEXT,
    delivery_claimed_at INTEGER,
    delivery_state TEXT NOT NULL CHECK (
        delivery_state IN ('pending', 'claimed', 'delivered')
    )
) STRICT;

CREATE UNIQUE INDEX IF NOT EXISTS parent_presence_nonce_identity
ON parent_presence_challenges(nonce_ref);

CREATE TABLE IF NOT EXISTS parent_step_up_intents (
    challenge_ref TEXT PRIMARY KEY NOT NULL,
    nonce_ref TEXT NOT NULL UNIQUE,
    intent_digest TEXT NOT NULL UNIQUE,
    family_id TEXT NOT NULL,
    trust_subject TEXT NOT NULL,
    parent_account_id TEXT NOT NULL,
    parent_device_id TEXT NOT NULL,
    child_device_id TEXT NOT NULL,
    installation_id TEXT NOT NULL,
    pairing_id TEXT NOT NULL,
    route_id TEXT NOT NULL,
    signer_public_key BLOB NOT NULL CHECK (length(signer_public_key) = 32),
    lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
    installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    correlation_id TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('issued', 'consumed')
    ),
    registration_state TEXT NOT NULL CHECK (
        registration_state IN ('pending', 'completed')
    ),
    parent_presence_receipt TEXT CHECK (
        parent_presence_receipt IS NULL OR length(parent_presence_receipt) = 64
    ),
    credential_id TEXT CHECK (credential_id IS NULL OR length(credential_id) BETWEEN 1 AND 512),
    credential_algorithm INTEGER CHECK (credential_algorithm IS NULL OR credential_algorithm = -8),
    credential_sign_count INTEGER CHECK (credential_sign_count IS NULL OR credential_sign_count >= 0),
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
        ON DELETE RESTRICT
) STRICT;
"#;

#[derive(PartialEq, Eq)]
pub(crate) struct ColumnShape {
    pub(crate) name: String,
    declared_type: String,
    not_null: bool,
    primary_key_position: i64,
    hidden_kind: i64,
}

struct IndexShape {
    name: String,
    unique: bool,
    origin: String,
    partial: bool,
    columns: Vec<String>,
}

#[derive(PartialEq, Eq)]
struct ForeignKeyShape {
    id: i64,
    sequence: i64,
    target_table: String,
    source_column: String,
    target_column: String,
    on_update: String,
    on_delete: String,
    match_kind: String,
}

pub(crate) fn open_initialized_store(
    path: &Path,
) -> Result<(Connection, StoreFileGuard), ParentPresenceStoreError> {
    publish_initialized_store_if_absent(path, initialize_temporary_store)?;
    let file_guard = open_store_file_guard(path)?;
    let connection = open_connection(path)?;
    // Legacy stores may legitimately omit the step-up intent table, so that
    // table is migrated on first open.  Validate every pre-existing core
    // object before that migration, however: malformed stores must be
    // rejected without receiving a new table or any other recovery write.
    let intent_object_exists = connection
        .query_row(
            "SELECT EXISTS(
                 SELECT 1 FROM sqlite_schema WHERE name = ?1
             )",
            [INTENT_TABLE],
            |row| row.get::<_, bool>(0),
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    if intent_object_exists {
        validate_store_schema(&connection)?;
    } else {
        validate_core_store_schema(&connection, false)?;
    }
    step_up::migrate(&connection)?;
    file_guard.validate_path_identity(path)?;
    validate_store_schema(&connection)?;
    runtime::configure_runtime_durability(&connection)?;
    file_guard.validate_path_identity(path)?;
    Ok((connection, file_guard))
}

fn initialize_temporary_store(path: &Path) -> Result<(), ParentPresenceStoreError> {
    let connection = open_connection(path)?;
    connection
        .execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = FULL;")
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    connection
        .execute_batch(INITIALIZE_PARENT_PRESENCE_STORE)
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    validate_store_schema(&connection)?;
    connection
        .execute_batch("PRAGMA optimize;")
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    connection
        .close()
        .map_err(|(_connection, _error)| ParentPresenceStoreError::Unavailable)
}

fn open_connection(path: &Path) -> Result<Connection, ParentPresenceStoreError> {
    let connection = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NOFOLLOW,
    )
    .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    connection
        .busy_timeout(Duration::from_secs(10))
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    Ok(connection)
}

pub(crate) fn validate_store_schema(
    connection: &Connection,
) -> Result<(), ParentPresenceStoreError> {
    validate_core_store_schema(connection, true)?;
    step_up::validate(connection)?;
    step_up::validate_rows(connection)?;
    Ok(())
}

fn validate_core_store_schema(
    connection: &Connection,
    include_intent_table: bool,
) -> Result<(), ParentPresenceStoreError> {
    validate_foreign_keys_enabled(connection)?;
    let mut expected_objects = vec![
        ("index", NONCE_IDENTITY_INDEX, CHALLENGE_TABLE),
        ("table", CHALLENGE_TABLE, CHALLENGE_TABLE),
        ("table", DECISION_OUTBOX_TABLE, DECISION_OUTBOX_TABLE),
        ("table", RECEIPT_TABLE, RECEIPT_TABLE),
    ];
    if include_intent_table {
        expected_objects.push(("table", INTENT_TABLE, INTENT_TABLE));
    }
    validate_schema_objects(connection, &expected_objects)?;
    validate_challenge_table(connection)?;
    validate_table_properties(connection, DECISION_OUTBOX_TABLE)?;
    require(
        load_columns(connection, DECISION_OUTBOX_TABLE)?
            == vec![
                column("decision_id", "TEXT", true, 1),
                column("envelope_json", "TEXT", true, 0),
                column("delivery_claim", "TEXT", false, 0),
                column("delivery_claimed_at", "INTEGER", false, 0),
                column("delivery_state", "TEXT", true, 0),
            ],
    )?;
    validate_index_signatures(
        connection,
        DECISION_OUTBOX_TABLE,
        &["decision_id|pk|true|false"],
    )?;
    let outbox_sql = table_sql(connection, DECISION_OUTBOX_TABLE)?;
    require(decision_outbox_table_is_canonical(&outbox_sql))?;
    validate_receipt_table(connection)?;
    validate_receipt_foreign_key(connection)?;
    validate_foreign_key_rows(connection)
}

fn validate_challenge_table(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    validate_table_properties(connection, CHALLENGE_TABLE)?;
    require(
        load_columns(connection, CHALLENGE_TABLE)?
            == vec![
                column("challenge_ref", "TEXT", true, 1),
                column("challenge_json", "TEXT", true, 0),
                column("privileged_action_json", "TEXT", true, 0),
                column("expires_at", "TEXT", true, 0),
                column("nonce_ref", "TEXT", true, 0),
                column("lifecycle_state", "TEXT", true, 0),
            ],
    )?;
    validate_index_signatures(
        connection,
        CHALLENGE_TABLE,
        &[
            "challenge_ref|pk|true|false",
            "nonce_ref|c|true|false",
            "nonce_ref|u|true|false",
        ],
    )?;
    validate_named_nonce_index(connection)?;
    let table_sql = table_sql(connection, CHALLENGE_TABLE)?;
    require(challenge_table_is_canonical(&table_sql))
}

fn validate_receipt_table(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    validate_table_properties(connection, RECEIPT_TABLE)?;
    require(
        load_columns(connection, RECEIPT_TABLE)?
            == vec![
                column("receipt_sequence", "INTEGER", false, 1),
                column("challenge_ref", "TEXT", true, 0),
                column("receipt_ref", "TEXT", true, 0),
            ],
    )?;
    validate_index_signatures(
        connection,
        RECEIPT_TABLE,
        &["challenge_ref|u|true|false", "receipt_ref|u|true|false"],
    )?;
    let table_sql = table_sql(connection, RECEIPT_TABLE)?;
    require(receipt_table_is_canonical(&table_sql))
}

pub(crate) fn validate_table_properties(
    connection: &Connection,
    table_name: &str,
) -> Result<(), ParentPresenceStoreError> {
    let properties = connection
        .query_row(
            "SELECT wr, strict FROM pragma_table_list WHERE schema = 'main' AND name = ?1 AND type = 'table'",
            [table_name],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    require(properties == (0, 1))
}

pub(crate) fn load_columns(
    connection: &Connection,
    table_name: &str,
) -> Result<Vec<ColumnShape>, ParentPresenceStoreError> {
    let mut statement = connection
        .prepare(
            "SELECT name, type, \"notnull\", pk, hidden FROM pragma_table_xinfo(?1) ORDER BY cid",
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let rows = statement
        .query_map([table_name], |row| {
            Ok(ColumnShape {
                name: row.get(0)?,
                declared_type: row.get(1)?,
                not_null: row.get::<_, i64>(2)? == 1,
                primary_key_position: row.get(3)?,
                hidden_kind: row.get(4)?,
            })
        })
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)
}

pub(crate) fn validate_index_signatures(
    connection: &Connection,
    table_name: &str,
    expected: &[&str],
) -> Result<(), ParentPresenceStoreError> {
    let mut actual = load_indexes(connection, table_name)?
        .iter()
        .map(index_signature)
        .collect::<Vec<_>>();
    actual.sort();
    let mut expected = expected
        .iter()
        .map(|signature| (*signature).to_owned())
        .collect::<Vec<_>>();
    expected.sort();
    require(actual == expected)
}

fn load_indexes(
    connection: &Connection,
    table_name: &str,
) -> Result<Vec<IndexShape>, ParentPresenceStoreError> {
    let index_rows = {
        let mut statement = connection
            .prepare(
                "SELECT name, \"unique\", origin, partial FROM pragma_index_list(?1) ORDER BY name",
            )
            .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
        let rows = statement
            .query_map([table_name], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)? == 1,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)? == 1,
                ))
            })
            .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?
    };
    index_rows
        .into_iter()
        .map(|(name, unique, origin, partial)| {
            let columns = load_index_columns(connection, &name)?;
            Ok(IndexShape {
                name,
                unique,
                origin,
                partial,
                columns,
            })
        })
        .collect()
}

fn load_index_columns(
    connection: &Connection,
    index_name: &str,
) -> Result<Vec<String>, ParentPresenceStoreError> {
    let mut statement = connection
        .prepare("SELECT name FROM pragma_index_info(?1) ORDER BY seqno")
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let rows = statement
        .query_map([index_name], |row| row.get::<_, String>(0))
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)
}

fn index_signature(index: &IndexShape) -> String {
    format!(
        "{}|{}|{}|{}",
        index.columns.join(","),
        index.origin,
        index.unique,
        index.partial
    )
}

fn validate_named_nonce_index(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    let indexes = load_indexes(connection, CHALLENGE_TABLE)?;
    let named = indexes
        .iter()
        .find(|index| index.name == NONCE_IDENTITY_INDEX);
    require(matches!(
        named,
        Some(index)
            if index.unique
                && !index.partial
                && index.origin == "c"
                && index.columns == ["nonce_ref"]
    ))
}

fn validate_receipt_foreign_key(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    let mut statement = connection
        .prepare(
            "SELECT id, seq, \"table\", \"from\", \"to\", on_update, on_delete, \"match\" FROM pragma_foreign_key_list(?1) ORDER BY id, seq",
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let rows = statement
        .query_map([RECEIPT_TABLE], |row| {
            Ok(ForeignKeyShape {
                id: row.get(0)?,
                sequence: row.get(1)?,
                target_table: row.get(2)?,
                source_column: row.get(3)?,
                target_column: row.get(4)?,
                on_update: row.get(5)?,
                on_delete: row.get(6)?,
                match_kind: row.get(7)?,
            })
        })
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let actual = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    require(
        actual
            == vec![ForeignKeyShape {
                id: 0,
                sequence: 0,
                target_table: CHALLENGE_TABLE.to_owned(),
                source_column: "challenge_ref".to_owned(),
                target_column: "challenge_ref".to_owned(),
                on_update: "NO ACTION".to_owned(),
                on_delete: "RESTRICT".to_owned(),
                match_kind: "NONE".to_owned(),
            }],
    )
}

pub(crate) fn table_sql(
    connection: &Connection,
    table_name: &str,
) -> Result<String, ParentPresenceStoreError> {
    connection
        .query_row(
            "SELECT sql FROM sqlite_schema WHERE type = 'table' AND name = ?1",
            [table_name],
            |row| row.get::<_, String>(0),
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)
}

pub(crate) fn column(
    name: &str,
    declared_type: &str,
    not_null: bool,
    primary_key_position: i64,
) -> ColumnShape {
    ColumnShape {
        name: name.to_owned(),
        declared_type: declared_type.to_owned(),
        not_null,
        primary_key_position,
        hidden_kind: 0,
    }
}

pub(crate) fn require(condition: bool) -> Result<(), ParentPresenceStoreError> {
    if condition {
        Ok(())
    } else {
        Err(ParentPresenceStoreError::IntegrityRejected)
    }
}
