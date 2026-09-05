use std::io::{self, Write};

use ocentra_eventing::{
    bus::reports::dead_letter::{DeadLetterEvent, DeadLetterReason},
    contract_registry::EventContractRegistry,
    envelope::EventSource,
    ids::{
        CorrelationId, EventCustody, EventId, EventNamespace, EventType, RuntimeInstanceId,
        RuntimeRole, SourceComponent, SourceService, SubscriberId, TargetHandler,
    },
    topology::{
        EventTopologyFamilyVariant, EventTopologyManifest, EventTopologyPublisher,
        EventTopologySubscriber,
    },
};

const EXAMPLE_ORIGINAL_EVENT_ID: &str = "eventing-topology-original-1";
const EXAMPLE_ORIGINAL_EVENT_TYPE: &str = "eventing.topology.original";
const EXAMPLE_CORRELATION_ID: &str = "eventing-topology-correlation-1";
const EXAMPLE_PUBLISHER: &str = "eventing-topology-example-publisher";
const EXAMPLE_SUBSCRIBER: &str = "eventing-topology-example-subscriber";
const EXAMPLE_TARGET: &str = "eventing-topology-example-target";
const EXAMPLE_FAMILY: &str = "eventing.topology.example-family";
const EXAMPLE_CUSTODY: &str = "eventing-topology-custody";
const EXAMPLE_ROLE: &str = "eventing-topology-role";
const EXAMPLE_SERVICE: &str = "eventing-topology-service";
const EXAMPLE_INSTANCE: &str = "eventing-topology-instance";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let custody = EventCustody::parse(EXAMPLE_CUSTODY)?;
    let event = DeadLetterEvent {
        original_event_id: EventId::parse(EXAMPLE_ORIGINAL_EVENT_ID)?,
        original_event_type: EventType::parse(EXAMPLE_ORIGINAL_EVENT_TYPE)?,
        original_correlation_id: CorrelationId::parse(EXAMPLE_CORRELATION_ID)?,
        original_causation_id: None,
        custody: custody.clone(),
        source: EventSource::new(
            custody,
            RuntimeRole::parse(EXAMPLE_ROLE)?,
            SourceService::parse(EXAMPLE_SERVICE)?,
            SourceComponent::parse(EXAMPLE_PUBLISHER)?,
            RuntimeInstanceId::parse(EXAMPLE_INSTANCE)?,
        ),
        reason: DeadLetterReason::NoSubscriber,
        retry_state:
            ocentra_eventing::bus::reports::dead_letter::DeadLetterRetryState::NotAttempted,
        subscriber_id: None,
        target_handler: None,
    };

    let mut registry = EventContractRegistry::new();
    let event_type = registry.register_event(&event)?.event_type().clone();
    let manifest = EventTopologyManifest::from_registry(
        &registry,
        &[EventTopologyPublisher {
            event_type: event_type.clone(),
            source_component: SourceComponent::parse(EXAMPLE_PUBLISHER)?,
        }],
        &[EventTopologySubscriber {
            event_type: event_type.clone(),
            subscriber_id: SubscriberId::parse(EXAMPLE_SUBSCRIBER)?,
            target_handler: TargetHandler::parse(EXAMPLE_TARGET)?,
        }],
        &[EventTopologyFamilyVariant {
            family: EventNamespace::parse(EXAMPLE_FAMILY)?,
            event_type,
        }],
        &[],
    );
    io::stdout()
        .lock()
        .write_all(manifest.render_markdown().as_bytes())?;
    Ok(())
}
