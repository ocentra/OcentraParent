use ocentra_eventing::{
    delivery::decide_event_delivery_route, delivery::validation::EventDeliveryBackpressurePolicy,
    delivery::validation::EventDeliveryDecisionError,
    delivery::validation::EventDeliveryDecisionInput,
    delivery::validation::EventDeliveryDecisionState, delivery::validation::EventDeliveryRouteKind,
    delivery::validation::EventDeliverySubscriberFilter, error::EventingError, ids::EventId,
    ids::EventNamespace, ids::EventType, ids::SourceComponent, ids::SubscriberId,
    ids::TargetHandler,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase;

use crate::network_capture::NetworkObservation;

use super::queue::{
    queue_network_runtime_flow_overflow_dead_letters,
    queue_network_runtime_flow_rejects_duplicate_idempotency,
};
use super::NetworkRuntimeEventPayload;

pub type NetworkRuntimeBrokerDeliverySemantics =
    ocentra_parent_agent_protocol::network_flow::broker_delivery::NetworkRuntimeBrokerDeliverySemantics;
pub type NetworkRuntimeBrokerDeliverySemanticsReport =
    ocentra_parent_agent_protocol::network_flow::broker_delivery::NetworkRuntimeBrokerDeliverySemanticsReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeBrokerDeliveryProofError {
    Eventing(EventingError),
    DeliveryDecision(EventDeliveryDecisionError),
    BrokerRequirementsNotSatisfied,
    DuplicateIdempotencyNotRejected,
}

pub async fn prove_network_runtime_broker_delivery_semantics(
) -> Result<NetworkRuntimeBrokerDeliverySemanticsReport, NetworkRuntimeBrokerDeliveryProofError> {
    let delivery_decision = decide_event_delivery_route(broker_delivery_input()?)
        .map_err(NetworkRuntimeBrokerDeliveryProofError::DeliveryDecision)?;
    if delivery_decision.decision_state
        != EventDeliveryDecisionState::ExternalTransportRouteRequirementsSatisfied
    {
        return Err(NetworkRuntimeBrokerDeliveryProofError::BrokerRequirementsNotSatisfied);
    }

    let duplicate_report = queue_network_runtime_flow_rejects_duplicate_idempotency(
        complete_domain_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await?;
    let overflow_report = queue_network_runtime_flow_overflow_dead_letters(
        complete_domain_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        complete_domain_observation(),
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .await?;

    let expected_flow_event_id = super::helpers::network_fallback_event_id(
        NetworkRuntimePhase::FlowObserved,
        &complete_domain_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )?;
    let queued_duplicate_rejected = deterministic_flow_duplicate_rejected(
        &duplicate_report.queued_duplicate_error,
        &expected_flow_event_id,
    );
    let registry_duplicate_rejected =
        duplicate_idempotency_key_rejected(&duplicate_report.registry_duplicate_error);
    let completed_duplicate_rejected =
        duplicate_idempotency_key_rejected(&duplicate_report.completed_duplicate_error);
    if !registry_duplicate_rejected || !queued_duplicate_rejected || !completed_duplicate_rejected {
        return Err(NetworkRuntimeBrokerDeliveryProofError::DuplicateIdempotencyNotRejected);
    }

    let duplicate_payloads = decode_payloads(&duplicate_report.stored_events)?;
    let overflow_payloads = decode_payloads(&overflow_report.stored_events)?;
    let enforcement_command_event_count = count_event_type(
        &duplicate_report.stored_events,
        constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED,
    ) + count_event_type(
        &overflow_report.stored_events,
        constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED,
    );
    let adapter_action_executed_count = duplicate_payloads
        .iter()
        .chain(overflow_payloads.iter())
        .filter(|payload| payload.claim_boundary.adapter_action_executed)
        .count();

    Ok(NetworkRuntimeBrokerDeliverySemanticsReport {
        replay_plan_ref: source_component(constants::network_flow::TEST_BROKER_REPLAY_PLAN_REF)?,
        dropped_event_audit_ref: source_component(
            constants::network_flow::TEST_BROKER_DROPPED_EVENT_AUDIT_REF,
        )?,
        adapter_action_ledger_ref: source_component(
            constants::network_flow::TEST_BROKER_ADAPTER_ACTION_LEDGER_REF,
        )?,
        queued_duplicate_rejected,
        completed_duplicate_rejected,
        dropped_event_dead_letter_count: overflow_report.dead_letters.len(),
        duplicate_stored_event_count: duplicate_report.stored_events.len(),
        enforcement_command_event_count,
        adapter_action_executed_count,
        external_transport_delivery_implemented: delivery_decision
            .external_transport_delivery_implemented,
        external_relay_delivery_implemented: delivery_decision.external_relay_delivery_implemented,
        delivery_decision,
        delivery_semantics: NetworkRuntimeBrokerDeliverySemantics::LocalIdempotencyQueueProof,
    })
}

fn broker_delivery_input() -> Result<EventDeliveryDecisionInput, EventingError> {
    let flow_event_type = EventType::parse(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED)?;
    let event_namespace = EventNamespace::from_event_type(&flow_event_type)?;
    Ok(EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::ExternalTransport,
        event_namespace: event_namespace.clone(),
        publisher_component: source_component(
            constants::network_flow::RUNTIME_COMPONENT_NETWORK_SPINE,
        )?,
        subscriber_filter: EventDeliverySubscriberFilter {
            subscriber_id: SubscriberId::parse(
                constants::network_flow::SUBSCRIBER_PORTAL_READ_MODEL,
            )?,
            target_handler: TargetHandler::parse(
                constants::network_flow::TARGET_PORTAL_READ_MODEL,
            )?,
            event_namespace,
            accepted_event_types: vec![
                flow_event_type,
                EventType::parse(constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED)?,
                EventType::parse(constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED)?,
            ],
        },
        backpressure_policy: EventDeliveryBackpressurePolicy {
            bounded_queue_capacity: 32,
            ttl_millis: 30_000,
            overflow_dead_letters: true,
            idempotency_required: true,
        },
        custody_proof_ref: component_ref(constants::network_flow::TEST_BROKER_CUSTODY_PROOF_REF)?,
        publisher_auth_ref: component_ref(constants::network_flow::TEST_BROKER_PUBLISHER_AUTH_REF)?,
        subscriber_auth_ref: component_ref(
            constants::network_flow::TEST_BROKER_SUBSCRIBER_AUTH_REF,
        )?,
        encryption_ref: component_ref(constants::network_flow::TEST_BROKER_ENCRYPTION_REF)?,
        retention_policy_ref: component_ref(
            constants::network_flow::TEST_BROKER_RETENTION_POLICY_REF,
        )?,
        replay_plan_ref: component_ref(constants::network_flow::TEST_BROKER_REPLAY_PLAN_REF)?,
        deletion_plan_ref: component_ref(constants::network_flow::TEST_BROKER_DELETION_PLAN_REF)?,
        offset_policy_ref: component_ref(constants::network_flow::TEST_BROKER_OFFSET_POLICY_REF)?,
        dedupe_policy_ref: component_ref(constants::network_flow::TEST_BROKER_DEDUPE_POLICY_REF)?,
        transport_config_ref: component_ref(constants::network_flow::TEST_BROKER_CONFIG_REF)?,
        relay_identity_ref: None,
        relay_policy_ref: None,
        external_transport_delivery_claimed: false,
        external_relay_delivery_claimed: false,
        decision_authority_claimed: false,
        side_effect_authority_claimed: false,
    })
}

fn source_component(value: &str) -> Result<SourceComponent, EventingError> {
    SourceComponent::parse(value)
}

fn component_ref(value: &str) -> Result<Option<SourceComponent>, EventingError> {
    source_component(value).map(Some)
}

fn decode_payloads(
    stored_events: &[ocentra_eventing::envelope::StoredEventEnvelope],
) -> Result<Vec<NetworkRuntimeEventPayload>, EventingError> {
    stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::envelope::EventEnvelope<NetworkRuntimeEventPayload> =
                event.decode()?;
            Ok(envelope.payload)
        })
        .collect()
}

fn count_event_type(
    stored_events: &[ocentra_eventing::envelope::StoredEventEnvelope],
    event_type: &str,
) -> usize {
    stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_type)
        .count()
}

fn deterministic_flow_duplicate_rejected(
    error: &EventingError,
    expected_event_id: &EventId,
) -> bool {
    matches!(
        error,
        EventingError::DuplicateEventId { event_id } if event_id == expected_event_id
    )
}

fn duplicate_idempotency_key_rejected(error: &EventingError) -> bool {
    matches!(
        error,
        EventingError::DuplicateIdempotencyKey { idempotency_key }
            if idempotency_key
                .as_str()
                .contains(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED)
    )
}

fn complete_domain_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 1,
    }
}

impl From<EventingError> for NetworkRuntimeBrokerDeliveryProofError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
