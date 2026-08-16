use ocentra_eventing::ids::EventType;
use ocentra_parent_agent_protocol::constants;

use super::NetworkRuntimeStreamEventKind;

pub(super) fn from_event_type(event_type: &EventType) -> Option<NetworkRuntimeStreamEventKind> {
    match event_type.as_str() {
        constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED => {
            Some(NetworkRuntimeStreamEventKind::PolicyEvaluationRequested)
        }
        constants::network_flow::EVENT_POLICY_DECISION_COMPLETED => {
            Some(NetworkRuntimeStreamEventKind::PolicyDecisionCompleted)
        }
        constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED => {
            Some(NetworkRuntimeStreamEventKind::EnforcementCommandIssued)
        }
        constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED => {
            Some(NetworkRuntimeStreamEventKind::EnforcementResultObserved)
        }
        constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED => {
            Some(NetworkRuntimeStreamEventKind::AuditEntryCommitted)
        }
        constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED => {
            Some(NetworkRuntimeStreamEventKind::PortalReadModelUpdated)
        }
        _ => None,
    }
}
