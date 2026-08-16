use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};

use super::super::TrustedDeviceRegistry;

pub(super) fn validate_intent_shape(
    registry: &TrustedDeviceRegistry,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    if intent.pairing_id.is_empty() || intent.proof_digest.is_empty() {
        return Err(LanPairingRejectionReason::Anonymous);
    }
    if intent.intent_id.is_empty() || intent.route_id.is_empty() {
        return Err(LanPairingRejectionReason::Malformed);
    }
    if registry.accepted_intent_ids.contains(&intent.intent_id) {
        return Err(LanPairingRejectionReason::Replayed);
    }
    Ok(())
}
