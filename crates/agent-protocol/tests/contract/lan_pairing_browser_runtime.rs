use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPairingDiscoverySource;
use ocentra_parent_agent_protocol::lan_pairing_browser_runtime::{
    LanBrowserAddDeviceRequest, LanBrowserDiscoveryScanRequest,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::AgentCommandName;
use ocentra_parent_agent_protocol::AgentEventName;
use ocentra_parent_agent_protocol::AgentMessageTarget;
use ocentra_parent_agent_protocol::AgentPeer;
use ocentra_parent_agent_protocol::AgentPeerRole;
use ocentra_parent_agent_protocol::AgentRoute;
use ocentra_parent_agent_protocol::LogFields;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::LAN_PAIRING_SCHEMA_VERSION;

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

    let json = serde_json::to_value(command).expect_value("browser discovery command serializes");
    assert_eq!(
        json[constants::field::COMMAND],
        serde_json::json!(constants::lan_pairing::COMMAND_BROWSER_DISCOVERY_SCAN)
    );
    assert_eq!(
        serde_json::to_value(AgentEventName::AgentLanPairingAddDeviceReported)
            .expect_value("add-device event serializes"),
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

    let scan_json = serde_json::to_value(scan).expect_value("scan payload serializes");
    let add_device_json =
        serde_json::to_value(add_device).expect_value("add-device payload serializes");
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
    assert_eq!(
        add_device_json[constants::field::LAN_CHILD_DEVICE_ID],
        serde_json::json!(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        add_device_json[constants::field::LAN_PARENT_DEVICE_ID],
        serde_json::json!(constants::lan_pairing::PARENT_DEVICE_ID)
    );
    assert_eq!(
        add_device_json["issuedAt"],
        serde_json::json!(constants::lan_pairing::ISSUED_AT)
    );
    assert_eq!(
        add_device_json["expiresAt"],
        serde_json::json!(constants::lan_pairing::EXPIRES_AT)
    );
}

#[test]
fn browser_runtime_payloads_reject_missing_required_fields() {
    let scan_error = serde_json::from_value::<LanBrowserDiscoveryScanRequest>(serde_json::json!({
        "schemaVersion": LAN_PAIRING_SCHEMA_VERSION
    }))
    .expect_err_value("scan payload must reject missing required fields");
    let add_device_error =
        serde_json::from_value::<LanBrowserAddDeviceRequest>(serde_json::json!({
            "schemaVersion": LAN_PAIRING_SCHEMA_VERSION,
            "childDeviceId": constants::lan_pairing::CHILD_DEVICE_ID
        }))
        .expect_err_value("add-device payload must require all required fields");
    let add_device_missing_route_error =
        serde_json::from_value::<LanBrowserAddDeviceRequest>(serde_json::json!({
            "schemaVersion": LAN_PAIRING_SCHEMA_VERSION,
            "childDeviceId": constants::lan_pairing::CHILD_DEVICE_ID,
            "parentDeviceId": constants::lan_pairing::PARENT_DEVICE_ID,
            "origin": constants::lan_pairing::ALLOWED_ORIGIN,
            "issuedAt": constants::lan_pairing::ISSUED_AT,
            "expiresAt": constants::lan_pairing::EXPIRES_AT
        }))
        .expect_err_value("add-device payload must require a route id");
    let add_device_missing_origin_error =
        serde_json::from_value::<LanBrowserAddDeviceRequest>(serde_json::json!({
            "schemaVersion": LAN_PAIRING_SCHEMA_VERSION,
            "childDeviceId": constants::lan_pairing::CHILD_DEVICE_ID,
            "parentDeviceId": constants::lan_pairing::PARENT_DEVICE_ID,
            "routeId": constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
            "issuedAt": constants::lan_pairing::ISSUED_AT,
            "expiresAt": constants::lan_pairing::EXPIRES_AT
        }))
        .expect_err_value("add-device payload must require an origin");

    assert_eq!(scan_error.classify(), serde_json::error::Category::Data);
    assert_eq!(
        add_device_error.classify(),
        serde_json::error::Category::Data
    );
    assert_eq!(
        add_device_missing_route_error.classify(),
        serde_json::error::Category::Data
    );
    assert_eq!(
        add_device_missing_origin_error.classify(),
        serde_json::error::Category::Data
    );
}
