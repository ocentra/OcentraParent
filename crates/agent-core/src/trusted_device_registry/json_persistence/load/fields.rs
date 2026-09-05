use ocentra_parent_agent_protocol::{
    constants, lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(super) fn strict_optional_string(value: &Value, key: &str) -> Option<Option<String>> {
    match value.get(key) {
        None | Some(Value::Null) => Some(None),
        Some(Value::String(value)) if !value.trim().is_empty() => Some(Some(value.clone())),
        Some(_) => None,
    }
}

pub(super) fn strict_optional_array<T: DeserializeOwned>(
    value: &Value,
    key: &str,
) -> Option<Vec<T>> {
    match value.get(key) {
        None => Some(Vec::new()),
        Some(value) => serde_json::from_value(value.clone()).ok(),
    }
}

pub(super) fn strict_optional_object<T: DeserializeOwned + Default>(
    value: &Value,
    key: &str,
) -> Option<T> {
    match value.get(key) {
        None => Some(T::default()),
        Some(value) => serde_json::from_value(value.clone()).ok(),
    }
}

pub(super) fn strict_household_decisions(value: &Value) -> Option<Vec<LanHouseholdDeviceDecision>> {
    let decisions = strict_optional_array(
        value,
        constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS,
    )?;
    let max = constants::lan_pairing::LAN_PAIRING_MAX_HOUSEHOLD_DECISION_HISTORY;
    let skip = decisions.len().saturating_sub(max);
    Some(decisions.into_iter().skip(skip).collect())
}
