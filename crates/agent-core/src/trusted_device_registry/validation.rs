use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingRejectionReason, LanPairingTrustState,
    LanParentIntentEnvelope,
};

use super::TrustedDeviceRegistry;

impl TrustedDeviceRegistry {
    pub(super) fn validate_intent_with_selection_requirement(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
    ) -> Result<(), LanPairingRejectionReason> {
        if intent.pairing_id.is_empty() || intent.proof_digest.is_empty() {
            return Err(LanPairingRejectionReason::Anonymous);
        }
        if intent.intent_id.is_empty() || intent.route_id.is_empty() {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if self.accepted_intent_ids.contains(&intent.intent_id) {
            return Err(LanPairingRejectionReason::Replayed);
        }

        let entry = self
            .entries
            .iter()
            .find(|candidate| candidate.pairing_id == intent.pairing_id)
            .ok_or(LanPairingRejectionReason::Anonymous)?;

        if entry.trust_state == LanPairingTrustState::Revoked || entry.revoked_at.is_some() {
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
        if require_selected_pairing
            && self.selected_pairing_id.as_deref() != Some(entry.pairing_id.as_str())
        {
            return Err(LanPairingRejectionReason::UnselectedDevice);
        }
        if require_selected_pairing {
            match self.selected_reachability_at(observed_at) {
                LanPairingDeviceReachability::Offline => {
                    return Err(LanPairingRejectionReason::Offline);
                }
                LanPairingDeviceReachability::Stale => {
                    return Err(LanPairingRejectionReason::Stale);
                }
                LanPairingDeviceReachability::Online => {}
            }
        }
        if observed_at > entry.expires_at.as_str() {
            return Err(LanPairingRejectionReason::Expired);
        }
        if observed_at > intent.expires_at.as_str() {
            return Err(LanPairingRejectionReason::Stale);
        }

        self.accepted_intent_ids.insert(intent.intent_id.clone());
        Ok(())
    }
}
