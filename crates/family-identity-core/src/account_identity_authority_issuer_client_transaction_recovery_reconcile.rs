use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityProvider,
};
use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Claims, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};
use ring::digest::{digest, SHA256};
use rusqlite::{params, OptionalExtension, Transaction};

use super::super::super::AccountIdentityAuthorityIssuerClientError;
use super::super::reservation::{
    timestamp, RESERVATION_ISSUED, RESERVATION_PREPARED, RESERVATION_SIGNING, SIGNER_SUCCEEDED,
};
use super::storage;
use super::validation;
use super::{RecoveryReservation, RECOVERY_BATCH_SIZE};

const COMPACT_ISSUED_SQL: &str = "DELETE FROM account_identity_issuer_v2_reservation
              WHERE reservation_id IN (
                  SELECT reservation.reservation_id
                    FROM account_identity_issuer_v2_reservation AS reservation
                    JOIN account_identity_issuer_v2_receipt AS receipt
                      ON receipt.receipt_id = reservation.receipt_id
                    JOIN account_identity_issuer_v2_outbox AS outbox
                      ON outbox.receipt_id = receipt.receipt_id
                   WHERE reservation.reservation_state = ?1
                     AND reservation.signer_status = ?2
                     AND receipt.provenance_state = 'exact'
                     AND receipt.receipt_state IN ('issued','acknowledged')
                     AND receipt.account_id = reservation.account_id
                     AND receipt.household_id = reservation.household_id
                     AND receipt.provider = reservation.provider
                     AND receipt.provider_subject = reservation.provider_subject
                     AND receipt.service = reservation.service
                     AND receipt.service_binding_id = reservation.service_binding_id
                     AND receipt.key_id = reservation.key_id
                     AND receipt.key_generation = reservation.key_generation
                     AND receipt.enrollment_generation = reservation.enrollment_generation
                     AND receipt.authority_generation = reservation.authority_generation
                     AND receipt.session_generation = reservation.session_generation
                     AND receipt.correlation_id = reservation.correlation_id
                     AND receipt.idempotency_key = reservation.idempotency_key
                     AND outbox.account_id = reservation.account_id
                     AND outbox.household_id = reservation.household_id
                     AND outbox.service = reservation.service
                     AND outbox.service_binding_id = reservation.service_binding_id
                     AND outbox.key_id = reservation.key_id
                     AND outbox.key_generation = reservation.key_generation
                     AND outbox.enrollment_generation = reservation.enrollment_generation
                     AND outbox.authority_generation = reservation.authority_generation
                     AND outbox.wire = receipt.wire
                   ORDER BY reservation.reservation_id
                   LIMIT ?3
              )";

pub(super) fn reconcile_issue_reservations(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    transaction
        .execute(
            COMPACT_ISSUED_SQL,
            params![RESERVATION_ISSUED, SIGNER_SUCCEEDED, RECOVERY_BATCH_SIZE],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    let candidates = load_recovery_candidates(transaction, now_text.as_str())?;
    for candidate in candidates {
        if validation::validate_recovery_reservation(transaction, &candidate, now)? {
            if candidate.reservation_state == RESERVATION_PREPARED {
                storage::delete_valid_prepared(transaction, &candidate, now_text.as_str())?;
            } else {
                storage::mark_valid_signing_manual(transaction, &candidate, now_text.as_str())?;
            }
        } else {
            // A malformed, stale, or unknown row is retained as an explicit
            // manual-required state. Startup never deletes a row merely
            // because its state and lease look expired.
            storage::mark_invalid_manual(transaction, &candidate, now_text.as_str())?;
        }
    }
    transaction
        .query_row(
            "SELECT EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_reservation
                 WHERE reservation_state = ?1 AND lease_expires_at <= ?2
                 LIMIT 1
             ) OR EXISTS(
                SELECT 1 FROM account_identity_issuer_v2_reservation
                 WHERE reservation_state = ?3
                 LIMIT 1
             )",
            params![RESERVATION_PREPARED, now_text, RESERVATION_SIGNING],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

fn load_recovery_candidates(
    transaction: &Transaction<'_>,
    now_text: &str,
) -> Result<Vec<RecoveryReservation>, AccountIdentityAuthorityIssuerClientError> {
    let mut statement = transaction
        .prepare(
            "SELECT reservation_id, account_id, household_id, provider, provider_subject,
                    service, service_binding_id, key_id, key_generation, enrollment_generation,
                    authority_generation, session_generation, correlation_id, idempotency_key,
                    request_digest, request_wire, reservation_state, signer_status,
                    attempt_token, lease_expires_at
               FROM account_identity_issuer_v2_reservation
              WHERE (reservation_state = ?1 AND lease_expires_at <= ?2)
                 OR reservation_state = ?3
              ORDER BY reservation_id
              LIMIT ?4",
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    let rows = statement
        .query_map(
            params![
                RESERVATION_PREPARED,
                now_text,
                RESERVATION_SIGNING,
                RECOVERY_BATCH_SIZE,
            ],
            |row| {
                Ok(RecoveryReservation {
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
                    attempt_token: row.get(18)?,
                    lease_expires_at: row.get(19)?,
                })
            },
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::ReservationUnavailable)
}

pub(super) struct CurrentAuthorityRow {
    mapping_status: String,
    authority_generation: i64,
    session_id: String,
    session_generation: i64,
    authority_json: String,
}

pub(super) fn load_current_authority(
    transaction: &Transaction<'_>,
    candidate: &RecoveryReservation,
) -> Result<Option<CurrentAuthorityRow>, AccountIdentityAuthorityIssuerClientError> {
    transaction
        .query_row(
            "SELECT mapping_status, authority_generation, session_id,
                    session_generation, authority_json
               FROM account_identity_current_authority
              WHERE provider = ?1 AND provider_subject = ?2",
            params![
                candidate.provider.as_str(),
                candidate.provider_subject.as_str()
            ],
            |row| {
                Ok(CurrentAuthorityRow {
                    mapping_status: row.get(0)?,
                    authority_generation: row.get(1)?,
                    session_id: row.get(2)?,
                    session_generation: row.get(3)?,
                    authority_json: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable)
}

pub(super) fn current_matches(
    candidate: &RecoveryReservation,
    claims: &AccountIdentityAuthorityProducerV2Claims,
    current: &CurrentAuthorityRow,
) -> bool {
    let Ok(handoff) = serde_json::from_str::<AccountIdentityCurrentMemberDeviceAuthorityHandoff>(
        current.authority_json.as_str(),
    ) else {
        return false;
    };
    if current.mapping_status != "active"
        || handoff.validate_shape().is_err()
        || provider_label(&handoff.mapping.provider) != candidate.provider
        || handoff.mapping.provider_subject.as_str() != candidate.provider_subject
        || handoff.member.account_id.to_string() != candidate.account_id
        || handoff.member.household_id.to_string() != candidate.household_id
        || handoff.member.account_id.to_string() != claims.account_id
        || handoff.member.household_id.to_string() != claims.household_id
        || provider_label(&handoff.mapping.provider) != claims.provider
        || handoff.mapping.provider_subject.as_str() != claims.provider_subject
        || handoff.member.member_id.as_str() != claims.member_id
        || handoff.member.device_id.as_str() != claims.device_id
        || handoff.member.session_id.as_str() != claims.session_id
    {
        return false;
    }
    i64::try_from(handoff.member.authority_generation).ok() == Some(candidate.authority_generation)
        && i64::try_from(handoff.member.session_generation).ok()
            == Some(candidate.session_generation)
        && current.authority_generation == candidate.authority_generation
        && current.session_id == claims.session_id
        && current.session_generation == candidate.session_generation
}

pub(super) struct ActiveKeyRow {
    key_generation: i64,
    enrollment_generation: i64,
    authority_generation: i64,
    service_binding_id: String,
    public_key: Vec<u8>,
}

pub(super) fn load_active_key(
    transaction: &Transaction<'_>,
    candidate: &RecoveryReservation,
) -> Result<Option<ActiveKeyRow>, AccountIdentityAuthorityIssuerClientError> {
    transaction
        .query_row(
            "SELECT key_generation, enrollment_generation, authority_generation,
                    service_binding_id, public_key
               FROM account_identity_issuer_v2_key_registry
              WHERE account_id = ?1 AND household_id = ?2 AND service = ?3
                AND service_binding_id = ?4 AND key_id = ?5 AND key_state = 'active'
              ORDER BY key_generation DESC LIMIT 1",
            params![
                candidate.account_id.as_str(),
                candidate.household_id.as_str(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                candidate.service_binding_id.as_str(),
                candidate.key_id.as_str(),
            ],
            |row| {
                Ok(ActiveKeyRow {
                    key_generation: row.get(0)?,
                    enrollment_generation: row.get(1)?,
                    authority_generation: row.get(2)?,
                    service_binding_id: row.get(3)?,
                    public_key: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::KeyUnavailable)
}

pub(super) fn key_matches(
    candidate: &RecoveryReservation,
    parsed_key_id: &str,
    key: &ActiveKeyRow,
) -> bool {
    let Ok(public_key) = <[u8; 65]>::try_from(key.public_key.as_slice()) else {
        return false;
    };
    public_key[0] == 0x04
        && public_key[1..].iter().any(|byte| *byte != 0)
        && key.service_binding_id == candidate.service_binding_id
        && key.key_generation == candidate.key_generation
        && key.enrollment_generation == candidate.enrollment_generation
        && key.authority_generation == candidate.authority_generation
        && crate::account_identity_authority_producer_v2::expected_key_id(&public_key)
            == parsed_key_id
}

pub(super) fn generations_match(
    candidate: &RecoveryReservation,
    key_generation: u64,
    enrollment_generation: u64,
    authority_generation: u64,
    session_generation: u64,
) -> bool {
    i64::try_from(key_generation).ok() == Some(candidate.key_generation)
        && i64::try_from(enrollment_generation).ok() == Some(candidate.enrollment_generation)
        && i64::try_from(authority_generation).ok() == Some(candidate.authority_generation)
        && i64::try_from(session_generation).ok() == Some(candidate.session_generation)
}

pub(super) fn expected_receipt_id(
    receipt_id: &str,
    correlation_id: &str,
    idempotency_key: &str,
    payload: &[u8],
) -> bool {
    let payload_digest = format!("sha256:{}", hex(digest(&SHA256, payload).as_ref()));
    let mut framed = Vec::new();
    framed.extend_from_slice(b"ocentra.account-authority-producer.receipt-id.v2\0");
    for value in [
        correlation_id.as_bytes(),
        idempotency_key.as_bytes(),
        payload_digest.as_bytes(),
    ] {
        let Ok(length) = u64::try_from(value.len()) else {
            return false;
        };
        framed.extend_from_slice(&length.to_be_bytes());
        framed.extend_from_slice(value);
    }
    receipt_id
        == format!(
            "sha256:receipt:{}",
            hex(digest(&SHA256, framed.as_slice()).as_ref())
        )
}

pub(super) fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

pub(super) fn hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    value
        .iter()
        .fold(String::with_capacity(value.len() * 2), |mut text, byte| {
            text.push(HEX[(byte >> 4) as usize] as char);
            text.push(HEX[(byte & 0x0f) as usize] as char);
            text
        })
}
