use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};
use serde_json::Value;

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
