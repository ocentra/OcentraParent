use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, AppGameAdapterExecutionReadinessReadModel, LogFieldValue,
    LogFields, AGENT_PROTOCOL_SCHEMA_VERSION, APP_GAME_TEST_TIMESTAMP,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn app_game_adapter_execution_readiness_command_reports_service_backed_read_model() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model = adapter_execution_readiness_payload(
        &event.payload[constants::field::APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL],
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameAdapterExecutionReadinessReadModelReported
    );
    assert_eq!(read_model.returned, 8);
    assert_eq!(read_model.execution_allowed_count, 1);
    assert_eq!(read_model.blocked_before_execution_count, 7);
    assert_eq!(read_model.adapter_execution_claimed_count, 1);
    assert_eq!(
        read_model.host_capability_available_count
            + read_model.host_capability_not_detected_count
            + read_model.host_capability_not_applicable_count,
        read_model.returned
    );
    assert!(read_model.host_capability_available_count >= 4);
    assert!(read_model.host_capability_probe_ref_count >= 4);
    assert!(!read_model.broad_installed_app_blocking_claimed);
    assert!(!read_model.child_device_delivery_claimed);
    assert!(!read_model.platform_enforcement_claimed);
    assert!(!read_model.provider_delivery_claimed);
    assert!(!read_model.private_diagnostics_claimed);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id:
            constants::event_id::ACTIVITY_APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_REPORTED
                .to_string(),
        sent_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet,
        payload: LogFields::new(),
    }
}

fn adapter_execution_readiness_payload(
    value: &LogFieldValue,
) -> AppGameAdapterExecutionReadinessReadModel {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
