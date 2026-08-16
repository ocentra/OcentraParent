use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::child_profile::child_profile_device_id;
use super::value_support::{compact_identifier, known_hostname, preferred_mac_identity};

pub(super) fn canonical_device_id(device: &LanPairingDeviceRef) -> String {
    if let Some(child_profile_id) = child_profile_device_id(device).as_deref() {
        return super::child_profile::canonical_child_profile_device_id(child_profile_id);
    }
    if let Some(mac) = preferred_mac_identity(device).as_deref() {
        let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
        id.push_str(&compact_identifier(mac));
        let device_suffix = compact_identifier(&device.device_id);
        if !device_suffix.is_empty() {
            id.push('-');
            id.push_str(&device_suffix);
        }
        return id;
    }
    let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_ID_PREFIX);
    id.push_str(&compact_identifier(&device.device_id));
    id
}

pub(super) fn display_name_for(device: &LanPairingDeviceRef) -> String {
    known_hostname(device).unwrap_or_else(|| device.label.clone())
}
