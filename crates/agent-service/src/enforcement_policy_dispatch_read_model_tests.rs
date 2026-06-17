use ocentra_parent_agent_core::validate_enforcement_policy_dispatch_read_model;
use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
    AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    EnforcementPolicyDispatchOutcomeState, EnforcementPolicyDispatchReadModel,
    EnforcementPolicyDispatchRejectionReason, EnforcementPolicyDispatchSourceState,
    EnforcementPolicyDispatchTimerState, LogFieldValue, LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    enforcement_policy_dispatch_read_model::v08_enforcement_policy_dispatch_read_model,
    lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test,
};

#[test]
fn policy_dispatch_read_model_exposes_validation_and_non_claim_states() {
    let read_model =
        v08_enforcement_policy_dispatch_read_model(policy_constants::TEST_EVALUATED_AT);
    let validation = validate_enforcement_policy_dispatch_read_model(&read_model).unwrap();

    assert_eq!(
        read_model.read_model_id,
        constants::v08_enforcement_policy_dispatch::READ_MODEL_ID
    );
    assert_eq!(read_model.entries.len(), 8);
    assert_eq!(validation.dispatch_ready_count, 2);
    assert_eq!(validation.dry_run_only_count, 1);
    assert_eq!(validation.manual_required_count, 1);
    assert_eq!(validation.report_only_count, 1);
    assert_eq!(validation.rejected_count, 3);
    assert_eq!(validation.recovery_needed_count, 1);
    assert!(read_model.entries.iter().any(|entry| {
        entry.matrix_row.outcome_state == EnforcementPolicyDispatchOutcomeState::ManualRequired
    }));
    assert!(read_model.entries.iter().any(|entry| {
        entry.intent.requested_parent_action
            == ocentra_parent_agent_protocol::V08EnforcementProductControlParentAction::AskParent
            && entry.matrix_row.outcome_state == EnforcementPolicyDispatchOutcomeState::DryRunOnly
            && entry.intent.dry_run
    }));
    assert!(read_model.entries.iter().any(|entry| {
        entry.matrix_row.rejection_reason
            == EnforcementPolicyDispatchRejectionReason::StalePolicyVersion
            && entry.intent.source_state == EnforcementPolicyDispatchSourceState::Stale
    }));
    assert!(read_model.entries.iter().any(|entry| {
        entry.matrix_row.rejection_reason
            == EnforcementPolicyDispatchRejectionReason::SourceNotReady
            && entry.intent.source_state == EnforcementPolicyDispatchSourceState::Missing
    }));
    assert!(read_model
        .entries
        .iter()
        .any(|entry| entry.timer_state == EnforcementPolicyDispatchTimerState::RestartRecovered));
}

#[tokio::test]
async fn policy_dispatch_websocket_command_returns_service_read_model() {
    let event = send_policy_dispatch_command().await;

    assert_eq!(
        event.event,
        AgentEventName::AgentEnforcementPolicyDispatchReported
    );
    assert_eq!(
        string_payload_field(&event, constants::field::READ_MODEL_ID),
        constants::v08_enforcement_policy_dispatch::READ_MODEL_ID
    );
    assert_eq!(
        number_payload_field(&event, constants::field::RETURNED),
        8.0
    );

    let read_model: EnforcementPolicyDispatchReadModel =
        serde_json::from_str(string_payload_field(
            &event,
            constants::field::ENFORCEMENT_POLICY_DISPATCH_READ_MODEL,
        ))
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        read_model.entries[0].intent.actor.actor_id,
        constants::v08_enforcement_policy_dispatch::PARENT_ACTOR_PRIMARY_ID
    );
    assert_eq!(
        read_model.entries[0].intent.device.device_id,
        constants::peer::LOCAL_DEV_AGENT
    );
    let manual_required_entry = read_model
        .entries
        .iter()
        .find(|entry| {
            entry.matrix_row.rejection_reason
                == EnforcementPolicyDispatchRejectionReason::AdapterManualRequired
        })
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let stale_entry = read_model
        .entries
        .iter()
        .find(|entry| {
            entry.matrix_row.rejection_reason
                == EnforcementPolicyDispatchRejectionReason::StalePolicyVersion
        })
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        manual_required_entry
            .matrix_row
            .rejection_reason
            .as_protocol_str(),
        constants::v08_enforcement_policy_dispatch::REJECTION_ADAPTER_MANUAL_REQUIRED
    );
    assert_eq!(
        stale_entry.matrix_row.rejection_reason.as_protocol_str(),
        constants::v08_enforcement_policy_dispatch::REJECTION_STALE_POLICY_VERSION
    );
}

async fn send_policy_dispatch_command() -> AgentEventEnvelope {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::v08_enforcement_policy_dispatch::READ_MODEL_ID.to_string(),
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
        command: AgentCommandName::AgentEnforcementPolicyDispatchGet,
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
