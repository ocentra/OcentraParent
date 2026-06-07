use ocentra_eventing::{
    EventContractRegistry, EventTopologyManifest, EventTopologyPublisher, EventTopologySubscriber,
    EventingError, SourceComponent, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::constants;

use crate::{BrowserRuntimeEventPayload, BrowserRuntimeInput, BrowserRuntimePhase};

pub fn browser_runtime_chain_topology_manifest() -> Result<EventTopologyManifest, EventingError> {
    let input = BrowserRuntimeInput::managed_decision_fixture();
    let mut registry = EventContractRegistry::new();
    let mut publishers = Vec::new();
    let mut subscribers = Vec::new();
    for phase in BrowserRuntimePhase::ordered_chain() {
        let payload = BrowserRuntimeEventPayload::from_input(*phase, &input);
        let event_type = registry.register_event(&payload)?.event_type().clone();
        publishers.push(EventTopologyPublisher {
            event_type: event_type.clone(),
            source_component: SourceComponent::parse(
                constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
            )?,
        });
        subscribers.push(EventTopologySubscriber {
            event_type,
            subscriber_id: SubscriberId::parse(phase.subscriber_id())?,
            target_handler: TargetHandler::parse(phase.target_handler())?,
        });
    }
    Ok(EventTopologyManifest::from_registry(
        &registry,
        &publishers,
        &subscribers,
        &[],
        &[],
    ))
}
