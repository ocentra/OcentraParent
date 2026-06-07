use ocentra_eventing::{
    decide_event_delivery_route, EventDeliveryBackpressurePolicy, EventDeliveryDecisionError,
    EventDeliveryDecisionInput, EventDeliveryDecisionProof, EventDeliveryDecisionState,
    EventDeliveryRouteKind, EventDeliverySubscriberFilter, EventNamespace, EventType,
    EventingError, SourceComponent, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::constants;

use crate::BrowserRuntimePhase;

#[derive(Clone, Debug)]
pub struct BrowserRuntimeDeliveryDecisionReport {
    pub chain_delivery: EventDeliveryDecisionProof,
    pub action_intent_status_delivery: EventDeliveryDecisionProof,
    pub external_transport_delivery: EventDeliveryDecisionProof,
    pub local_ready_route_count: usize,
    pub external_transport_manual_required: bool,
    pub external_transport_delivery_implemented: bool,
    pub external_relay_delivery_implemented: bool,
    pub adapter_dispatch_claimed: bool,
    pub browser_mutation_claimed: bool,
    pub child_intervention_execution_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserRuntimeDeliveryDecisionError {
    Eventing(EventingError),
    DeliveryDecision(EventDeliveryDecisionError),
    LocalRouteNotReady,
    ExternalTransportNotManualRequired,
}

pub fn prove_browser_runtime_delivery_decision(
) -> Result<BrowserRuntimeDeliveryDecisionReport, BrowserRuntimeDeliveryDecisionError> {
    let chain_delivery = decide_event_delivery_route(chain_delivery_input()?)
        .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let action_intent_status_delivery =
        decide_event_delivery_route(action_intent_delivery_input()?)
            .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let external_transport_delivery = decide_event_delivery_route(external_transport_input()?)
        .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;

    let local_ready_route_count =
        count_local_ready_routes(&chain_delivery, &action_intent_status_delivery);
    if local_ready_route_count != 2 {
        return Err(BrowserRuntimeDeliveryDecisionError::LocalRouteNotReady);
    }
    if external_transport_delivery.decision_state
        != EventDeliveryDecisionState::ExternalTransportRouteManualRequired
    {
        return Err(BrowserRuntimeDeliveryDecisionError::ExternalTransportNotManualRequired);
    }

    Ok(BrowserRuntimeDeliveryDecisionReport {
        external_transport_manual_required: true,
        external_transport_delivery_implemented: external_transport_delivery
            .external_transport_delivery_implemented,
        external_relay_delivery_implemented: external_transport_delivery
            .external_relay_delivery_implemented,
        adapter_dispatch_claimed: false,
        browser_mutation_claimed: false,
        child_intervention_execution_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
        local_ready_route_count,
        chain_delivery,
        action_intent_status_delivery,
        external_transport_delivery,
    })
}

fn chain_delivery_input() -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::LocalService,
        constants::browser::EVENT_BROWSER_EVIDENCE_OBSERVED,
        constants::browser::SUBSCRIBER_BROWSER_READ_MODEL,
        constants::browser::TARGET_BROWSER_READ_MODEL,
        BrowserRuntimePhase::ordered_chain()
            .iter()
            .map(|phase| phase.event_type())
            .collect(),
    )
}

fn action_intent_delivery_input() -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::LocalInProcess,
        constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED,
        constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_STATUS,
        constants::browser::TARGET_BROWSER_ACTION_INTENT_STATUS,
        vec![constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED],
    )
}

fn external_transport_input() -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::ExternalTransport,
        constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED,
        constants::browser::SUBSCRIBER_BROWSER_INTERVENTION_COMMAND,
        constants::browser::TARGET_BROWSER_INTERVENTION_ADAPTER,
        vec![
            constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED,
            constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED,
        ],
    )
}

fn delivery_input(
    route_kind: EventDeliveryRouteKind,
    namespace_event_type: &str,
    subscriber_id: &str,
    target_handler: &str,
    accepted_event_types: Vec<&str>,
) -> Result<EventDeliveryDecisionInput, EventingError> {
    let event_type = EventType::parse(namespace_event_type)?;
    let event_namespace = EventNamespace::from_event_type(&event_type)?;
    Ok(EventDeliveryDecisionInput {
        route_kind,
        event_namespace: event_namespace.clone(),
        publisher_component: SourceComponent::parse(
            constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
        )?,
        subscriber_filter: EventDeliverySubscriberFilter {
            subscriber_id: SubscriberId::parse(subscriber_id)?,
            target_handler: TargetHandler::parse(target_handler)?,
            event_namespace,
            accepted_event_types: parse_event_types(accepted_event_types)?,
        },
        backpressure_policy: EventDeliveryBackpressurePolicy {
            bounded_queue_capacity: 32,
            ttl_millis: 30_000,
            overflow_dead_letters: true,
            idempotency_required: true,
        },
        custody_proof_ref: None,
        publisher_auth_ref: None,
        subscriber_auth_ref: None,
        encryption_ref: None,
        retention_policy_ref: None,
        replay_plan_ref: None,
        deletion_plan_ref: None,
        offset_policy_ref: None,
        dedupe_policy_ref: None,
        transport_config_ref: None,
        relay_identity_ref: None,
        relay_policy_ref: None,
        external_transport_delivery_claimed: false,
        external_relay_delivery_claimed: false,
        decision_authority_claimed: false,
        side_effect_authority_claimed: false,
    })
}

fn parse_event_types(values: Vec<&str>) -> Result<Vec<EventType>, EventingError> {
    values.into_iter().map(EventType::parse).collect()
}

fn count_local_ready_routes(
    first: &EventDeliveryDecisionProof,
    second: &EventDeliveryDecisionProof,
) -> usize {
    [first, second]
        .iter()
        .filter(|proof| proof.decision_state == EventDeliveryDecisionState::LocalRouteReady)
        .count()
}

impl From<EventingError> for BrowserRuntimeDeliveryDecisionError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
