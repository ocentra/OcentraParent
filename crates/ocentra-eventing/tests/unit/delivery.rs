use ocentra_eventing::{
    decide_event_delivery_route, EventDeliveryBackpressurePolicy, EventDeliveryDecisionError,
    EventDeliveryDecisionInput, EventDeliveryDecisionState, EventDeliveryRouteKind,
    EventDeliverySubscriberFilter, EventNamespace, EventType, SourceComponent, SubscriberId,
    TargetHandler,
};

#[test]
fn local_event_delivery_requires_namespace_filtered_subscriber_and_backpressure() {
    let proof = decide_event_delivery_route(local_input()).expect("local route should decide");

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
    outside_namespace.subscriber_filter.accepted_event_types =
        vec![EventType::parse("browser.navigation.observed").expect("event type parses")];
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

fn local_input() -> EventDeliveryDecisionInput {
    let namespace = EventNamespace::parse("tracking").expect("namespace parses");

    EventDeliveryDecisionInput {
        route_kind: EventDeliveryRouteKind::LocalInProcess,
        event_namespace: namespace.clone(),
        publisher_component: SourceComponent::parse("tracking-core")
            .expect("source component parses"),
        subscriber_filter: EventDeliverySubscriberFilter {
            subscriber_id: SubscriberId::parse("child-runtime-tracking")
                .expect("subscriber id parses"),
            target_handler: TargetHandler::parse("child-runtime.tracking")
                .expect("target handler parses"),
            event_namespace: namespace,
            accepted_event_types: vec![
                EventType::parse("tracking.location.observed").expect("event type parses")
            ],
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
