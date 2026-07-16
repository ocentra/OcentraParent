use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, AGENT_TRANSPORT_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::{constants, LogFields, LogLevel};

#[test]
fn lan_pairing_transport_envelopes_keep_message_and_event_fields_explicit() {
    let command = AgentCommandEnvelope {
        schema_version: AGENT_TRANSPORT_SCHEMA_VERSION,
        message_id: constants::lan_pairing::INTENT_ID.to_string(),
        sent_at: constants::lan_pairing::ISSUED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::LocalNetwork,
        },
        command: AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
        payload: LogFields::new(),
    };
    let event = AgentEventEnvelope {
        schema_version: AGENT_TRANSPORT_SCHEMA_VERSION,
        event_id: constants::lan_pairing::AUDIT_EVENT_ID.to_string(),
        correlation_id: constants::lan_pairing::INTENT_ID.to_string(),
        sent_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentPeer {
            peer_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            role: AgentPeerRole::AgentService,
        },
        event: AgentEventName::AgentLanPairingAddDeviceReported,
        severity: LogLevel::Info,
        payload: LogFields::new(),
        snapshot: None,
    };

    let command_json = serde_json::to_value(command)
        .expect_value("lan pairing command envelope serializes: {error:?}");
    let event_json = serde_json::to_value(event)
        .expect_value("lan pairing event envelope serializes: {error:?}");

    assert_eq!(
        command_json[constants::field::COMMAND],
        serde_json::json!(constants::lan_pairing::COMMAND_BROWSER_DISCOVERY_SCAN)
    );
    assert_eq!(
        command_json[constants::field::MESSAGE_ID],
        serde_json::json!(constants::lan_pairing::INTENT_ID)
    );
    assert_eq!(
        command_json[constants::field::SENT_AT],
        serde_json::json!(constants::lan_pairing::ISSUED_AT)
    );
    assert_eq!(
        event_json[constants::field::EVENT_ID],
        serde_json::json!(constants::lan_pairing::AUDIT_EVENT_ID)
    );
    assert_eq!(
        event_json[constants::field::CORRELATION_ID],
        serde_json::json!(constants::lan_pairing::INTENT_ID)
    );
    assert_eq!(
        event_json[constants::field::SENT_AT],
        serde_json::json!(constants::lan_pairing::OBSERVED_AT)
    );
    assert_eq!(
        event_json[constants::field::EVENT],
        serde_json::json!(constants::lan_pairing::EVENT_ADD_DEVICE_REPORTED)
    );
}

#[test]
fn lan_pairing_transport_envelopes_reject_missing_required_fields() {
    let command_error = serde_json::from_value::<AgentCommandEnvelope>(serde_json::json!({
        "schemaVersion": AGENT_TRANSPORT_SCHEMA_VERSION,
        "messageId": constants::lan_pairing::INTENT_ID,
        "sentAt": constants::lan_pairing::ISSUED_AT
    }))
    .expect_err_value("transport command envelopes must reject missing required fields");
    let event_error = serde_json::from_value::<AgentEventEnvelope>(serde_json::json!({
        "schemaVersion": AGENT_TRANSPORT_SCHEMA_VERSION,
        "eventId": constants::lan_pairing::AUDIT_EVENT_ID
    }))
    .expect_err_value("transport event envelopes must reject missing required fields");

    assert_eq!(command_error.classify(), serde_json::error::Category::Data);
    assert_eq!(event_error.classify(), serde_json::error::Category::Data);
}
