use ocentra_eventing::ids::EventType;
use ocentra_parent_agent_protocol::constants;

use super::NetworkRuntimeStreamEventKind;

pub(super) fn from_event_type(event_type: &EventType) -> Option<NetworkRuntimeStreamEventKind> {
    match event_type.as_str() {
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED => {
            Some(NetworkRuntimeStreamEventKind::FlowObserved)
        }
        constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED => {
            Some(NetworkRuntimeStreamEventKind::DomainObserved)
        }
        constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED => {
            Some(NetworkRuntimeStreamEventKind::ActivityClassified)
        }
        constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED => {
            Some(NetworkRuntimeStreamEventKind::AiAnalysisRequested)
        }
        constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED => {
            Some(NetworkRuntimeStreamEventKind::AiAnalysisCompleted)
        }
        _ => None,
    }
}
