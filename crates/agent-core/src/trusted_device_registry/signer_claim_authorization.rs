use ocentra_lan_core::lan_pairing::signed_household_mesh_ingress::LanCryptographicallyVerifiedHouseholdMeshIngress;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingRejectionReason, LanPairingTrustState,
};
use std::cmp::Ordering;

use super::super::{
    signer_authority_types::{LanRegisteredSignedChildAuthority, LanTrustedDeviceSignerAnchor},
    TrustedDeviceRegistry,
};
use super::signer_anchor_binding::{is_lower_hex, validate_registry_authority_identifier};
use crate::trusted_device_registry_selection::rfc3339_cmp;

impl TrustedDeviceRegistry {
    /// Converts a cryptographically verified transport claim into a registry
    /// authority only when durable pairing, selected route, signer anchor, and
    /// expiry/revocation state all agree. Replay custody remains a later
    /// durable reservation step and is intentionally not performed here.
    pub fn authorize_signed_child_claim(
        &self,
        ingress: &LanCryptographicallyVerifiedHouseholdMeshIngress,
        observed_at: &str,
    ) -> Result<LanRegisteredSignedChildAuthority, LanPairingRejectionReason> {
        let claim = ingress.claim();
        let signer_public_key_id = ingress.signer_public_key_id();
        let signer_public_key_sha256 = ingress.signer_public_key_sha256();
        validate_identifiers(claim, signer_public_key_id, signer_public_key_sha256)?;
        let entry = self
            .entries
            .iter()
            .find(|candidate| candidate.pairing_id == claim.pairing_id.as_str())
            .ok_or(LanPairingRejectionReason::Anonymous)?;
        validate_entry_state(self, entry, claim, observed_at)?;
        validate_entry_binding(entry, claim)?;
        let anchor = self
            .signer_anchors
            .get(entry.pairing_id.as_str())
            .ok_or(LanPairingRejectionReason::SignedChildAgentContextUnavailable)?;
        if self
            .signer_anchor_generations
            .get(entry.pairing_id.as_str())
            != Some(&anchor.authority_generation)
        {
            return Err(LanPairingRejectionReason::Malformed);
        }
        validate_anchor(
            anchor,
            claim,
            signer_public_key_id,
            signer_public_key_sha256,
        )?;
        Ok(LanRegisteredSignedChildAuthority {
            pairing_id: entry.pairing_id.clone(),
            child_device_id: entry.child_device.device_id.clone(),
            target_device_id: entry.child_device.device_id.clone(),
            install_id: anchor.install_id.clone(),
            family_hash: anchor.family_hash.clone(),
            parent_device_id: anchor.parent_device_id.clone(),
            route_id: entry.route_id.clone(),
            registry_proof_digest: entry.proof_digest.clone(),
            message_kind: claim.message_kind.clone(),
            message_id: claim.message_id.as_str().to_string(),
            idempotency_key: claim.idempotency_key.as_str().to_string(),
            nonce: claim.nonce.as_str().to_string(),
            sequence: claim.sequence.value(),
            authority_generation: anchor.authority_generation,
            public_key_id: anchor.public_key_id.clone(),
            public_key_sha256: anchor.public_key_sha256.clone(),
        })
    }
}

fn validate_entry_binding(
    entry: &ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry,
    claim: &ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::
        LanSignedHouseholdMeshTransportClaimDto,
) -> Result<(), LanPairingRejectionReason> {
    if claim.child_device_id.as_str() != entry.child_device.device_id
        || claim.target_device_id.as_str() != entry.child_device.device_id
        || claim.parent_device_id.as_str() != entry.parent_device.device_id
        || claim.route_id.as_str() != entry.route_id
        || claim.registry_proof_digest.as_str() != entry.proof_digest
    {
        return Err(LanPairingRejectionReason::WrongDevice);
    }
    Ok(())
}

fn validate_identifiers(
    claim: &ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::
        LanSignedHouseholdMeshTransportClaimDto,
    signer_public_key_id: &str,
    signer_public_key_sha256: &str,
) -> Result<(), LanPairingRejectionReason> {
    let values = [
        claim.pairing_id.as_str(),
        claim.family_hash.as_str(),
        claim.child_device_id.as_str(),
        claim.target_device_id.as_str(),
        claim.parent_device_id.as_str(),
        claim.install_id.as_str(),
        claim.route_id.as_str(),
        claim.registry_proof_digest.as_str(),
        claim.message_id.as_str(),
        claim.idempotency_key.as_str(),
        claim.nonce.as_str(),
    ];
    if values
        .iter()
        .any(|value| !validate_registry_authority_identifier(value))
        || !is_lower_hex(signer_public_key_id, 32)
        || !is_lower_hex(signer_public_key_sha256, 64)
    {
        return Err(LanPairingRejectionReason::Malformed);
    }
    Ok(())
}

fn validate_entry_state(
    registry: &TrustedDeviceRegistry,
    entry: &ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry,
    claim: &ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::
        LanSignedHouseholdMeshTransportClaimDto,
    observed_at: &str,
) -> Result<(), LanPairingRejectionReason> {
    if entry.trust_state != LanPairingTrustState::Paired || entry.revoked_at.is_some() {
        return Err(if entry.revoked_at.is_some() {
            LanPairingRejectionReason::Revoked
        } else {
            LanPairingRejectionReason::SignedChildAgentContextUnavailable
        });
    }
    let Some(entry_expiry_ordering) = rfc3339_cmp(observed_at, entry.expires_at.as_str()) else {
        return Err(LanPairingRejectionReason::Malformed);
    };
    let Some(claim_expiry_ordering) = rfc3339_cmp(observed_at, claim.expires_at.as_str()) else {
        return Err(LanPairingRejectionReason::Malformed);
    };
    if entry_expiry_ordering != Ordering::Less || claim_expiry_ordering != Ordering::Less {
        return Err(LanPairingRejectionReason::Expired);
    }
    if registry.selected_pairing_id.as_deref() != Some(entry.pairing_id.as_str()) {
        return Err(LanPairingRejectionReason::UnselectedDevice);
    }
    match registry.selected_reachability_at(observed_at) {
        LanPairingDeviceReachability::Offline => Err(LanPairingRejectionReason::Offline),
        LanPairingDeviceReachability::Stale => Err(LanPairingRejectionReason::Stale),
        LanPairingDeviceReachability::Online => Ok(()),
    }
}

fn validate_anchor(
    anchor: &LanTrustedDeviceSignerAnchor,
    claim: &ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::
        LanSignedHouseholdMeshTransportClaimDto,
    signer_public_key_id: &str,
    signer_public_key_sha256: &str,
) -> Result<(), LanPairingRejectionReason> {
    if anchor.public_key_id != signer_public_key_id
        || anchor.public_key_sha256 != signer_public_key_sha256
        || anchor.install_id != claim.install_id.as_str()
        || anchor.family_hash != claim.family_hash.as_str()
        || anchor.parent_device_id != claim.parent_device_id.as_str()
    {
        return Err(LanPairingRejectionReason::WrongDevice);
    }
    Ok(())
}
