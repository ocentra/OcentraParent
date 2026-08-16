use super::ChildDomainRuntimeHop;
use ocentra_parent_agent_protocol::constants;

impl<'a> ChildDomainRuntimeHop<'a> {
    pub(super) fn source_component(self) -> &'static str {
        match self {
            Self::Observed(_)
            | Self::EvidenceRecorded(_)
            | Self::AiAnalysisRequested(_)
            | Self::PolicyEvaluationRequested(_)
            | Self::PolicyEvaluationRequestedFromAi(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME
            }
            Self::AiAnalysisCompleted(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_AI_RUNTIME
            }
            Self::PolicyViolationDetected(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_POLICY_RUNTIME
            }
            Self::NotificationRequested(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME
            }
        }
    }

    pub(super) fn runtime_role(self) -> &'static str {
        match self {
            Self::Observed(_)
            | Self::EvidenceRecorded(_)
            | Self::AiAnalysisRequested(_)
            | Self::PolicyEvaluationRequested(_)
            | Self::PolicyEvaluationRequestedFromAi(_) => constants::eventing_source::ROLE_AGENT,
            Self::AiAnalysisCompleted(_) => constants::eventing_source::ROLE_ANALYZER,
            Self::PolicyViolationDetected(_) => constants::eventing_source::ROLE_DECISION_ENGINE,
            Self::NotificationRequested(_) => constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
        }
    }
}
