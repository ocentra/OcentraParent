use rusqlite::{Connection, OptionalExtension};

use crate::household_authority::HouseholdAuthorityAction;
use crate::parent_presence::{ParentPresenceChallenge, ParentPresenceObservedAt};
use crate::parent_presence_store::ParentPresenceStoreError;
use crate::parent_presence_store_schema::load_columns;
use crate::parent_presence_store_sql_shape::{
    legacy_parent_step_up_intent_table_is_canonical, parent_step_up_intent_table_is_canonical,
};
use crate::parent_step_up_ceremony::{
    RegisterLanSignerAnchorIntent, RegisterLanSignerAnchorIntentInput,
};

#[path = "parent_presence_store_schema_step_up_lifecycle.rs"]
mod lifecycle;

const TABLE: &str = "parent_step_up_intents";

const CANONICAL_TABLE_SQL: &str = r#"
CREATE TABLE parent_step_up_intents (
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

const LEGACY_COLUMNS: &[&str] = &[
    "challenge_ref",
    "nonce_ref",
    "intent_digest",
    "family_id",
    "trust_subject",
    "parent_account_id",
    "parent_device_id",
    "child_device_id",
    "installation_id",
    "pairing_id",
    "route_id",
    "signer_public_key",
    "lifecycle_generation",
    "installation_binding_generation",
    "authority_generation",
    "correlation_id",
    "expires_at",
    "lifecycle_state",
    "registration_state",
];

const LEGACY_INDEXES: &[&str] = &[
    "sqlite_autoindex_parent_step_up_intents_1",
    "sqlite_autoindex_parent_step_up_intents_2",
    "sqlite_autoindex_parent_step_up_intents_3",
];

pub(crate) fn migrate(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    let object = existing_object(connection)?;
    let Some((object_type, sql)) = object else {
        return create_missing(connection);
    };
    if object_type != "table" {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    if sql
        .as_deref()
        .is_some_and(parent_step_up_intent_table_is_canonical)
    {
        return Ok(());
    }
    if !is_legacy_shape(connection, sql.as_deref())? {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    rebuild_legacy(connection)
}

fn existing_object(
    connection: &Connection,
) -> Result<Option<(String, Option<String>)>, ParentPresenceStoreError> {
    connection
        .query_row(
            "SELECT type, sql FROM sqlite_master WHERE name = ?1",
            [TABLE],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)
}

fn create_missing(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    connection
        .execute_batch(CANONICAL_TABLE_SQL)
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)
}

fn is_legacy_shape(
    connection: &Connection,
    sql: Option<&str>,
) -> Result<bool, ParentPresenceStoreError> {
    let columns = load_columns(connection, TABLE)?;
    let columns_match = columns.len() == LEGACY_COLUMNS.len()
        && columns
            .iter()
            .zip(LEGACY_COLUMNS)
            .all(|(column, expected)| column.name == *expected);
    if !columns_match || !sql.is_some_and(legacy_parent_step_up_intent_table_is_canonical) {
        return Ok(false);
    }
    validate_legacy_objects(connection)?;
    super::validate_index_signatures(
        connection,
        TABLE,
        &[
            "challenge_ref|pk|true|false",
            "nonce_ref|u|true|false",
            "intent_digest|u|true|false",
        ],
    )?;
    validate_foreign_key(connection)?;
    Ok(true)
}

fn validate_legacy_objects(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, sql FROM sqlite_master
             WHERE tbl_name = ?1 OR (name = ?1 AND type <> 'table')",
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let rows = statement
        .query_map([TABLE], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let objects = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    super::require(
        objects
            .iter()
            .all(|(kind, _name, _sql)| kind == "table" || kind == "index"),
    )?;
    let table_count = objects
        .iter()
        .filter(|(kind, name, _sql)| kind == "table" && name == TABLE)
        .count();
    let indexes = objects
        .iter()
        .filter(|(kind, _name, _sql)| kind == "index")
        .map(|(_kind, name, _sql)| name.as_str())
        .collect::<Vec<_>>();
    super::require(table_count == 1 && indexes.len() == LEGACY_INDEXES.len())?;
    super::require(
        LEGACY_INDEXES
            .iter()
            .all(|expected| indexes.contains(expected)),
    )
}

fn rebuild_legacy(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    let migration = (|| {
        connection.execute_batch("BEGIN IMMEDIATE;")?;
        connection.execute_batch(
            CANONICAL_TABLE_SQL
                .replace(
                    "CREATE TABLE parent_step_up_intents",
                    "CREATE TABLE parent_step_up_intents_migrated",
                )
                .as_str(),
        )?;
        connection.execute_batch(
            "INSERT INTO parent_step_up_intents_migrated (
                challenge_ref, nonce_ref, intent_digest, family_id, trust_subject,
                parent_account_id, parent_device_id, child_device_id, installation_id,
                pairing_id, route_id, signer_public_key, lifecycle_generation,
                installation_binding_generation, authority_generation, correlation_id,
                expires_at, lifecycle_state, registration_state
            ) SELECT challenge_ref, nonce_ref, intent_digest, family_id, trust_subject,
                parent_account_id, parent_device_id, child_device_id, installation_id,
                pairing_id, route_id, signer_public_key, lifecycle_generation,
                installation_binding_generation, authority_generation, correlation_id,
                expires_at, lifecycle_state, registration_state
                FROM parent_step_up_intents;
            DROP TABLE parent_step_up_intents;
            ALTER TABLE parent_step_up_intents_migrated RENAME TO parent_step_up_intents;
            COMMIT;",
        )
    })();
    if migration.is_err() {
        let _ = connection.execute_batch("ROLLBACK;");
    }
    migration.map_err(|_error| ParentPresenceStoreError::IntegrityRejected)
}
pub(crate) fn validate(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    super::validate_table_properties(connection, super::INTENT_TABLE)?;
    super::require(
        super::load_columns(connection, super::INTENT_TABLE)?
            == vec![
                super::column("challenge_ref", "TEXT", true, 1),
                super::column("nonce_ref", "TEXT", true, 0),
                super::column("intent_digest", "TEXT", true, 0),
                super::column("family_id", "TEXT", true, 0),
                super::column("trust_subject", "TEXT", true, 0),
                super::column("parent_account_id", "TEXT", true, 0),
                super::column("parent_device_id", "TEXT", true, 0),
                super::column("child_device_id", "TEXT", true, 0),
                super::column("installation_id", "TEXT", true, 0),
                super::column("pairing_id", "TEXT", true, 0),
                super::column("route_id", "TEXT", true, 0),
                super::column("signer_public_key", "BLOB", true, 0),
                super::column("lifecycle_generation", "INTEGER", true, 0),
                super::column("installation_binding_generation", "INTEGER", true, 0),
                super::column("authority_generation", "INTEGER", true, 0),
                super::column("correlation_id", "TEXT", true, 0),
                super::column("expires_at", "TEXT", true, 0),
                super::column("lifecycle_state", "TEXT", true, 0),
                super::column("registration_state", "TEXT", true, 0),
                super::column("parent_presence_receipt", "TEXT", false, 0),
                super::column("credential_id", "TEXT", false, 0),
                super::column("credential_algorithm", "INTEGER", false, 0),
                super::column("credential_sign_count", "INTEGER", false, 0),
            ],
    )?;
    super::validate_index_signatures(
        connection,
        super::INTENT_TABLE,
        &[
            "challenge_ref|pk|true|false",
            "nonce_ref|u|true|false",
            "intent_digest|u|true|false",
        ],
    )?;
    let sql = super::table_sql(connection, super::INTENT_TABLE)?;
    super::require(parent_step_up_intent_table_is_canonical(&sql))?;
    validate_foreign_key(connection)
}

pub(crate) fn validate_rows(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    let mut statement = connection
        .prepare(
            "SELECT i.challenge_ref, i.nonce_ref, i.intent_digest, i.family_id,
                    i.trust_subject, i.parent_account_id, i.parent_device_id,
                    i.child_device_id, i.installation_id, i.pairing_id, i.route_id,
                    i.signer_public_key, i.lifecycle_generation,
                    i.installation_binding_generation, i.authority_generation,
                    i.correlation_id, i.expires_at, i.lifecycle_state,
                    i.registration_state, i.parent_presence_receipt, i.credential_id,
                    i.credential_algorithm, i.credential_sign_count,
                    c.challenge_json, c.privileged_action_json,
                    c.lifecycle_state, r.receipt_ref
             FROM parent_step_up_intents i
             JOIN parent_presence_challenges c ON c.challenge_ref = i.challenge_ref
             LEFT JOIN parent_presence_receipts r ON r.challenge_ref = i.challenge_ref
             ORDER BY i.challenge_ref",
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let rows = statement
        .query_map([], read_row)
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    for row in rows {
        validate_row(&row.map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?)?;
    }
    Ok(())
}

struct PersistedIntentRow {
    challenge_ref: String,
    nonce_ref: String,
    intent_digest: String,
    family_id: String,
    trust_subject: String,
    parent_account_id: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    pairing_id: String,
    route_id: String,
    signer_public_key: Vec<u8>,
    lifecycle_generation: i64,
    installation_binding_generation: i64,
    authority_generation: i64,
    correlation_id: String,
    expires_at: String,
    lifecycle_state: String,
    registration_state: String,
    parent_presence_receipt: Option<String>,
    credential_id: Option<String>,
    credential_algorithm: Option<i32>,
    credential_sign_count: Option<i64>,
    challenge_json: String,
    privileged_action_json: String,
    challenge_lifecycle_state: String,
    receipt_ref: Option<String>,
}

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PersistedIntentRow> {
    Ok(PersistedIntentRow {
        challenge_ref: row.get(0)?,
        nonce_ref: row.get(1)?,
        intent_digest: row.get(2)?,
        family_id: row.get(3)?,
        trust_subject: row.get(4)?,
        parent_account_id: row.get(5)?,
        parent_device_id: row.get(6)?,
        child_device_id: row.get(7)?,
        installation_id: row.get(8)?,
        pairing_id: row.get(9)?,
        route_id: row.get(10)?,
        signer_public_key: row.get(11)?,
        lifecycle_generation: row.get(12)?,
        installation_binding_generation: row.get(13)?,
        authority_generation: row.get(14)?,
        correlation_id: row.get(15)?,
        expires_at: row.get(16)?,
        lifecycle_state: row.get(17)?,
        registration_state: row.get(18)?,
        parent_presence_receipt: row.get(19)?,
        credential_id: row.get(20)?,
        credential_algorithm: row.get(21)?,
        credential_sign_count: row.get(22)?,
        challenge_json: row.get(23)?,
        privileged_action_json: row.get(24)?,
        challenge_lifecycle_state: row.get(25)?,
        receipt_ref: row.get(26)?,
    })
}

fn validate_row(row: &PersistedIntentRow) -> Result<(), ParentPresenceStoreError> {
    let signer_public_key: [u8; 32] = row
        .signer_public_key
        .as_slice()
        .try_into()
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let intent = RegisterLanSignerAnchorIntent::new(RegisterLanSignerAnchorIntentInput {
        family_id: row.family_id.as_str(),
        trust_subject: row.trust_subject.as_str(),
        parent_account_id: row.parent_account_id.as_str(),
        parent_device_id: row.parent_device_id.as_str(),
        child_device_id: row.child_device_id.as_str(),
        installation_id: row.installation_id.as_str(),
        pairing_id: row.pairing_id.as_str(),
        route_id: row.route_id.as_str(),
        signer_public_key: &signer_public_key,
        lifecycle_generation: generation(row.lifecycle_generation)?,
        installation_binding_generation: generation(row.installation_binding_generation)?,
        authority_generation: generation(row.authority_generation)?,
        correlation_id: row.correlation_id.as_str(),
    })
    .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    super::require(intent.intent_digest() == row.intent_digest)?;
    validate_challenge_identity(row, &intent)?;
    validate_lifecycle(row)
}

fn generation(value: i64) -> Result<u64, ParentPresenceStoreError> {
    u64::try_from(value).map_err(|_error| ParentPresenceStoreError::IntegrityRejected)
}

fn validate_challenge_identity(
    row: &PersistedIntentRow,
    _intent: &RegisterLanSignerAnchorIntent,
) -> Result<(), ParentPresenceStoreError> {
    let challenge: ParentPresenceChallenge = serde_json::from_str(&row.challenge_json)
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    let privileged_action_json = serde_json::to_string(&challenge.privileged_action)
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    super::require(
        privileged_action_json == row.privileged_action_json
            && challenge.challenge_ref == row.challenge_ref
            && challenge.nonce_ref == row.nonce_ref
            && challenge.family_id == row.family_id
            && challenge.parent_account_id == row.parent_account_id
            && challenge.privileged_action == HouseholdAuthorityAction::RegisterLanSignerAnchor
            && challenge.action_device_id == row.parent_device_id
            && challenge.action_device_child_profile_id.as_deref() == Some(row.pairing_id.as_str())
            && challenge.target_child_profile_id.as_deref() == Some(row.child_device_id.as_str())
            && challenge.expires_at == row.expires_at
            && lifecycle::challenge_lifecycle_matches(row)
            && row
                .nonce_ref
                .strip_prefix(&format!("intent:{}:", row.intent_digest))
                .is_some_and(|nonce| !nonce.is_empty())
            && ParentPresenceObservedAt::from_canonical_utc(&row.expires_at).is_ok(),
    )
}

fn validate_lifecycle(row: &PersistedIntentRow) -> Result<(), ParentPresenceStoreError> {
    match (
        row.lifecycle_state.as_str(),
        row.registration_state.as_str(),
        row.parent_presence_receipt.as_deref(),
        row.receipt_ref.as_deref(),
        row.credential_id.as_deref(),
        row.credential_algorithm,
        row.credential_sign_count,
    ) {
        ("issued", "pending", None, None, None, None, None) => Ok(()),
        (
            "consumed",
            registration,
            Some(receipt),
            Some(stored_receipt),
            Some(credential),
            Some(-8),
            Some(count),
        ) if matches!(registration, "pending" | "completed")
            && receipt == stored_receipt
            && crate::parent_presence_store_receipt::is_valid_opaque_receipt_ref(receipt)
            && !credential.is_empty()
            && credential.len() <= 512
            && credential.trim() == credential
            && credential.is_ascii()
            && credential.bytes().all(|byte| (0x21..=0x7e).contains(&byte))
            && count >= 0 =>
        {
            Ok(())
        }
        _ => Err(ParentPresenceStoreError::IntegrityRejected),
    }
}

fn validate_foreign_key(connection: &Connection) -> Result<(), ParentPresenceStoreError> {
    let actual = connection
        .query_row(
            "SELECT \"table\", \"from\", \"to\", on_delete FROM pragma_foreign_key_list(?1)",
            [super::INTENT_TABLE],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            },
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    super::require(
        actual
            == (
                super::CHALLENGE_TABLE.to_owned(),
                "challenge_ref".to_owned(),
                "challenge_ref".to_owned(),
                "RESTRICT".to_owned(),
            ),
    )
}
