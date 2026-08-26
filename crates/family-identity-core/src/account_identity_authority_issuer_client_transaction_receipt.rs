use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use ring::digest::{digest, SHA256};
use rusqlite::{params, Transaction};

use crate::account_identity_authority_producer_v2::{
    self, AccountIdentityAuthorityProducerV2Error, AccountIdentityAuthorityProducerV2Transport,
};

use super::super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerOutboxClaim, AccountIdentityIssuerRecordedTransport,
};
use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerV2KeyRecord,
};

impl<'a> AccountIdentityAuthorityIssuerTransaction<'a> {
    pub fn record_issued_transport(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        transport: &AccountIdentityAuthorityProducerV2Transport,
    ) -> Result<AccountIdentityIssuerRecordedTransport, AccountIdentityAuthorityIssuerClientError>
    {
        self.ensure_current(currentness)?;
        let receipt = transport.receipt();
        validate_issue_receipt(currentness, receipt)?;
        let key = self.current_key(currentness)?;
        validate_receipt_key(&key, receipt)?;
        verify_transport(&self.transaction, currentness, &key, transport)?;
        if let Some(existing) = super::replay::load_stored_issue_by_idempotency(
            &self.transaction,
            currentness,
            receipt.idempotency_key.as_str(),
        )? {
            if !same_receipt_identity(&existing, receipt) {
                return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
            }
            let winner =
                super::replay::verify_stored_issue(&self.transaction, currentness, &key, existing)?;
            return Ok(AccountIdentityIssuerRecordedTransport {
                transport: winner,
                replayed: true,
            });
        }
        insert_receipt(&self.transaction, currentness, &key, transport)?;
        insert_outbox(&self.transaction, currentness, &key, transport)?;
        Ok(AccountIdentityIssuerRecordedTransport {
            transport: transport.clone_durable(),
            replayed: false,
        })
    }

    pub fn acknowledge_receipt(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
        protected_receipt_wire: &[u8],
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        self.transaction
            .execute_batch("SAVEPOINT account_identity_issuer_ack")
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
        let result = self.acknowledge_receipt_inner(currentness, claim, protected_receipt_wire);
        if let Err(error) = result {
            rollback_ack_savepoint(&self.transaction);
            return Err(error);
        }
        self.transaction
            .execute_batch("RELEASE account_identity_issuer_ack")
            .map_err(|_| {
                rollback_ack_savepoint(&self.transaction);
                AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable
            })
    }

    fn acknowledge_receipt_inner(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        claim: &AccountIdentityIssuerOutboxClaim,
        protected_receipt_wire: &[u8],
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
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
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
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
    Ok(())
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
                sql_generation(receipt.key_generation)?,
                sql_generation(receipt.enrollment_generation)?,
                sql_generation(receipt.authority_generation)?,
                sql_generation(receipt.session_generation)?,
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
                sql_generation(receipt.key_generation)?,
                sql_generation(receipt.enrollment_generation)?,
                sql_generation(receipt.authority_generation)?,
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

fn verify_transport(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    key: &AccountIdentityIssuerV2KeyRecord,
    transport: &AccountIdentityAuthorityProducerV2Transport,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let (_, now) = super::super::clock::now(transaction)?;
    let now = super::super::clock::parse_timestamp(&now)?;
    let verified = account_identity_authority_producer_v2::verify(
        transport.wire_bytes(),
        key.public_key(),
        now,
    )?;
    validate_verified_receipt(&verified, transport.receipt())?;
    if transport.receipt().account_id != currentness.account_id().as_str()
        || transport.receipt().household_id != currentness.household_id().as_str()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::CurrentnessRejected);
    }
    Ok(())
}

pub(super) fn validate_issue_receipt(
    currentness: &AccountIdentityIssuerCurrentness,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if receipt.operation != AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority {
        return Err(AccountIdentityAuthorityIssuerClientError::Producer(
            AccountIdentityAuthorityProducerV2Error::UnsupportedOperation,
        ));
    }
    receipt.validate_shape().map_err(|_| {
        AccountIdentityAuthorityIssuerClientError::Producer(
            AccountIdentityAuthorityProducerV2Error::InvalidWire,
        )
    })?;
    if receipt.account_id != currentness.account_id().as_str()
        || receipt.household_id != currentness.household_id().as_str()
        || receipt.authority_generation != currentness.authority_generation()
        || receipt.session_generation != currentness.session_generation()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::CurrentnessRejected);
    }
    Ok(())
}

pub(super) fn validate_receipt_key(
    key: &AccountIdentityIssuerV2KeyRecord,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if receipt.key_id != key.key_id().as_str()
        || receipt.key_generation != key.key_generation()
        || receipt.enrollment_generation != key.enrollment_generation()
        || receipt.service_binding_id != key.service_binding_id().as_str()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::KeyUnavailable);
    }
    Ok(())
}

pub(super) fn validate_verified_receipt(
    verified: &crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Verified,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if verified.operation() != AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority
        || verified.key_id() != receipt.key_id
        || verified.service_binding_id() != receipt.service_binding_id
        || verified.key_generation() != receipt.key_generation
        || verified.enrollment_generation() != receipt.enrollment_generation
        || verified.authority_generation() != receipt.authority_generation
        || verified.session_generation() != receipt.session_generation
        || verified.correlation_id() != receipt.correlation_id
        || verified.idempotency_key() != receipt.idempotency_key
        || verified.payload_digest() != receipt.payload_digest
        || verified.receipt_id() != receipt.receipt_id
        || verified.claims().account_id != receipt.account_id
        || verified.claims().household_id != receipt.household_id
    {
        return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
    }
    Ok(())
}

fn same_receipt_identity(
    left: &super::replay::StoredIssue,
    right: &AccountIdentityAuthorityProducerV2Receipt,
) -> bool {
    left.receipt_id == right.receipt_id
        && left.account_id == right.account_id
        && left.household_id == right.household_id
        && left.service == ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
        && left.service_binding_id == right.service_binding_id
        && left.key_id == right.key_id
        && left.key_generation == right.key_generation as i64
        && left.enrollment_generation == right.enrollment_generation as i64
        && left.authority_generation == right.authority_generation as i64
        && left.session_generation == right.session_generation as i64
        && left.correlation_id == right.correlation_id
        && left.idempotency_key == right.idempotency_key
        && left.payload_digest == right.payload_digest
        && left.issued_at == right.issued_at
        && left.expires_at == right.expires_at
        && left.wire.len() > 0
}

fn insert_receipt(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    key: &AccountIdentityIssuerV2KeyRecord,
    transport: &AccountIdentityAuthorityProducerV2Transport,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let receipt = transport.receipt();
    transaction
        .execute(
            "INSERT INTO account_identity_issuer_v2_receipt (
                receipt_id, account_id, household_id, service, service_binding_id, key_id,
                key_generation, enrollment_generation, authority_generation, session_generation,
                correlation_id,
                idempotency_key, payload_digest, issued_at, expires_at, wire, ack_wire,
                receipt_state
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16,
                       NULL, 'issued')",
            params![
                receipt.receipt_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                key.service_binding_id().as_str(),
                receipt.key_id,
                sql_generation(receipt.key_generation)?,
                sql_generation(receipt.enrollment_generation)?,
                sql_generation(receipt.authority_generation)?,
                sql_generation(receipt.session_generation)?,
                receipt.correlation_id,
                receipt.idempotency_key,
                receipt.payload_digest,
                receipt.issued_at,
                receipt.expires_at,
                transport.wire_bytes(),
            ],
        )
        .map(|_| ())
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)
}

fn insert_outbox(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    key: &AccountIdentityIssuerV2KeyRecord,
    transport: &AccountIdentityAuthorityProducerV2Transport,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let receipt = transport.receipt();
    transaction
        .execute(
            "INSERT INTO account_identity_issuer_v2_outbox (
                receipt_id, account_id, household_id, service, service_binding_id,
                key_id, key_generation, enrollment_generation, authority_generation, wire,
                delivery_state,
                claim_id, claimed_at, claim_expires_at, attempt_count,
                last_error_code, last_error_digest, last_result, ack_wire, next_attempt_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'pending',
                       NULL, NULL, NULL, 0, NULL, NULL, NULL, NULL, NULL)",
            params![
                receipt.receipt_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                key.service_binding_id().as_str(),
                receipt.key_id,
                sql_generation(receipt.key_generation)?,
                sql_generation(receipt.enrollment_generation)?,
                sql_generation(receipt.authority_generation)?,
                transport.wire_bytes(),
            ],
        )
        .map(|_| ())
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)
}

fn sql_generation(value: u64) -> Result<i64, AccountIdentityAuthorityIssuerClientError> {
    i64::try_from(value).map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

pub(super) fn from_sql(value: i64) -> rusqlite::Result<u64> {
    u64::try_from(value).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, value))
}
