use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenActionState, ScreenAiAuditState, ScreenDeletionState, ScreenEvidenceScope,
    ScreenPolicyState, ScreenRuntimePhase,
};

#[path = "screen_event_runtime_state_action.rs"]
mod screen_event_runtime_state_action;
#[path = "screen_event_runtime_state_audit.rs"]
mod screen_event_runtime_state_audit;
#[path = "screen_event_runtime_state_custody.rs"]
mod screen_event_runtime_state_custody;
#[path = "screen_event_runtime_state_deletion.rs"]
mod screen_event_runtime_state_deletion;
#[path = "screen_event_runtime_state_policy.rs"]
mod screen_event_runtime_state_policy;
#[path = "screen_event_runtime_state_scope.rs"]
mod screen_event_runtime_state_scope;

pub(crate) fn evidence_scope(phase: ScreenRuntimePhase) -> ScreenEvidenceScope {
    screen_event_runtime_state_scope::evidence_scope(phase)
}

pub(crate) fn ai_audit_state(phase: ScreenRuntimePhase) -> ScreenAiAuditState {
    screen_event_runtime_state_audit::ai_audit_state(phase)
}

pub(crate) fn policy_state(phase: ScreenRuntimePhase) -> ScreenPolicyState {
    screen_event_runtime_state_policy::policy_state(phase)
}

pub(crate) fn action_state(phase: ScreenRuntimePhase) -> ScreenActionState {
    screen_event_runtime_state_action::action_state(phase)
}

pub(crate) fn deletion_state(phase: ScreenRuntimePhase) -> ScreenDeletionState {
    screen_event_runtime_state_deletion::deletion_state(phase)
}

pub(crate) fn custody_state(phase: ScreenRuntimePhase) -> &'static str {
    screen_event_runtime_state_custody::custody_state(phase)
}
