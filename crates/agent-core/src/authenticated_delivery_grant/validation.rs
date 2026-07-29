use chrono::{DateTime, SecondsFormat, Utc};
use ed25519_dalek::Signature;
use ocentra_schema::authenticated_delivery_grant::{
    parse_authenticated_delivery_grant_instant, AuthenticatedDeliveryGrant,
    AuthenticatedDeliveryGrantInstant, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
};

use super::{
    digest, AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantExpectation,
    AuthenticatedDeliveryGrantTrustedIssuer,
};

pub(super) fn validate_grant(
    grant: &AuthenticatedDeliveryGrant,
    expected: &AuthenticatedDeliveryGrantExpectation,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    trusted_now: AuthenticatedDeliveryGrantInstant,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    grant
        .validate_shape()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::InvalidGrant)?;
    let signature = Signature::from_slice(&grant.signature)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::SignatureRejected)?;
    trusted_issuer
        .verifying_key
        .verify_strict(&grant.signing_bytes(), &signature)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::SignatureRejected)?;
    if grant.dry_run {
        return Err(AuthenticatedDeliveryGrantConsumeError::DryRunRejected);
    }
    validate_temporal_window_at(grant, trusted_now)?;
    if grant.revocation_version != expected.revocation_version {
        return Err(AuthenticatedDeliveryGrantConsumeError::Revoked);
    }
    if grant.issuer_key_id != trusted_issuer.key_id
        || grant.issuer_actor_id != expected.issuer_actor_id
        || grant.household_id != expected.household_id
        || grant.parent_device_id != expected.parent_device_id
        || grant.child_profile_id != expected.child_profile_id
        || grant.target_device_id != expected.target_device_id
        || grant.policy_decision_id != expected.policy_decision_id
        || grant.policy_version != expected.policy_version
        || grant.action_id != expected.action_id
        || grant.capability_id != expected.capability_id
        || grant.evidence_digest != expected.evidence_digest
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
    }
    Ok(())
}

pub(super) fn validate_correlation_id(
    correlation_id: &str,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    if correlation_id.trim().is_empty()
        || correlation_id.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
    }
    Ok(())
}

pub(super) fn validate_delivered_payload(
    grant: &AuthenticatedDeliveryGrant,
    delivered_payload: &[u8],
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    if delivered_payload.len() != grant.payload_length
        || delivered_payload.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES
        || grant.payload_digest != digest(delivered_payload)
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
    }
    Ok(())
}

pub(super) fn validate_expiry_at(
    grant: &AuthenticatedDeliveryGrant,
    trusted_now: AuthenticatedDeliveryGrantInstant,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    if grant
        .expires_at_instant()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::InvalidGrant)?
        <= trusted_now
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::Expired);
    }
    Ok(())
}

pub(super) fn validate_temporal_window_at(
    grant: &AuthenticatedDeliveryGrant,
    trusted_now: AuthenticatedDeliveryGrantInstant,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    if grant
        .issued_at_instant()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::InvalidGrant)?
        > trusted_now
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::NotYetValid);
    }
    validate_expiry_at(grant, trusted_now)
}

pub(super) fn trusted_now(
) -> Result<(AuthenticatedDeliveryGrantInstant, i64), AuthenticatedDeliveryGrantConsumeError> {
    parse_trusted_now(&Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true))
}

#[cfg(debug_assertions)]
pub(super) fn trusted_now_after_transaction(
    debug_trusted_now: Option<(AuthenticatedDeliveryGrantInstant, i64)>,
    fallback: (AuthenticatedDeliveryGrantInstant, i64),
) -> Result<(AuthenticatedDeliveryGrantInstant, i64), AuthenticatedDeliveryGrantConsumeError> {
    match debug_trusted_now {
        Some(trusted_now) => Ok(trusted_now),
        None => {
            let _ = fallback;
            trusted_now()
        }
    }
}

#[cfg(not(debug_assertions))]
pub(super) fn trusted_now_after_transaction(
    fallback: (AuthenticatedDeliveryGrantInstant, i64),
) -> Result<(AuthenticatedDeliveryGrantInstant, i64), AuthenticatedDeliveryGrantConsumeError> {
    let _ = fallback;
    trusted_now()
}

pub(super) fn parse_trusted_now(
    trusted_now: &str,
) -> Result<(AuthenticatedDeliveryGrantInstant, i64), AuthenticatedDeliveryGrantConsumeError> {
    Ok((
        parse_authenticated_delivery_grant_instant(trusted_now)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::BindingRejected)?,
        instant_nanos(trusted_now)?,
    ))
}

pub(super) fn instant_nanos(value: &str) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .and_then(|instant| instant.timestamp_nanos_opt())
        .ok_or(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
}
