use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::{Connection, OptionalExtension};

use super::super::AccountIdentityAuthorityIssuerClientError;

pub(super) fn validate_rows(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let invalid_key: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_key_registry
                 WHERE service != ?1 OR length(public_key) != 65
                    OR key_id NOT LIKE 'sha256:ecdsa-p256:%'
                    OR key_generation <= 0 OR authority_generation <= 0
                    OR key_state NOT IN ('active','revoked')
            )",
            [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let invalid_receipt: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_receipt
                 WHERE service != ?1 OR length(wire) = 0
                    OR key_generation <= 0 OR authority_generation <= 0
                    OR session_generation <= 0
                    OR receipt_state NOT IN ('issued','acknowledged')
            )",
            [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let invalid_outbox: bool = connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_outbox AS outbox
                LEFT JOIN account_identity_issuer_v2_receipt AS receipt
                  ON receipt.receipt_id = outbox.receipt_id
                 WHERE receipt.receipt_id IS NULL
                    OR outbox.service != ?1
                    OR outbox.account_id != receipt.account_id
                    OR outbox.household_id != receipt.household_id
                    OR outbox.service_binding_id != receipt.service_binding_id
                    OR outbox.key_id != receipt.key_id
                    OR outbox.key_generation != receipt.key_generation
                    OR outbox.authority_generation != receipt.authority_generation
                    OR outbox.wire != receipt.wire
                    OR outbox.delivery_state NOT IN ('pending','claimed','sent','failed','acknowledged')
            )",
            [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if invalid_key || invalid_receipt || invalid_outbox {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    let integrity = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if integrity != "ok" {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    let mut foreign_key_statement = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut foreign_keys = foreign_key_statement
        .query([])
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if foreign_keys
        .next()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .is_some()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    Ok(())
}
