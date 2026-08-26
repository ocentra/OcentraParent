use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use rusqlite::{params, Transaction};

use crate::account_identity_authority_producer_v2::{
    self, AccountIdentityAuthorityProducerV2Error, AccountIdentityAuthorityProducerV2Transport,
};

use super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::account_identity_authority_issuer_client_types::AccountIdentityIssuerRecordedTransport;
use super::super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityAuthorityIssuerTransaction,
    AccountIdentityIssuerCurrentness, AccountIdentityIssuerV2KeyRecord,
};

impl<'a> AccountIdentityAuthorityIssuerTransaction<'a> {
    pub(crate) fn record_issued_transport(
        &mut self,
        currentness: &AccountIdentityIssuerCurrentness,
        reservation: AccountIdentityIssuerReservation,
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
            if existing.provenance_state != "exact"
                || existing.provider.as_deref()
                    != Some(super::provider_label(currentness.authority().provider()))
                || existing.provider_subject.as_deref()
                    != Some(currentness.authority().provider_subject().as_str())
            {
                return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
            }
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
        super::reservation_validation::validate_signing_reservation(
            &self.transaction,
            currentness,
            &reservation,
            transport,
        )?;
        insert_receipt(&self.transaction, currentness, &key, transport)?;
        insert_outbox(&self.transaction, currentness, &key, transport)?;
        super::recovery::mark_issued(&self.transaction, &reservation, receipt.receipt_id.as_str())?;
        compact_issued_reservation(&self.transaction, &reservation, receipt, transport)?;
        Ok(AccountIdentityIssuerRecordedTransport {
            transport: transport.clone_durable(),
            replayed: false,
        })
    }
}

fn compact_issued_reservation(
    transaction: &Transaction<'_>,
    reservation: &AccountIdentityIssuerReservation,
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
    transport: &AccountIdentityAuthorityProducerV2Transport,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let changed = transaction
        .execute(
            "DELETE FROM account_identity_issuer_v2_reservation
              WHERE reservation_id = ?1 AND attempt_token = ?2
                AND reservation_state = 'issued' AND signer_status = 'succeeded'
                AND receipt_id = ?3
                AND request_digest = ?4 AND request_wire = ?5
                AND EXISTS (
                    SELECT 1
                      FROM account_identity_issuer_v2_receipt AS stored_receipt
                     WHERE stored_receipt.receipt_id = ?3
                       AND stored_receipt.provenance_state = 'exact'
                       AND stored_receipt.receipt_state IN ('issued','acknowledged')
                       AND stored_receipt.account_id = account_identity_issuer_v2_reservation.account_id
                       AND stored_receipt.household_id = account_identity_issuer_v2_reservation.household_id
                       AND stored_receipt.provider = account_identity_issuer_v2_reservation.provider
                       AND stored_receipt.provider_subject = account_identity_issuer_v2_reservation.provider_subject
                       AND stored_receipt.service = account_identity_issuer_v2_reservation.service
                       AND stored_receipt.service_binding_id = account_identity_issuer_v2_reservation.service_binding_id
                       AND stored_receipt.key_id = account_identity_issuer_v2_reservation.key_id
                       AND stored_receipt.key_generation = account_identity_issuer_v2_reservation.key_generation
                       AND stored_receipt.enrollment_generation = account_identity_issuer_v2_reservation.enrollment_generation
                       AND stored_receipt.authority_generation = account_identity_issuer_v2_reservation.authority_generation
                       AND stored_receipt.session_generation = account_identity_issuer_v2_reservation.session_generation
                       AND stored_receipt.correlation_id = account_identity_issuer_v2_reservation.correlation_id
                       AND stored_receipt.idempotency_key = account_identity_issuer_v2_reservation.idempotency_key
                       AND stored_receipt.payload_digest = ?6
                       AND stored_receipt.issued_at = ?7
                       AND stored_receipt.expires_at = ?8
                       AND stored_receipt.wire = ?9
                )
                AND EXISTS (
                    SELECT 1
                      FROM account_identity_issuer_v2_outbox AS stored_outbox
                     WHERE stored_outbox.receipt_id = ?3
                       AND stored_outbox.account_id = account_identity_issuer_v2_reservation.account_id
                       AND stored_outbox.household_id = account_identity_issuer_v2_reservation.household_id
                       AND stored_outbox.service = account_identity_issuer_v2_reservation.service
                       AND stored_outbox.service_binding_id = account_identity_issuer_v2_reservation.service_binding_id
                       AND stored_outbox.key_id = account_identity_issuer_v2_reservation.key_id
                       AND stored_outbox.key_generation = account_identity_issuer_v2_reservation.key_generation
                       AND stored_outbox.enrollment_generation = account_identity_issuer_v2_reservation.enrollment_generation
                       AND stored_outbox.authority_generation = account_identity_issuer_v2_reservation.authority_generation
                       AND stored_outbox.wire = ?9
                )",
            params![
                reservation.reservation_id(),
                reservation.attempt_token(),
                receipt.receipt_id,
                super::reservation::request_digest(reservation.request_wire()),
                reservation.request_wire(),
                receipt.payload_digest,
                receipt.issued_at,
                receipt.expires_at,
                transport.wire_bytes(),
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
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
    validate_verified_currentness(currentness, &verified)?;
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

pub(super) fn validate_verified_currentness(
    currentness: &AccountIdentityIssuerCurrentness,
    verified: &crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Verified,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let handoff = currentness.authority().handoff();
    let claims = verified.claims();
    if claims.account_id != currentness.account_id().as_str()
        || claims.household_id != currentness.household_id().as_str()
        || super::provider_label(&handoff.mapping.provider) != claims.provider
        || handoff.mapping.provider_subject.as_str() != claims.provider_subject
        || handoff.member.member_id.as_str() != claims.member_id
        || handoff.member.device_id.as_str() != claims.device_id
        || handoff.member.session_id.as_str() != claims.session_id
        || handoff.member.authority_generation != verified.authority_generation()
        || handoff.member.session_generation != verified.session_generation()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::CurrentnessRejected);
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
                receipt_state, provider, provider_subject, provenance_state
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16,
                       NULL, 'issued', ?17, ?18, 'exact')",
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
                super::provider_label(currentness.authority().provider()),
                currentness.authority().provider_subject().as_str(),
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

pub(super) fn sql_generation(value: u64) -> Result<i64, AccountIdentityAuthorityIssuerClientError> {
    i64::try_from(value).map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

pub(super) fn from_sql(value: i64) -> rusqlite::Result<u64> {
    u64::try_from(value).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, value))
}
