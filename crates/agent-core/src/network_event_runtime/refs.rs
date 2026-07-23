use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase;

use crate::NetworkObservation;

use ocentra_network_core::network_runtime::NetworkRuntimeDecision;

use super::{network_correlation_id, should_publish_phase_for_runtime_decision};

pub(super) struct NetworkRuntimeChainRefs {
    pub previous_phase_ref: Option<String>,
    pub evidence_ref: String,
    pub ai_request_ref: Option<String>,
    pub ai_analysis_ref: Option<String>,
    pub policy_evaluation_ref: Option<String>,
    pub policy_decision_ref: Option<String>,
    pub adapter_capability_ref: Option<String>,
    pub enforcement_command_ref: Option<String>,
    pub enforcement_result_ref: Option<String>,
    pub audit_entry_ref: Option<String>,
}

impl NetworkRuntimeChainRefs {
    pub(super) fn for_phase(
        phase: NetworkRuntimePhase,
        observation: &NetworkObservation,
        observed_at: &str,
        decision: &NetworkRuntimeDecision,
    ) -> Self {
        Self {
            previous_phase_ref: previous_phase_ref(phase, observation, observed_at, decision),
            evidence_ref: network_phase_ref(
                observation,
                observed_at,
                NetworkRuntimePhase::FlowObserved,
            ),
            ai_request_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::AiAnalysisRequested,
                observation,
                observed_at,
                decision,
            ),
            ai_analysis_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::AiAnalysisCompleted,
                observation,
                observed_at,
                decision,
            ),
            policy_evaluation_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::PolicyEvaluationRequested,
                observation,
                observed_at,
                decision,
            ),
            policy_decision_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::PolicyDecisionCompleted,
                observation,
                observed_at,
                decision,
            ),
            adapter_capability_ref: enforcement_reached(phase, observation, decision)
                .then(|| network_adapter_capability_ref(observation, observed_at)),
            enforcement_command_ref: reached_enforcement_phase_ref(
                phase,
                NetworkRuntimePhase::EnforcementCommandIssued,
                observation,
                observed_at,
                decision,
            ),
            enforcement_result_ref: reached_enforcement_phase_ref(
                phase,
                NetworkRuntimePhase::EnforcementResultObserved,
                observation,
                observed_at,
                decision,
            ),
            audit_entry_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::AuditEntryCommitted,
                observation,
                observed_at,
                decision,
            ),
        }
    }
}

fn previous_phase_ref(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    decision: &NetworkRuntimeDecision,
) -> Option<String> {
    previous_published_phase(phase, observation, decision)
        .map(|previous| network_phase_ref(observation, observed_at, previous))
}

fn reached_phase_ref(
    phase: NetworkRuntimePhase,
    threshold: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    decision: &NetworkRuntimeDecision,
) -> Option<String> {
    (phase_reaches(phase, threshold)
        && should_publish_phase_for_runtime_decision(threshold, observation, decision))
    .then(|| network_phase_ref(observation, observed_at, threshold))
}

fn reached_enforcement_phase_ref(
    phase: NetworkRuntimePhase,
    threshold: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    decision: &NetworkRuntimeDecision,
) -> Option<String> {
    (enforcement_reached(phase, observation, decision) && phase_reaches(phase, threshold))
        .then(|| network_phase_ref(observation, observed_at, threshold))
}

fn enforcement_reached(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    decision: &NetworkRuntimeDecision,
) -> bool {
    should_publish_phase_for_runtime_decision(
        NetworkRuntimePhase::EnforcementCommandIssued,
        observation,
        decision,
    ) && phase_reaches(phase, NetworkRuntimePhase::EnforcementCommandIssued)
}

fn previous_published_phase(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    decision: &NetworkRuntimeDecision,
) -> Option<NetworkRuntimePhase> {
    NetworkRuntimePhase::ordered_chain()
        .iter()
        .copied()
        .filter(|candidate| {
            should_publish_phase_for_runtime_decision(*candidate, observation, decision)
        })
        .take_while(|candidate| *candidate != phase)
        .last()
}

fn phase_reaches(phase: NetworkRuntimePhase, threshold: NetworkRuntimePhase) -> bool {
    phase_index(phase) >= phase_index(threshold)
}

fn phase_index(phase: NetworkRuntimePhase) -> usize {
    NetworkRuntimePhase::ordered_chain()
        .iter()
        .position(|candidate| *candidate == phase)
        .unwrap_or_default()
}

fn network_phase_ref(
    observation: &NetworkObservation,
    observed_at: &str,
    phase: NetworkRuntimePhase,
) -> String {
    let mut value = network_correlation_id(observation, observed_at);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(phase.event_type());
    value
}

fn network_adapter_capability_ref(observation: &NetworkObservation, observed_at: &str) -> String {
    let mut value = network_correlation_id(observation, observed_at);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(constants::network_flow::TARGET_ENFORCEMENT_DRY_RUN);
    value
}
