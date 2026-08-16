use ocentra_eventing::{
    delivery::decide_event_delivery_route, delivery::validation::EventDeliveryBackpressurePolicy,
    delivery::validation::EventDeliveryDecisionError,
    delivery::validation::EventDeliveryDecisionInput,
    delivery::validation::EventDeliveryDecisionState, delivery::validation::EventDeliveryRouteKind,
    delivery::validation::EventDeliverySubscriberFilter, error::EventingError, ids::EventNamespace,
    ids::EventType, ids::SourceComponent, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;

use super::broker_delivery::{
    prove_network_runtime_broker_delivery_semantics, NetworkRuntimeBrokerDeliveryProofError,
    NetworkRuntimeBrokerDeliverySemantics,
};

pub type NetworkRuntimeRemoteDeliveryState =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryState;
pub type NetworkRuntimeRemoteDeliveryStatusReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteDeliveryStatusReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryStatusError {
    BrokerProof(NetworkRuntimeBrokerDeliveryProofError),
    DeliveryDecision(EventDeliveryDecisionError),
    Eventing(EventingError),
}

pub async fn prove_network_runtime_remote_delivery_status(
) -> Result<NetworkRuntimeRemoteDeliveryStatusReport, NetworkRuntimeRemoteDeliveryStatusError> {
    let broker_semantics = prove_network_runtime_broker_delivery_semantics()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryStatusError::BrokerProof)?;
    let family_hub_decision = decide_event_delivery_route(family_hub_delivery_input()?)
        .map_err(NetworkRuntimeRemoteDeliveryStatusError::DeliveryDecision)?;

    Ok(NetworkRuntimeRemoteDeliveryStatusReport {
        broker_status: delivery_state_from_decision(
            broker_semantics.delivery_decision.decision_state,
            broker_semantics.external_transport_delivery_implemented,
        ),
        family_hub_status: delivery_state_from_decision(
            family_hub_decision.decision_state,
            family_hub_decision.external_relay_delivery_implemented,
        ),
        custody_proof_ref: source_component(
            constants::network_flow::TEST_BROKER_CUSTODY_PROOF_REF,
        )?,
        publisher_auth_ref: source_component(
            constants::network_flow::TEST_BROKER_PUBLISHER_AUTH_REF,
        )?,
        subscriber_auth_ref: source_component(
            constants::network_flow::TEST_BROKER_SUBSCRIBER_AUTH_REF,
        )?,
        encryption_ref: source_component(constants::network_flow::TEST_BROKER_ENCRYPTION_REF)?,
        retention_policy_ref: source_component(
            constants::network_flow::TEST_BROKER_RETENTION_POLICY_REF,
        )?,
        replay_plan_ref: source_component(constants::network_flow::TEST_BROKER_REPLAY_PLAN_REF)?,
        deletion_plan_ref: source_component(
            constants::network_flow::TEST_BROKER_DELETION_PLAN_REF,
        )?,
        offset_policy_ref: source_component(
            constants::network_flow::TEST_BROKER_OFFSET_POLICY_REF,
        )?,
        dedupe_policy_ref: source_component(
            constants::network_flow::TEST_BROKER_DEDUPE_POLICY_REF,
        )?,
        transport_config_ref: source_component(constants::network_flow::TEST_BROKER_CONFIG_REF)?,
        relay_identity_ref: source_component(
            constants::network_flow::TEST_FAMILY_HUB_IDENTITY_REF,
        )?,
        relay_policy_ref: source_component(
            constants::network_flow::TEST_FAMILY_HUB_RELAY_POLICY_REF,
        )?,
        broker_missing_artifact_count: broker_semantics.delivery_decision.missing_artifacts.len(),
        family_hub_missing_artifact_count: family_hub_decision.missing_artifacts.len(),
        accepted_event_type_count: family_hub_decision
            .subscriber_filter
            .accepted_event_types
            .len(),
        local_idempotency_queue_proved: broker_semantics.delivery_semantics
            == NetworkRuntimeBrokerDeliverySemantics::LocalIdempotencyQueueProof,
        dropped_event_dead_letter_count: broker_semantics.dropped_event_dead_letter_count,
        queued_duplicate_rejected: broker_semantics.queued_duplicate_rejected,
        completed_duplicate_rejected: broker_semantics.completed_duplicate_rejected,
        external_transport_delivery_implemented: broker_semantics
            .external_transport_delivery_implemented,
        family_hub_delivery_implemented: family_hub_decision.external_relay_delivery_implemented,
        cross_process_replay_implemented: false,
        remote_retention_delete_export_propagation_implemented: false,
        policy_authority: broker_semantics.delivery_decision.decision_authority
            || family_hub_decision.decision_authority,
        side_effect_authority: broker_semantics.delivery_decision.side_effect_authority
            || family_hub_decision.side_effect_authority,
        enforcement_command_event_count: broker_semantics.enforcement_command_event_count,
        adapter_action_executed_count: broker_semantics.adapter_action_executed_count,
        family_hub_decision,
        broker_semantics,
    })
}

fn delivery_state_from_decision(
    decision_state: EventDeliveryDecisionState,
    implemented: bool,
) -> NetworkRuntimeRemoteDeliveryState {
    match decision_state {
        EventDeliveryDecisionState::ExternalTransportRouteRequirementsSatisfied
        | EventDeliveryDecisionState::ExternalRelayRouteRequirementsSatisfied
            if !implemented =>
        {
            NetworkRuntimeRemoteDeliveryState::FixtureRequirementsRecordedButNotImplemented
        }
        _ => NetworkRuntimeRemoteDeliveryState::ManualRequired,
    }
}

fn family_hub_delivery_input() -> Result<EventDeliveryDecisionInput, EventingError> {
    let flow_event_type = EventType::parse(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED)?;
    let event_namespace = EventNamespace::from_event_type(&flow_event_type)?;
    Ok(EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::ExternalRelay,
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
        relay_identity_ref: component_ref(constants::network_flow::TEST_FAMILY_HUB_IDENTITY_REF)?,
        relay_policy_ref: component_ref(constants::network_flow::TEST_FAMILY_HUB_RELAY_POLICY_REF)?,
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

impl From<EventingError> for NetworkRuntimeRemoteDeliveryStatusError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
