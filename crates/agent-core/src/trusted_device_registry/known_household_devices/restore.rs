use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

pub(super) fn restore_known_household_device(
    mut device: LanCanonicalHouseholdDevice,
    observed_at: &str,
) -> LanCanonicalHouseholdDevice {
    if device.trust_state == LanPairingTrustState::Paired
        || device.trust_state == LanPairingTrustState::Revoked
    {
        return device;
    }

    if restore_as_offline(&device) {
        restore_offline(&mut device, observed_at);
    } else {
        restore_stale(&mut device, observed_at);
    }
    device
}

fn restore_as_offline(device: &LanCanonicalHouseholdDevice) -> bool {
    device.network_identity.reachability == LanPairingDeviceReachability::Offline
        || device.network_identity.offline_at.is_some()
        || device.discovery_state == LanPairingProductionDiscoveryState::Offline
}

fn restore_offline(device: &mut LanCanonicalHouseholdDevice, observed_at: &str) {
    device.discovery_state = LanPairingProductionDiscoveryState::Offline;
    device.network_identity.reachability = LanPairingDeviceReachability::Offline;
    if device.network_identity.offline_at.is_none() {
        device.network_identity.offline_at = Some(observed_at.to_string());
    }
}

fn restore_stale(device: &mut LanCanonicalHouseholdDevice, observed_at: &str) {
    device.discovery_state = match device.discovery_state {
        LanPairingProductionDiscoveryState::Revoked => LanPairingProductionDiscoveryState::Revoked,
        _ => LanPairingProductionDiscoveryState::Stale,
    };
    device.network_identity.reachability = LanPairingDeviceReachability::Stale;
    if device.network_identity.stale_at.is_none() {
        device.network_identity.stale_at = Some(observed_at.to_string());
    }
}
