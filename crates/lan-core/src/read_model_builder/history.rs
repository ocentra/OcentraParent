#[path = "history_builder.rs"]
mod builder;
#[path = "history_event_kind.rs"]
mod event_kind;
#[path = "history_rows.rs"]
mod rows;
#[path = "history_state.rs"]
mod state;

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
    builder::discovery_event_history(
        generated_at,
        unavailable_state,
        physical_household_lan_state,
        selected_device_readiness,
        devices,
        discovered_devices,
    )
}
