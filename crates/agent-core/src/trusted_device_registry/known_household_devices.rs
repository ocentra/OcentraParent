use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};
use serde_json::Value;

mod classification;
mod discovery;
mod display;
mod evidence;
mod evidence_rank;
mod identity;
mod inventory;
mod json;
mod merge;
mod network_identity;
mod rank;
mod restore;
mod route;
mod sources;
mod strings;
mod trust;

use self::merge::{merge_known_household_device, same_known_household_device};

pub(super) fn household_device_decisions_from_json(
    value: &Value,
) -> Option<Vec<LanHouseholdDeviceDecision>> {
    self::json::household_device_decisions_from_json(value)
}

pub(super) fn known_household_devices_from_json(
    value: &Value,
) -> Option<Vec<LanCanonicalHouseholdDevice>> {
    self::json::known_household_devices_from_json(value)
}

pub(super) fn optional_string(value: &Value, key: &str) -> Option<String> {
    self::json::optional_string(value, key)
}

pub(super) fn restore_known_household_device(
    device: LanCanonicalHouseholdDevice,
    observed_at: &str,
) -> LanCanonicalHouseholdDevice {
    self::restore::restore_known_household_device(device, observed_at)
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
