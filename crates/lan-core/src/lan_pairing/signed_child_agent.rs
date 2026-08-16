#![forbid(unsafe_code)]

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanSignedChildAgentClaim, LanSignedChildAgentEnvelope,
};
use sha2::{Digest, Sha256};

use super::{
    signed_child_agent_metadata::validate_signed_child_agent_metadata,
    LanSignedChildAgentReplayGuard, LanSignedChildAgentVerificationContext,
    LanSignedChildAgentVerificationError,
};

pub(super) fn verify_lan_signed_child_agent_envelope(
    envelope: &LanSignedChildAgentEnvelope,
    observed_at: &str,
    context: &LanSignedChildAgentVerificationContext,
    replay_guard: &mut LanSignedChildAgentReplayGuard,
) -> Result<LanSignedChildAgentClaim, LanSignedChildAgentVerificationError> {
    validate_signed_child_agent_schema(envelope)?;
    validate_signed_child_agent_required_fields(envelope)?;
    validate_signed_child_agent_time_window(&envelope.claim, observed_at)?;
    validate_signed_child_agent_algorithm(envelope)?;

    let verifying_key = signed_child_agent_verifying_key(envelope)?;
    validate_signed_child_agent_public_key_id(envelope, &verifying_key)?;
    verify_signed_child_agent_signature(envelope, &verifying_key)?;
    validate_signed_child_agent_context(&envelope.claim, context)?;
    validate_signed_child_agent_replay(&envelope.claim, replay_guard)?;

    Ok(envelope.claim.clone())
}

pub(super) fn signed_child_agent_public_key_id(verifying_key: &VerifyingKey) -> String {
    let digest = Sha256::digest(verifying_key.as_bytes());
    digest[..16]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn validate_signed_child_agent_schema(
    envelope: &LanSignedChildAgentEnvelope,
) -> Result<(), LanSignedChildAgentVerificationError> {
    if envelope.schema_version != constants::lan_pairing::SCHEMA_VERSION
        || envelope.claim.schema_version != constants::lan_pairing::SCHEMA_VERSION
    {
        return Err(LanSignedChildAgentVerificationError::UnsupportedSchemaVersion);
    }
    Ok(())
}

fn validate_signed_child_agent_required_fields(
    envelope: &LanSignedChildAgentEnvelope,
) -> Result<(), LanSignedChildAgentVerificationError> {
    validate_signed_child_agent_scalar_fields(envelope)?;
    validate_signed_child_agent_collection_fields(&envelope.claim)?;
    validate_signed_child_agent_metadata(&envelope.claim)
}

fn validate_signed_child_agent_scalar_fields(
    envelope: &LanSignedChildAgentEnvelope,
) -> Result<(), LanSignedChildAgentVerificationError> {
    let claim = &envelope.claim;
    let required_fields = [
        envelope.public_key_base64.as_str(),
        envelope.public_key_id.as_str(),
        envelope.signature_base64.as_str(),
        envelope.signature_algorithm.as_str(),
        claim.child_device_id.as_str(),
        claim.parent_device_id.as_str(),
        claim.install_id.as_str(),
        claim.family_hash.as_str(),
        claim.platform.as_str(),
        claim.hostname.as_str(),
        claim.agent_version.as_str(),
        claim.route_id.as_str(),
        claim.nonce.as_str(),
        claim.issued_at.as_str(),
        claim.expires_at.as_str(),
    ];
    if required_fields.iter().any(|value| value.trim().is_empty()) {
        return Err(LanSignedChildAgentVerificationError::EmptyRequiredField);
    }
    Ok(())
}

fn validate_signed_child_agent_collection_fields(
    claim: &LanSignedChildAgentClaim,
) -> Result<(), LanSignedChildAgentVerificationError> {
    if claim.capabilities.is_empty()
        || claim
            .child_profile_hash
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        || claim.local_ips.iter().any(|value| value.trim().is_empty())
        || claim
            .mac_addresses
            .iter()
            .any(|value| value.trim().is_empty())
        || claim
            .capabilities
            .iter()
            .any(|value| value.trim().is_empty())
    {
        return Err(LanSignedChildAgentVerificationError::EmptyRequiredField);
    }
    Ok(())
}

fn validate_signed_child_agent_algorithm(
    envelope: &LanSignedChildAgentEnvelope,
) -> Result<(), LanSignedChildAgentVerificationError> {
    if envelope.signature_algorithm
        != constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
    {
        return Err(LanSignedChildAgentVerificationError::UnsupportedAlgorithm);
    }
    Ok(())
}

fn signed_child_agent_verifying_key(
    envelope: &LanSignedChildAgentEnvelope,
) -> Result<VerifyingKey, LanSignedChildAgentVerificationError> {
    let public_key_bytes = STANDARD
        .decode(&envelope.public_key_base64)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidPublicKey)?;
    let key_bytes: [u8; 32] = public_key_bytes
        .as_slice()
        .try_into()
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidPublicKey)?;
    VerifyingKey::from_bytes(&key_bytes)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidPublicKey)
}

fn validate_signed_child_agent_public_key_id(
    envelope: &LanSignedChildAgentEnvelope,
    verifying_key: &VerifyingKey,
) -> Result<(), LanSignedChildAgentVerificationError> {
    let expected_key_id = signed_child_agent_public_key_id(verifying_key);
    if envelope.public_key_id != expected_key_id {
        return Err(LanSignedChildAgentVerificationError::PublicKeyIdMismatch);
    }
    Ok(())
}

fn verify_signed_child_agent_signature(
    envelope: &LanSignedChildAgentEnvelope,
    verifying_key: &VerifyingKey,
) -> Result<(), LanSignedChildAgentVerificationError> {
    let signature_bytes = STANDARD
        .decode(&envelope.signature_base64)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidSignature)?;
    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidSignature)?;
    let payload = serde_json::to_vec(&envelope.claim)
        .map_err(|_error| LanSignedChildAgentVerificationError::SerializationFailed)?;
    verifying_key
        .verify(&payload, &signature)
        .map_err(|_error| LanSignedChildAgentVerificationError::SignatureRejected)
}

fn validate_signed_child_agent_context(
    claim: &LanSignedChildAgentClaim,
    context: &LanSignedChildAgentVerificationContext,
) -> Result<(), LanSignedChildAgentVerificationError> {
    if claim.family_hash != context.expected_family_hash {
        return Err(LanSignedChildAgentVerificationError::WrongFamily);
    }
    if claim.parent_device_id != context.expected_parent_device_id {
        return Err(LanSignedChildAgentVerificationError::WrongParentDevice);
    }
    if claim.route_id != context.expected_route_id {
        return Err(LanSignedChildAgentVerificationError::WrongRoute);
    }
    if context
        .expected_child_device_id
        .as_ref()
        .is_some_and(|expected_child_device_id| claim.child_device_id != *expected_child_device_id)
    {
        return Err(LanSignedChildAgentVerificationError::WrongChildDevice);
    }
    Ok(())
}

fn validate_signed_child_agent_time_window(
    claim: &LanSignedChildAgentClaim,
    observed_at: &str,
) -> Result<(), LanSignedChildAgentVerificationError> {
    let observed_at = parse_signed_child_agent_timestamp(observed_at)?;
    let issued_at = parse_signed_child_agent_timestamp(&claim.issued_at)?;
    let expires_at = parse_signed_child_agent_timestamp(&claim.expires_at)?;
    if issued_at > observed_at {
        return Err(LanSignedChildAgentVerificationError::FutureIssuedAt);
    }
    if expires_at <= observed_at {
        return Err(LanSignedChildAgentVerificationError::Expired);
    }
    Ok(())
}

fn parse_signed_child_agent_timestamp(
    value: &str,
) -> Result<DateTime<Utc>, LanSignedChildAgentVerificationError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|_error| LanSignedChildAgentVerificationError::MalformedTimestamp)
}

fn validate_signed_child_agent_replay(
    claim: &LanSignedChildAgentClaim,
    replay_guard: &mut LanSignedChildAgentReplayGuard,
) -> Result<(), LanSignedChildAgentVerificationError> {
    let replay_key = signed_child_agent_replay_key(claim);
    if !replay_guard.observed_keys.insert(replay_key) {
        return Err(LanSignedChildAgentVerificationError::Replayed);
    }
    Ok(())
}

fn signed_child_agent_replay_key(claim: &LanSignedChildAgentClaim) -> String {
    format!(
        "{}|{}|{}|{}",
        claim.child_device_id, claim.route_id, claim.nonce, claim.sequence
    )
}
