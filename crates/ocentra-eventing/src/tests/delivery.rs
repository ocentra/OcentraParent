use crate::{
    decide_event_delivery_route, EventDeliveryBackpressurePolicy, EventDeliveryDecisionError,
    EventDeliveryDecisionInput, EventDeliveryDecisionState, EventDeliveryRequiredArtifact,
    EventDeliveryRouteKind, EventDeliverySubscriberFilter, EventNamespace, EventType,
    SourceComponent, SubscriberId, TargetHandler,
};

const NETWORK_NAMESPACE: &str = "network";
const NETWORK_FLOW_EVENT: &str = "network.flow.observed";
const NETWORK_ALERT_EVENT: &str = "network.alert.observed";
const OTHER_EVENT: &str = "screen.evidence.observed";

#[test]
fn delivery_decision_allows_local_first_route_with_filter_and_backpressure() {
    let proof = decide_event_delivery_route(local_input(EventDeliveryRouteKind::LocalInProcess))
        .expect("local in-process event route should be ready");

    assert_eq!(proof.route_kind, EventDeliveryRouteKind::LocalInProcess);
    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::LocalRouteReady
    );
    assert_eq!(proof.event_namespace.as_str(), NETWORK_NAMESPACE);
    assert_eq!(
        proof.publisher_component.as_str(),
        "network-runtime-publisher"
    );
    assert_eq!(proof.required_artifacts, Vec::new());
    assert_eq!(proof.missing_artifacts, Vec::new());
    assert_eq!(proof.backpressure_policy.bounded_queue_capacity, 32);
    assert_eq!(proof.backpressure_policy.ttl_millis, 30_000);
    assert!(proof.backpressure_policy.overflow_dead_letters);
    assert!(proof.backpressure_policy.idempotency_required);
    assert!(proof.local_delivery_ready);
    assert!(proof.subscriber_filtering_enabled);
    assert!(!proof.broker_delivery_implemented);
    assert!(!proof.family_hub_delivery_implemented);
    assert!(!proof.policy_authority);
    assert!(!proof.adapter_authority);
}

#[test]
fn delivery_decision_marks_broker_route_manual_required_without_required_artifacts() {
    let proof = decide_event_delivery_route(EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::BrokerBacked,
        ..local_input(EventDeliveryRouteKind::LocalInProcess)
    })
    .expect("broker route decision should be reportable when artifacts are missing");

    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::BrokerRouteManualRequired
    );
    assert_eq!(proof.required_artifacts, broker_requirements());
    assert_eq!(
        proof.missing_artifacts,
        vec![
            EventDeliveryRequiredArtifact::CustodyProof,
            EventDeliveryRequiredArtifact::PublisherAuthProof,
            EventDeliveryRequiredArtifact::SubscriberAuthProof,
            EventDeliveryRequiredArtifact::EncryptionProof,
            EventDeliveryRequiredArtifact::RetentionPolicy,
            EventDeliveryRequiredArtifact::ReplayPlan,
            EventDeliveryRequiredArtifact::DeletionPlan,
            EventDeliveryRequiredArtifact::OffsetPolicy,
            EventDeliveryRequiredArtifact::DedupePolicy,
            EventDeliveryRequiredArtifact::BrokerConfig
        ]
    );
    assert!(!proof.broker_delivery_implemented);
    assert!(!proof.family_hub_delivery_implemented);
}

#[test]
fn delivery_decision_marks_family_hub_route_manual_required_for_family_artifacts() {
    let proof = decide_event_delivery_route(EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::FamilyHub,
        custody_proof_ref: component("custody-proof-45"),
        publisher_auth_ref: component("publisher-auth-proof-45"),
        subscriber_auth_ref: component("subscriber-auth-proof-45"),
        encryption_ref: component("encryption-proof-45"),
        retention_policy_ref: component("retention-policy-proof-45"),
        replay_plan_ref: component("replay-plan-proof-45"),
        deletion_plan_ref: component("deletion-plan-proof-45"),
        offset_policy_ref: component("offset-policy-proof-45"),
        dedupe_policy_ref: component("dedupe-policy-proof-45"),
        broker_config_ref: component("broker-config-proof-45"),
        ..local_input(EventDeliveryRouteKind::LocalInProcess)
    })
    .expect("family-hub route should require family-hub specific artifacts");

    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::FamilyHubRouteManualRequired
    );
    assert_eq!(
        proof.missing_artifacts,
        vec![
            EventDeliveryRequiredArtifact::FamilyHubIdentity,
            EventDeliveryRequiredArtifact::FamilyHubRelayPolicy
        ]
    );
    assert!(!proof.family_hub_delivery_implemented);
}

#[test]
fn delivery_decision_preserves_satisfied_broker_requirements_without_live_broker() {
    let proof = decide_event_delivery_route(broker_requirements_satisfied_input())
        .expect("complete broker route requirements should be distinguishable from live delivery");

    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::BrokerRouteRequirementsSatisfied
    );
    assert_eq!(proof.missing_artifacts, Vec::new());
    assert_eq!(
        proof
            .retention_policy_ref
            .as_ref()
            .expect("retention ref exists")
            .as_str(),
        "retention-policy-proof-45"
    );
    assert!(!proof.broker_delivery_implemented);
    assert!(!proof.family_hub_delivery_implemented);
    assert!(proof.local_delivery_ready);
}

#[test]
fn delivery_decision_rejects_live_claims_and_invalid_route_metadata() {
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            broker_delivery_claimed: true,
            ..local_input(EventDeliveryRouteKind::BrokerBacked)
        }),
        Err(EventDeliveryDecisionError::LiveBrokerDeliveryClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            family_hub_delivery_claimed: true,
            ..local_input(EventDeliveryRouteKind::FamilyHub)
        }),
        Err(EventDeliveryDecisionError::LiveFamilyHubDeliveryClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            policy_authority_claimed: true,
            ..local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::PolicyAuthorityClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            adapter_authority_claimed: true,
            ..local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::AdapterAuthorityClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            backpressure_policy: EventDeliveryBackpressurePolicy {
                bounded_queue_capacity: 0,
                ..backpressure_policy()
            },
            ..local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::InvalidBackpressureCapacity)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            subscriber_filter: EventDeliverySubscriberFilter {
                accepted_event_types: vec![event_type(OTHER_EVENT)],
                ..subscriber_filter()
            },
            ..local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::SubscriberFilterOutsideNamespace)
    );
}

fn local_input(route_kind: EventDeliveryRouteKind) -> EventDeliveryDecisionInput {
    EventDeliveryDecisionInput {
        route_kind,
        event_namespace: event_namespace(),
        publisher_component: source_component("network-runtime-publisher"),
        subscriber_filter: subscriber_filter(),
        backpressure_policy: backpressure_policy(),
        custody_proof_ref: None,
        publisher_auth_ref: None,
        subscriber_auth_ref: None,
        encryption_ref: None,
        retention_policy_ref: None,
        replay_plan_ref: None,
        deletion_plan_ref: None,
        offset_policy_ref: None,
        dedupe_policy_ref: None,
        broker_config_ref: None,
        family_hub_identity_ref: None,
        family_hub_relay_policy_ref: None,
        broker_delivery_claimed: false,
        family_hub_delivery_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
    }
}

fn broker_requirements_satisfied_input() -> EventDeliveryDecisionInput {
    EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::BrokerBacked,
        custody_proof_ref: component("custody-proof-45"),
        publisher_auth_ref: component("publisher-auth-proof-45"),
        subscriber_auth_ref: component("subscriber-auth-proof-45"),
        encryption_ref: component("encryption-proof-45"),
        retention_policy_ref: component("retention-policy-proof-45"),
        replay_plan_ref: component("replay-plan-proof-45"),
        deletion_plan_ref: component("deletion-plan-proof-45"),
        offset_policy_ref: component("offset-policy-proof-45"),
        dedupe_policy_ref: component("dedupe-policy-proof-45"),
        broker_config_ref: component("broker-config-proof-45"),
        ..local_input(EventDeliveryRouteKind::BrokerBacked)
    }
}

fn subscriber_filter() -> EventDeliverySubscriberFilter {
    EventDeliverySubscriberFilter {
        subscriber_id: SubscriberId::parse("network-read-model-subscriber")
            .expect("subscriber id parses"),
        target_handler: TargetHandler::parse("network-read-model-projector")
            .expect("target handler parses"),
        event_namespace: event_namespace(),
        accepted_event_types: vec![
            event_type(NETWORK_FLOW_EVENT),
            event_type(NETWORK_ALERT_EVENT),
        ],
    }
}

fn backpressure_policy() -> EventDeliveryBackpressurePolicy {
    EventDeliveryBackpressurePolicy {
        bounded_queue_capacity: 32,
        ttl_millis: 30_000,
        overflow_dead_letters: true,
        idempotency_required: true,
    }
}

fn broker_requirements() -> Vec<EventDeliveryRequiredArtifact> {
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
        EventDeliveryRequiredArtifact::BrokerConfig,
    ]
}

fn event_namespace() -> EventNamespace {
    EventNamespace::parse(NETWORK_NAMESPACE).expect("namespace parses")
}

fn event_type(value: &str) -> EventType {
    EventType::parse(value).expect("event type parses")
}

fn source_component(value: &str) -> SourceComponent {
    SourceComponent::parse(value).expect("source component parses")
}

fn component(value: &str) -> Option<SourceComponent> {
    Some(source_component(value))
}
