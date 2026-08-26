use chrono::{DateTime, SecondsFormat, Utc};
use getrandom::fill;
use ring::digest::{digest, SHA256};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;

use super::super::account_identity_authority_issuer_client_reservation::AccountIdentityIssuerReservation;
use super::super::{AccountIdentityAuthorityIssuerClientError, AccountIdentityIssuerCurrentness};

pub(super) const RESERVATION_PREPARED: &str = "prepared";
pub(super) const RESERVATION_SIGNING: &str = "signing";
pub(super) const RESERVATION_MANUAL_REQUIRED: &str = "manual-required";
pub(super) const RESERVATION_ISSUED: &str = "issued";
pub(super) const SIGNER_NOT_STARTED: &str = "not-started";
pub(super) const SIGNER_IN_FLIGHT: &str = "in-flight";
pub(super) const SIGNER_UNCERTAIN: &str = "uncertain";
pub(super) const SIGNER_SUCCEEDED: &str = "succeeded";

const RESERVATION_LEASE_MILLIS: i64 = 60 * 1_000;
const MAX_UNFINALIZED_RESERVATIONS_PER_ACCOUNT: i64 = 1_024;
const DELETE_EXPIRED_SQL: &str = "DELETE FROM account_identity_issuer_v2_reservation
              WHERE reservation_id = ?1 AND reservation_state = ?2
                AND signer_status = ?3 AND lease_expires_at <= ?4";
const SELECT_EXISTING_SQL: &str = "SELECT reservation_id, account_id, household_id, provider,
                      provider_subject, service, service_binding_id, key_id, key_generation,
                      enrollment_generation, authority_generation, session_generation,
                      correlation_id, idempotency_key, request_digest, request_wire,
                      reservation_state, signer_status, lease_expires_at
                FROM account_identity_issuer_v2_reservation
               WHERE account_id = ?1 AND service = ?2 AND idempotency_key = ?3";
const INSERT_RESERVATION_SQL: &str = "INSERT INTO account_identity_issuer_v2_reservation (
                reservation_id, account_id, household_id, provider, provider_subject,
                service, service_binding_id, key_id, key_generation, enrollment_generation,
                authority_generation, session_generation, correlation_id, idempotency_key,
                request_digest, request_wire, reservation_state, signer_status,
                attempt_token, lease_expires_at, reserved_at, signing_started_at,
                uncertain_at, receipt_id
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                       ?15, ?16, ?17, ?18, ?19, ?20, ?21, NULL, NULL, NULL)";

pub(super) fn reserve_issue(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    request: &AccountIdentityAuthorityProducerV2Request,
    now: i64,
) -> Result<AccountIdentityIssuerReservation, AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    let provider = super::provider_label(currentness.authority().provider()).to_owned();
    let provider_subject = currentness
        .authority()
        .provider_subject()
        .as_str()
        .to_owned();
    let binding = request.binding();
    let request_wire = request.signing_bytes().to_vec();
    let request_digest = request_digest(request_wire.as_slice());
    let lease_expires_at = lease_expires_at(now)?;
    if let Some(existing) = existing_reservation(
        transaction,
        currentness.account_id().as_str(),
        binding.idempotency_key.as_str(),
    )? {
        reconcile_existing_reservation(
            transaction,
            &existing,
            currentness,
            request,
            now_text.as_str(),
            request_digest.as_str(),
            request_wire.as_slice(),
        )?;
    }
    ensure_reservation_capacity(transaction, currentness.account_id().as_str())?;
    insert_reservation(
        transaction,
        currentness,
        binding,
        provider,
        provider_subject,
        request_digest,
        request_wire,
        now_text,
        lease_expires_at,
    )
}

fn reconcile_existing_reservation(
    transaction: &Transaction<'_>,
    existing: &StoredReservation,
    currentness: &AccountIdentityIssuerCurrentness,
    request: &AccountIdentityAuthorityProducerV2Request,
    now_text: &str,
    expected_request_digest: &str,
    expected_request_wire: &[u8],
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if existing.reservation_state == RESERVATION_PREPARED
        && existing.signer_status == SIGNER_NOT_STARTED
    {
        return reconcile_expired_prepared_reservation(
            transaction,
            existing,
            currentness,
            request,
            now_text,
            expected_request_digest,
            expected_request_wire,
        );
    }
    if existing.reservation_state == RESERVATION_SIGNING
        && existing.signer_status == SIGNER_IN_FLIGHT
        && existing.lease_expires_at.as_str() <= now_text
    {
        super::recovery::mark_manual_required(
            transaction,
            existing.reservation_id.as_str(),
            now_text,
        )?;
        return Err(AccountIdentityAuthorityIssuerClientError::ReservationExpired);
    }
    existing_reservation_error(
        existing.reservation_state.as_str(),
        existing.signer_status.as_str(),
    )
}

fn reconcile_expired_prepared_reservation(
    transaction: &Transaction<'_>,
    existing: &StoredReservation,
    currentness: &AccountIdentityIssuerCurrentness,
    request: &AccountIdentityAuthorityProducerV2Request,
    now_text: &str,
    expected_request_digest: &str,
    expected_request_wire: &[u8],
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if existing.lease_expires_at.as_str() > now_text {
        return existing_reservation_error(
            existing.reservation_state.as_str(),
            existing.signer_status.as_str(),
        );
    }
    if !expired_prepared_matches(
        existing,
        currentness,
        request,
        expected_request_digest,
        expected_request_wire,
    )? {
        return Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected);
    }
    transaction
        .execute(
            DELETE_EXPIRED_SQL,
            params![
                existing.reservation_id,
                RESERVATION_PREPARED,
                SIGNER_NOT_STARTED,
                now_text
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    Ok(())
}

fn ensure_reservation_capacity(
    transaction: &Transaction<'_>,
    account_id: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    // Issued rows are compacted only after their exact receipt and outbox
    // records prove replay; those durable records retain the provenance.
    // Outstanding and manual-required rows remain bounded, so callers receive
    // backpressure rather than allowing an unbounded recovery queue to grow.
    let at_capacity: bool = transaction
        .query_row(
            "SELECT EXISTS(
                SELECT reservation_id
                  FROM account_identity_issuer_v2_reservation
                 WHERE account_id = ?1 AND service = ?2
                   AND reservation_state IN ('prepared','signing','manual-required')
                 ORDER BY reservation_id
                 LIMIT 1 OFFSET ?3
             )",
            params![
                account_id,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                MAX_UNFINALIZED_RESERVATIONS_PER_ACCOUNT
            ],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    (!at_capacity)
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

fn existing_reservation(
    transaction: &Transaction<'_>,
    account_id: &str,
    idempotency_key: &str,
) -> Result<Option<StoredReservation>, AccountIdentityAuthorityIssuerClientError> {
    transaction
        .query_row(
            SELECT_EXISTING_SQL,
            params![
                account_id,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                idempotency_key
            ],
            |row| {
                Ok(StoredReservation {
                    reservation_id: row.get(0)?,
                    account_id: row.get(1)?,
                    household_id: row.get(2)?,
                    provider: row.get(3)?,
                    provider_subject: row.get(4)?,
                    service: row.get(5)?,
                    service_binding_id: row.get(6)?,
                    key_id: row.get(7)?,
                    key_generation: row.get(8)?,
                    enrollment_generation: row.get(9)?,
                    authority_generation: row.get(10)?,
                    session_generation: row.get(11)?,
                    correlation_id: row.get(12)?,
                    idempotency_key: row.get(13)?,
                    request_digest: row.get(14)?,
                    request_wire: row.get(15)?,
                    reservation_state: row.get(16)?,
                    signer_status: row.get(17)?,
                    lease_expires_at: row.get(18)?,
                })
            },
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

struct StoredReservation {
    reservation_id: String,
    account_id: String,
    household_id: String,
    provider: String,
    provider_subject: String,
    service: String,
    service_binding_id: String,
    key_id: String,
    key_generation: i64,
    enrollment_generation: i64,
    authority_generation: i64,
    session_generation: i64,
    correlation_id: String,
    idempotency_key: String,
    request_digest: String,
    request_wire: Vec<u8>,
    reservation_state: String,
    signer_status: String,
    lease_expires_at: String,
}

fn expired_prepared_matches(
    existing: &StoredReservation,
    currentness: &AccountIdentityIssuerCurrentness,
    request: &AccountIdentityAuthorityProducerV2Request,
    expected_request_digest: &str,
    expected_request_wire: &[u8],
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    let binding = request.binding();
    Ok(existing.account_id == currentness.account_id().as_str()
        && existing.household_id == currentness.household_id().as_str()
        && existing.provider == super::provider_label(currentness.authority().provider())
        && existing.provider_subject == currentness.authority().provider_subject().as_str()
        && existing.service == ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
        && existing.service_binding_id == binding.service_binding_id
        && existing.key_id == binding.key_id
        && existing.key_generation == sql_generation(binding.key_generation)?
        && existing.enrollment_generation == sql_generation(binding.enrollment_generation)?
        && existing.authority_generation == sql_generation(binding.authority_generation)?
        && existing.session_generation == sql_generation(binding.session_generation)?
        && existing.correlation_id == binding.correlation_id
        && existing.idempotency_key == binding.idempotency_key
        && existing.request_digest == expected_request_digest
        && existing.request_wire.as_slice() == expected_request_wire)
}

fn existing_reservation_error(
    state: &str,
    signer_status: &str,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    match state {
        RESERVATION_MANUAL_REQUIRED => {
            Err(AccountIdentityAuthorityIssuerClientError::ManualRequired)
        }
        RESERVATION_ISSUED => Err(AccountIdentityAuthorityIssuerClientError::ReplayDetected),
        RESERVATION_PREPARED if signer_status == SIGNER_NOT_STARTED => {
            Err(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
        }
        RESERVATION_SIGNING if signer_status == SIGNER_IN_FLIGHT => {
            Err(AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
        }
        _ => Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema),
    }
}

fn insert_reservation(
    transaction: &Transaction<'_>,
    currentness: &AccountIdentityIssuerCurrentness,
    binding: &ocentra_schema::account_identity_authority_producer_v2::
        AccountIdentityAuthorityProducerV2Binding,
    provider: String,
    provider_subject: String,
    request_digest: String,
    request_wire: Vec<u8>,
    now_text: String,
    lease_expires_at: String,
) -> Result<AccountIdentityIssuerReservation, AccountIdentityAuthorityIssuerClientError> {
    let reservation_id = opaque_token("reservation")?;
    let attempt_token = opaque_token("attempt")?;
    transaction
        .execute(
            INSERT_RESERVATION_SQL,
            params![
                reservation_id,
                currentness.account_id().as_str(),
                currentness.household_id().as_str(),
                provider,
                provider_subject,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                binding.service_binding_id,
                binding.key_id,
                sql_generation(binding.key_generation)?,
                sql_generation(binding.enrollment_generation)?,
                sql_generation(binding.authority_generation)?,
                sql_generation(binding.session_generation)?,
                binding.correlation_id,
                binding.idempotency_key,
                request_digest,
                request_wire,
                RESERVATION_PREPARED,
                SIGNER_NOT_STARTED,
                attempt_token,
                lease_expires_at,
                now_text,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    Ok(AccountIdentityIssuerReservation::from_storage(
        reservation_id,
        currentness.account_id().as_str().to_owned(),
        currentness.household_id().as_str().to_owned(),
        provider,
        provider_subject,
        binding.service_binding_id.clone(),
        binding.key_id.clone(),
        binding.key_generation,
        binding.enrollment_generation,
        binding.authority_generation,
        binding.session_generation,
        binding.correlation_id.clone(),
        binding.idempotency_key.clone(),
        request_wire,
        attempt_token,
    ))
}

pub(super) fn opaque_token(
    prefix: &str,
) -> Result<String, AccountIdentityAuthorityIssuerClientError> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
    Ok(format!("{prefix}:{}", hex(bytes.as_slice())))
}

pub(super) fn request_digest(request_wire: &[u8]) -> String {
    format!(
        "sha256:request:{}",
        hex(digest(&SHA256, request_wire).as_ref())
    )
}

pub(super) fn lease_expires_at(
    now: i64,
) -> Result<String, AccountIdentityAuthorityIssuerClientError> {
    let lease = now
        .checked_add(RESERVATION_LEASE_MILLIS)
        .ok_or(AccountIdentityAuthorityIssuerClientError::ClockUnavailable)?;
    timestamp(lease)
}

pub(super) fn timestamp(value: i64) -> Result<String, AccountIdentityAuthorityIssuerClientError> {
    DateTime::<Utc>::from_timestamp_millis(value)
        .map(|value| value.to_rfc3339_opts(SecondsFormat::Millis, true))
        .ok_or(AccountIdentityAuthorityIssuerClientError::ClockUnavailable)
}

pub(super) fn sql_generation(value: u64) -> Result<i64, AccountIdentityAuthorityIssuerClientError> {
    i64::try_from(value).map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

pub(super) fn hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(value.len() * 2);
    for byte in value {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}
