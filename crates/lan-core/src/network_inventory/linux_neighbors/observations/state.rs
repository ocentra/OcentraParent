use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::network_inventory_command::value_text;

pub(super) fn reachability_from_linux_state(
    state: Option<&serde_json::Value>,
) -> Option<LanPairingDeviceReachability> {
    let states = linux_state_labels(state);
    if states.is_empty()
        || states
            .iter()
            .any(|state| state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_INCOMPLETE)
    {
        return None;
    }
    if states.iter().any(|state| is_online_state(state)) {
        return Some(LanPairingDeviceReachability::Online);
    }
    if states
        .iter()
        .any(|state| state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_STALE)
    {
        return Some(LanPairingDeviceReachability::Stale);
    }
    if states
        .iter()
        .any(|state| state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_FAILED)
    {
        return Some(LanPairingDeviceReachability::Offline);
    }
    Some(LanPairingDeviceReachability::Stale)
}

pub(super) fn linux_state_labels(state: Option<&serde_json::Value>) -> Vec<String> {
    match state {
        Some(serde_json::Value::Array(values)) => values
            .iter()
            .filter_map(value_text)
            .map(|value| value.to_ascii_lowercase())
            .collect(),
        Some(serde_json::Value::String(value)) => vec![value.trim().to_ascii_lowercase()],
        Some(other) => value_text(other)
            .map(|value| vec![value.to_ascii_lowercase()])
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

fn is_online_state(state: &str) -> bool {
    [
        constants::lan_pairing::LINUX_NEIGHBOR_STATE_REACHABLE,
        constants::lan_pairing::LINUX_NEIGHBOR_STATE_PERMANENT,
        constants::lan_pairing::LINUX_NEIGHBOR_STATE_DELAY,
        constants::lan_pairing::LINUX_NEIGHBOR_STATE_PROBE,
    ]
    .contains(&state)
}
