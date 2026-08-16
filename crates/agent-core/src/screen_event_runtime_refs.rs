use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

use crate::screen_event_runtime_input::ScreenRuntimeInput;

#[path = "screen_event_runtime_refs/ai.rs"]
mod ai;
#[path = "screen_event_runtime_refs/chain.rs"]
mod chain;
#[path = "screen_event_runtime_refs/policy.rs"]
mod policy;
#[path = "screen_event_runtime_refs/portal.rs"]
mod portal;
#[path = "screen_event_runtime_refs/previous.rs"]
mod previous;
#[path = "screen_event_runtime_refs/queue.rs"]
mod queue;

pub(crate) fn screen_correlation_id(queue_job_id: &str) -> String {
    chain::screen_correlation_id(queue_job_id)
}

pub(crate) fn previous_phase_ref(phase: ScreenRuntimePhase) -> Option<String> {
    previous::previous_phase_ref(phase)
}

pub(crate) fn queue_event_ref(phase: ScreenRuntimePhase) -> Option<String> {
    queue::queue_event_ref(phase)
}

pub(crate) fn ai_request_ref(phase: ScreenRuntimePhase) -> Option<String> {
    ai::ai_request_ref(phase)
}

pub(crate) fn ai_result_ref(phase: ScreenRuntimePhase) -> Option<String> {
    ai::ai_result_ref(phase)
}

pub(crate) fn summary_ref(phase: ScreenRuntimePhase) -> Option<String> {
    policy::summary_ref(phase)
}

pub(crate) fn policy_decision_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    policy::policy_decision_ref(phase, input)
}

pub(crate) fn policy_action(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    policy::policy_action(phase, input)
}

pub(crate) fn parent_rule_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    policy::parent_rule_ref(phase, input)
}

pub(crate) fn action_ref(phase: ScreenRuntimePhase, input: &ScreenRuntimeInput) -> Option<String> {
    portal::action_ref(phase, input)
}

pub(crate) fn deletion_proof_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    portal::deletion_proof_ref(phase, input)
}

pub(crate) fn portal_read_model_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    portal::portal_read_model_ref(phase, input)
}
