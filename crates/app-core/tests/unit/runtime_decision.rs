use ocentra_app_core::runtime_decision::{
    app_runtime_observed_event, evaluate_app_runtime, AppAiHandoffState, AppCapabilityState,
    AppClassificationState, AppForegroundState, AppPolicyHandoffState, AppRuntimeActionState,
    AppRuntimeInput,
};
use ocentra_app_core::{app_evidence_recorded_event, AppObservationIntent};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_evidence_ref_from_observation_id, child_domain_observation_id_from_subject_ref,
    ChildRuntimeDomain,
};

#[test]
fn foreground_known_policy_app_publishes_policy_without_ai() {
    let decision = evaluate_app_runtime(AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Foreground,
        classification_state: AppClassificationState::KnownPolicyApp,
    });

    assert_eq!(
        decision.observation_intent,
        AppObservationIntent::ForegroundAppRequiresPolicy
    );
    assert_eq!(
        decision.runtime_action_state,
        AppRuntimeActionState::RecordForeground
    );
    assert_eq!(decision.ai_handoff_state, AppAiHandoffState::NotRequired);
    assert_eq!(
        decision.policy_handoff_state,
        AppPolicyHandoffState::Publish
    );
}

#[test]
fn foreground_unknown_app_routes_to_ai_boundary() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Foreground,
        classification_state: AppClassificationState::UnknownApp,
    };
    let decision = evaluate_app_runtime(input);
    let observed = app_runtime_observed_event(input);

    assert_eq!(
        decision.observation_intent,
        AppObservationIntent::UnknownAppRequiresAi
    );
    assert_eq!(
        decision.runtime_action_state,
        AppRuntimeActionState::RecordForeground
    );
    assert_eq!(decision.ai_handoff_state, AppAiHandoffState::Required);
    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::App.observed_event_type()
    );
    assert_eq!(
        observed.observation_id,
        child_domain_observation_id_from_subject_ref(
            ChildRuntimeDomain::App,
            &observed.subject_ref,
            &observed.observed_state
        )
    );
}

#[test]
fn missing_capability_forces_manual_review_without_handoffs() {
    let decision = evaluate_app_runtime(AppRuntimeInput {
        capability_state: AppCapabilityState::Missing,
        foreground_state: AppForegroundState::Foreground,
        classification_state: AppClassificationState::KnownPolicyApp,
    });

    assert_eq!(
        decision.observation_intent,
        AppObservationIntent::InventoryObservationOnly
    );
    assert_eq!(
        decision.runtime_action_state,
        AppRuntimeActionState::ManualRequired
    );
    assert_eq!(decision.ai_handoff_state, AppAiHandoffState::NotRequired);
    assert_eq!(
        decision.policy_handoff_state,
        AppPolicyHandoffState::DoNotPublish
    );
}

#[test]
fn app_runtime_observed_event_drives_derived_evidence_chain() {
    let observed = app_runtime_observed_event(AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Foreground,
        classification_state: AppClassificationState::KnownPolicyApp,
    });
    let evidence = app_evidence_recorded_event(&observed);

    assert_eq!(
        evidence.evidence_ref,
        child_domain_evidence_ref_from_observation_id(
            ChildRuntimeDomain::App,
            &observed.observation_id
        )
    );
}
