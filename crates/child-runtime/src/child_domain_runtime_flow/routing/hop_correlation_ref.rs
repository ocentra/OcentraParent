use super::ChildDomainRuntimeHop;

impl<'a> ChildDomainRuntimeHop<'a> {
    pub(super) fn correlation_ref(self) -> &'a str {
        match self {
            Self::Observed(value) => value.as_str(),
            Self::EvidenceRecorded(value)
            | Self::AiAnalysisRequested(value)
            | Self::PolicyEvaluationRequested(value) => value.as_str(),
            Self::AiAnalysisCompleted(value) => value.as_str(),
            Self::PolicyEvaluationRequestedFromAi(value) => value.as_str(),
            Self::PolicyViolationDetected(value) => value.as_str(),
            Self::NotificationRequested(value) => value.as_str(),
        }
    }
}
