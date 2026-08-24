use ed25519_dalek::VerifyingKey;
use rusqlite::{Connection, OptionalExtension, Row, params};

use super::key_custody::AccountIdentityIssuerSigningHandle;
use super::service_binding::AccountIdentityIssuerServiceBinding;
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

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
        PRIMARY KEY (account_id, household_id, service_binding_id, key_version),
        UNIQUE (key_id)
    ) STRICT;
    CREATE INDEX IF NOT EXISTS account_identity_issuer_key_registry_current
        ON account_identity_issuer_key_registry (
            account_id, household_id, service_binding_id, key_state, key_version
        );";

pub(crate) struct RegisteredIssuerKey {
    pub(crate) handle: AccountIdentityIssuerSigningHandle,
    pub(crate) verifying_key: VerifyingKey,
}

pub(crate) fn ensure_schema(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    connection
        .execute_batch(KEY_REGISTRY_SCHEMA_SQL)
        .map_err(|_| AccountIdentityIssuerError::Unavailable)
}

pub(crate) fn register(
    connection: &mut Connection,
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
    let transaction = connection
        .transaction()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let latest_version: Option<i64> = transaction
        .query_row(
            "SELECT MAX(key_version) FROM account_identity_issuer_key_registry
             WHERE account_id = ?1 AND household_id = ?2 AND service_binding_id = ?3",
            params![account_id, household_id, service_binding_id],
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
             WHERE account_id = ?1 AND household_id = ?2 AND service_binding_id = ?3
               AND key_state = 'active'",
            params![
                account_id,
                household_id,
                service_binding_id,
                authority_generation
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    transaction
        .execute(
            "INSERT INTO account_identity_issuer_key_registry (
                account_id, household_id, service_binding_id, key_id, key_version,
                public_key, key_state, authority_generation, revoked_generation
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'active', ?7, NULL)",
            params![
                account_id,
                household_id,
                service_binding_id,
                key_id,
                key_version,
                public_key_bytes.as_slice(),
                authority_generation,
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::KeyAlreadyRegistered)?;
    transaction
        .commit()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    registered_key(
        binding,
        key_id,
        from_sql_generation(key_version)?,
        public_key_bytes,
    )
}

pub(crate) fn revoke(
    connection: &mut Connection,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    handle: &AccountIdentityIssuerSigningHandle,
) -> Result<(), AccountIdentityIssuerError> {
    ensure_binding(authority, binding)?;
    if handle.account_id() != authority.account_id().to_string()
        || handle.household_id() != authority.household_id().to_string()
        || handle.service_binding_id() != binding.binding_id()
    {
        return Err(AccountIdentityIssuerError::BindingMismatch);
    }
    let changed = connection
        .execute(
            "UPDATE account_identity_issuer_key_registry
             SET key_state = 'revoked', revoked_generation = ?6
             WHERE account_id = ?1 AND household_id = ?2 AND service_binding_id = ?3
               AND key_id = ?4 AND key_version = ?5 AND key_state = 'active'",
            params![
                handle.account_id(),
                handle.household_id(),
                handle.service_binding_id(),
                handle.key_id(),
                to_sql_generation(handle.key_version())?,
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
            "SELECT key_id, key_version, public_key
             FROM account_identity_issuer_key_registry
             WHERE account_id = ?1 AND household_id = ?2 AND service_binding_id = ?3
               AND key_state = 'active'
             ORDER BY key_version DESC
             LIMIT 1",
            params![
                authority.account_id().to_string(),
                authority.household_id().to_string(),
                binding.binding_id(),
            ],
            |row| {
                let key_id = row.get::<_, String>(0)?;
                let key_version = row.get::<_, i64>(1)?;
                let public_key = row.get::<_, Vec<u8>>(2)?;
                Ok((key_id, key_version, public_key))
            },
        )
        .optional()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?
        .ok_or(AccountIdentityIssuerError::KeyUnavailable)?;
    let (key_id, key_version, public_key) = row;
    let key_version = from_sql_generation(key_version)?;
    let public_key: [u8; 32] = public_key
        .try_into()
        .map_err(|_| AccountIdentityIssuerError::InvalidPublicKey)?;
    registered_key(binding, key_id, key_version, public_key)
}

pub(crate) fn validate_durable_state(
    connection: &Connection,
) -> Result<u64, AccountIdentityIssuerError> {
    let mut statement = connection
        .prepare(
            "SELECT account_id, household_id, service_binding_id, key_id, key_version,
                    public_key, key_state, authority_generation, revoked_generation
             FROM account_identity_issuer_key_registry",
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let mut rows = statement
        .query([])
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let duplicate_active_binding = connection
        .query_row(
            "SELECT 1
             FROM account_identity_issuer_key_registry
             WHERE key_state = 'active'
             GROUP BY account_id, household_id, service_binding_id
             HAVING COUNT(*) > 1
             LIMIT 1",
            [],
            |_row| Ok(()),
        )
        .optional()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?
        .is_some();
    if duplicate_active_binding {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let mut active_count = 0_u64;
    while let Some(row) = rows
        .next()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?
    {
        if validate_durable_key_row(row)? {
            active_count = active_count
                .checked_add(1)
                .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)?;
        }
    }
    Ok(active_count)
}

struct DurableKeyRow {
    account_id: String,
    household_id: String,
    binding_id: String,
    key_id: String,
    key_version: i64,
    public_key: Vec<u8>,
    key_state: String,
    authority_generation: i64,
    revoked_generation: Option<i64>,
}

fn validate_durable_key_row(row: &Row<'_>) -> Result<bool, AccountIdentityIssuerError> {
    let durable = DurableKeyRow {
        account_id: row
            .get(0)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        household_id: row
            .get(1)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        binding_id: row
            .get(2)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        key_id: row
            .get(3)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        key_version: row
            .get(4)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        public_key: row
            .get(5)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        key_state: row
            .get(6)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        authority_generation: row
            .get(7)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
        revoked_generation: row
            .get(8)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
    };
    validate_durable_key_record(&durable)
}

fn validate_durable_key_record(
    durable: &DurableKeyRow,
) -> Result<bool, AccountIdentityIssuerError> {
    let has_valid_shape = !durable.account_id.trim().is_empty()
        && !durable.household_id.trim().is_empty()
        && !durable.binding_id.trim().is_empty()
        && !durable.key_id.trim().is_empty()
        && durable.key_version > 0
        && durable.authority_generation > 0
        && durable.public_key.len() == 32;
    if !has_valid_shape {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let public_key: [u8; 32] = durable
        .public_key
        .clone()
        .try_into()
        .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let key_id_matches =
        crate::account_identity_authority_producer::expected_key_id(&verifying_key)
            == durable.key_id;
    if !key_id_matches {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    match durable.key_state.as_str() {
        "active" => Ok(durable.revoked_generation.is_none()),
        "revoked" => Ok(durable.revoked_generation.is_some()),
        _ => Err(AccountIdentityIssuerError::InvalidKeyRecord),
    }
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
