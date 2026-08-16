use super::*;

pub fn evaluate_browser_runtime(input: BrowserRuntimeInput) -> BrowserRuntimeDecision {
    if input.capability_state == BrowserCapabilityState::Missing {
        return inventory_only_manual_required_decision();
    }

    if input.foreground_state != BrowserForegroundState::Foreground {
        return inventory_only_record_decision();
    }

    foreground_navigation_decision_for(input.classification_state)
}

fn inventory_only_manual_required_decision() -> BrowserRuntimeDecision {
    BrowserRuntimeDecision {
        observation_intent: BrowserObservationIntent::InventoryObservationOnly,
        runtime_action_state: BrowserRuntimeActionState::ManualRequired,
        ai_handoff_state: BrowserAiHandoffState::NotRequired,
        policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
    }
}

fn inventory_only_record_decision() -> BrowserRuntimeDecision {
    BrowserRuntimeDecision {
        observation_intent: BrowserObservationIntent::InventoryObservationOnly,
        runtime_action_state: BrowserRuntimeActionState::RecordInventory,
        ai_handoff_state: BrowserAiHandoffState::NotRequired,
        policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
    }
}

fn foreground_navigation_decision_for(
    classification_state: BrowserClassificationState,
) -> BrowserRuntimeDecision {
    match classification_state {
        BrowserClassificationState::KnownPolicyNavigation => BrowserRuntimeDecision {
            observation_intent: BrowserObservationIntent::KnownPolicyNavigationRequiresPolicy,
            runtime_action_state: BrowserRuntimeActionState::RecordForegroundNavigation,
            ai_handoff_state: BrowserAiHandoffState::NotRequired,
            policy_handoff_state: BrowserPolicyHandoffState::Publish,
        },
        BrowserClassificationState::AmbiguousNavigation => BrowserRuntimeDecision {
            observation_intent: BrowserObservationIntent::AmbiguousNavigationRequiresAi,
            runtime_action_state: BrowserRuntimeActionState::RecordForegroundNavigation,
            ai_handoff_state: BrowserAiHandoffState::Required,
            policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
        },
        BrowserClassificationState::InventoryOnly => BrowserRuntimeDecision {
            observation_intent: BrowserObservationIntent::InventoryObservationOnly,
            runtime_action_state: BrowserRuntimeActionState::RecordForegroundNavigation,
            ai_handoff_state: BrowserAiHandoffState::NotRequired,
            policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
        },
    }
}

pub fn browser_runtime_observed_event(input: BrowserRuntimeInput) -> ChildDomainObservedEvent {
    browser_observed_event(evaluate_browser_runtime(input).observation_intent)
}

pub fn browser_runtime_decision_recorded_event(
    aggregate_id: BrowserAggregateId,
    decision_id: BrowserRuntimeDecisionId,
    input: BrowserRuntimeInput,
) -> BrowserRuntimeDecisionRecordedEvent {
    BrowserRuntimeDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_browser_runtime(input),
    }
}
