use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAuditState, NetworkEvidenceScope, NetworkRiskBudgetState, NetworkRuntimeEvidenceGrade,
    NetworkRuntimePhase,
};

use crate::network_capture::NetworkObservation;

#[path = "network_event_runtime_state/helpers.rs"]
mod helpers;

pub(crate) fn evidence_scope(observation: &NetworkObservation) -> NetworkEvidenceScope {
    helpers::evidence_scope(observation)
}

pub(crate) fn evidence_grade(observation: &NetworkObservation) -> NetworkRuntimeEvidenceGrade {
    helpers::evidence_grade(observation)
}

pub(crate) fn ai_audit_state(phase: NetworkRuntimePhase) -> NetworkAiAuditState {
    helpers::ai_audit_state(phase)
}

pub(crate) fn risk_budget_state(observation: &NetworkObservation) -> NetworkRiskBudgetState {
    helpers::risk_budget_state(observation)
}
