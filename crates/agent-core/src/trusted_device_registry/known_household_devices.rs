use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};
use serde_json::Value;

mod merge;
use self::merge::{merge_known_household_device, same_known_household_device};

pub(super) fn household_device_decisions_from_json(
    value: &Value,
) -> Option<Vec<LanHouseholdDeviceDecision>> {
    value
        .get(constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS)
        .and_then(|decisions| serde_json::from_value(decisions.clone()).ok())
}

pub(super) fn known_household_devices_from_json(
    value: &Value,
) -> Option<Vec<LanCanonicalHouseholdDevice>> {
    value
        .get(constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES)
        .and_then(|devices| serde_json::from_value(devices.clone()).ok())
}

pub(super) fn optional_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn upsert_known_household_device(
    devices: &mut Vec<LanCanonicalHouseholdDevice>,
    incoming: LanCanonicalHouseholdDevice,
) -> bool {
    if let Some(existing) = devices
        .iter_mut()
        .find(|device| same_known_household_device(device, &incoming))
    {
        let before = existing.clone();
        merge_known_household_device(existing, incoming);
        return before != *existing;
    }

    devices.push(incoming);
    true
}

pub(super) fn restore_known_household_device(
    mut device: LanCanonicalHouseholdDevice,
    observed_at: &str,
) -> LanCanonicalHouseholdDevice {
    if device.trust_state != LanPairingTrustState::Paired
        && device.trust_state != LanPairingTrustState::Revoked
    {
        let offline_persisted = device.network_identity.reachability
            == LanPairingDeviceReachability::Offline
            || device.network_identity.offline_at.is_some()
            || device.discovery_state == LanPairingProductionDiscoveryState::Offline;
        if offline_persisted {
            device.discovery_state = LanPairingProductionDiscoveryState::Offline;
            device.network_identity.reachability = LanPairingDeviceReachability::Offline;
            if device.network_identity.offline_at.is_none() {
                device.network_identity.offline_at = Some(observed_at.to_string());
            }
        } else {
            device.discovery_state = match device.discovery_state {
                LanPairingProductionDiscoveryState::Revoked => {
                    LanPairingProductionDiscoveryState::Revoked
                }
                _ => LanPairingProductionDiscoveryState::Stale,
            };
            device.network_identity.reachability = LanPairingDeviceReachability::Stale;
            if device.network_identity.stale_at.is_none() {
                device.network_identity.stale_at = Some(observed_at.to_string());
            }
        }
    }
    device
}
