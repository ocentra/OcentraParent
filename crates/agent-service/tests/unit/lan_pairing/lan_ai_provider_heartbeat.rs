use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::app::lan_pairing::LanPairingRuntime;

#[test]
fn stale_lan_ai_provider_heartbeat_blocks_lower_level_routing_and_degrades_status() {
    let runtime = LanPairingRuntime::empty();
    runtime.mark_lan_ai_provider_heartbeat_stale_for_test();

    assert_eq!(
        runtime.lan_ai_provider_heartbeat_reachability(),
        Some(LanPairingDeviceReachability::Stale)
    );
    assert!(!runtime.lan_ai_provider_heartbeat_allows_routing());
    assert_eq!(
        runtime.lan_ai_provider_status_value().0,
        constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    );
}

#[test]
fn offline_lan_ai_provider_heartbeat_blocks_lower_level_routing_and_reports_unavailable() {
    let runtime = LanPairingRuntime::empty();
    runtime.mark_lan_ai_provider_heartbeat_offline_for_test();

    assert_eq!(
        runtime.lan_ai_provider_heartbeat_reachability(),
        Some(LanPairingDeviceReachability::Offline)
    );
    assert!(!runtime.lan_ai_provider_heartbeat_allows_routing());
    assert_eq!(
        runtime.lan_ai_provider_status_value().0,
        constants::value::LAN_AI_PROVIDER_STATUS_UNAVAILABLE
    );
}
