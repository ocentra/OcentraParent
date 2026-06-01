use crate::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LanBrowserAddDeviceRequest,
    LanBrowserDiscoveryScanRequest, LanPairingDiscoverySource, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION, LAN_PAIRING_SCHEMA_VERSION,
};

#[test]
fn browser_runtime_command_and_event_names_serialize_for_portal_consumption() {
    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::lan_pairing::INTENT_ID.to_string(),
        sent_at: constants::lan_pairing::ISSUED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::LocalNetwork,
        },
        command: AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
        payload: LogFields::new(),
    };

    let json = serde_json::to_value(command).expect("browser discovery command serializes");
    assert_eq!(
        json[constants::field::COMMAND],
        serde_json::json!(constants::lan_pairing::COMMAND_BROWSER_DISCOVERY_SCAN)
    );
    assert_eq!(
        serde_json::to_value(AgentEventName::AgentLanPairingAddDeviceReported)
            .expect("add-device event serializes"),
        serde_json::json!(constants::lan_pairing::EVENT_ADD_DEVICE_REPORTED)
    );
}

#[test]
fn browser_runtime_payloads_serialize_without_fixture_devices() {
    let scan = LanBrowserDiscoveryScanRequest {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        requested_discovery_source: LanPairingDiscoverySource::LocalService,
    };
    let add_device = LanBrowserAddDeviceRequest {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
    };

    let scan_json = serde_json::to_value(scan).expect("scan payload serializes");
    let add_device_json = serde_json::to_value(add_device).expect("add-device payload serializes");
    assert_eq!(
        scan_json["requestedDiscoverySource"],
        serde_json::json!(constants::value::LAN_DISCOVERY_SOURCE_LOCAL_SERVICE)
    );
    assert_eq!(
        add_device_json[constants::field::LAN_CHILD_DEVICE_ID],
        serde_json::json!(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        add_device_json[constants::field::LAN_ROUTE_ID],
        serde_json::json!(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK)
    );
}
