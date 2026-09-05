use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Receipt, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use ring::digest::{digest, SHA256};
use rusqlite::{params, Transaction};

use crate::account_identity_authority_producer_v2;

use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim;
use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
    AccountIdentityIssuerCurrentness,
};

impl<'a> AccountIdentityAuthorityIssuerTransaction<'a> {
    pub(crate) fn acknowledge_receipt(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
        protected_receipt_wire: &[u8],
    ) -> Result<AccountIdentityAuthorityProducerV2Receipt, AccountIdentityAuthorityIssuerClientError>
    {
        self.transaction
            .execute_batch("SAVEPOINT account_identity_issuer_ack")
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
        let receipt =
            match self.acknowledge_receipt_inner(currentness, claim, protected_receipt_wire) {
                Ok(receipt) => receipt,
                Err(error) => {
                    rollback_ack_savepoint(&self.transaction);
                    return Err(error);
                }
            };
        match self
            .transaction
            .execute_batch("RELEASE account_identity_issuer_ack")
        {
            Ok(()) => Ok(receipt),
            Err(_) => {
                rollback_ack_savepoint(&self.transaction);
                Err(AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)
            }
        }
    }

    fn acknowledge_receipt_inner(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
        protected_receipt_wire: &[u8],
    ) -> Result<AccountIdentityAuthorityProducerV2Receipt, AccountIdentityAuthorityIssuerClientError>
    {
        self.ensure_current(currentness)?;
        let key = self.current_key(currentness)?;
        let stored = super::receipt_load::load_verified_claimed_issue(
            &self.transaction,
            currentness,
            &key,
            claim,
        )?;
        let (_, now_text) = super::super::clock::now(&self.transaction)?;
        let now = super::super::clock::parse_timestamp(&now_text)?;
        let verified_ack = account_identity_authority_producer_v2::verify_receipt(
            protected_receipt_wire,
            key.public_key(),
            now,
        )?;
        if verified_ack.receipt() != &stored.receipt {
            return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
        }
        acknowledge_receipt_rows(
            &self.transaction,
            currentness,
            claim,
            &stored.receipt,
            stored.wire.as_slice(),
            protected_receipt_wire,
            now_text.as_str(),
        )
    }
}

fn rollback_ack_savepoint(transaction: &Transaction<'_>) {
    let _ = transaction.execute_batch(
        "ROLLBACK TO account_identity_issuer_ack;
         RELEASE account_identity_issuer_ack",
    );
}

fn acknowledge_receipt_rows(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    claim: &AccountIdentityIssuerOutboxClaim,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
    stored_wire: &[u8],
    protected_receipt_wire: &[u8],
    now_text: &str,
) -> Result<AccountIdentityAuthorityProducerV2Receipt, AccountIdentityAuthorityIssuerClientError> {
    update_receipt_row(
        transaction,
        currentness,
        receipt,
        stored_wire,
        protected_receipt_wire,
    )?;
    update_outbox_row(
        transaction,
        currentness,
        claim,
        receipt,
        stored_wire,
        protected_receipt_wire,
        now_text,
    )?;
    Ok(receipt.clone())
}

fn update_receipt_row(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
    stored_wire: &[u8],
    protected_receipt_wire: &[u8],
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_v2_receipt
                SET receipt_state = 'acknowledged', ack_wire = ?1
              WHERE receipt_id = ?2 AND account_id = ?3 AND household_id = ?4
                AND service = ?5 AND provider = ?6 AND provider_subject = ?7
                AND provenance_state = 'exact'
                AND service_binding_id = ?8 AND key_id = ?9
                AND key_generation = ?10 AND enrollment_generation = ?11
                AND authority_generation = ?12 AND session_generation = ?13
                AND correlation_id = ?14 AND idempotency_key = ?15
                AND payload_digest = ?16 AND wire = ?17
                AND receipt_state = 'issued' AND ack_wire IS NULL",
            params![
                protected_receipt_wire,
                receipt.receipt_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                super::provider_label(currentness.authority().provider()),
                currentness.authority().provider_subject().as_str(),
                receipt.service_binding_id,
                receipt.key_id,
                super::receipt::sql_generation(receipt.key_generation)?,
                super::receipt::sql_generation(receipt.enrollment_generation)?,
                super::receipt::sql_generation(receipt.authority_generation)?,
                super::receipt::sql_generation(receipt.session_generation)?,
                receipt.correlation_id,
                receipt.idempotency_key,
                receipt.payload_digest,
                stored_wire,
            ],
        )
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
    if changed == 1 {
        Ok(())
    } else {
        Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected)
    }
}

fn update_outbox_row(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    claim: &AccountIdentityIssuerOutboxClaim,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
    stored_wire: &[u8],
    protected_receipt_wire: &[u8],
    now_text: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_v2_outbox
                SET delivery_state = 'acknowledged', claim_id = NULL,
                    claimed_at = NULL, claim_expires_at = NULL,
                    last_error_code = NULL, last_error_digest = NULL,
                    last_result = ?1, ack_wire = ?2
              WHERE receipt_id = ?3 AND account_id = ?4 AND household_id = ?5
                AND service = ?6 AND service_binding_id = ?7 AND key_id = ?8
                AND key_generation = ?9 AND enrollment_generation = ?10
                AND authority_generation = ?11 AND wire = ?12
                AND claim_id = ?13 AND claim_expires_at = ?14
                AND claim_expires_at > ?15
                AND delivery_state = 'claimed' AND ack_wire IS NULL
                AND EXISTS (
                    SELECT 1 FROM account_identity_issuer_v2_receipt AS receipt
                     WHERE receipt.receipt_id = ?3 AND receipt.account_id = ?4
                       AND receipt.household_id = ?5 AND receipt.service = ?6
                       AND receipt.service_binding_id = ?7 AND receipt.key_id = ?8
                       AND receipt.key_generation = ?9
                       AND receipt.enrollment_generation = ?10
                       AND receipt.authority_generation = ?11
                       AND receipt.session_generation = ?18
                       AND receipt.provider = ?16 AND receipt.provider_subject = ?17
                       AND receipt.provenance_state = 'exact'
                       AND receipt.correlation_id = ?19
                       AND receipt.idempotency_key = ?20
                       AND receipt.payload_digest = ?21
                       AND receipt.wire = ?12
                       AND receipt.receipt_state = 'acknowledged'
                       AND receipt.ack_wire = ?2
                )",
            params![
                ack_result_digest(protected_receipt_wire),
                protected_receipt_wire,
                receipt.receipt_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                receipt.service_binding_id,
                receipt.key_id,
                super::receipt::sql_generation(receipt.key_generation)?,
                super::receipt::sql_generation(receipt.enrollment_generation)?,
                super::receipt::sql_generation(receipt.authority_generation)?,
                stored_wire,
                claim.claim_id(),
                claim.claim_expires_at(),
                now_text,
                super::provider_label(currentness.authority().provider()),
                currentness.authority().provider_subject().as_str(),
                super::receipt::sql_generation(receipt.session_generation)?,
                receipt.correlation_id,
                receipt.idempotency_key,
                receipt.payload_digest,
            ],
        )
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
    if changed == 1 {
        Ok(())
    } else {
        Err(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)
    }
}

fn ack_result_digest(wire: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let value = digest(&SHA256, wire);
    let mut hex = String::with_capacity(value.as_ref().len() * 2);
    for byte in value.as_ref() {
        hex.push(HEX[(byte >> 4) as usize] as char);
        hex.push(HEX[(byte & 0x0f) as usize] as char);
    }
    format!("sha256:delivery-ack:{hex}")
}
