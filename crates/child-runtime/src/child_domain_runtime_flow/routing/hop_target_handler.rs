use super::ChildDomainRuntimeHop;
use ocentra_parent_agent_protocol::constants;

impl<'a> ChildDomainRuntimeHop<'a> {
    pub(super) fn target_handler(self) -> &'static str {
        match self {
            Self::Observed(_) | Self::EvidenceRecorded(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER
            }
            Self::AiAnalysisRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_AI_ANALYZER
            }
            Self::AiAnalysisCompleted(_) | Self::PolicyEvaluationRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR
            }
            Self::PolicyEvaluationRequestedFromAi(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR
            }
            Self::PolicyViolationDetected(_) | Self::NotificationRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_BRIDGE
            }
        }
    }
}
