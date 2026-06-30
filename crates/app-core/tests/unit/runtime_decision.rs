use ocentra_app_core::runtime_decision::{
    app_runtime_decision_recorded_event, app_runtime_observed_event, evaluate_app_runtime,
    AppAggregateId, AppAiHandoffState, AppCapabilityState, AppClassificationState,
    AppForegroundState, AppPolicyHandoffState, AppRuntimeActionState, AppRuntimeDecisionId,
    AppRuntimeInput,
};
use ocentra_app_core::{app_evidence_recorded_event, AppObservationIntent};
use ocentra_eventing::envelope::DomainEvent;
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
fn background_inventory_state_records_decision_event_with_typed_contract() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Background,
        classification_state: AppClassificationState::InventoryOnly,
    };
    let aggregate_id_result = AppAggregateId::parse("app.aggregate.child-device-1");
    assert!(
        aggregate_id_result.is_ok(),
        "aggregate id parses: {aggregate_id_result:?}"
    );
    let Ok(aggregate_id) = aggregate_id_result else {
        return;
    };
    let decision_id_result = AppRuntimeDecisionId::parse("app.runtime-decision-1");
    assert!(
        decision_id_result.is_ok(),
        "decision id parses: {decision_id_result:?}"
    );
    let Ok(decision_id) = decision_id_result else {
        return;
    };

    let recorded = app_runtime_decision_recorded_event(aggregate_id, decision_id, input);

    assert_eq!(
        recorded.decision.observation_intent,
        AppObservationIntent::InventoryObservationOnly
    );
    assert_eq!(
        recorded.decision.runtime_action_state,
        AppRuntimeActionState::RecordInventory
    );
    let contract_result = recorded.contract();
    assert!(
        contract_result.is_ok(),
        "app runtime contract parses: {contract_result:?}"
    );
    let Ok(contract) = contract_result else {
        return;
    };
    assert_eq!(
        contract.event_type.as_str(),
        "app.runtime.decision-recorded"
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
