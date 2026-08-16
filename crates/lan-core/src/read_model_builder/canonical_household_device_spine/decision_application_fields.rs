use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdRouteState, LanHouseholdDeviceDecision,
};

use super::super::super::values::surfaces_for;

pub(super) fn apply_display_name(
    device: &mut LanCanonicalHouseholdDevice,
    decision: &LanHouseholdDeviceDecision,
) {
    let Some(display_name) = decision
        .display_name
        .as_ref()
        .filter(|value| !value.is_empty())
    else {
        return;
    };
    device.display_name = display_name.clone();
    if let Some(inventory) = device.child_agent_inventory.as_mut() {
        inventory.device_name = display_name.clone();
    }
}

pub(super) fn mark_device_revoked(device: &mut LanCanonicalHouseholdDevice) {
    device.discovery_state = LanPairingProductionDiscoveryState::Revoked;
    device.trust_state = LanPairingTrustState::Revoked;
    device.enrollable = false;
    device.route_id = None;
    device.route_state = LanCanonicalHouseholdRouteState::Unavailable;
    device.policy_target_surfaces = surfaces_for(false);
}

pub(super) fn restore_device(device: &mut LanCanonicalHouseholdDevice) {
    if device.discovery_state == LanPairingProductionDiscoveryState::Revoked {
        device.discovery_state = LanPairingProductionDiscoveryState::Discovered;
    }
    if device.trust_state == LanPairingTrustState::Revoked {
        device.trust_state = LanPairingTrustState::Unpaired;
    }
    device.enrollable =
        device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent;
    device.policy_target_surfaces = surfaces_for(device.enrollable);
}

pub(super) fn mark_device_paired(device: &mut LanCanonicalHouseholdDevice) {
    device.trust_state = LanPairingTrustState::Paired;
    if let Some(inventory) = device.child_agent_inventory.as_mut() {
        inventory.pairing_trust_state = LanPairingTrustState::Paired;
    }
}
