use ocentra_parent_agent_protocol::{constants, NetworkRuntimePhase};

use crate::NetworkObservation;

use super::{network_correlation_id, should_publish_phase};

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
    ) -> Self {
        Self {
            previous_phase_ref: previous_phase_ref(phase, observation, observed_at),
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
            ),
            ai_analysis_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::AiAnalysisCompleted,
                observation,
                observed_at,
            ),
            policy_evaluation_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::PolicyEvaluationRequested,
                observation,
                observed_at,
            ),
            policy_decision_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::PolicyDecisionCompleted,
                observation,
                observed_at,
            ),
            adapter_capability_ref: enforcement_reached(phase, observation)
                .then(|| network_adapter_capability_ref(observation, observed_at)),
            enforcement_command_ref: reached_enforcement_phase_ref(
                phase,
                NetworkRuntimePhase::EnforcementCommandIssued,
                observation,
                observed_at,
            ),
            enforcement_result_ref: reached_enforcement_phase_ref(
                phase,
                NetworkRuntimePhase::EnforcementResultObserved,
                observation,
                observed_at,
            ),
            audit_entry_ref: reached_phase_ref(
                phase,
                NetworkRuntimePhase::AuditEntryCommitted,
                observation,
                observed_at,
            ),
        }
    }
}

fn previous_phase_ref(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
) -> Option<String> {
    previous_published_phase(phase, observation)
        .map(|previous| network_phase_ref(observation, observed_at, previous))
}

fn reached_phase_ref(
    phase: NetworkRuntimePhase,
    threshold: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
) -> Option<String> {
    phase_reaches(phase, threshold).then(|| network_phase_ref(observation, observed_at, threshold))
}

fn reached_enforcement_phase_ref(
    phase: NetworkRuntimePhase,
    threshold: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
) -> Option<String> {
    (enforcement_reached(phase, observation) && phase_reaches(phase, threshold))
        .then(|| network_phase_ref(observation, observed_at, threshold))
}

fn enforcement_reached(phase: NetworkRuntimePhase, observation: &NetworkObservation) -> bool {
    should_publish_phase(NetworkRuntimePhase::EnforcementCommandIssued, observation)
        && phase_reaches(phase, NetworkRuntimePhase::EnforcementCommandIssued)
}

fn previous_published_phase(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
) -> Option<NetworkRuntimePhase> {
    NetworkRuntimePhase::ordered_chain()
        .iter()
        .copied()
        .filter(|candidate| should_publish_phase(*candidate, observation))
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
