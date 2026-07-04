use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_lan_core::lan_pairing::discovery::{LanAggregateId, LanDiscoveryDecisionId};
use ocentra_lan_core::lan_pairing::{
    evaluate_lan_discovery, lan_ai_analysis_requested_event, lan_discovery_decision_recorded_event,
    lan_evidence_recorded_event, lan_observed_event, lan_policy_evaluation_requested_event,
    LanDiscoveryActionState, LanDiscoveryInput, LanInterfaceState, LanObservationIntent,
    LanPairingActionState, LanPeerTrustState, LanRelayState,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainObservedSignal, ChildRuntimeDomain,
};

#[test]
fn unknown_peer_requests_ai_and_policy_evidence() {
    let observed = lan_observed_event(LanObservationIntent::UnknownPeerRequiresAi);
    let evidence = lan_evidence_recorded_event(&observed);

    assert_eq!(observed.domain, ChildRuntimeDomain::Lan);
    assert_eq!(
        observed.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    assert_eq!(
        lan_ai_analysis_requested_event(&evidence)
            .expect_value("lan ai request")
            .evidence_refs,
        vec![evidence.evidence_ref.clone()]
    );
    assert_eq!(lan_policy_evaluation_requested_event(&evidence), None);
}

#[test]
fn unavailable_interface_blocks_pairing_and_requires_manual_discovery() {
    let decision = evaluate_lan_discovery(LanDiscoveryInput {
        interface_state: LanInterfaceState::Unavailable,
        peer_trust_state: LanPeerTrustState::Trusted,
        relay_state: LanRelayState::LocalDirect,
    });

    assert_eq!(
        decision.discovery_action_state,
        LanDiscoveryActionState::ManualRequired
    );
    assert_eq!(decision.pairing_action_state, LanPairingActionState::Block);
}

#[test]
fn discovery_decision_event_has_aggregate_and_idempotency_contract() {
    let event = lan_discovery_decision_recorded_event(
        LanAggregateId::parse("lan-aggregate-family-device").expect_value("lan aggregate"),
        LanDiscoveryDecisionId::parse("lan-decision-001").expect_value("lan decision"),
        LanDiscoveryInput {
            interface_state: LanInterfaceState::Available,
            peer_trust_state: LanPeerTrustState::Trusted,
            relay_state: LanRelayState::LocalDirect,
        },
    );

    assert_eq!(
        event.aggregate_key().expect_value("aggregate").as_str(),
        event.aggregate_id.as_str()
    );
    assert!(event
        .idempotency_key()
        .expect_value("idempotency")
        .as_str()
        .ends_with(event.decision_id.as_str()));
}
