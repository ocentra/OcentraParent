use serde::{Deserialize, Serialize};

use crate::ids::{EventNamespace, EventType, SourceComponent, SubscriberId, TargetHandler};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventDeliveryRouteKind {
    LocalInProcess,
    LocalService,
    ExternalTransport,
    ExternalRelay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventDeliveryDecisionState {
    LocalRouteReady,
    ExternalTransportRouteManualRequired,
    ExternalRelayRouteManualRequired,
    ExternalTransportRouteRequirementsSatisfied,
    ExternalRelayRouteRequirementsSatisfied,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventDeliveryRequiredArtifact {
    CustodyProof,
    PublisherAuthProof,
    SubscriberAuthProof,
    EncryptionProof,
    RetentionPolicy,
    ReplayPlan,
    DeletionPlan,
    BackpressurePolicy,
    OffsetPolicy,
    DedupePolicy,
    TransportConfig,
    ExternalRelayIdentity,
    ExternalRelayPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDeliveryBackpressurePolicy {
    pub bounded_queue_capacity: usize,
    pub ttl_millis: u64,
    pub overflow_dead_letters: bool,
    pub idempotency_required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDeliverySubscriberFilter {
    pub subscriber_id: SubscriberId,
    pub target_handler: TargetHandler,
    pub event_namespace: EventNamespace,
    pub accepted_event_types: Vec<EventType>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDeliveryDecisionInput {
    pub route_kind: EventDeliveryRouteKind,
    pub event_namespace: EventNamespace,
    pub publisher_component: SourceComponent,
    pub subscriber_filter: EventDeliverySubscriberFilter,
    pub backpressure_policy: EventDeliveryBackpressurePolicy,
    pub custody_proof_ref: Option<SourceComponent>,
    pub publisher_auth_ref: Option<SourceComponent>,
    pub subscriber_auth_ref: Option<SourceComponent>,
    pub encryption_ref: Option<SourceComponent>,
    pub retention_policy_ref: Option<SourceComponent>,
    pub replay_plan_ref: Option<SourceComponent>,
    pub deletion_plan_ref: Option<SourceComponent>,
    pub offset_policy_ref: Option<SourceComponent>,
    pub dedupe_policy_ref: Option<SourceComponent>,
    pub transport_config_ref: Option<SourceComponent>,
    pub relay_identity_ref: Option<SourceComponent>,
    pub relay_policy_ref: Option<SourceComponent>,
    pub external_transport_delivery_claimed: bool,
    pub external_relay_delivery_claimed: bool,
    pub decision_authority_claimed: bool,
    pub side_effect_authority_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDeliveryDecisionProof {
    pub route_kind: EventDeliveryRouteKind,
    pub decision_state: EventDeliveryDecisionState,
    pub event_namespace: EventNamespace,
    pub publisher_component: SourceComponent,
    pub subscriber_filter: EventDeliverySubscriberFilter,
    pub required_artifacts: Vec<EventDeliveryRequiredArtifact>,
    pub missing_artifacts: Vec<EventDeliveryRequiredArtifact>,
    pub backpressure_policy: EventDeliveryBackpressurePolicy,
    pub retention_policy_ref: Option<SourceComponent>,
    pub local_delivery_ready: bool,
    pub external_transport_delivery_implemented: bool,
    pub external_relay_delivery_implemented: bool,
    pub subscriber_filtering_enabled: bool,
    pub decision_authority: bool,
    pub side_effect_authority: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventDeliveryDecisionError {
    EmptySubscriberAcceptedEvents,
    SubscriberFilterOutsideNamespace,
    InvalidBackpressureCapacity,
    InvalidBackpressureTtl,
    LiveExternalTransportDeliveryClaimRejected,
    LiveExternalRelayDeliveryClaimRejected,
    DecisionAuthorityClaimRejected,
    SideEffectAuthorityClaimRejected,
}

pub fn decide_event_delivery_route(
    input: EventDeliveryDecisionInput,
) -> Result<EventDeliveryDecisionProof, EventDeliveryDecisionError> {
    reject_claims(&input)?;
    validate_subscriber_filter(&input)?;
    validate_backpressure(&input.backpressure_policy)?;

    let required_artifacts = required_artifacts(input.route_kind);
    let missing_artifacts = missing_artifacts(&input, &required_artifacts);
    let decision_state = decision_state(input.route_kind, missing_artifacts.is_empty());
    let local_delivery_ready = matches!(
        decision_state,
        EventDeliveryDecisionState::LocalRouteReady
            | EventDeliveryDecisionState::ExternalTransportRouteRequirementsSatisfied
            | EventDeliveryDecisionState::ExternalRelayRouteRequirementsSatisfied
    );

    Ok(EventDeliveryDecisionProof {
        route_kind: input.route_kind,
        decision_state,
        event_namespace: input.event_namespace,
        publisher_component: input.publisher_component,
        subscriber_filter: input.subscriber_filter,
        required_artifacts,
        missing_artifacts,
        backpressure_policy: input.backpressure_policy,
        retention_policy_ref: input.retention_policy_ref,
        local_delivery_ready,
        external_transport_delivery_implemented: false,
        external_relay_delivery_implemented: false,
        subscriber_filtering_enabled: true,
        decision_authority: false,
        side_effect_authority: false,
    })
}

fn reject_claims(input: &EventDeliveryDecisionInput) -> Result<(), EventDeliveryDecisionError> {
    if input.external_transport_delivery_claimed {
        return Err(EventDeliveryDecisionError::LiveExternalTransportDeliveryClaimRejected);
    }
    if input.external_relay_delivery_claimed {
        return Err(EventDeliveryDecisionError::LiveExternalRelayDeliveryClaimRejected);
    }
    if input.decision_authority_claimed {
        return Err(EventDeliveryDecisionError::DecisionAuthorityClaimRejected);
    }
    if input.side_effect_authority_claimed {
        return Err(EventDeliveryDecisionError::SideEffectAuthorityClaimRejected);
    }
    Ok(())
}

fn validate_subscriber_filter(
    input: &EventDeliveryDecisionInput,
) -> Result<(), EventDeliveryDecisionError> {
    if input.subscriber_filter.accepted_event_types.is_empty() {
        return Err(EventDeliveryDecisionError::EmptySubscriberAcceptedEvents);
    }
    if input.subscriber_filter.event_namespace != input.event_namespace {
        return Err(EventDeliveryDecisionError::SubscriberFilterOutsideNamespace);
    }
    if input
        .subscriber_filter
        .accepted_event_types
        .iter()
        .any(|event_type| !input.event_namespace.matches_event_type(event_type))
    {
        return Err(EventDeliveryDecisionError::SubscriberFilterOutsideNamespace);
    }
    Ok(())
}

fn validate_backpressure(
    policy: &EventDeliveryBackpressurePolicy,
) -> Result<(), EventDeliveryDecisionError> {
    if policy.bounded_queue_capacity == 0 {
        return Err(EventDeliveryDecisionError::InvalidBackpressureCapacity);
    }
    if policy.ttl_millis == 0 {
        return Err(EventDeliveryDecisionError::InvalidBackpressureTtl);
    }
    Ok(())
}

fn required_artifacts(route_kind: EventDeliveryRouteKind) -> Vec<EventDeliveryRequiredArtifact> {
    match route_kind {
        EventDeliveryRouteKind::LocalInProcess | EventDeliveryRouteKind::LocalService => Vec::new(),
        EventDeliveryRouteKind::ExternalTransport => external_transport_required_artifacts(),
        EventDeliveryRouteKind::ExternalRelay => {
            let mut requirements = external_transport_required_artifacts();
            requirements.push(EventDeliveryRequiredArtifact::ExternalRelayIdentity);
            requirements.push(EventDeliveryRequiredArtifact::ExternalRelayPolicy);
            requirements
        }
    }
}

fn external_transport_required_artifacts() -> Vec<EventDeliveryRequiredArtifact> {
    vec![
        EventDeliveryRequiredArtifact::CustodyProof,
        EventDeliveryRequiredArtifact::PublisherAuthProof,
        EventDeliveryRequiredArtifact::SubscriberAuthProof,
        EventDeliveryRequiredArtifact::EncryptionProof,
        EventDeliveryRequiredArtifact::RetentionPolicy,
        EventDeliveryRequiredArtifact::ReplayPlan,
        EventDeliveryRequiredArtifact::DeletionPlan,
        EventDeliveryRequiredArtifact::BackpressurePolicy,
        EventDeliveryRequiredArtifact::OffsetPolicy,
        EventDeliveryRequiredArtifact::DedupePolicy,
        EventDeliveryRequiredArtifact::TransportConfig,
    ]
}

fn missing_artifacts(
    input: &EventDeliveryDecisionInput,
    required_artifacts: &[EventDeliveryRequiredArtifact],
) -> Vec<EventDeliveryRequiredArtifact> {
    required_artifacts
        .iter()
        .copied()
        .filter(|artifact| artifact_ref(input, *artifact).is_none())
        .collect()
}

fn artifact_ref(
    input: &EventDeliveryDecisionInput,
    artifact: EventDeliveryRequiredArtifact,
) -> Option<&SourceComponent> {
    match artifact {
        EventDeliveryRequiredArtifact::CustodyProof => input.custody_proof_ref.as_ref(),
        EventDeliveryRequiredArtifact::PublisherAuthProof => input.publisher_auth_ref.as_ref(),
        EventDeliveryRequiredArtifact::SubscriberAuthProof => input.subscriber_auth_ref.as_ref(),
        EventDeliveryRequiredArtifact::EncryptionProof => input.encryption_ref.as_ref(),
        EventDeliveryRequiredArtifact::RetentionPolicy => input.retention_policy_ref.as_ref(),
        EventDeliveryRequiredArtifact::ReplayPlan => input.replay_plan_ref.as_ref(),
        EventDeliveryRequiredArtifact::DeletionPlan => input.deletion_plan_ref.as_ref(),
        EventDeliveryRequiredArtifact::BackpressurePolicy => Some(&input.publisher_component),
        EventDeliveryRequiredArtifact::OffsetPolicy => input.offset_policy_ref.as_ref(),
        EventDeliveryRequiredArtifact::DedupePolicy => input.dedupe_policy_ref.as_ref(),
        EventDeliveryRequiredArtifact::TransportConfig => input.transport_config_ref.as_ref(),
        EventDeliveryRequiredArtifact::ExternalRelayIdentity => input.relay_identity_ref.as_ref(),
        EventDeliveryRequiredArtifact::ExternalRelayPolicy => input.relay_policy_ref.as_ref(),
    }
}

fn decision_state(
    route_kind: EventDeliveryRouteKind,
    requirements_satisfied: bool,
) -> EventDeliveryDecisionState {
    match (route_kind, requirements_satisfied) {
        (EventDeliveryRouteKind::LocalInProcess | EventDeliveryRouteKind::LocalService, _) => {
            EventDeliveryDecisionState::LocalRouteReady
        }
        (EventDeliveryRouteKind::ExternalTransport, true) => {
            EventDeliveryDecisionState::ExternalTransportRouteRequirementsSatisfied
        }
        (EventDeliveryRouteKind::ExternalTransport, false) => {
            EventDeliveryDecisionState::ExternalTransportRouteManualRequired
        }
        (EventDeliveryRouteKind::ExternalRelay, true) => {
            EventDeliveryDecisionState::ExternalRelayRouteRequirementsSatisfied
        }
        (EventDeliveryRouteKind::ExternalRelay, false) => {
            EventDeliveryDecisionState::ExternalRelayRouteManualRequired
        }
    }
}
