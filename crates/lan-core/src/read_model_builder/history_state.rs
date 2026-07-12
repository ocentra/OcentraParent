use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistory, LanDiscoveryEventHistoryState, LanDiscoveryEventRow,
    LanSelectedDeviceReadiness,
};

pub(super) fn unavailable_event_history(generated_at: &str) -> LanDiscoveryEventHistory {
    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        state: LanDiscoveryEventHistoryState::Unavailable,
        latest_event_id: None,
        latest_observed_at: None,
        rows: Vec::new(),
    }
}

pub(super) fn history_state(
    rows: &[LanDiscoveryEventRow],
    physical_household_lan_state: &LanPairingProductionDiscoveryState,
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanDiscoveryEventHistoryState {
    if has_agent_offline_history_state(selected_device_readiness) {
        LanDiscoveryEventHistoryState::AgentOffline
    } else if rows.is_empty()
        && *physical_household_lan_state == LanPairingProductionDiscoveryState::ManualRequired
    {
        LanDiscoveryEventHistoryState::ManualRequired
    } else if rows.is_empty() {
        LanDiscoveryEventHistoryState::Empty
    } else {
        LanDiscoveryEventHistoryState::Ready
    }
}

fn has_agent_offline_history_state(selected_device_readiness: &LanSelectedDeviceReadiness) -> bool {
    selected_device_readiness.selected_child_device_id.is_some()
        && selected_device_readiness.route_id.is_none()
        && selected_device_readiness.trust_state == LanPairingTrustState::Paired
        && selected_device_readiness.reachability == LanPairingDeviceReachability::Online
        && !selected_device_readiness.ready_for_control
}

pub(super) fn scan_session_id(generated_at: &str) -> String {
    let mut id = String::from("lan-scan-");
    id.push_str(&super::super::history_time::compact_event_identifier(
        generated_at,
    ));
    id
}
