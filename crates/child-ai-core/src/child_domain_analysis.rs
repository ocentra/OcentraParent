use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_analysis_completed_event, ChildDomainAiAnalysisCompletedEvent,
    ChildDomainAiAnalysisRequestedEvent,
};

pub fn complete_child_domain_ai_analysis(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> ChildDomainAiAnalysisCompletedEvent {
    child_domain_ai_analysis_completed_event(event)
}
