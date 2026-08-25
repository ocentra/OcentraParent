use ed25519_dalek::VerifyingKey;
use rusqlite::{params, Connection, OptionalExtension, Transaction};

use super::key_custody::AccountIdentityIssuerSigningHandle;
use super::service_binding::AccountIdentityIssuerServiceBinding;
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

#[path = "account_identity_authority_issuer_key_registry_lineage.rs"]
mod lineage;
#[path = "account_identity_authority_issuer_key_registry_receipts.rs"]
pub(crate) mod receipts;
#[path = "account_identity_authority_issuer_key_registry_rows.rs"]
pub(crate) mod rows;
#[path = "account_identity_authority_issuer_key_registry_schema.rs"]
pub(crate) mod schema;

pub(crate) const KEY_REGISTRY_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_issuer_key_registry (
        account_id TEXT NOT NULL CHECK (length(account_id) > 0),
        household_id TEXT NOT NULL CHECK (length(household_id) > 0),
        service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
        key_id TEXT NOT NULL CHECK (length(key_id) > 0),
        key_version INTEGER NOT NULL CHECK (key_version > 0),
        public_key BLOB NOT NULL CHECK (length(public_key) = 32),
        key_state TEXT NOT NULL CHECK (key_state IN ('active','revoked')),
        authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
        revoked_generation INTEGER,
        service_label TEXT NOT NULL CHECK (length(service_label) > 0),
        PRIMARY KEY (account_id, household_id, service_binding_id, key_version),
        UNIQUE (key_id)
    ) STRICT;
    CREATE INDEX IF NOT EXISTS account_identity_issuer_key_registry_current
        ON account_identity_issuer_key_registry (
            account_id, household_id, service_label, key_state, key_version
        );";

pub(crate) const TRANSPORT_RECEIPT_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_issuer_transport_receipt (
        receipt_id TEXT PRIMARY KEY CHECK (length(receipt_id) > 0),
        account_id TEXT NOT NULL CHECK (length(account_id) > 0),
        household_id TEXT NOT NULL CHECK (length(household_id) > 0),
        service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
        service_label TEXT NOT NULL CHECK (length(service_label) > 0),
        authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
        key_id TEXT NOT NULL CHECK (length(key_id) > 0),
        key_version INTEGER NOT NULL CHECK (key_version > 0),
        issued_at_millis INTEGER NOT NULL CHECK (issued_at_millis >= 0),
        expires_at_millis INTEGER NOT NULL CHECK (expires_at_millis > issued_at_millis),
        receipt_state TEXT NOT NULL CHECK (receipt_state IN ('issued','consumed')),
        consumed_at_millis INTEGER,
        CHECK (
            (receipt_state = 'issued' AND consumed_at_millis IS NULL)
            OR (receipt_state = 'consumed' AND consumed_at_millis IS NOT NULL)
        )
    ) STRICT;
    CREATE INDEX IF NOT EXISTS account_identity_issuer_transport_receipt_lookup
        ON account_identity_issuer_transport_receipt (
            account_id, household_id, service_binding_id, key_id, key_version, receipt_state
        );";

pub(crate) const CLOCK_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_issuer_clock (
        clock_id INTEGER PRIMARY KEY CHECK (clock_id = 1),
        last_unix_millis INTEGER NOT NULL CHECK (last_unix_millis >= 0)
    ) STRICT;
    INSERT INTO account_identity_issuer_clock (clock_id, last_unix_millis)
        VALUES (1, 0)
        ON CONFLICT(clock_id) DO NOTHING;";

pub(crate) struct RegisteredIssuerKey {
    pub(crate) handle: AccountIdentityIssuerSigningHandle,
    pub(crate) verifying_key: VerifyingKey,
}

pub(crate) fn ensure_schema(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    connection
        .execute_batch(KEY_REGISTRY_SCHEMA_SQL)
        .and_then(|_| connection.execute_batch(TRANSPORT_RECEIPT_SCHEMA_SQL))
        .and_then(|_| connection.execute_batch(CLOCK_SCHEMA_SQL))
        .and_then(|_| connection.execute_batch(super::outbox::OUTBOX_SCHEMA_SQL))
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    schema::validate(connection)
}

pub(crate) fn register(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    public_key_bytes: [u8; 32],
) -> Result<RegisteredIssuerKey, AccountIdentityIssuerError> {
    ensure_binding(authority, binding)?;
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
        .map_err(|_| AccountIdentityIssuerError::InvalidPublicKey)?;
    let key_id = crate::account_identity_authority_producer::expected_key_id(&verifying_key);
    let account_id = authority.account_id().to_string();
    let household_id = authority.household_id().to_string();
    let service_binding_id = binding.binding_id().to_owned();
    let authority_generation = to_sql_generation(authority.authority_generation())?;
    let latest_version: Option<i64> = transaction
        .query_row(
            "SELECT MAX(key_version) FROM account_identity_issuer_key_registry
             WHERE account_id = ?1 AND household_id = ?2 AND service_label = ?3",
            params![account_id, household_id, binding.service().label()],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let key_version = latest_version
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(AccountIdentityIssuerError::InvalidKeyVersion)?;
    transaction
        .execute(
            "UPDATE account_identity_issuer_key_registry
             SET key_state = 'revoked', revoked_generation = ?4
             WHERE account_id = ?1 AND household_id = ?2 AND service_label = ?3
               AND key_state = 'active'",
            params![
                account_id,
                household_id,
                binding.service().label(),
                authority_generation
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    transaction
        .execute(
            "INSERT INTO account_identity_issuer_key_registry (
                account_id, household_id, service_binding_id, key_id, key_version,
                public_key, key_state, authority_generation, revoked_generation, service_label
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'active', ?7, NULL, ?8)",
            params![
                account_id,
                household_id,
                service_binding_id,
                key_id,
                key_version,
                public_key_bytes.as_slice(),
                authority_generation,
                binding.service().label(),
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::KeyAlreadyRegistered)?;
    registered_key(
        binding,
        key_id,
        from_sql_generation(key_version)?,
        public_key_bytes,
    )
}

pub(crate) fn revoke(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
) -> Result<(), AccountIdentityIssuerError> {
    ensure_binding(authority, binding)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_key_registry
                SET key_state = 'revoked', revoked_generation = ?6
              WHERE account_id = ?1 AND household_id = ?2 AND service_binding_id = ?3
               AND service_label = ?4 AND authority_generation = ?5
               AND key_state = 'active'",
            params![
                authority.account_id().to_string(),
                authority.household_id().to_string(),
                binding.binding_id(),
                binding.service().label(),
                to_sql_generation(authority.authority_generation())?,
                to_sql_generation(authority.authority_generation())?,
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityIssuerError::KeyUnavailable)
}

pub(crate) fn current(
    connection: &Connection,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
) -> Result<RegisteredIssuerKey, AccountIdentityIssuerError> {
    ensure_binding(authority, binding)?;
    let row = connection
        .query_row(
            "SELECT key_id, key_version, public_key, service_label, authority_generation,
                    revoked_generation
             FROM account_identity_issuer_key_registry AS current_key
             WHERE account_id = ?1 AND household_id = ?2 AND service_binding_id = ?3
               AND service_label = ?4 AND key_state = 'active'
               AND NOT EXISTS (
                    SELECT 1 FROM account_identity_issuer_key_registry AS newer
                     WHERE newer.account_id = current_key.account_id
                       AND newer.household_id = current_key.household_id
                       AND newer.service_label = current_key.service_label
                       AND newer.key_version > current_key.key_version
               )
             ORDER BY key_version DESC
             LIMIT 1",
            params![
                authority.account_id().to_string(),
                authority.household_id().to_string(),
                binding.binding_id(),
                binding.service().label(),
            ],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, Option<i64>>(5)?,
                ))
            },
        )
        .optional()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?
        .ok_or(AccountIdentityIssuerError::KeyUnavailable)?;
    let (key_id, key_version, public_key, service_label, generation, revoked) = row;
    if service_label != binding.service().label()
        || generation != to_sql_generation(authority.authority_generation())?
        || revoked.is_some()
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let public_key: [u8; 32] = public_key
        .try_into()
        .map_err(|_| AccountIdentityIssuerError::InvalidPublicKey)?;
    registered_key(
        binding,
        key_id,
        from_sql_generation(key_version)?,
        public_key,
    )
}

pub(crate) fn validate_durable_state(
    connection: &Connection,
) -> Result<u64, AccountIdentityIssuerError> {
    let active_count = rows::validate(connection)?;
    receipts::validate_transport_receipts(connection)?;
    receipts::validate_clock_state(connection)?;
    super::outbox::validate(connection)?;
    Ok(active_count)
}

fn registered_key(
    binding: &AccountIdentityIssuerServiceBinding,
    key_id: String,
    key_version: u64,
    public_key: [u8; 32],
) -> Result<RegisteredIssuerKey, AccountIdentityIssuerError> {
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|_| AccountIdentityIssuerError::InvalidPublicKey)?;
    if crate::account_identity_authority_producer::expected_key_id(&verifying_key) != key_id {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let handle = AccountIdentityIssuerSigningHandle::new(
        key_id,
        key_version,
        binding.account_id().to_owned(),
        binding.household_id().to_owned(),
        binding.binding_id().to_owned(),
    )?;
    Ok(RegisteredIssuerKey {
        handle,
        verifying_key,
    })
}

fn ensure_binding(
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
) -> Result<(), AccountIdentityIssuerError> {
    (binding.matches_authority(authority)
        && binding.account_id() == authority.account_id().to_string()
        && binding.household_id() == authority.household_id().to_string())
    .then_some(())
    .ok_or(AccountIdentityIssuerError::BindingMismatch)
}

fn to_sql_generation(value: u64) -> Result<i64, AccountIdentityIssuerError> {
    i64::try_from(value).map_err(|_| AccountIdentityIssuerError::InvalidKeyVersion)
}

fn from_sql_generation(value: i64) -> Result<u64, AccountIdentityIssuerError> {
    u64::try_from(value).map_err(|_| AccountIdentityIssuerError::InvalidKeyVersion)
}
