#[path = "event_kind/flow.rs"]
mod flow;
#[path = "event_kind/governance.rs"]
mod governance;

use ocentra_eventing::ids::EventType;

#[derive(Clone, Copy)]
pub(super) enum NetworkRuntimeStreamEventKind {
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

pub(super) fn from_event_type(event_type: &EventType) -> Option<NetworkRuntimeStreamEventKind> {
    flow::from_event_type(event_type).or_else(|| governance::from_event_type(event_type))
}
