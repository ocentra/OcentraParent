use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSpineReadModel;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;
use std::primitive::str as TestStr;
use std::string::String as TestString;

type TestResult = Result<(), TestString>;

#[tokio::test]
async fn product_control_spine_dispatcher_returns_typed_runtime_read_model() -> TestResult {
    let event = send_product_control_command().await?;

    assert_eq!(
        event.event,
        AgentEventName::AgentEnforcementProductControlSpineReported
    );
    assert_eq!(
        string_payload_field(&event, constants::field::READ_MODEL_ID)?,
        constants::v08_enforcement_product_control_spine::READ_MODEL_ID
    );
    assert_eq!(
        number_payload_field(&event, constants::field::RETURNED)?,
        15.0
    );

    let read_model: V08EnforcementProductControlSpineReadModel = ok(
        serde_json::from_str(string_payload_field(
            &event,
            constants::field::ENFORCEMENT_PRODUCT_CONTROL_SPINE_READ_MODEL,
        )?),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;

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

    Ok(())
}

async fn send_product_control_command() -> Result<AgentEventEnvelope, TestString> {
    let body = ok(
        serde_json::to_string(&command_envelope()),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    Ok(handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await)
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

fn string_payload_field<'a>(
    event: &'a AgentEventEnvelope,
    field: &TestStr,
) -> Result<&'a TestStr, TestString> {
    match event.payload.get(field) {
        Some(LogFieldValue::String(value)) => Ok(value.as_str()),
        _ => Err(constants::error::AGENT_EVENT_SERIALIZES.to_string()),
    }
}

fn number_payload_field(event: &AgentEventEnvelope, field: &TestStr) -> Result<f64, TestString> {
    match event.payload.get(field) {
        Some(LogFieldValue::Number(value)) => Ok(*value),
        _ => Err(constants::error::AGENT_EVENT_SERIALIZES.to_string()),
    }
}

fn ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &TestStr) -> Result<T, TestString> {
    result.map_err(|error| format!("{context}: {error:?}"))
}
