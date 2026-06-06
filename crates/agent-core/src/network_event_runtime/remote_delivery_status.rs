use ocentra_eventing::{
    decide_event_delivery_route, EventDeliveryBackpressurePolicy, EventDeliveryDecisionError,
    EventDeliveryDecisionInput, EventDeliveryDecisionProof, EventDeliveryDecisionState,
    EventDeliveryRouteKind, EventDeliverySubscriberFilter, EventNamespace, EventType,
    EventingError, SourceComponent, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::constants;

use super::{
    prove_network_runtime_broker_delivery_semantics, NetworkRuntimeBrokerDeliveryProofError,
    NetworkRuntimeBrokerDeliverySemantics, NetworkRuntimeBrokerDeliverySemanticsReport,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryState {
    RequirementsSatisfiedButNotImplemented,
    ManualRequired,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryStatusReport {
    pub broker_semantics: NetworkRuntimeBrokerDeliverySemanticsReport,
    pub broker_status: NetworkRuntimeRemoteDeliveryState,
    pub family_hub_status: NetworkRuntimeRemoteDeliveryState,
    pub family_hub_decision: EventDeliveryDecisionProof,
    pub custody_proof_ref: SourceComponent,
    pub publisher_auth_ref: SourceComponent,
    pub subscriber_auth_ref: SourceComponent,
    pub encryption_ref: SourceComponent,
    pub retention_policy_ref: SourceComponent,
    pub replay_plan_ref: SourceComponent,
    pub deletion_plan_ref: SourceComponent,
    pub offset_policy_ref: SourceComponent,
    pub dedupe_policy_ref: SourceComponent,
    pub transport_config_ref: SourceComponent,
    pub relay_identity_ref: SourceComponent,
    pub relay_policy_ref: SourceComponent,
    pub broker_missing_artifact_count: usize,
    pub family_hub_missing_artifact_count: usize,
    pub accepted_event_type_count: usize,
    pub local_idempotency_queue_proved: bool,
    pub dropped_event_dead_letter_count: usize,
    pub queued_duplicate_rejected: bool,
    pub completed_duplicate_rejected: bool,
    pub cross_process_replay_ref: SourceComponent,
    pub remote_retention_delete_export_ref: SourceComponent,
    pub remote_delivery_ack_ref: SourceComponent,
    pub remote_lifecycle_followup_ref: SourceComponent,
    pub remote_lifecycle_missing_artifact_count: usize,
    pub remote_lifecycle_manual_required: bool,
    pub durable_envelope_schema_ref: SourceComponent,
    pub durable_envelope_journal_ref: SourceComponent,
    pub durable_envelope_replay_readiness_ref: SourceComponent,
    pub durable_envelope_delete_export_readiness_ref: SourceComponent,
    pub durable_envelope_support_status_ref: SourceComponent,
    pub durable_envelope_ready: bool,
    pub durable_envelope_missing_artifact_count: usize,
    pub external_transport_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub cross_process_replay_implemented: bool,
    pub remote_retention_delete_export_propagation_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub product_ready_claimed: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
}

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
    let delivery_refs = remote_delivery_refs()?;
    let remote_lifecycle_refs = remote_lifecycle_refs()?;
    let durable_envelope_refs = remote_durable_envelope_refs()?;

    Ok(NetworkRuntimeRemoteDeliveryStatusReport {
        broker_status: delivery_state_from_decision(
            broker_semantics.delivery_decision.decision_state,
            broker_semantics.external_transport_delivery_implemented,
        ),
        family_hub_status: delivery_state_from_decision(
            family_hub_decision.decision_state,
            family_hub_decision.external_relay_delivery_implemented,
        ),
        custody_proof_ref: delivery_refs.custody_proof_ref,
        publisher_auth_ref: delivery_refs.publisher_auth_ref,
        subscriber_auth_ref: delivery_refs.subscriber_auth_ref,
        encryption_ref: delivery_refs.encryption_ref,
        retention_policy_ref: delivery_refs.retention_policy_ref,
        replay_plan_ref: delivery_refs.replay_plan_ref,
        deletion_plan_ref: delivery_refs.deletion_plan_ref,
        offset_policy_ref: delivery_refs.offset_policy_ref,
        dedupe_policy_ref: delivery_refs.dedupe_policy_ref,
        transport_config_ref: delivery_refs.transport_config_ref,
        relay_identity_ref: delivery_refs.relay_identity_ref,
        relay_policy_ref: delivery_refs.relay_policy_ref,
        broker_missing_artifact_count: broker_semantics.delivery_decision.missing_artifacts.len(),
        family_hub_missing_artifact_count: family_hub_decision.missing_artifacts.len(),
        accepted_event_type_count: accepted_event_type_count(&family_hub_decision),
        local_idempotency_queue_proved: broker_semantics.delivery_semantics
            == NetworkRuntimeBrokerDeliverySemantics::LocalIdempotencyQueueProof,
        dropped_event_dead_letter_count: broker_semantics.dropped_event_dead_letter_count,
        queued_duplicate_rejected: broker_semantics.queued_duplicate_rejected,
        completed_duplicate_rejected: broker_semantics.completed_duplicate_rejected,
        cross_process_replay_ref: remote_lifecycle_refs.cross_process_replay_ref,
        remote_retention_delete_export_ref: remote_lifecycle_refs
            .remote_retention_delete_export_ref,
        remote_delivery_ack_ref: remote_lifecycle_refs.remote_delivery_ack_ref,
        remote_lifecycle_followup_ref: remote_lifecycle_refs.remote_lifecycle_followup_ref,
        remote_lifecycle_missing_artifact_count: 3,
        remote_lifecycle_manual_required: true,
        durable_envelope_schema_ref: durable_envelope_refs.schema_ref,
        durable_envelope_journal_ref: durable_envelope_refs.journal_ref,
        durable_envelope_replay_readiness_ref: durable_envelope_refs.replay_readiness_ref,
        durable_envelope_delete_export_readiness_ref: durable_envelope_refs
            .delete_export_readiness_ref,
        durable_envelope_support_status_ref: durable_envelope_refs.support_status_ref,
        durable_envelope_ready: true,
        durable_envelope_missing_artifact_count: 0,
        external_transport_delivery_implemented: broker_semantics
            .external_transport_delivery_implemented,
        family_hub_delivery_implemented: family_hub_decision.external_relay_delivery_implemented,
        cross_process_replay_implemented: false,
        remote_retention_delete_export_propagation_implemented: false,
        provider_delivery_implemented: false,
        child_device_delivery_implemented: false,
        product_ready_claimed: false,
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

fn accepted_event_type_count(decision: &EventDeliveryDecisionProof) -> usize {
    decision.subscriber_filter.accepted_event_types.len()
}

struct NetworkRuntimeRemoteDeliveryRefs {
    custody_proof_ref: SourceComponent,
    publisher_auth_ref: SourceComponent,
    subscriber_auth_ref: SourceComponent,
    encryption_ref: SourceComponent,
    retention_policy_ref: SourceComponent,
    replay_plan_ref: SourceComponent,
    deletion_plan_ref: SourceComponent,
    offset_policy_ref: SourceComponent,
    dedupe_policy_ref: SourceComponent,
    transport_config_ref: SourceComponent,
    relay_identity_ref: SourceComponent,
    relay_policy_ref: SourceComponent,
}

fn remote_delivery_refs() -> Result<NetworkRuntimeRemoteDeliveryRefs, EventingError> {
    Ok(NetworkRuntimeRemoteDeliveryRefs {
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
    })
}

struct NetworkRuntimeRemoteLifecycleRefs {
    cross_process_replay_ref: SourceComponent,
    remote_retention_delete_export_ref: SourceComponent,
    remote_delivery_ack_ref: SourceComponent,
    remote_lifecycle_followup_ref: SourceComponent,
}

fn remote_lifecycle_refs() -> Result<NetworkRuntimeRemoteLifecycleRefs, EventingError> {
    Ok(NetworkRuntimeRemoteLifecycleRefs {
        cross_process_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_LIFECYCLE_CROSS_PROCESS_REPLAY_REF,
        )?,
        remote_retention_delete_export_ref: source_component(
            constants::network_flow::TEST_REMOTE_LIFECYCLE_RETENTION_DELETE_EXPORT_REF,
        )?,
        remote_delivery_ack_ref: source_component(
            constants::network_flow::TEST_REMOTE_LIFECYCLE_DELIVERY_ACK_REF,
        )?,
        remote_lifecycle_followup_ref: source_component(
            constants::network_flow::TEST_REMOTE_LIFECYCLE_FOLLOWUP_REF,
        )?,
    })
}

struct NetworkRuntimeRemoteDurableEnvelopeRefs {
    schema_ref: SourceComponent,
    journal_ref: SourceComponent,
    replay_readiness_ref: SourceComponent,
    delete_export_readiness_ref: SourceComponent,
    support_status_ref: SourceComponent,
}

fn remote_durable_envelope_refs() -> Result<NetworkRuntimeRemoteDurableEnvelopeRefs, EventingError>
{
    Ok(NetworkRuntimeRemoteDurableEnvelopeRefs {
        schema_ref: source_component(
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SCHEMA_REF,
        )?,
        journal_ref: source_component(
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_JOURNAL_REF,
        )?,
        replay_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_REPLAY_REF,
        )?,
        delete_export_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_DELETE_EXPORT_REF,
        )?,
        support_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SUPPORT_STATUS_REF,
        )?,
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
            NetworkRuntimeRemoteDeliveryState::RequirementsSatisfiedButNotImplemented
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
