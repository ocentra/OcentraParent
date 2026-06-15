use ocentra_eventing::envelope::DomainEvent;
use ocentra_lan_core::{
    evaluate_lan_discovery, lan_discovery_decision_recorded_event, LanAggregateId,
    LanDiscoveryActionState, LanDiscoveryDecisionId, LanDiscoveryInput, LanInterfaceState,
    LanPairingActionState, LanPeerTrustState, LanRelayState,
};
use ocentra_parent_agent_protocol::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};

#[test]
fn lan_observation_records_presence_evidence_and_requests_policy() {
    let observed = ocentra_lan_core::default_lan_observed_event();
    let evidence = ocentra_lan_core::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_ai_analysis_requested_event(&evidence);
    let policy = ocentra_lan_core::lan_policy_evaluation_requested_event(&evidence)
        .expect("LAN policy request is expected");

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::Lan.observed_event_type()
    );
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::Lan.evidence_recorded_event_type()
    );
    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert!(ai.is_none());
    assert_eq!(
        policy.event_type,
        ChildRuntimeDomain::Lan.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn lan_unknown_peer_requests_ai_before_policy() {
    let observed = ocentra_lan_core::lan_observed_event(
        ocentra_lan_core::LanObservationIntent::UnknownPeerRequiresAi,
    );
    let evidence = ocentra_lan_core::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_ai_analysis_requested_event(&evidence)
        .expect("unknown LAN peer requires AI boundary");
    let policy = ocentra_lan_core::lan_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert_eq!(
        ai.event_type,
        ChildRuntimeDomain::Lan.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert!(policy.is_none());
}

#[test]
fn lan_discovery_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_lan_core::lan_observed_event(
        ocentra_lan_core::LanObservationIntent::DiscoveryObservationOnly,
    );
    let evidence = ocentra_lan_core::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_ai_analysis_requested_event(&evidence);
    let policy = ocentra_lan_core::lan_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::NotRequired
    );
    assert!(ai.is_none());
    assert!(policy.is_none());
}

#[test]
fn lan_discovery_allows_signed_pairing_for_trusted_local_peer() {
    let decision = evaluate_lan_discovery(LanDiscoveryInput {
        interface_state: LanInterfaceState::Available,
        peer_trust_state: LanPeerTrustState::Trusted,
        relay_state: LanRelayState::LocalDirect,
    });

    assert_eq!(
        decision.discovery_action_state,
        LanDiscoveryActionState::AdvertiseAndListen
    );
    assert_eq!(
        decision.pairing_action_state,
        LanPairingActionState::AllowSignedPairing
    );
}

#[test]
fn lan_discovery_unknown_peer_requires_review_not_pairing_authority() {
    let decision = evaluate_lan_discovery(LanDiscoveryInput {
        interface_state: LanInterfaceState::Available,
        peer_trust_state: LanPeerTrustState::Unknown,
        relay_state: LanRelayState::LocalDirect,
    });

    assert_eq!(
        decision.pairing_action_state,
        LanPairingActionState::RequireAiOrManualReview
    );
}

#[test]
fn lan_discovery_blocks_when_interface_is_unavailable() {
    let decision = evaluate_lan_discovery(LanDiscoveryInput {
        interface_state: LanInterfaceState::Unavailable,
        peer_trust_state: LanPeerTrustState::Trusted,
        relay_state: LanRelayState::RelayRequired,
    });

    assert_eq!(
        decision.discovery_action_state,
        LanDiscoveryActionState::ManualRequired
    );
    assert_eq!(decision.pairing_action_state, LanPairingActionState::Block);
}

#[test]
fn lan_discovery_decision_is_recorded_as_typed_event() {
    let event = lan_discovery_decision_recorded_event(
        LanAggregateId::parse("lan-child-default").expect("lan aggregate"),
        LanDiscoveryDecisionId::parse("lan-discovery-decision-default")
            .expect("lan discovery decision"),
        LanDiscoveryInput {
            interface_state: LanInterfaceState::Available,
            peer_trust_state: LanPeerTrustState::Trusted,
            relay_state: LanRelayState::LocalDirect,
        },
    );

    assert_eq!(
        event.decision.discovery_action_state,
        LanDiscoveryActionState::AdvertiseAndListen
    );
    assert_eq!(
        event.contract().expect("lan contract").event_type.as_str(),
        "lan.discovery.decision-recorded"
    );
}
