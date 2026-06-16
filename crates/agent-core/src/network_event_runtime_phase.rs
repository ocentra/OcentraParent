use ocentra_eventing::ids::RuntimeRole;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

const NETWORK_RUNTIME_PHASES: [NetworkRuntimePhase; 11] = [
    NetworkRuntimePhase::FlowObserved,
    NetworkRuntimePhase::DomainObserved,
    NetworkRuntimePhase::ActivityClassified,
    NetworkRuntimePhase::AiAnalysisRequested,
    NetworkRuntimePhase::AiAnalysisCompleted,
    NetworkRuntimePhase::PolicyEvaluationRequested,
    NetworkRuntimePhase::PolicyDecisionCompleted,
    NetworkRuntimePhase::EnforcementCommandIssued,
    NetworkRuntimePhase::EnforcementResultObserved,
    NetworkRuntimePhase::AuditEntryCommitted,
    NetworkRuntimePhase::PortalReadModelUpdated,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRuntimePhase {
    FlowObserved,
    DomainObserved,
    ActivityClassified,
    AiAnalysisRequested,
    AiAnalysisCompleted,
    PolicyEvaluationRequested,
    PolicyDecisionCompleted,
    EnforcementCommandIssued,
    EnforcementResultObserved,
    AuditEntryCommitted,
    PortalReadModelUpdated,
}

impl NetworkRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &NETWORK_RUNTIME_PHASES
    }

    pub(crate) fn event_type(self) -> &'static str {
        match self {
            Self::FlowObserved => constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            Self::DomainObserved => constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED,
            Self::ActivityClassified => constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
            Self::AiAnalysisRequested => constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED,
            Self::AiAnalysisCompleted => constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED,
            Self::PolicyEvaluationRequested => {
                constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED
            }
            Self::PolicyDecisionCompleted => {
                constants::network_flow::EVENT_POLICY_DECISION_COMPLETED
            }
            Self::EnforcementCommandIssued => {
                constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
            }
            Self::EnforcementResultObserved => {
                constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
            }
            Self::AuditEntryCommitted => constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED,
            Self::PortalReadModelUpdated => {
                constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED
            }
        }
    }

    pub(crate) fn subscriber_id(self) -> &'static str {
        match self {
            Self::FlowObserved => constants::network_flow::SUBSCRIBER_NETWORK_OBSERVER,
            Self::DomainObserved => constants::network_flow::SUBSCRIBER_DOMAIN_OBSERVER,
            Self::ActivityClassified => constants::network_flow::SUBSCRIBER_ACTIVITY_CLASSIFIER,
            Self::AiAnalysisRequested => constants::network_flow::SUBSCRIBER_AI_REQUEST,
            Self::AiAnalysisCompleted => constants::network_flow::SUBSCRIBER_AI_COMPLETE,
            Self::PolicyEvaluationRequested => constants::network_flow::SUBSCRIBER_POLICY_REQUEST,
            Self::PolicyDecisionCompleted => constants::network_flow::SUBSCRIBER_POLICY_DECISION,
            Self::EnforcementCommandIssued => {
                constants::network_flow::SUBSCRIBER_ENFORCEMENT_COMMAND
            }
            Self::EnforcementResultObserved => {
                constants::network_flow::SUBSCRIBER_ENFORCEMENT_RESULT
            }
            Self::AuditEntryCommitted => constants::network_flow::SUBSCRIBER_AUDIT_ENTRY,
            Self::PortalReadModelUpdated => constants::network_flow::SUBSCRIBER_PORTAL_READ_MODEL,
        }
    }

    pub(crate) fn target_handler(self) -> &'static str {
        match self {
            Self::FlowObserved => constants::network_flow::TARGET_NETWORK_OBSERVER,
            Self::DomainObserved => constants::network_flow::TARGET_DOMAIN_OBSERVER,
            Self::ActivityClassified => constants::network_flow::TARGET_ACTIVITY_CLASSIFIER,
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::network_flow::TARGET_AI_ANALYZER
            }
            Self::PolicyEvaluationRequested | Self::PolicyDecisionCompleted => {
                constants::network_flow::TARGET_POLICY_ENGINE
            }
            Self::EnforcementCommandIssued | Self::EnforcementResultObserved => {
                constants::network_flow::TARGET_ENFORCEMENT_DRY_RUN
            }
            Self::AuditEntryCommitted => constants::network_flow::TARGET_AUDIT_WRITER,
            Self::PortalReadModelUpdated => constants::network_flow::TARGET_PORTAL_READ_MODEL,
        }
    }

    pub(crate) fn runtime_role(self) -> RuntimeRole {
        let value = match self {
            Self::FlowObserved | Self::DomainObserved | Self::ActivityClassified => {
                constants::eventing_source::ROLE_AGENT
            }
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::eventing_source::ROLE_ANALYZER
            }
            Self::PolicyEvaluationRequested | Self::PolicyDecisionCompleted => {
                constants::eventing_source::ROLE_DECISION_ENGINE
            }
            Self::EnforcementCommandIssued | Self::EnforcementResultObserved => {
                constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER
            }
            Self::AuditEntryCommitted => constants::eventing_source::ROLE_AUDIT_WRITER,
            Self::PortalReadModelUpdated => constants::eventing_source::ROLE_READ_MODEL,
        };
        match RuntimeRole::parse(value) {
            Ok(role) => role,
            Err(_) => std::process::abort(),
        }
    }
}
