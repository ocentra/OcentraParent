use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority_producer_v2::{
    self, AccountIdentityAuthorityProducerV2Error, AccountIdentityAuthorityProducerV2Transport,
};

use super::super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerReceiptProof, AccountIdentityIssuerRecordedTransport,
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

    pub fn prepare_receipt_proof(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        receipt_id: &str,
    ) -> Result<AccountIdentityIssuerReceiptProof, AccountIdentityAuthorityIssuerClientError> {
        self.ensure_current(currentness)?;
        let key = self.current_key(currentness)?;
        super::receipt_load::load_receipt_proof(&self.transaction, currentness, &key, receipt_id)
    }

    pub fn acknowledge_receipt(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        proof: &AccountIdentityIssuerReceiptProof,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        self.ensure_current(currentness)?;
        let key = self.current_key(currentness)?;
        let stored = super::receipt_load::load_receipt_proof(
            &self.transaction,
            currentness,
            &key,
            proof.receipt_id(),
        )?;
        if stored.receipt != proof.receipt || stored.wire != proof.wire {
            return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
        }
        let receipt = stored.receipt;
        let changed = self
            .transaction
            .execute(
                "UPDATE account_identity_issuer_v2_receipt
                SET receipt_state = 'acknowledged'
              WHERE receipt_id = ?1 AND account_id = ?2 AND household_id = ?3
                AND service = ?4 AND service_binding_id = ?5 AND key_id = ?6
                AND key_generation = ?7 AND authority_generation = ?8
                AND session_generation = ?9 AND correlation_id = ?10
                AND idempotency_key = ?11 AND payload_digest = ?12
                AND wire = ?13 AND receipt_state = 'issued'",
                params![
                    receipt.receipt_id,
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                    receipt.service_binding_id,
                    receipt.key_id,
                    sql_generation(receipt.key_generation)?,
                    sql_generation(receipt.authority_generation)?,
                    sql_generation(receipt.session_generation)?,
                    receipt.correlation_id,
                    receipt.idempotency_key,
                    receipt.payload_digest,
                    stored.wire,
                ],
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
        if changed != 1 {
            return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
        }
        let outbox_changed = self
            .transaction
            .execute(
                "UPDATE account_identity_issuer_v2_outbox
                SET delivery_state = 'acknowledged', claim_id = NULL,
                    claimed_at = NULL, last_result = ?1
              WHERE receipt_id = ?2 AND account_id = ?3 AND household_id = ?4
                AND service = ?5 AND service_binding_id = ?6 AND key_id = ?7
                AND key_generation = ?8 AND authority_generation = ?9
                AND wire = ?10 AND delivery_state IN ('pending','claimed','sent','failed')",
                params![
                    receipt.receipt_id,
                    receipt.receipt_id,
                    currentness.account_id().as_str(),
                    currentness.household_id().as_str(),
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                    receipt.service_binding_id,
                    receipt.key_id,
                    sql_generation(receipt.key_generation)?,
                    sql_generation(receipt.authority_generation)?,
                    stored.wire,
                ],
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable)?;
        if outbox_changed != 1 {
            return Err(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable);
        }
        Ok(())
    }
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
        && left.correlation_id == right.correlation_id
        && left.idempotency_key == right.idempotency_key
        && left.payload_digest == right.payload_digest
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
                key_generation, authority_generation, session_generation, correlation_id,
                idempotency_key, payload_digest, issued_at, expires_at, wire, receipt_state
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, 'issued')",
            params![
                receipt.receipt_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                key.service_binding_id().as_str(),
                receipt.key_id,
                sql_generation(receipt.key_generation)?,
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
                key_id, key_generation, authority_generation, wire, delivery_state,
                claim_id, claimed_at, attempt_count, last_error, last_result,
                next_attempt_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'pending',
                       NULL, NULL, 0, NULL, NULL, NULL)",
            params![
                receipt.receipt_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                key.service_binding_id().as_str(),
                receipt.key_id,
                sql_generation(receipt.key_generation)?,
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
