use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::test_invariants::{require_json_decode, require_log_string_field, serialize_test_json};
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

const APP_GAME_TEST_TIMESTAMP: &str = "2026-06-03T22:15:00Z";

#[tokio::test]
async fn app_game_adapter_dispatch_preflight_command_reports_service_backed_read_model() {
    let body = serialize_test_json(&command_envelope());
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let read_model = adapter_dispatch_preflight_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported
    );
    assert_eq!(read_model.returned, 8);
    assert_eq!(read_model.dispatch_eligible_count, 1);
    assert_eq!(read_model.blocked_before_dispatch_count, 7);
    assert_eq!(read_model.adapter_dispatch_eligible_count, 1);
    assert_eq!(read_model.adapter_dispatch_executed_claimed_count, 0);
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
            constants::event_id::ACTIVITY_APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_REPORTED
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
        command: AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet,
        payload: LogFields::new(),
    }
}

fn adapter_dispatch_preflight_payload(
    value: &LogFieldValue,
) -> AppGameAdapterDispatchPreflightReadModel {
    let text = require_log_string_field(Some(value), constants::error::AGENT_EVENT_SERIALIZES);
    require_json_decode(text, constants::error::AGENT_EVENT_SERIALIZES)
}
