use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::Connection;

use super::{has_legacy_table, AccountIdentityAuthorityIssuerClientError, CANONICAL_SCHEMA_SQL};

pub(super) fn rebuild_legacy(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    backup_legacy_tables(connection)?;
    connection
        .execute_batch(CANONICAL_SCHEMA_SQL)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    copy_key_registry(connection)?;
    copy_receipts(connection)?;
    copy_outbox(connection)?;
    drop_legacy_tables(connection)
}

fn backup_legacy_tables(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    for (table, legacy) in [
        (
            "account_identity_issuer_v2_key_registry",
            "account_identity_issuer_v2_key_registry_legacy",
        ),
        (
            "account_identity_issuer_v2_receipt",
            "account_identity_issuer_v2_receipt_legacy",
        ),
        (
            "account_identity_issuer_v2_outbox",
            "account_identity_issuer_v2_outbox_legacy",
        ),
    ] {
        if has_legacy_table(connection, table)? {
            connection
                .execute_batch(&format!(
                    "CREATE TABLE {legacy} AS SELECT * FROM {table}; DROP TABLE {table};"
                ))
                .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
        }
    }
    Ok(())
}

fn copy_key_registry(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if has_legacy_table(connection, "account_identity_issuer_v2_key_registry_legacy")? {
        connection
            .execute(
                "INSERT INTO account_identity_issuer_v2_key_registry
                    (account_id, household_id, service, service_binding_id, key_id,
                     key_generation, public_key, authority_generation, key_state)
                 SELECT account_id, household_id, ?1, service_binding_id, key_id,
                        key_generation, public_key, authority_generation, key_state
                   FROM account_identity_issuer_v2_key_registry_legacy",
                [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    }
    Ok(())
}

fn copy_receipts(connection: &Connection) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if has_legacy_table(connection, "account_identity_issuer_v2_receipt_legacy")? {
        connection
            .execute(
                "INSERT INTO account_identity_issuer_v2_receipt
                    (receipt_id, account_id, household_id, service,
                     service_binding_id, key_id, key_generation, authority_generation,
                     session_generation, correlation_id, idempotency_key, payload_digest,
                     issued_at, expires_at, wire, receipt_state)
                 SELECT receipt_id, account_id, household_id, ?1, service_binding_id,
                        key_id, key_generation, authority_generation, session_generation,
                        correlation_id, idempotency_key, payload_digest, issued_at,
                        expires_at, wire, receipt_state
                   FROM account_identity_issuer_v2_receipt_legacy",
                [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    }
    Ok(())
}

fn copy_outbox(connection: &Connection) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if has_legacy_table(connection, "account_identity_issuer_v2_outbox_legacy")? {
        connection
            .execute(
                "INSERT INTO account_identity_issuer_v2_outbox
                    (receipt_id, account_id, household_id, service, service_binding_id,
                     key_id, key_generation, authority_generation, wire, delivery_state,
                     claim_id, claimed_at, attempt_count, last_error, last_result,
                     next_attempt_at)
                 SELECT outbox.receipt_id, receipt.account_id, receipt.household_id, ?1,
                        receipt.service_binding_id, receipt.key_id, receipt.key_generation,
                        receipt.authority_generation, outbox.wire,
                        CASE outbox.delivery_state
                            WHEN 'pending' THEN 'pending'
                            WHEN 'sent' THEN 'sent'
                            WHEN 'failed' THEN 'failed'
                            ELSE 'failed'
                        END,
                        NULL, NULL, outbox.attempt_count, outbox.last_error, NULL, NULL
                   FROM account_identity_issuer_v2_outbox_legacy AS outbox
                   JOIN account_identity_issuer_v2_receipt AS receipt
                     ON receipt.receipt_id = outbox.receipt_id",
                [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    }
    Ok(())
}

fn drop_legacy_tables(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    connection
        .execute_batch(
            "DROP TABLE IF EXISTS account_identity_issuer_v2_key_registry_legacy;
             DROP TABLE IF EXISTS account_identity_issuer_v2_receipt_legacy;
             DROP TABLE IF EXISTS account_identity_issuer_v2_outbox_legacy;",
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}
