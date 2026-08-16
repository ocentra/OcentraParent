use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice, LanDiscoveryEventHistory,
    LanDiscoveryEventHistoryState, LanSelectedDeviceReadiness,
};

pub(super) fn discovery_event_history(
    generated_at: &str,
    unavailable_state: &LanDiscoveryEventHistoryState,
    physical_household_lan_state: &LanPairingProductionDiscoveryState,
    selected_device_readiness: &LanSelectedDeviceReadiness,
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> LanDiscoveryEventHistory {
    if *unavailable_state == LanDiscoveryEventHistoryState::Unavailable {
        return super::state::unavailable_event_history(generated_at);
    }

    let scan_session_id = super::state::scan_session_id(generated_at);
    let scan_started_at = super::super::history_time::earliest_canonical_or_discovered_observed_at(
        devices,
        discovered_devices,
    )
    .unwrap_or_else(|| generated_at.to_string());
    let scan_finished_at = super::super::history_time::latest_canonical_or_discovered_observed_at(
        devices,
        discovered_devices,
    )
    .unwrap_or_else(|| generated_at.to_string());
    let mut rows = Vec::new();
    super::rows::push_scan_started_row(
        &mut rows,
        scan_started_at.as_str(),
        &scan_session_id,
        devices,
        discovered_devices,
    );
    super::rows::push_discovered_agent_event_rows(&mut rows, &scan_session_id, discovered_devices);
    super::rows::push_canonical_device_event_rows(
        &mut rows,
        generated_at,
        &scan_session_id,
        devices,
    );
    super::rows::push_scan_finished_row(
        &mut rows,
        scan_finished_at.as_str(),
        &scan_session_id,
        devices,
        discovered_devices,
    );
    super::rows::normalize_discovery_event_rows(&mut rows);

    let latest_event_id = rows.last().map(|row| row.event_id.clone());
    let latest_observed_at = rows.last().map(|row| row.occurred_at.clone());
    let state = super::state::history_state(
        &rows,
        physical_household_lan_state,
        selected_device_readiness,
    );
    LanDiscoveryEventHistory {
        schema_version: ocentra_parent_agent_protocol::constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        state,
        latest_event_id,
        latest_observed_at,
        rows,
    }
}
