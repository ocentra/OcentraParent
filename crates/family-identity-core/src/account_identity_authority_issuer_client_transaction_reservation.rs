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
const DELETE_EXPIRED_SQL: &str = "DELETE FROM account_identity_issuer_v2_reservation
              WHERE account_id = ?1 AND service = ?2 AND idempotency_key = ?3
                AND reservation_state = ?4 AND lease_expires_at <= ?5";
const SELECT_EXISTING_SQL: &str = "SELECT reservation_state, signer_status
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
    transaction
        .execute(
            DELETE_EXPIRED_SQL,
            params![
                currentness.account_id().as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                binding.idempotency_key,
                RESERVATION_PREPARED,
                now_text
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    if let Some((state, signer_status)) = existing_reservation(
        transaction,
        currentness.account_id().as_str(),
        binding.idempotency_key.as_str(),
    )? {
        return existing_reservation_error(state.as_str(), signer_status.as_str());
    }
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

fn existing_reservation(
    transaction: &Transaction<'_>,
    account_id: &str,
    idempotency_key: &str,
) -> Result<Option<(String, String)>, AccountIdentityAuthorityIssuerClientError> {
    transaction
        .query_row(
            SELECT_EXISTING_SQL,
            params![
                account_id,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                idempotency_key
            ],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

fn existing_reservation_error(
    state: &str,
    signer_status: &str,
) -> Result<AccountIdentityIssuerReservation, AccountIdentityAuthorityIssuerClientError> {
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
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE.to_owned(),
        binding.service_binding_id.clone(),
        binding.key_id.clone(),
        binding.key_generation,
        binding.enrollment_generation,
        binding.authority_generation,
        binding.session_generation,
        binding.correlation_id.clone(),
        binding.idempotency_key.clone(),
        request_digest,
        request_wire,
        RESERVATION_PREPARED.to_owned(),
        SIGNER_NOT_STARTED.to_owned(),
        attempt_token,
        lease_expires_at,
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
