use ocentra_parent_agent_core::enforcement_policy_dispatch::validate_enforcement_policy_dispatch_read_model;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchOutcomeState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchReadModel;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchRejectionReason;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchSourceState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchTimerState;
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

use super::test_text::{optional_log_string, test_ok, test_some, TestResult, TestText};
use crate::enforcement_policy_dispatch_read_model::v08_enforcement_policy_dispatch_read_model;

#[test]
fn policy_dispatch_read_model_exposes_validation_and_non_claim_states() -> TestResult {
    let read_model =
        v08_enforcement_policy_dispatch_read_model(policy_constants::TEST_EVALUATED_AT);
    let validation = test_ok(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        constants::v08_enforcement_policy_dispatch::READ_MODEL_ID,
    )?;

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
            == ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction::AskParent
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

    Ok(())
}

#[tokio::test]
async fn policy_dispatch_websocket_command_returns_service_read_model() -> TestResult {
    let event = send_policy_dispatch_command().await?;

    assert_eq!(
        event.event,
        AgentEventName::AgentEnforcementPolicyDispatchReported
    );
    assert_eq!(
        test_some(
            optional_log_string(&event.payload, constants::field::READ_MODEL_ID),
            constants::error::AGENT_EVENT_SERIALIZES,
        )?
        .to_string(),
        constants::v08_enforcement_policy_dispatch::READ_MODEL_ID
    );
    assert_eq!(
        number_payload_field(&event, constants::field::RETURNED)?,
        8.0
    );

    let read_model: EnforcementPolicyDispatchReadModel = test_ok(
        serde_json::from_str(
            test_some(
                optional_log_string(
                    &event.payload,
                    constants::field::ENFORCEMENT_POLICY_DISPATCH_READ_MODEL,
                ),
                constants::error::AGENT_EVENT_SERIALIZES,
            )?
            .as_ref(),
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;

    assert_eq!(
        read_model.entries[0].intent.actor.actor_id,
        constants::v08_enforcement_policy_dispatch::PARENT_ACTOR_PRIMARY_ID
    );
    assert_eq!(
        read_model.entries[0].intent.device.device_id,
        constants::peer::LOCAL_DEV_AGENT
    );
    let manual_required_entry = test_some(
        read_model.entries.iter().find(|entry| {
            entry.matrix_row.rejection_reason
                == EnforcementPolicyDispatchRejectionReason::AdapterManualRequired
        }),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let stale_entry = test_some(
        read_model.entries.iter().find(|entry| {
            entry.matrix_row.rejection_reason
                == EnforcementPolicyDispatchRejectionReason::StalePolicyVersion
        }),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;

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

    Ok(())
}

async fn send_policy_dispatch_command() -> Result<AgentEventEnvelope, TestText> {
    let body = test_ok(
        serde_json::to_string(&command_envelope()),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    Ok(handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await)
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

fn number_payload_field(
    event: &AgentEventEnvelope,
    field: impl std::fmt::Display,
) -> Result<f64, TestText> {
    let field_name = field.to_string();
    match event.payload.get(field_name.as_str()) {
        Some(LogFieldValue::Number(value)) => Ok(*value),
        _ => Err(TestText::from_display(
            constants::error::AGENT_EVENT_SERIALIZES,
        )),
    }
}
