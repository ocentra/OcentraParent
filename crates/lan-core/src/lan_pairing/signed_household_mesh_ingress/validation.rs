use chrono::{DateTime, TimeDelta, Utc};
use ocentra_parent_agent_protocol::{
    constants,
    household_mesh::{
        HouseholdMeshAuthenticationState, HouseholdMeshBridgeState, HouseholdMeshPolicyAuthority,
        HouseholdMeshTransportEnvelope,
    },
    lan_pairing::signed_household_mesh_ingress::transport::{
        LanHouseholdMeshIngressSchemaVersionDto, LanHouseholdMeshTimestamp,
        LanSignedChildBeaconIngressEnvelope, LanSignedHouseholdMeshTransportClaimDto,
        LanSignedHouseholdMeshTransportEnvelope,
    },
    lan_pairing::LanSignedChildAgentClaim,
};

use super::{
    lan_household_mesh_payload_sha256, LanSignedHouseholdMeshCryptographicVerificationContext,
    LanSignedHouseholdMeshIngressVerificationError,
};

const MAX_SIGNED_FIELD_BYTES: usize = 4_096;
const MAX_SIGNED_HOUSEHOLD_MESH_CLAIM_LIFETIME_SECONDS: i64 = 300;

pub(super) fn validate_schema(
    packet: &LanSignedChildBeaconIngressEnvelope,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    let current = LanHouseholdMeshIngressSchemaVersionDto::current();
    if packet.schema_version != current
        || packet.signed_transport.schema_version != current
        || packet.signed_transport.claim.schema_version != current
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::UnsupportedSchemaVersion);
    }
    Ok(())
}

pub(super) fn validate_required_fields(
    envelope: &LanSignedHouseholdMeshTransportEnvelope,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    let payload = &envelope.payload;
    let required = [
        payload.message_id.as_str(),
        payload.idempotency_key.as_str(),
        payload.family_id.as_str(),
        payload.target_child_device_id.as_str(),
        payload.source_peer_id.as_str(),
        payload.local_event_ref.as_str(),
        payload.lan_message_type.as_str(),
    ];
    if required
        .iter()
        .any(|value| value.trim().is_empty() || value.len() > MAX_SIGNED_FIELD_BYTES)
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::EmptyRequiredField);
    }
    Ok(())
}

pub(super) fn validate_beacon_binding(
    beacon: &LanSignedChildAgentClaim,
    transport: &LanSignedHouseholdMeshTransportClaimDto,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    if beacon.message_kind != transport.message_kind
        || beacon.family_hash.as_str() != transport.family_hash.as_str()
        || beacon.parent_device_id.as_str() != transport.parent_device_id.as_str()
        || beacon.child_device_id.as_str() != transport.child_device_id.as_str()
        || beacon.install_id.as_str() != transport.install_id.as_str()
        || beacon.route_id.as_str() != transport.route_id.as_str()
        || beacon.nonce.as_str() != transport.nonce.as_str()
        || beacon.sequence != transport.sequence.value()
        || beacon.issued_at.as_str() != transport.issued_at.as_str()
        || beacon.expires_at.as_str() != transport.expires_at.as_str()
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::BeaconBindingMismatch);
    }
    Ok(())
}

pub(super) fn validate_authority_binding(
    claim: &LanSignedHouseholdMeshTransportClaimDto,
    context: &LanSignedHouseholdMeshCryptographicVerificationContext,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    if claim.family_hash != context.expected_family_hash
        || claim.parent_device_id != context.expected_parent_device_id
        || claim.child_device_id != context.expected_child_device_id
        || claim.target_device_id != context.expected_target_device_id
        || claim.install_id != context.expected_install_id
        || claim.route_id != context.expected_route_id
        || claim.pairing_id != context.expected_pairing_id
        || claim.registry_proof_digest != context.expected_registry_proof_digest
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::AuthorityBindingMismatch);
    }
    Ok(())
}

pub(super) fn validate_transport_binding(
    envelope: &LanSignedHouseholdMeshTransportEnvelope,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    let claim = &envelope.claim;
    let payload = &envelope.payload;
    let expected_event_ref = claim
        .lan_message_type
        .local_event_ref()
        .ok_or(LanSignedHouseholdMeshIngressVerificationError::TransportBindingMismatch)?;
    if claim.message_id.as_str() != payload.message_id.as_str()
        || claim.idempotency_key.as_str() != payload.idempotency_key.as_str()
        || claim.local_event_ref.as_str() != expected_event_ref
        || claim.local_event_ref.as_str() != payload.local_event_ref.as_str()
        || claim.lan_message_type.as_str() != payload.lan_message_type.as_str()
        || claim.family_hash.as_str() != payload.family_id.as_str()
        || claim.child_device_id.as_str() != payload.source_peer_id.as_str()
        || claim.target_device_id.as_str() != payload.target_child_device_id.as_str()
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::TransportBindingMismatch);
    }
    let payload_sha256 = lan_household_mesh_payload_sha256(payload)?;
    if claim.canonical_payload_sha256 != payload_sha256 {
        return Err(LanSignedHouseholdMeshIngressVerificationError::PayloadDigestMismatch);
    }
    Ok(())
}

pub(super) fn validate_time_window(
    claim: &LanSignedHouseholdMeshTransportClaimDto,
    observed_at: &LanHouseholdMeshTimestamp,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    let observed_at = parse_timestamp(observed_at.as_str())?;
    let issued_at = parse_timestamp(claim.issued_at.as_str())?;
    let expires_at = parse_timestamp(claim.expires_at.as_str())?;
    if issued_at > observed_at {
        return Err(LanSignedHouseholdMeshIngressVerificationError::FutureIssuedAt);
    }
    if expires_at <= observed_at || expires_at <= issued_at {
        return Err(LanSignedHouseholdMeshIngressVerificationError::Expired);
    }
    if expires_at - issued_at > TimeDelta::seconds(MAX_SIGNED_HOUSEHOLD_MESH_CLAIM_LIFETIME_SECONDS)
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::ClaimLifetimeExceeded);
    }
    Ok(())
}

pub(super) fn validate_payload_window(
    envelope: &LanSignedHouseholdMeshTransportEnvelope,
    observed_at: &LanHouseholdMeshTimestamp,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    let issued_at = parse_timestamp(envelope.claim.issued_at.as_str())?;
    let expires_at = parse_timestamp(envelope.claim.expires_at.as_str())?;
    let observed_at = parse_timestamp(observed_at.as_str())?;
    let payload_sent_at_seconds = i64::try_from(envelope.payload.sent_at_epoch_seconds)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::PayloadWindowMismatch)?;
    let payload_sent_at = DateTime::<Utc>::from_timestamp(payload_sent_at_seconds, 0)
        .ok_or(LanSignedHouseholdMeshIngressVerificationError::PayloadWindowMismatch)?;
    let payload_stale_after_seconds = i64::try_from(envelope.payload.stale_after_seconds)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::PayloadWindowMismatch)?;
    let payload_expires_at = payload_sent_at
        .checked_add_signed(TimeDelta::seconds(payload_stale_after_seconds))
        .ok_or(LanSignedHouseholdMeshIngressVerificationError::PayloadWindowMismatch)?;
    if payload_sent_at != issued_at
        || envelope.payload.stale_after_seconds == 0
        || payload_stale_after_seconds > MAX_SIGNED_HOUSEHOLD_MESH_CLAIM_LIFETIME_SECONDS
        || payload_expires_at > expires_at
        || payload_expires_at < observed_at
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::PayloadWindowMismatch);
    }
    Ok(())
}

pub(super) fn validate_safe_payload(
    payload: &HouseholdMeshTransportEnvelope,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    if payload.schema_version != constants::household_mesh::EVENT_SCHEMA_VERSION
        || payload.bridge_state != HouseholdMeshBridgeState::ExportSelected
        || payload.authentication_state != HouseholdMeshAuthenticationState::PairedTrustedDevice
        || payload.policy_authority != HouseholdMeshPolicyAuthority::ChildAgentOnly
        || payload.direct_remote_publish_requested
        || payload.raw_payload_included
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::UnsupportedPayload);
    }
    Ok(())
}

fn parse_timestamp(
    value: &str,
) -> Result<DateTime<Utc>, LanSignedHouseholdMeshIngressVerificationError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::MalformedTimestamp)
}
