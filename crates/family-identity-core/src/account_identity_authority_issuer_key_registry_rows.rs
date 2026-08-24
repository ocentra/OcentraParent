use std::collections::HashSet;

use ed25519_dalek::VerifyingKey;
use rusqlite::{Connection, OptionalExtension, Row};

use super::super::service_binding::{
    AccountIdentityIssuerService, AccountIdentityIssuerServiceBinding,
};
use super::AccountIdentityIssuerError;

pub(crate) fn validate(connection: &Connection) -> Result<u64, AccountIdentityIssuerError> {
    let duplicate_active = connection
        .query_row(
            "SELECT 1 FROM account_identity_issuer_key_registry
             WHERE key_state = 'active'
             GROUP BY account_id, household_id, service_binding_id
             HAVING COUNT(*) > 1 LIMIT 1",
            [],
            |_row| Ok(()),
        )
        .optional()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?
        .is_some();
    if duplicate_active {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let mut statement = connection
        .prepare(
            "SELECT account_id, household_id, service_binding_id, key_id, key_version,
                    public_key, key_state, authority_generation, revoked_generation, service_label
             FROM account_identity_issuer_key_registry",
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let mut rows = statement
        .query([])
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let mut active_count = 0_u64;
    let mut seen_key_ids = HashSet::new();
    while let Some(row) = rows
        .next()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?
    {
        if validate_row(row, &mut seen_key_ids)? {
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
    service_label: String,
}

fn validate_row(
    row: &Row<'_>,
    seen_key_ids: &mut HashSet<String>,
) -> Result<bool, AccountIdentityIssuerError> {
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
        service_label: row
            .get(9)
            .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?,
    };
    validate_record(&durable, seen_key_ids)
}

fn validate_record(
    durable: &DurableKeyRow,
    seen_key_ids: &mut HashSet<String>,
) -> Result<bool, AccountIdentityIssuerError> {
    if durable.account_id.trim().is_empty()
        || durable.household_id.trim().is_empty()
        || durable.binding_id.trim().is_empty()
        || durable.key_id.trim().is_empty()
        || durable.service_label.trim().is_empty()
        || durable.key_version <= 0
        || durable.authority_generation <= 0
        || durable.public_key.len() != 32
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let public_key: [u8; 32] = durable
        .public_key
        .clone()
        .try_into()
        .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?;
    if crate::account_identity_authority_producer::expected_key_id(&verifying_key) != durable.key_id
        || !seen_key_ids.insert(durable.key_id.clone())
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let service = AccountIdentityIssuerService::from_label(&durable.service_label)
        .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)?;
    let generation = u64::try_from(durable.authority_generation)
        .map_err(|_| AccountIdentityIssuerError::InvalidKeyRecord)?;
    if AccountIdentityIssuerServiceBinding::expected_binding_id(
        service,
        &durable.account_id,
        &durable.household_id,
        generation,
    ) != durable.binding_id
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    match durable.key_state.as_str() {
        "active" => Ok(durable.revoked_generation.is_none()),
        "revoked" => Ok(durable
            .revoked_generation
            .is_some_and(|value| value > 0 && value >= durable.authority_generation)),
        _ => Err(AccountIdentityIssuerError::InvalidKeyRecord),
    }
}
