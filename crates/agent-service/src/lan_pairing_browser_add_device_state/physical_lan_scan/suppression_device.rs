use chrono::{DateTime, Utc};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceRef, LanPairingText, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
};

pub(super) fn household_device_should_suppress_redundant_scan_work(
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    matches!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
            | LanCanonicalHouseholdDeviceClassification::ChildAgent
    ) || matches!(
        device.trust_state,
        LanPairingTrustState::Paired | LanPairingTrustState::Revoked
    ) || device.child_agent_inventory.is_some()
}

pub(super) fn household_scan_suppression_device(
    device: &LanCanonicalHouseholdDevice,
) -> Option<LanPairingDeviceRef> {
    let platform = scan_suppression_platform(device);
    let mut truth_device = LanPairingDeviceRef::new(
        device.canonical_device_id.clone(),
        None,
        device.display_name.clone(),
        platform,
    );
    truth_device.ip_address = device.network_identity.ip_addresses.first().cloned();
    truth_device.mac_address = device.network_identity.mac_address.clone();
    truth_device.hostname = device.network_identity.hostname.clone();
    truth_device.network_interface = device.network_identity.network_interfaces.first().cloned();
    (truth_device.ip_address.is_some() || truth_device.mac_address.is_some())
        .then_some(truth_device)
}

pub(super) fn push_unique_scan_truth_device(
    devices: &mut Vec<LanPairingDeviceRef>,
    candidate: LanPairingDeviceRef,
) {
    if devices.iter().any(|existing| {
        existing
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
        return;
    }
    devices.push(candidate);
}

pub(super) fn scan_session_id(now: DateTime<Utc>) -> LanPairingText {
    LanPairingText(format!("lan-scan-{}", now.timestamp_millis()))
}

fn scan_suppression_platform(device: &LanCanonicalHouseholdDevice) -> LanPairingText {
    if device.classification == LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure {
        return LanPairingText(constants::lan_pairing::PLATFORM_ROUTER.to_string());
    }
    device
        .child_agent_inventory
        .as_ref()
        .map(|inventory| LanPairingText(inventory.platform.clone()))
        .unwrap_or_else(|| LanPairingText(constants::lan_pairing::PLATFORM_UNKNOWN.to_string()))
}
