use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};
use ocentra_parent_agent_protocol::LanTrustedDeviceRegistryEntry;

use super::super::TrustedDeviceRegistry;

pub(super) fn validate_intent_entry<'a>(
    registry: &'a TrustedDeviceRegistry,
    intent: &LanParentIntentEnvelope,
    origin: Option<&str>,
) -> Result<&'a LanTrustedDeviceRegistryEntry, LanPairingRejectionReason> {
    let entry = registry
        .entries
        .iter()
        .find(|candidate| candidate.pairing_id == intent.pairing_id)
        .ok_or(LanPairingRejectionReason::Anonymous)?;

    if entry.trust_state
        == ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState::Revoked
        || entry.revoked_at.is_some()
    {
        return Err(LanPairingRejectionReason::Revoked);
    }
    if origin != Some(entry.origin.as_str()) {
        return Err(LanPairingRejectionReason::WrongOrigin);
    }
    if intent.target_child_device_id.as_str() != entry.child_device.device_id.as_str() {
        return Err(LanPairingRejectionReason::WrongDevice);
    }
    if intent.route_id.as_str() != entry.route_id.as_str() {
        return Err(LanPairingRejectionReason::UnsupportedRoute);
    }
    if intent.proof_digest.as_str() != entry.proof_digest.as_str() {
        return Err(LanPairingRejectionReason::Malformed);
    }

    Ok(entry)
}
