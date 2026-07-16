use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingNetworkMode, LanPairingTrustState,
    LanSelectedRouteTarget,
};
use ocentra_parent_agent_service::lan_pairing_status::route_trust_state_for_selected_target;

#[test]
fn route_trust_state_reports_pairing_selected_target() {
    let target = LanSelectedRouteTarget {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        trust_state: LanPairingTrustState::Pairing,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        stale_at: None,
        offline_at: None,
    };

    assert_eq!(
        route_trust_state_for_selected_target(Some(&target)).0,
        constants::value::LAN_PAIRING_PAIRING
    );
}
