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
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
        protected_receipt_wire: &[u8],
    ) -> Result<AccountIdentityAuthorityProducerV2Receipt, AccountIdentityAuthorityIssuerClientError>
    {
        self.transaction
            .execute_batch("SAVEPOINT account_identity_issuer_ack")
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
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
        &mut self,
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
                AND service = ?5 AND service_binding_id = ?6 AND key_id = ?7
                AND key_generation = ?8 AND enrollment_generation = ?9
                AND authority_generation = ?10 AND session_generation = ?11
                AND correlation_id = ?12 AND idempotency_key = ?13
                AND payload_digest = ?14 AND wire = ?15
                AND receipt_state = 'issued' AND ack_wire IS NULL",
            params![
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
                super::receipt::sql_generation(receipt.session_generation)?,
                receipt.correlation_id,
                receipt.idempotency_key,
                receipt.payload_digest,
                stored_wire,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
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
                AND delivery_state = 'claimed' AND ack_wire IS NULL",
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
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
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
