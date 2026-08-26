use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority_producer_v2;

use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerReceiptProof;
use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityIssuerCurrentness,
    AccountIdentityIssuerV2KeyRecord,
};
use super::receipt::{
    from_sql, validate_issue_receipt, validate_receipt_key, validate_verified_receipt,
};

pub(super) fn load_receipt_proof(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    key: &AccountIdentityIssuerV2KeyRecord,
    receipt_id: &str,
) -> Result<AccountIdentityIssuerReceiptProof, AccountIdentityAuthorityIssuerClientError> {
    let (receipt, wire, outbox_state) = load_receipt_row(transaction, currentness, receipt_id)?;
    if outbox_state == "acknowledged" {
        return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
    }
    validate_issue_receipt(currentness, &receipt)?;
    validate_receipt_key(key, &receipt)?;
    let transport = account_identity_authority_producer_v2::from_durable_transport(
        wire.clone(),
        receipt.clone(),
    );
    let (_, now) = super::super::clock::now(transaction)?;
    let now = super::super::clock::parse_timestamp(&now)?;
    let verified = account_identity_authority_producer_v2::verify(
        transport.wire_bytes(),
        key.public_key(),
        now,
    )?;
    validate_verified_receipt(&verified, &receipt)?;
    Ok(AccountIdentityIssuerReceiptProof { receipt, wire })
}

fn load_receipt_row(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    receipt_id: &str,
) -> Result<
    (AccountIdentityAuthorityProducerV2Receipt, Vec<u8>, String),
    AccountIdentityAuthorityIssuerClientError,
> {
    transaction
        .query_row(
            "SELECT receipt.receipt_id, receipt.account_id, receipt.household_id,
                    receipt.service_binding_id, receipt.key_id, receipt.key_generation,
                    receipt.authority_generation, receipt.session_generation,
                    receipt.correlation_id, receipt.idempotency_key, receipt.payload_digest,
                    receipt.issued_at, receipt.expires_at, receipt.wire, outbox.delivery_state
               FROM account_identity_issuer_v2_receipt AS receipt
               JOIN account_identity_issuer_v2_outbox AS outbox
                 ON outbox.receipt_id = receipt.receipt_id
              WHERE receipt.receipt_id = ?1 AND receipt.account_id = ?2
                AND receipt.household_id = ?3 AND receipt.service = ?4
                AND outbox.account_id = receipt.account_id
                AND outbox.household_id = receipt.household_id
                AND outbox.service = receipt.service
                AND outbox.service_binding_id = receipt.service_binding_id
                AND outbox.key_id = receipt.key_id
                AND outbox.key_generation = receipt.key_generation
                AND outbox.authority_generation = receipt.authority_generation
                AND outbox.wire = receipt.wire",
            params![
                receipt_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
            ],
            |row| {
                Ok((
                    AccountIdentityAuthorityProducerV2Receipt {
                        receipt_id: row.get(0)?,
                        operation:
                            AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
                        account_id: row.get(1)?,
                        household_id: row.get(2)?,
                        service_binding_id: row.get(3)?,
                        key_id: row.get(4)?,
                        key_generation: from_sql(row.get(5)?)?,
                        authority_generation: from_sql(row.get(6)?)?,
                        session_generation: from_sql(row.get(7)?)?,
                        correlation_id: row.get(8)?,
                        idempotency_key: row.get(9)?,
                        payload_digest: row.get(10)?,
                        issued_at: row.get(11)?,
                        expires_at: row.get(12)?,
                    },
                    row.get(13)?,
                    row.get(14)?,
                ))
            },
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)
}
