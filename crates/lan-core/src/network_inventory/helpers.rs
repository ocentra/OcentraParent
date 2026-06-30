use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingProductionDiscoveryState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

use super::LanNetworkInventoryDevice;

pub(super) fn discovered_child_device_ref(
    network_device: &LanNetworkInventoryDevice,
) -> LanPairingDeviceRef {
    let mut child_device = LanPairingDeviceRef::new(
        network_device.device_id.clone(),
        None,
        network_device.label.clone(),
        network_device.platform.clone(),
    );
    child_device.ip_address = Some(network_device.ip_address.clone());
    child_device.mac_address = Some(network_device.mac_address.clone());
    child_device.hostname = network_device.hostname.clone();
    child_device.network_interface = network_device.network_interface.clone();
    child_device.agent_status = network_device.agent_status.clone();
    child_device
}

pub(super) fn discovery_hint_sources(
    network_device: &LanNetworkInventoryDevice,
) -> Vec<LanDiscoveryEvidenceSource> {
    if network_device.used_previous_scan_hint {
        vec![LanDiscoveryEvidenceSource::PreviousScanSnapshot]
    } else {
        Vec::new()
    }
}

pub(super) fn discovery_state_for_reachability(
    reachability: &LanPairingDeviceReachability,
) -> LanPairingProductionDiscoveryState {
    match reachability {
        LanPairingDeviceReachability::Online => LanPairingProductionDiscoveryState::Discovered,
        LanPairingDeviceReachability::Stale => LanPairingProductionDiscoveryState::Stale,
        LanPairingDeviceReachability::Offline => LanPairingProductionDiscoveryState::Offline,
    }
}

pub(super) fn trimmed_non_empty(value: impl AsRef<str>) -> Option<String> {
    let value = value.as_ref().trim().to_string();
    (!value.is_empty()).then_some(value)
}

pub(super) fn normalized_lookup_key(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_ascii_lowercase())
}
