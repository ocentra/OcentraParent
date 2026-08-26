use ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Operation;
use rusqlite::Transaction;

use super::super::super::AccountIdentityAuthorityIssuerClientError;
use super::super::reservation::{
    request_digest, RESERVATION_PREPARED, RESERVATION_SIGNING, SIGNER_IN_FLIGHT, SIGNER_NOT_STARTED,
};
use super::reconcile;
use super::RecoveryReservation;

pub(super) fn validate_recovery_reservation(
    transaction: &Transaction<'_>,
    candidate: &RecoveryReservation,
    now: i64,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    if !valid_state(candidate) || !valid_identity(candidate) {
        return Ok(false);
    }
    let Some(lease) = valid_lease(candidate)? else {
        return Ok(false);
    };
    if lease > now {
        return Ok(false);
    }
    let Some(parsed) = parse_request(candidate) else {
        return Ok(false);
    };
    if !request_matches(candidate, &parsed) {
        return Ok(false);
    }
    let claims = match serde_json::from_slice(parsed.payload.as_slice()) {
        Ok(claims) => claims,
        Err(_) => return Ok(false),
    };
    if !claims_are_canonical(&claims, &parsed.payload)? {
        return Ok(false);
    }
    let Some(current) = reconcile::load_current_authority(transaction, candidate)? else {
        return Ok(false);
    };
    if !reconcile::current_matches(candidate, &claims, &current) {
        return Ok(false);
    }
    let Some(key) = reconcile::load_active_key(transaction, candidate)? else {
        return Ok(false);
    };
    Ok(reconcile::key_matches(
        candidate,
        parsed.key_id.as_str(),
        &key,
    ))
}

fn valid_state(candidate: &RecoveryReservation) -> bool {
    (candidate.reservation_state == RESERVATION_PREPARED
        && candidate.signer_status == SIGNER_NOT_STARTED)
        || (candidate.reservation_state == RESERVATION_SIGNING
            && candidate.signer_status == SIGNER_IN_FLIGHT)
}

fn valid_identity(candidate: &RecoveryReservation) -> bool {
    !candidate.reservation_id.trim().is_empty()
        && !candidate.account_id.trim().is_empty()
        && !candidate.household_id.trim().is_empty()
        && !candidate.provider_subject.trim().is_empty()
        && candidate.service == ocentra_schema::account_identity_authority_producer_v2::
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
        && !candidate.service_binding_id.trim().is_empty()
        && !candidate.key_id.trim().is_empty()
        && !candidate.correlation_id.trim().is_empty()
        && !candidate.idempotency_key.trim().is_empty()
        && !candidate.attempt_token.trim().is_empty()
        && !candidate.request_wire.is_empty()
        && candidate.request_digest == request_digest(candidate.request_wire.as_slice())
}

fn valid_lease(
    candidate: &RecoveryReservation,
) -> Result<Option<i64>, AccountIdentityAuthorityIssuerClientError> {
    Ok(
        match super::super::super::clock::parse_timestamp(candidate.lease_expires_at.as_str()) {
            Ok(value) => Some(value.timestamp_millis()),
            Err(_) => None,
        },
    )
}

fn parse_request(
    candidate: &RecoveryReservation,
) -> Option<crate::account_identity_authority_envelope_v2::ParsedAuthorityProducerV2Envelope> {
    let mut wire = candidate.request_wire.clone();
    wire.extend_from_slice(&[0_u8; 64]);
    crate::account_identity_authority_envelope_v2::parse(wire.as_slice()).ok()
}

fn request_matches(
    candidate: &RecoveryReservation,
    parsed: &crate::account_identity_authority_envelope_v2::ParsedAuthorityProducerV2Envelope,
) -> bool {
    parsed.operation == AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority
        && parsed.signing_bytes == candidate.request_wire
        && parsed.key_id == candidate.key_id
        && parsed.service_binding_id == candidate.service_binding_id
        && parsed.correlation_id == candidate.correlation_id
        && parsed.idempotency_key == candidate.idempotency_key
        && reconcile::generations_match(
            candidate,
            parsed.key_generation,
            parsed.enrollment_generation,
            parsed.authority_generation,
            parsed.session_generation,
        )
        && reconcile::expected_receipt_id(
            parsed.receipt_id.as_str(),
            parsed.correlation_id.as_str(),
            parsed.idempotency_key.as_str(),
            parsed.payload.as_slice(),
        )
}

fn claims_are_canonical(
    claims: &ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Claims,
    payload: &[u8],
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    if claims.validate_shape().is_err() {
        return Ok(false);
    }
    Ok(serde_json::to_vec(claims)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
        == payload)
}
