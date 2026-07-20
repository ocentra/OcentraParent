use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
};
use ocentra_parent_agent_protocol::logging::LogFields;
use serde_json::Value;

use super::{required_text_field, LAN_REPLAY_CONTEXT};
use crate::agent_service_client::snapshots_common::optional_bool_field;

pub(super) fn validate_history_state<'a>(
    payload: &LogFields,
    entry_count: u64,
    mut event_kinds: impl Iterator<Item = &'a LanDiscoveryEventKind>,
) -> Result<(), String> {
    let history_state_label =
        required_text_field(payload, constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE)?;
    let history_state = serde_json::from_value::<LanDiscoveryEventHistoryState>(Value::String(
        history_state_label.to_string(),
    ))
    .map_err(|error| format!("{LAN_REPLAY_CONTEXT} history state parse failed: {error}"))?;
    let manual_required =
        optional_bool_field(payload, constants::field::LAN_RUNTIME_MANUAL_REQUIRED_STATE)
            .ok_or_else(|| {
                format!(
                    "{LAN_REPLAY_CONTEXT} missing {}",
                    constants::field::LAN_RUNTIME_MANUAL_REQUIRED_STATE
                )
            })?;
    if manual_required != matches!(history_state, LanDiscoveryEventHistoryState::ManualRequired) {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected inconsistent history and manual-required state"
        ));
    }
    let has_material_entries = event_kinds.any(is_material_discovery_event_kind);
    let state_matches_entries = match history_state {
        LanDiscoveryEventHistoryState::Unavailable
        | LanDiscoveryEventHistoryState::Degraded
        | LanDiscoveryEventHistoryState::AgentOffline => true,
        LanDiscoveryEventHistoryState::Ready => has_material_entries,
        LanDiscoveryEventHistoryState::Empty => !has_material_entries,
        LanDiscoveryEventHistoryState::ManualRequired => entry_count == 0,
    };
    if !state_matches_entries {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected history state that does not match stream entries"
        ));
    }
    Ok(())
}

fn is_material_discovery_event_kind(event_kind: &LanDiscoveryEventKind) -> bool {
    !matches!(
        event_kind,
        LanDiscoveryEventKind::InterfaceChanged
            | LanDiscoveryEventKind::ScanStarted
            | LanDiscoveryEventKind::ScanFinished
    )
}
