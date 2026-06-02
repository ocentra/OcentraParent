use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
    AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue,
    LogFields, V08EnforcementProductControlSpineReadModel, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn product_control_spine_dispatcher_returns_typed_runtime_read_model() {
    let event = send_product_control_command().await;

    assert_eq!(
        event.event,
        AgentEventName::AgentEnforcementProductControlSpineReported
    );
    assert_eq!(
        string_payload_field(&event, constants::field::READ_MODEL_ID),
        constants::v08_enforcement_product_control_spine::READ_MODEL_ID
    );
    assert_eq!(
        number_payload_field(&event, constants::field::RETURNED),
        15.0
    );

    let read_model: V08EnforcementProductControlSpineReadModel =
        serde_json::from_str(string_payload_field(
            &event,
            constants::field::ENFORCEMENT_PRODUCT_CONTROL_SPINE_READ_MODEL,
        ))
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        read_model.read_model_id,
        constants::v08_enforcement_product_control_spine::READ_MODEL_ID
    );
    assert_eq!(read_model.entries.len(), 15);
    assert!(read_model.source_read_model_ids.contains(
        &constants::v08_enforcement_product_control_spine::SOURCE_BROWSER_DOMAIN.to_string()
    ));
    assert!(read_model.entries.iter().all(|entry| {
        !entry.broad_app_blocking_claimed
            && !entry.network_domain_blocking_claimed
            && !entry.managed_exact_url_blocking_claimed
            && !entry.unmanaged_exact_url_claimed
            && !entry.tamper_resistance_claimed
            && !entry.notification_delivery_claimed
    }));
}

async fn send_product_control_command() -> AgentEventEnvelope {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::v08_enforcement_product_control_spine::READ_MODEL_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentEnforcementProductControlSpineGet,
        payload: LogFields::new(),
    }
}

fn string_payload_field<'a>(event: &'a AgentEventEnvelope, field: &str) -> &'a str {
    match event.payload.get(field) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn number_payload_field(event: &AgentEventEnvelope, field: &str) -> f64 {
    match event.payload.get(field) {
        Some(LogFieldValue::Number(value)) => *value,
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
