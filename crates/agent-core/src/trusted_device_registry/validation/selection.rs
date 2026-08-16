use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingRejectionReason, LanParentIntentEnvelope,
    LanTrustedDeviceRegistryEntry,
};

use super::super::TrustedDeviceRegistry;

pub(super) fn validate_selected_pairing(
    registry: &TrustedDeviceRegistry,
    intent: &LanParentIntentEnvelope,
    entry: &LanTrustedDeviceRegistryEntry,
    observed_at: &str,
    require_selected_pairing: bool,
) -> Result<(), LanPairingRejectionReason> {
    if require_selected_pairing
        && registry.selected_pairing_id.as_deref() != Some(entry.pairing_id.as_str())
    {
        return Err(LanPairingRejectionReason::UnselectedDevice);
    }
    if require_selected_pairing {
        match registry.selected_reachability_at(observed_at) {
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

    Ok(())
}
