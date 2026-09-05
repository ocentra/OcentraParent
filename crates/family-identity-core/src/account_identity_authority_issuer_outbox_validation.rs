use rusqlite::Connection;

use super::super::service_binding::{
    AccountIdentityIssuerService, AccountIdentityIssuerServiceBinding,
};
use super::super::AccountIdentityIssuerError;
use super::is_sha256_digest;

#[derive(Clone, Copy)]
struct DeliveryStateShape<'a> {
    state: &'a str,
    claim_id: Option<&'a str>,
    claim_expires_at: Option<i64>,
    attempt_count: i64,
    acknowledgement_id: Option<&'a str>,
    acknowledged_at: Option<i64>,
    created_at: i64,
    expires_at: i64,
    terminal_at: Option<i64>,
}

pub(super) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validate_receipt_bindings(connection)?;
    validate_rows(connection)
}

fn validate_receipt_bindings(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    let invalid: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_transport_outbox AS outbox
                LEFT JOIN account_identity_issuer_transport_receipt AS receipt
                  ON receipt.receipt_id = outbox.receipt_id
                WHERE receipt.receipt_id IS NULL
                   OR receipt.account_id != outbox.account_id
                   OR receipt.household_id != outbox.household_id
                   OR receipt.service_binding_id != outbox.service_binding_id
                   OR receipt.service_label != outbox.service_label
                   OR receipt.authority_generation != outbox.authority_generation
                   OR receipt.key_id != outbox.key_id
                   OR receipt.key_version != outbox.key_version
                   OR receipt.issued_at_millis != outbox.created_at_millis
            )",
            [],
            |row| row.get(0),
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    (!invalid)
        .then_some(())
        .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)
}

fn validate_rows(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    let mut statement = connection
        .prepare(
            "SELECT outbox.receipt_id, outbox.service_label, outbox.account_id,
                    outbox.household_id, outbox.authority_generation,
                    outbox.service_binding_id, outbox.key_id, outbox.key_version,
                    outbox.wire, outbox.created_at_millis, receipt.expires_at_millis,
                    outbox.delivery_state, outbox.claim_id,
                    outbox.claim_expires_at_millis, outbox.attempt_count,
                    outbox.acknowledgement_id, outbox.acknowledged_at_millis,
                    outbox.terminal_at_millis, registry.public_key
             FROM account_identity_issuer_transport_outbox AS outbox
             JOIN account_identity_issuer_transport_receipt AS receipt
               ON receipt.receipt_id = outbox.receipt_id
             JOIN account_identity_issuer_key_registry AS registry
               ON registry.account_id = outbox.account_id
              AND registry.household_id = outbox.household_id
              AND registry.service_binding_id = outbox.service_binding_id
              AND registry.service_label = outbox.service_label
              AND registry.key_id = outbox.key_id
              AND registry.key_version = outbox.key_version",
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    while let Some(row) = rows
        .next()
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?
    {
        validate_row(row)?;
    }
    Ok(())
}

fn validate_row(row: &rusqlite::Row<'_>) -> Result<(), AccountIdentityIssuerError> {
    let receipt_id: String = row_value(row, 0)?;
    let service_label: String = row_value(row, 1)?;
    let account_id: String = row_value(row, 2)?;
    let household_id: String = row_value(row, 3)?;
    let generation: i64 = row_value(row, 4)?;
    let binding_id: String = row_value(row, 5)?;
    let key_id: String = row_value(row, 6)?;
    let key_version: i64 = row_value(row, 7)?;
    let wire: Vec<u8> = row_value(row, 8)?;
    let created_at: i64 = row_value(row, 9)?;
    let expires_at: i64 = row_value(row, 10)?;
    let state: String = row_value(row, 11)?;
    let claim_id: Option<String> = row_value(row, 12)?;
    let claim_expires_at: Option<i64> = row_value(row, 13)?;
    let attempt_count: i64 = row_value(row, 14)?;
    let acknowledgement_id: Option<String> = row_value(row, 15)?;
    let acknowledged_at: Option<i64> = row_value(row, 16)?;
    let terminal_at: Option<i64> = row_value(row, 17)?;
    let public_key: Vec<u8> = row_value(row, 18)?;
    let service = AccountIdentityIssuerService::from_label(&service_label)
        .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)?;
    let generation =
        u64::try_from(generation).map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let key_version = u64::try_from(key_version)
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)?;
    let binding_matches = AccountIdentityIssuerServiceBinding::expected_binding_id(
        service,
        &account_id,
        &household_id,
        generation,
    ) == binding_id;
    let state_matches = validate_state(DeliveryStateShape {
        state: &state,
        claim_id: claim_id.as_deref(),
        claim_expires_at,
        attempt_count,
        acknowledgement_id: acknowledgement_id.as_deref(),
        acknowledged_at,
        created_at,
        expires_at,
        terminal_at,
    });
    if !is_sha256_digest(&receipt_id)
        || !is_sha256_digest(&key_id)
        || created_at < 0
        || expires_at <= created_at
        || key_version == 0
        || attempt_count < 0
        || !binding_matches
        || !state_matches
    {
        return Err(AccountIdentityIssuerError::InvalidKeyRecord);
    }
    super::super::transport::validate_stored_wire(
        &wire,
        &super::super::transport::StoredIssuerTransportExpectation {
            receipt_id: &receipt_id,
            service_label: &service_label,
            binding_id: &binding_id,
            account_id: &account_id,
            household_id: &household_id,
            authority_generation: generation,
            key_id: &key_id,
            key_version,
            issued_at_millis: created_at,
            expires_at_millis: expires_at,
        },
        &public_key,
    )
}

fn validate_state(shape: DeliveryStateShape<'_>) -> bool {
    match shape.state {
        "pending" => {
            shape.claim_id.is_none()
                && shape.claim_expires_at.is_none()
                && shape.acknowledgement_id.is_none()
                && shape.acknowledged_at.is_none()
                && shape.terminal_at.is_none()
        }
        "claimed" => {
            shape.claim_id.is_some_and(is_sha256_digest)
                && shape
                    .claim_expires_at
                    .is_some_and(|expires| expires > shape.created_at)
                && shape.attempt_count > 0
                && shape.acknowledgement_id.is_none()
                && shape.acknowledged_at.is_none()
                && shape.terminal_at.is_none()
        }
        "acknowledged" => {
            shape.claim_id.is_none()
                && shape.claim_expires_at.is_none()
                && shape.attempt_count > 0
                && shape.acknowledgement_id.is_some_and(is_sha256_digest)
                && shape
                    .acknowledged_at
                    .is_some_and(|acknowledged| acknowledged >= shape.created_at)
                && shape.terminal_at.is_none()
        }
        "expired" => {
            shape.claim_id.is_none()
                && shape.claim_expires_at.is_none()
                && shape.acknowledgement_id.is_none()
                && shape.acknowledged_at.is_none()
                && shape
                    .terminal_at
                    .is_some_and(|terminal| terminal >= shape.expires_at)
        }
        "superseded" => {
            shape.claim_id.is_none()
                && shape.claim_expires_at.is_none()
                && shape.acknowledgement_id.is_none()
                && shape.acknowledged_at.is_none()
                && shape
                    .terminal_at
                    .is_some_and(|terminal| terminal >= shape.created_at)
        }
        _ => false,
    }
}

fn row_value<T: rusqlite::types::FromSql>(
    row: &rusqlite::Row<'_>,
    index: usize,
) -> Result<T, AccountIdentityIssuerError> {
    row.get(index)
        .map_err(|_error| AccountIdentityIssuerError::InvalidKeyRecord)
}
