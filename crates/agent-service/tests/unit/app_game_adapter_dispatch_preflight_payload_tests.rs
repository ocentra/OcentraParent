use crate::test_require_json_decode::require_json_decode;
use crate::test_require_log_string_field::require_log_string_field;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::{
    APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED, APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY, APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
};
use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use super::app_game_adapter_dispatch_preflight_payload::{
    app_game_adapter_dispatch_preflight_payload, app_game_adapter_dispatch_preflight_read_model,
    build_activity_app_game_adapter_dispatch_preflight_report,
};
use super::app_game_adapter_execution_readiness_payload::GeneratedAtText;

const APP_GAME_TEST_TIMESTAMP: &str = "2026-06-03T22:15:00Z";

#[tokio::test]
async fn app_game_adapter_dispatch_preflight_default_report_preserves_command_correlation() {
    let command = dispatch_preflight_command();
    let correlation_id = command.message_id.clone();
    let event = build_activity_app_game_adapter_dispatch_preflight_report(command).await;
    let decoded = require_json_decode::<AppGameAdapterDispatchPreflightReadModel>(
        &string_payload(
            &event.payload,
            constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported
    );
    assert_eq!(event.correlation_id, correlation_id);
    assert_eq!(event.target.peer_id, constants::peer::PORTAL_DEV);
    assert_eq!(decoded.returned, 8);
}

#[test]
fn app_game_adapter_dispatch_preflight_reports_one_scoped_dispatch_eligible_row() {
    let read_model = app_game_adapter_dispatch_preflight_read_model(GeneratedAtText(
        APP_GAME_TEST_TIMESTAMP.to_string(),
    ));
    let payload = app_game_adapter_dispatch_preflight_payload(&read_model);
    let decoded = require_json_decode::<AppGameAdapterDispatchPreflightReadModel>(
        &string_payload(
            &payload,
            constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(decoded.returned, 8);
    assert_eq!(decoded.dispatch_eligible_count, 1);
    assert_eq!(decoded.blocked_before_dispatch_count, 7);
    assert_eq!(decoded.adapter_dispatch_eligible_count, 1);
    assert_eq!(decoded.adapter_dispatch_executed_claimed_count, 0);
    assert_eq!(
        decoded.host_capability_available_count
            + decoded.host_capability_not_detected_count
            + decoded.host_capability_not_applicable_count,
        decoded.returned
    );
    assert!(decoded.host_capability_available_count >= 4);
    assert!(decoded.host_capability_probe_ref_count >= 4);
    assert!(!decoded.broad_installed_app_blocking_claimed);
    assert!(!decoded.child_device_delivery_claimed);
    assert!(!decoded.platform_enforcement_claimed);
    assert!(!decoded.provider_delivery_claimed);
    assert!(!decoded.private_diagnostics_claimed);
    assert_eq!(
        decoded.rows[0].dispatch_preflight_state,
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE
    );
    assert_eq!(
        decoded.rows[0].dispatch_decision,
        APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE
    );
    assert_eq!(
        decoded.rows[0].dispatch_outcome_state,
        APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY
    );
    assert!(decoded.rows[0].adapter_dispatch_eligible);
    assert_eq!(
        decoded.rows[0].host_capability_state,
        APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
    );
    assert_eq!(
        decoded.rows[0].host_capability_evidence_refs,
        vec![
            constants::v08_supported_adapter_runtime_proof::REF_ADAPTER_CAPABILITY_STATE
                .to_string()
        ]
    );
    assert_eq!(
        decoded.rows[0].host_capability_probe_refs,
        vec![
            constants::v08_supported_adapter_runtime_proof::REF_WINDOWS_HOST_LOCAL_PROBE
                .to_string()
        ]
    );
    assert!(!decoded.rows[0].adapter_dispatch_executed_claimed);
    assert!(decoded
        .rows
        .iter()
        .skip(1)
        .all(|row| row.dispatch_decision == APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED));
    assert!(decoded
        .rows
        .iter()
        .all(|row| !row.adapter_dispatch_executed_claimed));
}

fn string_payload(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    field_name: impl std::fmt::Display,
) -> String {
    let field_name = field_name.to_string();
    require_log_string_field(
        payload.get(field_name.as_str()),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
    .clone()
}

fn dispatch_preflight_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id:
            ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID
                .to_string(),
        sent_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet,
        payload: LogFields::new(),
    }
}
