use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};

use super::household_authority_runtime_cas_recovery::HouseholdAuthorityRuntimeCasError;
use super::{HouseholdAuthorityAction, HouseholdAuthorityRuntimeEffectTarget};
use crate::device_trust_lifecycle::DeviceTrustLifecycleState;

const CLOCK_TABLE: &str = "household_authority_runtime_cas_clock";
const EFFECT_TABLE: &str = "household_authority_runtime_effect";
const STATUS_INDEX: &str = "household_authority_runtime_effect_status";

pub(super) const SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS household_authority_runtime_cas_clock (
    clock_id INTEGER PRIMARY KEY CHECK (clock_id = 1),
    last_epoch_millis INTEGER NOT NULL CHECK (last_epoch_millis > 0)
) STRICT;
CREATE TABLE IF NOT EXISTS household_authority_runtime_effect (
    operation_digest BLOB NOT NULL CHECK (length(operation_digest) = 32),
    consumption_nonce BLOB NOT NULL CHECK (length(consumption_nonce) = 32),
    target_digest BLOB NOT NULL CHECK (length(target_digest) = 32),
    action INTEGER NOT NULL CHECK (action BETWEEN 1 AND 11),
    provider INTEGER NOT NULL CHECK (provider IN (1, 2)),
    device_state INTEGER NOT NULL CHECK (device_state BETWEEN 1 AND 4),
    account_authority_generation INTEGER NOT NULL CHECK (account_authority_generation > 0),
    session_generation INTEGER NOT NULL CHECK (session_generation > 0),
    device_authority_generation INTEGER NOT NULL CHECK (device_authority_generation > 0),
    capability_authority_generation INTEGER,
    capability_expires_at_epoch_millis INTEGER,
    capability_revocation_epoch INTEGER,
    controller_lease_authority_generation INTEGER,
    controller_lease_expires_at_epoch_millis INTEGER,
    controller_lease_revocation_epoch INTEGER,
    parent_step_up_authority_generation INTEGER,
    parent_step_up_expires_at_epoch_millis INTEGER,
    parent_step_up_receipt_epoch INTEGER,
    status TEXT NOT NULL CHECK (status IN ('prepared', 'reserved', 'committed', 'aborted', 'ambiguous')),
    handoff_digest BLOB CHECK (handoff_digest IS NULL OR length(handoff_digest) = 32),
    created_at_epoch_millis INTEGER NOT NULL CHECK (created_at_epoch_millis > 0),
    updated_at_epoch_millis INTEGER NOT NULL CHECK (updated_at_epoch_millis >= created_at_epoch_millis),
    recovery_epoch INTEGER NOT NULL CHECK (recovery_epoch > 0),
    PRIMARY KEY (operation_digest),
    UNIQUE (consumption_nonce),
    CHECK (
        (capability_authority_generation IS NULL
            AND capability_expires_at_epoch_millis IS NULL
            AND capability_revocation_epoch IS NULL)
        OR (capability_authority_generation > 0
            AND capability_expires_at_epoch_millis > 0
            AND capability_revocation_epoch > 0)
    ),
    CHECK (
        (controller_lease_authority_generation IS NULL
            AND controller_lease_expires_at_epoch_millis IS NULL
            AND controller_lease_revocation_epoch IS NULL)
        OR (controller_lease_authority_generation > 0
            AND controller_lease_expires_at_epoch_millis > 0
            AND controller_lease_revocation_epoch > 0)
    ),
    CHECK (
        (parent_step_up_authority_generation IS NULL
            AND parent_step_up_expires_at_epoch_millis IS NULL
            AND parent_step_up_receipt_epoch IS NULL)
        OR (parent_step_up_authority_generation > 0
            AND parent_step_up_expires_at_epoch_millis > 0
            AND parent_step_up_receipt_epoch > 0)
    ),
    CHECK (
        (status = 'committed' AND handoff_digest IS NOT NULL)
        OR (status <> 'committed' AND handoff_digest IS NULL)
    )
) STRICT;
CREATE INDEX IF NOT EXISTS household_authority_runtime_effect_status
    ON household_authority_runtime_effect(status, updated_at_epoch_millis);
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TargetRecord {
    pub(super) action: i64,
    pub(super) provider: i64,
    pub(super) device_state: i64,
    pub(super) account_authority_generation: i64,
    pub(super) session_generation: i64,
    pub(super) device_authority_generation: i64,
    pub(super) capability_authority_generation: Option<i64>,
    pub(super) capability_expires_at_epoch_millis: Option<i64>,
    pub(super) capability_revocation_epoch: Option<i64>,
    pub(super) controller_lease_authority_generation: Option<i64>,
    pub(super) controller_lease_expires_at_epoch_millis: Option<i64>,
    pub(super) controller_lease_revocation_epoch: Option<i64>,
    pub(super) parent_step_up_authority_generation: Option<i64>,
    pub(super) parent_step_up_expires_at_epoch_millis: Option<i64>,
    pub(super) parent_step_up_receipt_epoch: Option<i64>,
}

pub(super) fn install(connection: &Connection, initial_epoch_millis: i64) -> Result<(), ()> {
    if initial_epoch_millis <= 0 {
        return Err(());
    }
    connection.execute_batch(SCHEMA_SQL).map_err(|_| ())?;
    connection
        .execute(
            "INSERT OR IGNORE INTO household_authority_runtime_cas_clock
             (clock_id, last_epoch_millis) VALUES (1, ?1)",
            params![initial_epoch_millis],
        )
        .map_err(|_| ())?;
    Ok(())
}

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    require_pragma_ok(connection, "PRAGMA integrity_check")?;
    require_no_foreign_key_violations(connection)?;
    validate_objects(connection)?;
    validate_columns(connection)?;
    validate_indexes(connection)?;
    validate_rows(connection)
}

pub(super) fn target_record(
    target: &HouseholdAuthorityRuntimeEffectTarget,
) -> Result<TargetRecord, HouseholdAuthorityRuntimeCasError> {
    let capability = optional_record(
        target.capability_authority_generation,
        target.capability_expires_at.clone(),
        target.capability_revocation_epoch,
    )?;
    let controller_lease = optional_record(
        target.controller_lease_authority_generation,
        target.controller_lease_expires_at.clone(),
        target.controller_lease_revocation_epoch,
    )?;
    let parent_step_up = optional_record(
        target.parent_step_up_authority_generation,
        target.parent_step_up_expires_at.clone(),
        target.parent_step_up_receipt_epoch,
    )?;
    let account_authority_generation = to_sql_generation(target.account_authority_generation)?;
    let session_generation = to_sql_generation(target.session_generation)?;
    let device_authority_generation = to_sql_generation(target.device_authority_generation)?;
    Ok(TargetRecord {
        action: action_code(target.action)?,
        provider: provider_code(&target.provider)?,
        device_state: device_state_code(target.device_state)?,
        account_authority_generation,
        session_generation,
        device_authority_generation,
        capability_authority_generation: capability.map(|value| value.0),
        capability_expires_at_epoch_millis: capability.map(|value| value.1),
        capability_revocation_epoch: capability.map(|value| value.2),
        controller_lease_authority_generation: controller_lease.map(|value| value.0),
        controller_lease_expires_at_epoch_millis: controller_lease.map(|value| value.1),
        controller_lease_revocation_epoch: controller_lease.map(|value| value.2),
        parent_step_up_authority_generation: parent_step_up.map(|value| value.0),
        parent_step_up_expires_at_epoch_millis: parent_step_up.map(|value| value.1),
        parent_step_up_receipt_epoch: parent_step_up.map(|value| value.2),
    })
}

pub(super) fn target_digest(
    target: &HouseholdAuthorityRuntimeEffectTarget,
) -> Result<[u8; 32], HouseholdAuthorityRuntimeCasError> {
    let record = target_record(target)?;
    let mut digest = Sha256::new();
    digest.update(b"ocentra-account-household-runtime-effect-target-v1");
    append_i64(&mut digest, record.action);
    append_i64(&mut digest, record.provider);
    append_i64(&mut digest, record.device_state);
    append_i64(&mut digest, record.account_authority_generation);
    append_i64(&mut digest, record.session_generation);
    append_i64(&mut digest, record.device_authority_generation);
    append_optional_i64(&mut digest, record.capability_authority_generation);
    append_optional_i64(&mut digest, record.capability_expires_at_epoch_millis);
    append_optional_i64(&mut digest, record.capability_revocation_epoch);
    append_optional_i64(&mut digest, record.controller_lease_authority_generation);
    append_optional_i64(&mut digest, record.controller_lease_expires_at_epoch_millis);
    append_optional_i64(&mut digest, record.controller_lease_revocation_epoch);
    append_optional_i64(&mut digest, record.parent_step_up_authority_generation);
    append_optional_i64(&mut digest, record.parent_step_up_expires_at_epoch_millis);
    append_optional_i64(&mut digest, record.parent_step_up_receipt_epoch);
    append_string(&mut digest, &target.household_id);
    append_string(&mut digest, &target.account_id);
    append_string(&mut digest, &target.parent_device_id);
    append_string(&mut digest, &target.child_profile_id);
    append_string(&mut digest, &target.child_device_id);
    append_string(&mut digest, &target.provider_subject);
    append_string(&mut digest, &target.session_id);
    append_string(&mut digest, &target.session_expires_at);
    append_string(&mut digest, &target.installation_id);
    append_string(&mut digest, &target.pairing_id);
    append_string(&mut digest, &target.route_id);
    append_string(&mut digest, &target.device_trust_subject);
    append_string(&mut digest, &target.device_signer_key_id);
    append_string(&mut digest, &target.device_signer_key_sha256);
    append_i64(
        &mut digest,
        to_sql_generation(target.device_lifecycle_generation)?,
    );
    append_i64(
        &mut digest,
        to_sql_generation(target.device_installation_binding_generation)?,
    );
    Ok(digest.finalize().into())
}

pub(super) fn operation_digest(target_digest: &[u8; 32], nonce: &[u8; 32]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"ocentra-account-household-runtime-effect-operation-v1");
    digest.update(target_digest);
    digest.update(nonce);
    digest.finalize().into()
}

pub(super) fn consumption_record_digest(operation_digest: &[u8; 32]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"ocentra-account-household-runtime-consumption-record-v1");
    digest.update(operation_digest);
    digest.finalize().into()
}

fn optional_record(
    generation: Option<u64>,
    expires_at: Option<DateTime<Utc>>,
    epoch: Option<u64>,
) -> Result<Option<(i64, i64, i64)>, HouseholdAuthorityRuntimeCasError> {
    if generation.is_none() && expires_at.is_none() && epoch.is_none() {
        return Ok(None);
    }
    let Some(((generation, expires_at), epoch)) = generation.zip(expires_at).zip(epoch) else {
        return Err(HouseholdAuthorityRuntimeCasError::InvalidTarget);
    };
    Ok(Some((
        to_sql_generation(generation)?,
        to_epoch_millis(expires_at.timestamp_millis())?,
        to_sql_generation(epoch)?,
    )))
}

fn to_sql_generation(value: u64) -> Result<i64, HouseholdAuthorityRuntimeCasError> {
    i64::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(HouseholdAuthorityRuntimeCasError::InvalidGeneration)
}

fn to_epoch_millis(value: i64) -> Result<i64, HouseholdAuthorityRuntimeCasError> {
    (value > 0)
        .then_some(value)
        .ok_or(HouseholdAuthorityRuntimeCasError::InvalidTimestamp)
}

fn action_code(action: HouseholdAuthorityAction) -> Result<i64, HouseholdAuthorityRuntimeCasError> {
    match action {
        HouseholdAuthorityAction::SealParentDeviceTrust => Ok(1),
        HouseholdAuthorityAction::PairChildDevice => Ok(2),
        HouseholdAuthorityAction::RegisterLanSignerAnchor => Ok(3),
        HouseholdAuthorityAction::RevokeChildDevice => Ok(4),
        HouseholdAuthorityAction::ViewChildStatus => Ok(5),
        HouseholdAuthorityAction::ChangePolicy => Ok(6),
        HouseholdAuthorityAction::StartRemoteView => Ok(7),
        HouseholdAuthorityAction::StartRemoteControl => Ok(8),
        HouseholdAuthorityAction::ExportDeleteData => Ok(9),
        HouseholdAuthorityAction::ImportRestoreData => Ok(10),
        HouseholdAuthorityAction::ManageBilling => Ok(11),
    }
}

fn provider_code(
    provider: &ocentra_schema::account_identity_authority::AccountIdentityProvider,
) -> Result<i64, HouseholdAuthorityRuntimeCasError> {
    match provider {
        &ocentra_schema::account_identity_authority::AccountIdentityProvider::Authjs => Ok(1),
        &ocentra_schema::account_identity_authority::AccountIdentityProvider::Firebase => Ok(2),
    }
}

fn device_state_code(
    state: DeviceTrustLifecycleState,
) -> Result<i64, HouseholdAuthorityRuntimeCasError> {
    match state {
        DeviceTrustLifecycleState::Pending => Ok(1),
        DeviceTrustLifecycleState::Trusted => Ok(2),
        DeviceTrustLifecycleState::Revoked => Ok(3),
        DeviceTrustLifecycleState::ResetRequired => Ok(4),
    }
}

fn append_i64(digest: &mut Sha256, value: i64) {
    digest.update(value.to_be_bytes());
}

fn append_optional_i64(digest: &mut Sha256, value: Option<i64>) {
    value
        .map(|value| {
            digest.update([1]);
            append_i64(digest, value);
        })
        .unwrap_or_else(|| digest.update([0]));
}

fn append_string(digest: &mut Sha256, value: &str) {
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value.as_bytes());
}

fn require_pragma_ok(connection: &Connection, sql: &str) -> Result<(), ()> {
    let value = connection
        .query_row(sql, [], |row| row.get::<_, String>(0))
        .map_err(|_| ())?;
    (value == "ok").then_some(()).ok_or(())
}

fn require_no_foreign_key_violations(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    rows.next()
        .map_err(|_| ())?
        .is_none()
        .then_some(())
        .ok_or(())
}

fn validate_objects(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, sql FROM sqlite_master
             WHERE type IN ('table', 'index', 'trigger', 'view')
               AND (lower(name) LIKE 'household_authority_runtime_%'
                    OR lower(name) LIKE 'sqlite_autoindex_household_authority_runtime_%'
                    OR lower(COALESCE(sql, '')) LIKE '%household_authority_runtime_%')",
        )
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let object_type = row.get::<_, String>(0).map_err(|_| ())?;
        let name = row.get::<_, String>(1).map_err(|_| ())?;
        let definition = row.get::<_, Option<String>>(2).map_err(|_| ())?;
        match object_type.as_str() {
            "table" if name == CLOCK_TABLE => {
                if !canonical_definition_matches(definition.as_deref().ok_or(())?, CLOCK_TABLE) {
                    return Err(());
                }
            }
            "table" if name == EFFECT_TABLE => {
                if !canonical_definition_matches(definition.as_deref().ok_or(())?, EFFECT_TABLE) {
                    return Err(());
                }
            }
            "index" if name == STATUS_INDEX => {
                if !canonical_index_definition_matches(definition.as_deref().ok_or(())?) {
                    return Err(());
                }
            }
            "index"
                if matches!(
                    name.as_str(),
                    "sqlite_autoindex_household_authority_runtime_cas_clock_1"
                        | "sqlite_autoindex_household_authority_runtime_effect_1"
                        | "sqlite_autoindex_household_authority_runtime_effect_2"
                ) => {}
            "trigger" | "view" => return Err(()),
            _ => return Err(()),
        }
    }
    Ok(())
}

fn canonical_definition_matches(actual: &str, table: &str) -> bool {
    let expected = match table {
        CLOCK_TABLE => "CREATE TABLE IF NOT EXISTS household_authority_runtime_cas_clock ( clock_id INTEGER PRIMARY KEY CHECK (clock_id = 1), last_epoch_millis INTEGER NOT NULL CHECK (last_epoch_millis > 0) ) STRICT",
        EFFECT_TABLE => "CREATE TABLE IF NOT EXISTS household_authority_runtime_effect ( operation_digest BLOB NOT NULL CHECK (length(operation_digest) = 32), consumption_nonce BLOB NOT NULL CHECK (length(consumption_nonce) = 32), target_digest BLOB NOT NULL CHECK (length(target_digest) = 32), action INTEGER NOT NULL CHECK (action BETWEEN 1 AND 11), provider INTEGER NOT NULL CHECK (provider IN (1, 2)), device_state INTEGER NOT NULL CHECK (device_state BETWEEN 1 AND 4), account_authority_generation INTEGER NOT NULL CHECK (account_authority_generation > 0), session_generation INTEGER NOT NULL CHECK (session_generation > 0), device_authority_generation INTEGER NOT NULL CHECK (device_authority_generation > 0), capability_authority_generation INTEGER, capability_expires_at_epoch_millis INTEGER, capability_revocation_epoch INTEGER, controller_lease_authority_generation INTEGER, controller_lease_expires_at_epoch_millis INTEGER, controller_lease_revocation_epoch INTEGER, parent_step_up_authority_generation INTEGER, parent_step_up_expires_at_epoch_millis INTEGER, parent_step_up_receipt_epoch INTEGER, status TEXT NOT NULL CHECK (status IN ('prepared', 'reserved', 'committed', 'aborted', 'ambiguous')), handoff_digest BLOB CHECK (handoff_digest IS NULL OR length(handoff_digest) = 32), created_at_epoch_millis INTEGER NOT NULL CHECK (created_at_epoch_millis > 0), updated_at_epoch_millis INTEGER NOT NULL CHECK (updated_at_epoch_millis >= created_at_epoch_millis), recovery_epoch INTEGER NOT NULL CHECK (recovery_epoch > 0), PRIMARY KEY (operation_digest), UNIQUE (consumption_nonce), CHECK ( (capability_authority_generation IS NULL AND capability_expires_at_epoch_millis IS NULL AND capability_revocation_epoch IS NULL) OR (capability_authority_generation > 0 AND capability_expires_at_epoch_millis > 0 AND capability_revocation_epoch > 0) ), CHECK ( (controller_lease_authority_generation IS NULL AND controller_lease_expires_at_epoch_millis IS NULL AND controller_lease_revocation_epoch IS NULL) OR (controller_lease_authority_generation > 0 AND controller_lease_expires_at_epoch_millis > 0 AND controller_lease_revocation_epoch > 0) ), CHECK ( (parent_step_up_authority_generation IS NULL AND parent_step_up_expires_at_epoch_millis IS NULL AND parent_step_up_receipt_epoch IS NULL) OR (parent_step_up_authority_generation > 0 AND parent_step_up_expires_at_epoch_millis > 0 AND parent_step_up_receipt_epoch > 0) ), CHECK ( (status = 'committed' AND handoff_digest IS NOT NULL) OR (status <> 'committed' AND handoff_digest IS NULL) ) ) STRICT",
        _ => return false,
    };
    compact_sql(actual).replace("IFNOTEXISTS", "")
        == compact_sql(expected).replace("IFNOTEXISTS", "")
}

fn canonical_index_definition_matches(actual: &str) -> bool {
    compact_sql(actual) == compact_sql(
        "CREATE INDEX IF NOT EXISTS household_authority_runtime_effect_status ON household_authority_runtime_effect(status, updated_at_epoch_millis)",
    )
        || compact_sql(actual).replace("IFNOTEXISTS", "")
            == compact_sql(
                "CREATE INDEX IF NOT EXISTS household_authority_runtime_effect_status ON household_authority_runtime_effect(status, updated_at_epoch_millis)",
            )
            .replace("IFNOTEXISTS", "")
}

fn compact_sql(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}

fn validate_columns(connection: &Connection) -> Result<(), ()> {
    validate_table_columns(
        connection,
        CLOCK_TABLE,
        &[
            ("clock_id", "INTEGER", 0_i64, 1_i64),
            ("last_epoch_millis", "INTEGER", 1_i64, 0_i64),
        ],
    )?;
    validate_table_columns(
        connection,
        EFFECT_TABLE,
        &[
            ("operation_digest", "BLOB", 1, 1),
            ("consumption_nonce", "BLOB", 1, 0),
            ("target_digest", "BLOB", 1, 0),
            ("action", "INTEGER", 1, 0),
            ("provider", "INTEGER", 1, 0),
            ("device_state", "INTEGER", 1, 0),
            ("account_authority_generation", "INTEGER", 1, 0),
            ("session_generation", "INTEGER", 1, 0),
            ("device_authority_generation", "INTEGER", 1, 0),
            ("capability_authority_generation", "INTEGER", 0, 0),
            ("capability_expires_at_epoch_millis", "INTEGER", 0, 0),
            ("capability_revocation_epoch", "INTEGER", 0, 0),
            ("controller_lease_authority_generation", "INTEGER", 0, 0),
            ("controller_lease_expires_at_epoch_millis", "INTEGER", 0, 0),
            ("controller_lease_revocation_epoch", "INTEGER", 0, 0),
            ("parent_step_up_authority_generation", "INTEGER", 0, 0),
            ("parent_step_up_expires_at_epoch_millis", "INTEGER", 0, 0),
            ("parent_step_up_receipt_epoch", "INTEGER", 0, 0),
            ("status", "TEXT", 1, 0),
            ("handoff_digest", "BLOB", 0, 0),
            ("created_at_epoch_millis", "INTEGER", 1, 0),
            ("updated_at_epoch_millis", "INTEGER", 1, 0),
            ("recovery_epoch", "INTEGER", 1, 0),
        ],
    )
}

fn validate_table_columns(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info('{table}')"))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut index = 0;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let Some(expected_column) = expected.get(index) else {
            return Err(());
        };
        if row.get::<_, String>(1).map_err(|_| ())? != expected_column.0
            || row
                .get::<_, String>(2)
                .map_err(|_| ())?
                .to_ascii_uppercase()
                != expected_column.1
            || row.get::<_, i64>(3).map_err(|_| ())? != expected_column.2
            || row.get::<_, i64>(5).map_err(|_| ())? != expected_column.3
        {
            return Err(());
        }
        index += 1;
    }
    (index == expected.len()).then_some(()).ok_or(())
}

fn validate_indexes(connection: &Connection) -> Result<(), ()> {
    validate_clock_indexes(connection)?;
    let mut statement = connection
        .prepare(&format!("PRAGMA index_list('{}')", EFFECT_TABLE))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut autoindex_count = 0;
    let mut status_found = false;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let name = row.get::<_, String>(1).map_err(|_| ())?;
        let unique = row.get::<_, i64>(2).map_err(|_| ())?;
        if matches!(
            name.as_str(),
            "sqlite_autoindex_household_authority_runtime_effect_1"
                | "sqlite_autoindex_household_authority_runtime_effect_2"
        ) {
            autoindex_count += 1;
        } else if name == STATUS_INDEX {
            if unique != 0
                || index_columns(connection, &name)?
                    != vec!["status".to_owned(), "updated_at_epoch_millis".to_owned()]
            {
                return Err(());
            }
            status_found = true;
        } else {
            return Err(());
        }
    }
    (autoindex_count == 2 && status_found)
        .then_some(())
        .ok_or(())
}

fn validate_clock_indexes(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_list('{}')", CLOCK_TABLE))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut count = 0;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let name = row.get::<_, String>(1).map_err(|_| ())?;
        if name != "sqlite_autoindex_household_authority_runtime_cas_clock_1" {
            return Err(());
        }
        count += 1;
    }
    (count == 0).then_some(()).ok_or(())
}

fn index_columns(connection: &Connection, index: &str) -> Result<Vec<String>, ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA index_info('{index}')"))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut columns = Vec::new();
    while let Some(row) = rows.next().map_err(|_| ())? {
        columns.push(row.get::<_, String>(2).map_err(|_| ())?);
    }
    Ok(columns)
}

fn validate_rows(connection: &Connection) -> Result<(), ()> {
    let clock_count = connection
        .query_row(
            "SELECT count(*) FROM household_authority_runtime_cas_clock",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    if clock_count != 1
        || connection
            .query_row(
                "SELECT last_epoch_millis FROM household_authority_runtime_cas_clock WHERE clock_id = 1",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| ())?
            <= 0
    {
        return Err(());
    }

    let mut statement = connection
        .prepare(
            "SELECT operation_digest, consumption_nonce, target_digest, action, provider,
                    device_state, account_authority_generation, session_generation,
                    device_authority_generation, capability_authority_generation,
                    capability_expires_at_epoch_millis, capability_revocation_epoch,
                    controller_lease_authority_generation, controller_lease_expires_at_epoch_millis,
                    controller_lease_revocation_epoch, parent_step_up_authority_generation,
                    parent_step_up_expires_at_epoch_millis, parent_step_up_receipt_epoch,
                    status, handoff_digest, created_at_epoch_millis, updated_at_epoch_millis,
                    recovery_epoch
             FROM household_authority_runtime_effect",
        )
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let stored_operation_digest = digest_column(row, 0)?;
        let consumption_nonce = digest_column(row, 1)?;
        let target_digest = digest_column(row, 2)?;
        if stored_operation_digest != operation_digest(&target_digest, &consumption_nonce)
            || !(1..=11).contains(&row.get::<_, i64>(3).map_err(|_| ())?)
            || !(1..=2).contains(&row.get::<_, i64>(4).map_err(|_| ())?)
            || !(1..=4).contains(&row.get::<_, i64>(5).map_err(|_| ())?)
            || row.get::<_, i64>(6).map_err(|_| ())? <= 0
            || row.get::<_, i64>(7).map_err(|_| ())? <= 0
            || row.get::<_, i64>(8).map_err(|_| ())? <= 0
            || !optional_columns_valid(row, 9)?
            || !optional_columns_valid(row, 12)?
            || !optional_columns_valid(row, 15)?
            || !status_row_valid(row, &stored_operation_digest)?
            || row.get::<_, i64>(20).map_err(|_| ())? <= 0
            || row.get::<_, i64>(21).map_err(|_| ())? < row.get::<_, i64>(20).map_err(|_| ())?
            || row.get::<_, i64>(22).map_err(|_| ())? <= 0
        {
            return Err(());
        }
    }
    Ok(())
}

fn digest_column(row: &rusqlite::Row<'_>, index: usize) -> Result<[u8; 32], ()> {
    row.get::<_, Vec<u8>>(index)
        .map_err(|_| ())?
        .try_into()
        .map_err(|_| ())
}

fn optional_columns_valid(row: &rusqlite::Row<'_>, offset: usize) -> Result<bool, ()> {
    let values = [
        row.get::<_, Option<i64>>(offset).map_err(|_| ())?,
        row.get::<_, Option<i64>>(offset + 1).map_err(|_| ())?,
        row.get::<_, Option<i64>>(offset + 2).map_err(|_| ())?,
    ];
    Ok(match values {
        [None, None, None] => true,
        [Some(first), Some(second), Some(third)] => first > 0 && second > 0 && third > 0,
        _ => false,
    })
}

fn status_row_valid(row: &rusqlite::Row<'_>, operation_digest: &[u8; 32]) -> Result<bool, ()> {
    let status = row.get::<_, String>(18).map_err(|_| ())?;
    let handoff = row.get::<_, Option<Vec<u8>>>(19).map_err(|_| ())?;
    let handoff_valid = handoff.map_or(true, |value| {
        let expected = consumption_record_digest(operation_digest);
        value.as_slice() == expected.as_slice()
    });
    Ok(handoff_valid
        && match status.as_str() {
            "committed" => handoff.is_some(),
            "prepared" | "reserved" | "aborted" | "ambiguous" => handoff.is_none(),
            _ => false,
        })
}
