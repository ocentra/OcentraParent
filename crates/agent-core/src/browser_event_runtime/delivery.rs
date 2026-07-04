use ocentra_eventing::{
    delivery::decide_event_delivery_route, delivery::validation::EventDeliveryBackpressurePolicy,
    delivery::validation::EventDeliveryDecisionError,
    delivery::validation::EventDeliveryDecisionInput,
    delivery::validation::EventDeliveryDecisionProof,
    delivery::validation::EventDeliveryDecisionState, delivery::validation::EventDeliveryRouteKind,
    delivery::validation::EventDeliverySubscriberFilter, error::EventingError, ids::EventNamespace,
    ids::EventType, ids::SourceComponent, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;
use ocentra_parent_agent_protocol::constants;

pub type BrowserRuntimeDeliveryDecisionReport =
    ocentra_parent_agent_protocol::browser::delivery::BrowserRuntimeDeliveryDecisionReport;

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
    let action_intent_handoff_delivery =
        decide_event_delivery_route(action_intent_handoff_delivery_input()?)
            .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let runtime_stream_report_delivery =
        decide_event_delivery_route(runtime_stream_report_delivery_input()?)
            .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let social_provider_receipt_status_delivery =
        decide_event_delivery_route(social_provider_receipt_status_delivery_input()?)
            .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let social_report_writer_delivery_status_delivery =
        decide_event_delivery_route(social_report_writer_delivery_status_delivery_input()?)
            .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let social_parent_notification_delivery_status_delivery =
        decide_event_delivery_route(social_parent_notification_delivery_status_delivery_input()?)
            .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let social_parent_surface_status_delivery =
        decide_event_delivery_route(social_parent_surface_status_delivery_input()?)
            .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;
    let external_transport_delivery = decide_event_delivery_route(external_transport_input()?)
        .map_err(BrowserRuntimeDeliveryDecisionError::DeliveryDecision)?;

    let local_routes = [
        &chain_delivery,
        &action_intent_status_delivery,
        &action_intent_handoff_delivery,
        &runtime_stream_report_delivery,
        &social_provider_receipt_status_delivery,
        &social_report_writer_delivery_status_delivery,
        &social_parent_notification_delivery_status_delivery,
        &social_parent_surface_status_delivery,
    ];
    let local_ready_route_count = count_local_ready_routes(&local_routes);
    if local_ready_route_count != local_routes.len() {
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
        action_intent_handoff_delivery,
        runtime_stream_report_delivery,
        social_provider_receipt_status_delivery,
        social_report_writer_delivery_status_delivery,
        social_parent_notification_delivery_status_delivery,
        social_parent_surface_status_delivery,
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

fn action_intent_handoff_delivery_input() -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::LocalInProcess,
        constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED,
        constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_HANDOFF,
        constants::browser::TARGET_BROWSER_ACTION_INTENT_HANDOFF,
        vec![constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED],
    )
}

fn runtime_stream_report_delivery_input() -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::LocalInProcess,
        constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED,
        constants::browser::SUBSCRIBER_BROWSER_RUNTIME_STREAM_REPORT,
        constants::browser::TARGET_BROWSER_RUNTIME_STREAM_REPORT,
        vec![constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED],
    )
}

fn social_provider_receipt_status_delivery_input(
) -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::LocalInProcess,
        constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED,
        constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
        constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
        vec![constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED],
    )
}

fn social_report_writer_delivery_status_delivery_input(
) -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::LocalInProcess,
        constants::browser::EVENT_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_REQUESTED,
        constants::browser::SUBSCRIBER_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS,
        constants::browser::TARGET_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS,
        vec![constants::browser::EVENT_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_REQUESTED],
    )
}

fn social_parent_notification_delivery_status_delivery_input(
) -> Result<EventDeliveryDecisionInput, EventingError> {
    delivery_input(
        EventDeliveryRouteKind::LocalInProcess,
        constants::browser::EVENT_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_REQUESTED,
        constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS,
        constants::browser::TARGET_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS,
        vec![
            constants::browser::EVENT_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_REQUESTED,
        ],
    )
}

fn social_parent_surface_status_delivery_input() -> Result<EventDeliveryDecisionInput, EventingError>
{
    delivery_input(
        EventDeliveryRouteKind::LocalInProcess,
        constants::browser::EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED,
        constants::browser::SUBSCRIBER_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS,
        constants::browser::TARGET_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS,
        vec![constants::browser::EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED],
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

fn count_local_ready_routes(routes: &[&EventDeliveryDecisionProof]) -> usize {
    routes
        .iter()
        .filter(|proof| proof.decision_state == EventDeliveryDecisionState::LocalRouteReady)
        .count()
}

impl From<EventingError> for BrowserRuntimeDeliveryDecisionError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
