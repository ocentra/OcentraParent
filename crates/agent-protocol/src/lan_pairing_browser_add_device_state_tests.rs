use crate::{
    constants, LanBrowserAddDeviceReadModel, LanPairingDeviceReachability,
    LanPairingDiscoverySource, LanPairingParentAuthority, LanPairingProductionDiscoveryState,
    LanPairingTrustState, LanSelectedDeviceReadiness, LAN_PAIRING_SCHEMA_VERSION,
};

#[test]
fn browser_add_device_read_model_serializes_honest_states() {
    let model = LanBrowserAddDeviceReadModel {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: "2026-06-01T15:20:00.000Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        add_device_state: LanPairingProductionDiscoveryState::Pending,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Pending,
        physical_household_lan_state: LanPairingProductionDiscoveryState::ManualRequired,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: Vec::new(),
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
        route_requirement_labels: vec![
            constants::lan_pairing::ROUTE_REQUIREMENT_ALLOWED_ORIGIN.to_string()
        ],
        audit_check_labels: vec![
            constants::value::LAN_REASON_WRONG_ORIGIN.to_string(),
            constants::value::LAN_REASON_REPLAYED.to_string(),
        ],
        honest_non_claims: vec![
            constants::value::LAN_NON_CLAIM_CLOUD_RELAY_NOT_IMPLEMENTED.to_string()
        ],
    };

    let json = serde_json::to_string(&model).expect("read model serializes");
    let value: serde_json::Value = serde_json::from_str(&json).expect("read model parses");
    assert_eq!(
        value[constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED)
    );
    assert_eq!(
        value[constants::field::LAN_CLOUD_RELAY_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE)
    );
    assert_eq!(
        value["selectedDeviceReadiness"]["readyForControl"],
        serde_json::json!(false)
    );
    assert_eq!(value["trustedDeviceRegistry"], serde_json::json!([]));
}
