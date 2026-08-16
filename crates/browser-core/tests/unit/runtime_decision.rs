use ocentra_browser_core::runtime_decision::{
    browser_runtime_decision_recorded_event, browser_runtime_observed_event,
    evaluate_browser_runtime, BrowserAggregateId, BrowserAiHandoffState, BrowserCapabilityState,
    BrowserClassificationState, BrowserForegroundState, BrowserPolicyHandoffState,
    BrowserRuntimeActionState, BrowserRuntimeDecisionId, BrowserRuntimeInput,
};
use ocentra_browser_core::{
    browser_ai_analysis_requested_event, browser_evidence_recorded_event,
    browser_policy_evaluation_requested_event, BrowserObservationIntent,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_evidence_ref_from_observation_id, child_domain_observation_id_from_subject_ref,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainEvidenceRecordedEvent, ChildRuntimeDomain,
};

#[test]
fn foreground_known_policy_navigation_publishes_policy_without_ai() {
    let input = BrowserRuntimeInput {
        capability_state: BrowserCapabilityState::Supported,
        foreground_state: BrowserForegroundState::Foreground,
        classification_state: BrowserClassificationState::KnownPolicyNavigation,
    };

    let decision = evaluate_browser_runtime(input);
    let observed = browser_runtime_observed_event(input);
    let evidence = browser_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        BrowserObservationIntent::KnownPolicyNavigationRequiresPolicy
    );
    assert_eq!(
        decision.runtime_action_state,
        BrowserRuntimeActionState::RecordForegroundNavigation
    );
    assert_eq!(screened_ai(&evidence), None);
    assert_eq!(
        browser_policy_evaluation_requested_event(&evidence)
            .expect_value("known navigation should publish policy evidence")
            .evidence_refs,
        vec![evidence.evidence_ref]
    );
    assert_eq!(
        observed.observation_id,
        child_domain_observation_id_from_subject_ref(
            ChildRuntimeDomain::Browser,
            &observed.subject_ref,
            &observed.observed_state
        )
    );
}

#[test]
fn foreground_ambiguous_navigation_routes_to_ai_boundary() {
    let input = BrowserRuntimeInput {
        capability_state: BrowserCapabilityState::Supported,
        foreground_state: BrowserForegroundState::Foreground,
        classification_state: BrowserClassificationState::AmbiguousNavigation,
    };

    let decision = evaluate_browser_runtime(input);
    let observed = browser_runtime_observed_event(input);
    let evidence = browser_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        BrowserObservationIntent::AmbiguousNavigationRequiresAi
    );
    assert_eq!(
        decision.runtime_action_state,
        BrowserRuntimeActionState::RecordForegroundNavigation
    );
    assert_eq!(
        screened_ai(&evidence)
            .expect_value("ambiguous navigation should request AI")
            .evidence_refs,
        vec![evidence.evidence_ref.clone()]
    );
    assert_eq!(browser_policy_evaluation_requested_event(&evidence), None);
}

#[test]
fn missing_browser_capability_forces_manual_review_without_handoffs() {
    let input = BrowserRuntimeInput {
        capability_state: BrowserCapabilityState::Missing,
        foreground_state: BrowserForegroundState::Foreground,
        classification_state: BrowserClassificationState::KnownPolicyNavigation,
    };

    let decision = evaluate_browser_runtime(input);
    let observed = browser_runtime_observed_event(input);
    let evidence = browser_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        BrowserObservationIntent::InventoryObservationOnly
    );
    assert_eq!(
        decision.runtime_action_state,
        BrowserRuntimeActionState::ManualRequired
    );
    assert_eq!(screened_ai(&evidence), None);
    assert_eq!(browser_policy_evaluation_requested_event(&evidence), None);
}

#[test]
fn background_inventory_navigation_records_typed_runtime_decision() {
    let input = BrowserRuntimeInput {
        capability_state: BrowserCapabilityState::Supported,
        foreground_state: BrowserForegroundState::Background,
        classification_state: BrowserClassificationState::InventoryOnly,
    };
    let event = browser_runtime_decision_recorded_event(
        BrowserAggregateId::parse("browser.aggregate.1").expect_value("aggregate id"),
        BrowserRuntimeDecisionId::parse("browser.runtime-decision.1").expect_value("decision id"),
        input,
    );

    assert_eq!(
        event.decision.runtime_action_state,
        BrowserRuntimeActionState::RecordInventory
    );
    assert_eq!(
        event.decision.ai_handoff_state,
        BrowserAiHandoffState::NotRequired
    );
    assert_eq!(
        event.decision.policy_handoff_state,
        BrowserPolicyHandoffState::DoNotPublish
    );
}

#[test]
fn browser_runtime_observed_event_drives_derived_evidence_chain() {
    let observed = browser_runtime_observed_event(BrowserRuntimeInput {
        capability_state: BrowserCapabilityState::Supported,
        foreground_state: BrowserForegroundState::Foreground,
        classification_state: BrowserClassificationState::KnownPolicyNavigation,
    });
    let evidence = browser_evidence_recorded_event(&observed);

    assert_eq!(
        evidence.evidence_ref,
        child_domain_evidence_ref_from_observation_id(
            ChildRuntimeDomain::Browser,
            &observed.observation_id
        )
    );
}

fn screened_ai(
    evidence: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    browser_ai_analysis_requested_event(evidence)
}
