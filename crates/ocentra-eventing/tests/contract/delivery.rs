use ocentra_eventing::delivery::decide_event_delivery_route;
use ocentra_eventing::delivery::validation::{
    EventDeliveryBackpressurePolicy, EventDeliveryDecisionError, EventDeliveryDecisionInput,
    EventDeliveryDecisionState, EventDeliveryRequiredArtifact, EventDeliveryRouteKind,
    EventDeliverySubscriberFilter,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    EventNamespace, EventType, SourceComponent, SubscriberId, TargetHandler,
};

const NETWORK_NAMESPACE: &str = "network";
const NETWORK_FLOW_EVENT: &str = "network.flow.observed";
const NETWORK_ALERT_EVENT: &str = "network.alert.observed";
const TRACKING_NAMESPACE: &str = "tracking";
const TRACKING_LOCATION_SUFFIX: &str = "location.observed";
const SCREEN_NAMESPACE: &str = "screen";
const SCREEN_EVIDENCE_SUFFIX: &str = "evidence.observed";
const BROWSER_NAMESPACE: &str = "browser";
const BROWSER_NAVIGATION_SUFFIX: &str = "navigation.observed";

#[derive(Clone)]
pub(super) struct TestText(pub(super) String);

#[test]
fn local_event_delivery_requires_namespace_filtered_subscriber_and_backpressure() {
    let proof =
        decide_event_delivery_route(local_input()).expect_value("local route should decide");

    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::LocalRouteReady
    );
    assert!(proof.local_delivery_ready);
    assert!(proof.required_artifacts.is_empty());
    assert!(proof.missing_artifacts.is_empty());
    assert!(proof.subscriber_filtering_enabled);
    assert!(!proof.decision_authority);
    assert!(!proof.side_effect_authority);
}

#[test]
fn delivery_rejects_empty_or_out_of_namespace_subscriber_filters() {
    let mut empty_filter = local_input();
    empty_filter.subscriber_filter.accepted_event_types = Vec::new();
    assert_eq!(
        decide_event_delivery_route(empty_filter),
        Err(EventDeliveryDecisionError::EmptySubscriberAcceptedEvents)
    );

    let mut outside_namespace = local_input();
    outside_namespace.subscriber_filter.accepted_event_types = vec![event_type_with_suffix(
        TestText(BROWSER_NAMESPACE.to_owned()),
        TestText(BROWSER_NAVIGATION_SUFFIX.to_owned()),
    )];
    assert_eq!(
        decide_event_delivery_route(outside_namespace),
        Err(EventDeliveryDecisionError::SubscriberFilterOutsideNamespace)
    );
}

#[test]
fn delivery_rejects_live_external_or_authority_claims_without_artifact_path() {
    let mut transport_claim = local_input();
    transport_claim.external_transport_delivery_claimed = true;
    assert_eq!(
        decide_event_delivery_route(transport_claim),
        Err(EventDeliveryDecisionError::LiveExternalTransportDeliveryClaimRejected)
    );

    let mut authority_claim = local_input();
    authority_claim.decision_authority_claimed = true;
    assert_eq!(
        decide_event_delivery_route(authority_claim),
        Err(EventDeliveryDecisionError::DecisionAuthorityClaimRejected)
    );
}

#[test]
fn delivery_decision_allows_local_first_route_with_filter_and_backpressure() {
    let proof =
        decide_event_delivery_route(network_local_input(EventDeliveryRouteKind::LocalInProcess))
            .expect_value("local in-process event route should be ready");

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
    assert!(!proof.external_transport_delivery_implemented);
    assert!(!proof.external_relay_delivery_implemented);
    assert!(!proof.decision_authority);
    assert!(!proof.side_effect_authority);
}

#[test]
fn delivery_decision_marks_external_transport_manual_required_without_required_artifacts() {
    let proof = decide_event_delivery_route(EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::ExternalTransport,
        ..network_local_input(EventDeliveryRouteKind::LocalInProcess)
    })
    .expect_value(
        "external transport route decision should be reportable when artifacts are missing",
    );

    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::ExternalTransportRouteManualRequired
    );
    assert_eq!(
        proof.required_artifacts,
        network_external_transport_requirements()
    );
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
            EventDeliveryRequiredArtifact::TransportConfig
        ]
    );
    assert!(!proof.external_transport_delivery_implemented);
    assert!(!proof.external_relay_delivery_implemented);
}

#[test]
fn delivery_decision_marks_external_relay_manual_required_for_relay_artifacts() {
    let proof = decide_event_delivery_route(EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::ExternalRelay,
        custody_proof_ref: network_component(TestText("custody-proof-45".to_owned())),
        publisher_auth_ref: network_component(TestText("publisher-auth-proof-45".to_owned())),
        subscriber_auth_ref: network_component(TestText("subscriber-auth-proof-45".to_owned())),
        encryption_ref: network_component(TestText("encryption-proof-45".to_owned())),
        retention_policy_ref: network_component(TestText("retention-policy-proof-45".to_owned())),
        replay_plan_ref: network_component(TestText("replay-plan-proof-45".to_owned())),
        deletion_plan_ref: network_component(TestText("deletion-plan-proof-45".to_owned())),
        offset_policy_ref: network_component(TestText("offset-policy-proof-45".to_owned())),
        dedupe_policy_ref: network_component(TestText("dedupe-policy-proof-45".to_owned())),
        transport_config_ref: network_component(TestText("transport-config-proof-45".to_owned())),
        ..network_local_input(EventDeliveryRouteKind::LocalInProcess)
    })
    .expect_value("external relay route should require relay specific artifacts");

    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::ExternalRelayRouteManualRequired
    );
    assert_eq!(
        proof.missing_artifacts,
        vec![
            EventDeliveryRequiredArtifact::ExternalRelayIdentity,
            EventDeliveryRequiredArtifact::ExternalRelayPolicy
        ]
    );
    assert!(!proof.external_relay_delivery_implemented);
}

#[test]
fn delivery_decision_preserves_satisfied_external_transport_requirements_without_live_transport() {
    let proof =
        decide_event_delivery_route(network_external_transport_requirements_satisfied_input())
            .expect_value(
            "complete external transport requirements should be distinguishable from live delivery",
        );

    assert_eq!(
        proof.decision_state,
        EventDeliveryDecisionState::ExternalTransportRouteRequirementsSatisfied
    );
    assert_eq!(proof.missing_artifacts, Vec::new());
    assert_eq!(
        proof
            .retention_policy_ref
            .as_ref()
            .expect_value("retention ref exists")
            .as_str(),
        "retention-policy-proof-45"
    );
    assert!(!proof.external_transport_delivery_implemented);
    assert!(!proof.external_relay_delivery_implemented);
    assert!(proof.local_delivery_ready);
}

#[test]
fn delivery_decision_rejects_live_claims_and_invalid_route_metadata() {
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            external_transport_delivery_claimed: true,
            ..network_local_input(EventDeliveryRouteKind::ExternalTransport)
        }),
        Err(EventDeliveryDecisionError::LiveExternalTransportDeliveryClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            external_relay_delivery_claimed: true,
            ..network_local_input(EventDeliveryRouteKind::ExternalRelay)
        }),
        Err(EventDeliveryDecisionError::LiveExternalRelayDeliveryClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            decision_authority_claimed: true,
            ..network_local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::DecisionAuthorityClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            side_effect_authority_claimed: true,
            ..network_local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::SideEffectAuthorityClaimRejected)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            backpressure_policy: EventDeliveryBackpressurePolicy {
                bounded_queue_capacity: 0,
                ..network_backpressure_policy()
            },
            ..network_local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::InvalidBackpressureCapacity)
    );
    assert_eq!(
        decide_event_delivery_route(EventDeliveryDecisionInput {
            subscriber_filter: EventDeliverySubscriberFilter {
                accepted_event_types: vec![event_type_with_suffix(
                    TestText(SCREEN_NAMESPACE.to_owned()),
                    TestText(SCREEN_EVIDENCE_SUFFIX.to_owned()),
                )],
                ..network_subscriber_filter()
            },
            ..network_local_input(EventDeliveryRouteKind::LocalService)
        }),
        Err(EventDeliveryDecisionError::SubscriberFilterOutsideNamespace)
    );
}

fn local_input() -> EventDeliveryDecisionInput {
    let namespace = EventNamespace::parse(TRACKING_NAMESPACE).expect_value("namespace parses");

    EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::LocalInProcess,
        event_namespace: namespace.clone(),
        publisher_component: SourceComponent::parse("tracking-core")
            .expect_value("source component parses"),
        subscriber_filter: EventDeliverySubscriberFilter {
            subscriber_id: SubscriberId::parse("child-runtime-tracking")
                .expect_value("subscriber id parses"),
            target_handler: TargetHandler::parse("child-runtime.tracking")
                .expect_value("target handler parses"),
            event_namespace: namespace,
            accepted_event_types: vec![event_type_with_suffix(
                TestText(TRACKING_NAMESPACE.to_owned()),
                TestText(TRACKING_LOCATION_SUFFIX.to_owned()),
            )],
        },
        backpressure_policy: EventDeliveryBackpressurePolicy {
            bounded_queue_capacity: 128,
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
    }
}

fn network_local_input(route_kind: EventDeliveryRouteKind) -> EventDeliveryDecisionInput {
    EventDeliveryDecisionInput {
        route_kind,
        event_namespace: network_event_namespace(),
        publisher_component: network_source_component(TestText(
            "network-runtime-publisher".to_owned(),
        )),
        subscriber_filter: network_subscriber_filter(),
        backpressure_policy: network_backpressure_policy(),
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
    }
}

fn network_external_transport_requirements_satisfied_input() -> EventDeliveryDecisionInput {
    EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::ExternalTransport,
        custody_proof_ref: network_component(TestText("custody-proof-45".to_owned())),
        publisher_auth_ref: network_component(TestText("publisher-auth-proof-45".to_owned())),
        subscriber_auth_ref: network_component(TestText("subscriber-auth-proof-45".to_owned())),
        encryption_ref: network_component(TestText("encryption-proof-45".to_owned())),
        retention_policy_ref: network_component(TestText("retention-policy-proof-45".to_owned())),
        replay_plan_ref: network_component(TestText("replay-plan-proof-45".to_owned())),
        deletion_plan_ref: network_component(TestText("deletion-plan-proof-45".to_owned())),
        offset_policy_ref: network_component(TestText("offset-policy-proof-45".to_owned())),
        dedupe_policy_ref: network_component(TestText("dedupe-policy-proof-45".to_owned())),
        transport_config_ref: network_component(TestText("transport-config-proof-45".to_owned())),
        ..network_local_input(EventDeliveryRouteKind::ExternalTransport)
    }
}

fn network_subscriber_filter() -> EventDeliverySubscriberFilter {
    EventDeliverySubscriberFilter {
        subscriber_id: SubscriberId::parse("network-read-model-subscriber")
            .expect_value("subscriber id parses"),
        target_handler: TargetHandler::parse("network-read-model-projector")
            .expect_value("target handler parses"),
        event_namespace: network_event_namespace(),
        accepted_event_types: vec![
            network_event_type(TestText(NETWORK_FLOW_EVENT.to_owned())),
            network_event_type(TestText(NETWORK_ALERT_EVENT.to_owned())),
        ],
    }
}

fn network_backpressure_policy() -> EventDeliveryBackpressurePolicy {
    EventDeliveryBackpressurePolicy {
        bounded_queue_capacity: 32,
        ttl_millis: 30_000,
        overflow_dead_letters: true,
        idempotency_required: true,
    }
}

fn network_external_transport_requirements() -> Vec<EventDeliveryRequiredArtifact> {
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

fn network_event_namespace() -> EventNamespace {
    EventNamespace::parse(NETWORK_NAMESPACE).expect_value("namespace parses")
}

fn network_event_type(value: TestText) -> EventType {
    EventType::parse(value.0).expect_value("event type parses")
}

fn event_type_with_suffix(namespace: TestText, suffix: TestText) -> EventType {
    let namespace = namespace.0;
    let suffix = suffix.0;
    network_event_type(TestText(format!("{namespace}.{suffix}")))
}

fn network_source_component(value: TestText) -> SourceComponent {
    SourceComponent::parse(value.0).expect_value("source component parses")
}

fn network_component(value: TestText) -> Option<SourceComponent> {
    Some(network_source_component(value))
}
