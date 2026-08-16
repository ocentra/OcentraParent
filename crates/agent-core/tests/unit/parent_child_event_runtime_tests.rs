use ocentra_parent_agent_protocol::child_agent::child_agent_events::{
    ChildCommandDecision, ChildCommandKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentChildCommandDeliveryState, ParentChildCommandTransportBoundary,
    ParentCommandValidationState,
};
use ocentra_parent_agent_protocol::transport::parent_child_runtime_input::ParentChildRuntimeInput;
use ocentra_parent_agent_protocol::transport::ParentChildRuntimePhase;

use crate::test_text::TestText;
use ocentra_parent_agent_core::parent_child_event_runtime::publish_parent_child_runtime_for_validated_intent;
mod parent_child_event_runtime_child_command;
mod parent_child_event_runtime_child_state;
mod parent_child_event_runtime_decode;
mod parent_child_event_runtime_parent;
mod parent_child_event_runtime_parent_read_model;
use parent_child_event_runtime_child_command::{child_accepted, child_received};
use parent_child_event_runtime_child_state::{child_capability, child_health};
use parent_child_event_runtime_decode::{decode_payloads, ok};
use parent_child_event_runtime_parent::{
    parent_action, parent_forward_requested, parent_forwarded, parent_validated,
};
use parent_child_event_runtime_parent_read_model::parent_read_model;

type TestResult = Result<(), TestText>;

#[tokio::test]
async fn parent_child_runtime_publishes_validated_intent_before_child_handoff() -> TestResult {
    let report = ok(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::validated_review_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;

    assert_eq!(
        report.publish_reports.len(),
        ParentChildRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        report.stored_events.len(),
        ParentChildRuntimePhase::ordered_chain().len()
    );
    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(
        report.stored_events[0].contract.event_type.as_str(),
        constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED
    );
    assert_eq!(
        report.stored_events[1].contract.event_type.as_str(),
        constants::parent_controller::EVENT_COMMAND_VALIDATED
    );
    assert_eq!(
        report.stored_events[0].source.role,
        ok(
            ocentra_eventing::ids::RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER),
            constants::eventing_source::ERROR_RUNTIME_ROLE_PARSES,
        )?
    );

    let validated = parent_validated(&payloads)?;
    let received = child_received(&payloads)?;
    assert_eq!(
        validated.validation_state,
        ParentCommandValidationState::Validated
    );
    assert_eq!(
        validated.child_command_ref.as_deref(),
        Some(received.child_command_ref.as_str())
    );

    Ok(())
}

#[tokio::test]
async fn parent_child_transport_handoff_preserves_forwarded_refs() -> TestResult {
    let report = ok(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::validated_review_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;
    let requested = parent_forward_requested(&payloads)?;
    let forwarded = parent_forwarded(&payloads)?;
    let received = child_received(&payloads)?;

    assert_eq!(
        requested.transport_boundary,
        ParentChildCommandTransportBoundary::TypedLocalServiceTransport
    );
    assert_eq!(forwarded.child_command_ref, requested.child_command_ref);
    assert_eq!(received.child_command_ref, forwarded.child_command_ref);
    assert_eq!(
        received.parent_controller_event_ref,
        forwarded.forwarded_event_ref
    );
    assert_eq!(
        received.transport_message_ref,
        forwarded.transport_message_ref
    );
    assert_eq!(
        forwarded.delivery_state,
        ParentChildCommandDeliveryState::Forwarded
    );

    Ok(())
}

#[tokio::test]
async fn child_agent_receive_publishes_local_events_and_parent_read_model() -> TestResult {
    let report = ok(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::validated_review_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;
    let received = child_received(&payloads)?;
    let accepted = child_accepted(&payloads)?;
    let capability = child_capability(&payloads)?;
    let health = child_health(&payloads)?;
    let read_model = parent_read_model(&payloads)?;

    assert_eq!(received.command_kind, ChildCommandKind::ObserveNetwork);
    assert_eq!(accepted.decision, ChildCommandDecision::Accepted);
    assert_eq!(accepted.child_command_ref, received.child_command_ref);
    assert_eq!(
        capability.previous_event_ref,
        accepted.command_accepted_event_ref
    );
    assert_eq!(
        health.previous_event_ref,
        capability.capability_state_event_ref
    );
    assert_eq!(
        read_model.previous_event_ref,
        health.runtime_health_event_ref
    );
    assert_eq!(
        report.stored_events[4].source.custody,
        ok(
            ocentra_eventing::ids::EventCustody::parse(
                constants::eventing_source::CUSTODY_LOCAL_JOURNAL
            ),
            constants::eventing_source::ERROR_EVENT_CUSTODY_PARSES,
        )?
    );
    assert_eq!(
        report.stored_events[8].contract.event_type.as_str(),
        constants::parent_controller::EVENT_READ_MODEL_PROJECTED
    );

    Ok(())
}

#[tokio::test]
async fn browser_action_intent_handoff_uses_parent_child_event_sequence_without_execution(
) -> TestResult {
    let report = ok(
        publish_parent_child_runtime_for_validated_intent(
            ParentChildRuntimeInput::browser_action_intent_handoff_fixture(),
        )
        .await,
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;
    let parent_action = parent_action(&payloads)?;
    let validated = parent_validated(&payloads)?;
    let forwarded = parent_forwarded(&payloads)?;
    let received = child_received(&payloads)?;
    let accepted = child_accepted(&payloads)?;
    let read_model = parent_read_model(&payloads)?;

    assert_eq!(
        parent_action.parent_intent_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
    );
    assert_eq!(
        received.command_kind,
        ChildCommandKind::BrowserActionIntentHandoff
    );
    assert_eq!(
        validated.child_command_ref.as_deref(),
        Some(received.child_command_ref.as_str())
    );
    assert_eq!(
        received.parent_controller_event_ref,
        forwarded.forwarded_event_ref
    );
    assert_eq!(accepted.decision, ChildCommandDecision::Accepted);
    assert!(read_model.visible_to_portal);
    assert_eq!(
        report.stored_events.len(),
        ParentChildRuntimePhase::ordered_chain().len()
    );
    assert_eq!(report.dead_letters.len(), 0);

    Ok(())
}
