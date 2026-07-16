use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
};
use serde_json::Value;

use crate::json_contract;

pub(super) struct LanRuntimeStreamLabel(pub(super) String);

pub(super) fn discovery_event_kind_label(kind: &LanDiscoveryEventKind) -> LanRuntimeStreamLabel {
    match json_contract::serialize_json_value(kind) {
        Value::String(label) => LanRuntimeStreamLabel(label),
        _ => LanRuntimeStreamLabel(constants::value::EMPTY.to_string()),
    }
}

pub(super) fn discovery_history_state_label(
    state: &LanDiscoveryEventHistoryState,
) -> LanRuntimeStreamLabel {
    match json_contract::serialize_json_value(state) {
        Value::String(label) => LanRuntimeStreamLabel(label),
        _ => LanRuntimeStreamLabel(constants::value::EMPTY.to_string()),
    }
}
