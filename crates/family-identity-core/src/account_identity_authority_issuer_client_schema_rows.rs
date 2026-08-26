use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::Connection;

use super::super::AccountIdentityAuthorityIssuerClientError;

pub(super) fn validate_rows(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if validate_key_rows(connection)?
        || validate_receipt_rows(connection)?
        || validate_outbox_rows(connection)?
    {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    validate_integrity(connection)
}

fn validate_key_rows(
    connection: &Connection,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_key_registry
                 WHERE service != ?1 OR length(public_key) != 65
                    OR key_id NOT LIKE 'sha256:ecdsa-p256:%'
                    OR key_generation <= 0
                    OR enrollment_generation <= 0
                    OR enrollment_generation > 9223372036854775807
                    OR authority_generation <= 0
                    OR key_state NOT IN ('active','revoked')
            )",
            [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_receipt_rows(
    connection: &Connection,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    connection
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_receipt
                 WHERE service != ?1 OR length(wire) = 0
                    OR key_generation <= 0
                    OR enrollment_generation <= 0
                    OR enrollment_generation > 9223372036854775807
                    OR authority_generation <= 0
                    OR session_generation <= 0
                    OR receipt_state NOT IN ('issued','acknowledged')
                    OR (receipt_state = 'issued' AND ack_wire IS NOT NULL)
                    OR (receipt_state = 'acknowledged' AND (ack_wire IS NULL OR length(ack_wire) = 0))
            )",
            [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_outbox_rows(
    connection: &Connection,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    connection
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
                    OR outbox.enrollment_generation != receipt.enrollment_generation
                    OR outbox.authority_generation != receipt.authority_generation
                    OR outbox.enrollment_generation <= 0
                    OR outbox.enrollment_generation > 9223372036854775807
                    OR outbox.wire != receipt.wire
                    OR outbox.delivery_state NOT IN ('pending','claimed','sent','failed','acknowledged')
                    OR (outbox.delivery_state = 'claimed' AND (
                        outbox.claim_id IS NULL OR outbox.claimed_at IS NULL
                        OR outbox.claim_expires_at IS NULL
                    ))
                    OR (outbox.delivery_state <> 'claimed' AND (
                        outbox.claim_id IS NOT NULL OR outbox.claimed_at IS NOT NULL
                        OR outbox.claim_expires_at IS NOT NULL
                    ))
                    OR (outbox.delivery_state = 'acknowledged' AND (
                        outbox.ack_wire IS NULL OR length(outbox.ack_wire) = 0
                        OR outbox.last_result IS NULL
                    ))
                    OR (outbox.delivery_state <> 'acknowledged' AND outbox.ack_wire IS NOT NULL)
                    OR outbox.last_error_code NOT IN ('delivery_failed','lease_expired')
                    OR (outbox.last_error_code IS NULL AND outbox.last_error_digest IS NOT NULL)
                    OR (outbox.last_error_code = 'delivery_failed'
                        AND outbox.last_error_digest IS NULL)
                    OR (outbox.last_error_code = 'lease_expired'
                        AND outbox.last_error_digest IS NOT NULL)
                    OR (outbox.last_error_digest IS NOT NULL AND (
                        outbox.last_error_digest NOT LIKE 'sha256:delivery-detail:%'
                        OR length(outbox.last_error_digest) !=
                           length('sha256:delivery-detail:') + 64
                        OR substr(
                            outbox.last_error_digest,
                            length('sha256:delivery-detail:') + 1
                        ) GLOB '*[^0-9A-Fa-f]*'
                    ))
            )",
            [ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn validate_integrity(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let integrity = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if integrity != "ok" {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    validate_foreign_keys(connection)
}

fn validate_foreign_keys(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let mut statement = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut rows = statement
        .query([])
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if rows
        .next()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        .is_some()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    Ok(())
}
