use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingDeviceRef, LanPairingTrustState};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;

use super::known_household_devices::upsert_known_household_device;

pub(super) fn household_scan_truth_device(
    device: &LanCanonicalHouseholdDevice,
) -> Option<LanPairingDeviceRef> {
    (!household_device_should_suppress_redundant_scan_work(device))
        .then(|| {
            let platform = scan_truth_platform(device);
            let mut truth_device = LanPairingDeviceRef::new(
                device.canonical_device_id.clone(),
                None,
                device.display_name.clone(),
                platform,
            );
            truth_device.ip_address = device.network_identity.ip_addresses.first().cloned();
            truth_device.mac_address = device.network_identity.mac_address.clone();
            truth_device.hostname = device.network_identity.hostname.clone();
            truth_device.network_interface =
                device.network_identity.network_interfaces.first().cloned();
            truth_device
        })
        .and_then(|truth_device| {
            (truth_device.ip_address.is_some() || truth_device.mac_address.is_some())
                .then_some(truth_device)
        })
}

pub(super) fn household_device_should_suppress_redundant_scan_work(
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    let suppress_by_classification =
        [LanCanonicalHouseholdDeviceClassification::ChildAgent].contains(&device.classification);
    let suppress_by_trust =
        [LanPairingTrustState::Paired, LanPairingTrustState::Revoked].contains(&device.trust_state);
    let suppress_by_inventory = device.child_agent_inventory.is_some();

    suppress_by_classification || suppress_by_trust || suppress_by_inventory
}

pub(super) fn scan_truth_platform(device: &LanCanonicalHouseholdDevice) -> String {
    if device.classification
        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
    {
        return constants::lan_pairing::PLATFORM_ROUTER.to_string();
    }
    device
        .child_agent_inventory
        .as_ref()
        .map(|inventory| inventory.platform.clone())
        .unwrap_or_else(|| constants::lan_pairing::PLATFORM_UNKNOWN.to_string())
}

pub(super) fn merge_known_household_device_by_canonical_id(
    devices: &mut Vec<LanCanonicalHouseholdDevice>,
    incoming: LanCanonicalHouseholdDevice,
) -> bool {
    if let Some(existing_index) = devices
        .iter()
        .position(|device| device.canonical_device_id == incoming.canonical_device_id)
    {
        let existing = devices.remove(existing_index);
        let mut merged = vec![existing.clone()];
        let changed = upsert_known_household_device(&mut merged, incoming);
        let merged_device = merged.pop().unwrap_or_else(|| existing.clone());
        let changed = changed || merged_device != existing;
        devices.insert(existing_index, merged_device);
        return changed;
    }

    devices.push(incoming);
    true
}

pub(super) fn push_unique_scan_truth_device(
    devices: &mut Vec<LanPairingDeviceRef>,
    candidate: LanPairingDeviceRef,
) {
    if let Some(existing) = devices.iter_mut().find(|existing| {
        existing
            .device_id
            .eq_ignore_ascii_case(&candidate.device_id)
            || existing
                .mac_address
                .as_deref()
                .zip(candidate.mac_address.as_deref())
                .map(|(left, right)| left.eq_ignore_ascii_case(right))
                .unwrap_or(false)
            || existing
                .ip_address
                .as_deref()
                .zip(candidate.ip_address.as_deref())
                .map(|(left, right)| left.eq_ignore_ascii_case(right))
                .unwrap_or(false)
    }) {
        merge_scan_truth_device(existing, candidate);
        return;
    }
    devices.push(candidate);
}

pub(super) fn merge_scan_truth_device(
    existing: &mut LanPairingDeviceRef,
    incoming: LanPairingDeviceRef,
) {
    if existing.child_profile_id.is_none() {
        existing.child_profile_id = incoming.child_profile_id;
    }
    if existing.ip_address.is_none() {
        existing.ip_address = incoming.ip_address;
    }
    if existing.mac_address.is_none() {
        existing.mac_address = incoming.mac_address;
    }
    if existing.hostname.is_none() {
        existing.hostname = incoming.hostname;
    }
    if existing.network_interface.is_none() {
        existing.network_interface = incoming.network_interface;
    }
    if existing.agent_status.is_none() {
        existing.agent_status = incoming.agent_status;
    }
    if existing.hardware_profile.is_none() {
        existing.hardware_profile = incoming.hardware_profile;
    }
    if existing.platform == constants::lan_pairing::PLATFORM_UNKNOWN
        && incoming.platform != constants::lan_pairing::PLATFORM_UNKNOWN
    {
        existing.platform = incoming.platform;
    }
    if !scan_truth_label_is_specific(&existing.label)
        && scan_truth_label_is_specific(&incoming.label)
    {
        existing.label = incoming.label;
    }
}

pub(super) fn scan_truth_label_is_specific(label: &str) -> bool {
    !label.trim().is_empty()
        && !label.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
}
