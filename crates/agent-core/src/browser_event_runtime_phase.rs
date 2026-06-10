use ocentra_eventing::RuntimeRole;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

const BROWSER_RUNTIME_PHASES: [BrowserRuntimePhase; 10] = [
    BrowserRuntimePhase::EvidenceObserved,
    BrowserRuntimePhase::EvidenceJournaled,
    BrowserRuntimePhase::AiAnalysisRequested,
    BrowserRuntimePhase::AiAnalysisCompleted,
    BrowserRuntimePhase::PolicyEvaluationRequested,
    BrowserRuntimePhase::PolicyDecisionCompleted,
    BrowserRuntimePhase::InterventionCommandIssued,
    BrowserRuntimePhase::InterventionResultObserved,
    BrowserRuntimePhase::AuditEntryCommitted,
    BrowserRuntimePhase::ReadModelProjected,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserRuntimePhase {
    EvidenceObserved,
    EvidenceJournaled,
    AiAnalysisRequested,
    AiAnalysisCompleted,
    PolicyEvaluationRequested,
    PolicyDecisionCompleted,
    InterventionCommandIssued,
    InterventionResultObserved,
    AuditEntryCommitted,
    ReadModelProjected,
}

impl BrowserRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &BROWSER_RUNTIME_PHASES
    }

    pub(crate) fn event_type(self) -> &'static str {
        match self {
            Self::EvidenceObserved => constants::browser::EVENT_BROWSER_EVIDENCE_OBSERVED,
            Self::EvidenceJournaled => constants::browser::EVENT_BROWSER_EVIDENCE_JOURNALED,
            Self::AiAnalysisRequested => constants::browser::EVENT_BROWSER_AI_ANALYSIS_REQUESTED,
            Self::AiAnalysisCompleted => constants::browser::EVENT_BROWSER_AI_ANALYSIS_COMPLETED,
            Self::PolicyEvaluationRequested => {
                constants::browser::EVENT_BROWSER_POLICY_EVALUATION_REQUESTED
            }
            Self::PolicyDecisionCompleted => {
                constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED
            }
            Self::InterventionCommandIssued => {
                constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED
            }
            Self::InterventionResultObserved => {
                constants::browser::EVENT_BROWSER_INTERVENTION_RESULT_OBSERVED
            }
            Self::AuditEntryCommitted => constants::browser::EVENT_BROWSER_AUDIT_ENTRY_COMMITTED,
            Self::ReadModelProjected => constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED,
        }
    }

    pub(crate) fn subscriber_id(self) -> &'static str {
        match self {
            Self::EvidenceObserved => constants::browser::SUBSCRIBER_BROWSER_EVIDENCE_OBSERVER,
            Self::EvidenceJournaled => constants::browser::SUBSCRIBER_BROWSER_EVIDENCE_JOURNAL,
            Self::AiAnalysisRequested => constants::browser::SUBSCRIBER_BROWSER_AI_REQUEST,
            Self::AiAnalysisCompleted => constants::browser::SUBSCRIBER_BROWSER_AI_COMPLETE,
            Self::PolicyEvaluationRequested => {
                constants::browser::SUBSCRIBER_BROWSER_POLICY_REQUEST
            }
            Self::PolicyDecisionCompleted => constants::browser::SUBSCRIBER_BROWSER_POLICY_DECISION,
            Self::InterventionCommandIssued => {
                constants::browser::SUBSCRIBER_BROWSER_INTERVENTION_COMMAND
            }
            Self::InterventionResultObserved => {
                constants::browser::SUBSCRIBER_BROWSER_INTERVENTION_RESULT
            }
            Self::AuditEntryCommitted => constants::browser::SUBSCRIBER_BROWSER_AUDIT_ENTRY,
            Self::ReadModelProjected => constants::browser::SUBSCRIBER_BROWSER_READ_MODEL,
        }
    }

    pub(crate) fn target_handler(self) -> &'static str {
        match self {
            Self::EvidenceObserved => constants::browser::TARGET_BROWSER_EVIDENCE_OBSERVER,
            Self::EvidenceJournaled => constants::browser::TARGET_BROWSER_EVIDENCE_JOURNAL,
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::browser::TARGET_BROWSER_AI_ANALYZER
            }
            Self::PolicyEvaluationRequested | Self::PolicyDecisionCompleted => {
                constants::browser::TARGET_BROWSER_POLICY_ENGINE
            }
            Self::InterventionCommandIssued | Self::InterventionResultObserved => {
                constants::browser::TARGET_BROWSER_INTERVENTION_ADAPTER
            }
            Self::AuditEntryCommitted => constants::browser::TARGET_BROWSER_AUDIT_WRITER,
            Self::ReadModelProjected => constants::browser::TARGET_BROWSER_READ_MODEL,
        }
    }

    pub(crate) fn runtime_role(self) -> RuntimeRole {
        let value = match self {
            Self::EvidenceObserved | Self::EvidenceJournaled => {
                constants::eventing_source::ROLE_AGENT
            }
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::eventing_source::ROLE_ANALYZER
            }
            Self::PolicyEvaluationRequested | Self::PolicyDecisionCompleted => {
                constants::eventing_source::ROLE_DECISION_ENGINE
            }
            Self::InterventionCommandIssued | Self::InterventionResultObserved => {
                constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER
            }
            Self::AuditEntryCommitted => constants::eventing_source::ROLE_AUDIT_WRITER,
            Self::ReadModelProjected => constants::eventing_source::ROLE_READ_MODEL,
        };
        RuntimeRole::parse(value)
            .expect(constants::eventing_source::ERROR_RUNTIME_ROLE_CONSTANT_PARSES)
    }
}
