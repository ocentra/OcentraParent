use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority_producer_v2;

use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim;
use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityIssuerCurrentness,
    AccountIdentityIssuerV2KeyRecord,
};
use super::receipt::{
    from_sql, validate_issue_receipt, validate_receipt_key, validate_verified_currentness,
    validate_verified_receipt,
};

pub(super) struct VerifiedClaimedIssue {
    pub(super) receipt: AccountIdentityAuthorityProducerV2Receipt,
    pub(super) wire: Vec<u8>,
}

pub(super) fn load_verified_claimed_issue(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    key: &AccountIdentityIssuerV2KeyRecord,
    claim: &AccountIdentityIssuerOutboxClaim,
) -> Result<VerifiedClaimedIssue, AccountIdentityAuthorityIssuerClientError> {
    let (receipt, wire, outbox_state) = load_claimed_receipt_row(transaction, currentness, claim)?;
    if outbox_state != "claimed" {
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
    validate_verified_currentness(currentness, &verified)?;
    Ok(VerifiedClaimedIssue { receipt, wire })
}

fn load_claimed_receipt_row(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    claim: &AccountIdentityIssuerOutboxClaim,
) -> Result<
    (AccountIdentityAuthorityProducerV2Receipt, Vec<u8>, String),
    AccountIdentityAuthorityIssuerClientError,
> {
    let (_, now_text) = super::super::clock::now(transaction)?;
    transaction
        .query_row(
            "SELECT receipt.receipt_id, receipt.account_id, receipt.household_id,
                    receipt.service_binding_id, receipt.key_id, receipt.key_generation,
                    receipt.enrollment_generation, receipt.authority_generation,
                    receipt.session_generation,
                    receipt.correlation_id, receipt.idempotency_key, receipt.payload_digest,
                    receipt.issued_at, receipt.expires_at, receipt.wire, outbox.delivery_state
               FROM account_identity_issuer_v2_receipt AS receipt
               JOIN account_identity_issuer_v2_outbox AS outbox
                 ON outbox.receipt_id = receipt.receipt_id
              WHERE receipt.receipt_id = ?1 AND receipt.account_id = ?2
                AND receipt.household_id = ?3 AND receipt.service = ?4
                AND receipt.provider = ?5 AND receipt.provider_subject = ?6
                AND receipt.provenance_state = 'exact'
                AND receipt.receipt_state = 'issued' AND receipt.ack_wire IS NULL
                AND outbox.claim_id = ?7 AND outbox.claim_expires_at = ?8
                AND outbox.delivery_state = 'claimed'
                AND outbox.account_id = receipt.account_id
                AND outbox.household_id = receipt.household_id
                AND outbox.service = receipt.service
                AND outbox.service_binding_id = receipt.service_binding_id
                AND outbox.key_id = receipt.key_id
                AND outbox.key_generation = receipt.key_generation
                AND outbox.enrollment_generation = receipt.enrollment_generation
                AND outbox.authority_generation = receipt.authority_generation
                AND outbox.wire = receipt.wire
                AND outbox.wire = ?9 AND outbox.claim_expires_at > ?10",
            params![
                claim.receipt_id(),
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                super::provider_label(currentness.authority().provider()),
                currentness.authority().provider_subject().as_str(),
                claim.claim_id(),
                claim.claim_expires_at(),
                claim.wire(),
                now_text,
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
                        enrollment_generation: from_sql(row.get(6)?)?,
                        authority_generation: from_sql(row.get(7)?)?,
                        session_generation: from_sql(row.get(8)?)?,
                        correlation_id: row.get(9)?,
                        idempotency_key: row.get(10)?,
                        payload_digest: row.get(11)?,
                        issued_at: row.get(12)?,
                        expires_at: row.get(13)?,
                    },
                    row.get(14)?,
                    row.get(15)?,
                ))
            },
        )
        .optional()
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)
}
