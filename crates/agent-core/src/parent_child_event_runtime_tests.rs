use ocentra_parent_agent_protocol::{
    constants, ChildCommandDecision, ChildCommandKind, ParentChildCommandDeliveryState,
    ParentChildCommandTransportBoundary, ParentCommandValidationState,
};
use ocentra_eventing::envelope::EventEnvelope;
use ocentra_eventing::ids::{EventCustody, RuntimeRole};

use super::{
    publish_parent_child_runtime_for_validated_intent, ParentChildRuntimeEventPayload,
    ParentChildRuntimeInput, ParentChildRuntimePhase, ParentChildRuntimeReport,
};

#[tokio::test]
async fn parent_child_runtime_publishes_validated_intent_before_child_handoff() {
    let report = publish_parent_child_runtime_for_validated_intent(
        ParentChildRuntimeInput::validated_review_fixture(),
    )
    .await
    .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_eq!(
        report.publish_reports.len(),
        ParentChildRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        report.stored_events.len(),
        ParentChildRuntimePhase::ordered_chain().len()
    );
    assert!(report.dead_letters.is_empty());
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
        RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)
            .expect(constants::eventing_source::ERROR_RUNTIME_ROLE_PARSES)
    );

    let validated = parent_validated(&payloads);
    assert_eq!(
        validated.validation_state,
        ParentCommandValidationState::Validated
    );
    assert!(validated.child_command_ref.is_some());
}

#[tokio::test]
async fn parent_child_transport_handoff_preserves_forwarded_refs() {
    let report = publish_parent_child_runtime_for_validated_intent(
        ParentChildRuntimeInput::validated_review_fixture(),
    )
    .await
    .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES);
    let payloads = decode_payloads(&report);
    let requested = parent_forward_requested(&payloads);
    let forwarded = parent_forwarded(&payloads);
    let received = child_received(&payloads);

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
}

#[tokio::test]
async fn child_agent_receive_publishes_local_events_and_parent_read_model() {
    let report = publish_parent_child_runtime_for_validated_intent(
        ParentChildRuntimeInput::validated_review_fixture(),
    )
    .await
    .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES);
    let payloads = decode_payloads(&report);
    let received = child_received(&payloads);
    let accepted = child_accepted(&payloads);
    let capability = child_capability(&payloads);
    let health = child_health(&payloads);
    let read_model = parent_read_model(&payloads);

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
        EventCustody::parse(constants::eventing_source::CUSTODY_LOCAL_JOURNAL)
            .expect(constants::eventing_source::ERROR_EVENT_CUSTODY_PARSES)
    );
    assert_eq!(
        report.stored_events[8].contract.event_type.as_str(),
        constants::parent_controller::EVENT_READ_MODEL_PROJECTED
    );
}

#[tokio::test]
async fn browser_action_intent_handoff_uses_parent_child_event_sequence_without_execution() {
    let report = publish_parent_child_runtime_for_validated_intent(
        ParentChildRuntimeInput::browser_action_intent_handoff_fixture(),
    )
    .await
    .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PUBLISHES);
    let payloads = decode_payloads(&report);
    let parent_action = parent_action(&payloads);
    let validated = parent_validated(&payloads);
    let forwarded = parent_forwarded(&payloads);
    let received = child_received(&payloads);
    let accepted = child_accepted(&payloads);
    let read_model = parent_read_model(&payloads);

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
    assert!(report.dead_letters.is_empty());
}

fn decode_payloads(report: &ParentChildRuntimeReport) -> Vec<ParentChildRuntimeEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: EventEnvelope<ParentChildRuntimeEventPayload> =
                event.decode().expect(
                    constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
                );
            envelope.payload
        })
        .collect()
}

fn parent_action(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ParentActionReceivedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentActionReceived(event) => Some(event.clone()),
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn parent_validated(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ParentCommandValidatedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentCommandValidated(event) => Some(event.clone()),
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn parent_forward_requested(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ParentChildCommandForwardRequestedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentChildCommandForwardRequested(event) => {
                Some(event.clone())
            }
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn parent_forwarded(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ParentChildCommandForwardedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentChildCommandForwarded(event) => {
                Some(event.clone())
            }
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn child_received(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ChildCommandReceivedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCommandReceived(event) => Some(event.clone()),
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn child_accepted(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ChildCommandAcceptedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCommandAccepted(event) => Some(event.clone()),
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn child_capability(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ChildCapabilityStateUpdatedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCapabilityStateUpdated(event) => {
                Some(event.clone())
            }
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn child_health(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ChildRuntimeHealthUpdatedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildRuntimeHealthUpdated(event) => Some(event.clone()),
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}

fn parent_read_model(
    payloads: &[ParentChildRuntimeEventPayload],
) -> ocentra_parent_agent_protocol::ParentReadModelProjectedEvent {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentReadModelProjected(event) => Some(event.clone()),
            _ => None,
        })
        .expect(constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES)
}
