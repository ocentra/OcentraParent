use std::collections::HashSet;

use rusqlite::{params, Connection};

use super::super::service_binding::{
    AccountIdentityIssuerService, AccountIdentityIssuerServiceBinding,
};
use super::AccountIdentityIssuerError;

pub(crate) fn validate_transport_receipts(
    connection: &Connection,
) -> Result<(), AccountIdentityIssuerError> {
    let rows = load_receipt_rows(connection)?;
    let mut receipt_ids = HashSet::new();
    for receipt in rows {
        validate_receipt_shape(&receipt, &mut receipt_ids)?;
        validate_receipt_key(connection, &receipt)?;
    }
    Ok(())
}

type ReceiptRow = (
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    i64,
    i64,
    String,
    Option<i64>,
);

fn load_receipt_rows(
    connection: &Connection,
) -> Result<Vec<ReceiptRow>, AccountIdentityIssuerError> {
    let mut statement = connection
        .prepare(
            "SELECT receipt_id, account_id, household_id, service_binding_id, service_label,
                    authority_generation, key_id, key_version, issued_at_millis,
                    expires_at_millis, receipt_state, consumed_at_millis
             FROM account_identity_issuer_transport_receipt",
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
                row.get(9)?,
                row.get(10)?,
                row.get(11)?,
            ))
        })
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?
        .collect::<Result<_, _>>()
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    Ok(rows)
}

fn validate_receipt_shape(
    receipt: &ReceiptRow,
    receipt_ids: &mut HashSet<String>,
) -> Result<(), AccountIdentityIssuerError> {
    let (
        receipt_id,
        account_id,
        household_id,
        binding_id,
        service_label,
        authority_generation,
        key_id,
        key_version,
        issued_at_millis,
        expires_at_millis,
        receipt_state,
        consumed_at_millis,
    ) = receipt;
    if !receipt_ids.insert(receipt_id.clone())
        || !is_sha256_digest(receipt_id)
        || account_id.trim().is_empty()
        || household_id.trim().is_empty()
        || key_id.trim().is_empty()
        || !is_sha256_digest(key_id)
        || *authority_generation <= 0
        || *key_version <= 0
        || *issued_at_millis < 0
        || *expires_at_millis <= *issued_at_millis
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    let service = AccountIdentityIssuerService::from_label(service_label)
        .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)?;
    let generation = u64::try_from(*authority_generation)
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    if AccountIdentityIssuerServiceBinding::expected_binding_id(
        service,
        account_id,
        household_id,
        generation,
    ) != *binding_id
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    match (receipt_state.as_str(), consumed_at_millis) {
        ("issued", None) => {}
        ("consumed", Some(consumed_at))
            if *consumed_at >= *issued_at_millis && *consumed_at >= 0 => {}
        _ => return Err(AccountIdentityIssuerError::InvalidKeyRecord),
    }
    Ok(())
}

fn validate_receipt_key(
    connection: &Connection,
    receipt: &ReceiptRow,
) -> Result<(), AccountIdentityIssuerError> {
    let (_, account_id, household_id, binding_id, _, _, key_id, key_version, _, _, _, _) = receipt;
    let key_exists: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_key_registry
                 WHERE account_id = ?1 AND household_id = ?2
                   AND service_binding_id = ?3 AND key_id = ?4 AND key_version = ?5
            )",
            params![account_id, household_id, binding_id, key_id, key_version],
            |row| row.get(0),
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    if !key_exists {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    Ok(())
}

fn is_sha256_digest(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return false;
    };
    hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn validate_clock_state(
    connection: &Connection,
) -> Result<(), AccountIdentityIssuerError> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM account_identity_issuer_clock",
            [],
            |row| row.get(0),
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidClock)?;
    let (clock_id, last_unix_millis): (i64, i64) = connection
        .query_row(
            "SELECT clock_id, last_unix_millis FROM account_identity_issuer_clock",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidClock)?;
    if count != 1 || clock_id != 1 || last_unix_millis < 0 {
        return Err(AccountIdentityIssuerError::InvalidClock);
    }
    Ok(())
}
