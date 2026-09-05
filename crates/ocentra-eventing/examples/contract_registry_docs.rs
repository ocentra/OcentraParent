use std::io::{self, Write};

use ocentra_eventing::{
    bus::reports::dead_letter::{DeadLetterEvent, DeadLetterReason},
    contract_registry::EventContractRegistry,
    envelope::EventSource,
    ids::{
        CorrelationId, EventCustody, EventId, EventType, RuntimeInstanceId, RuntimeRole,
        SourceComponent, SourceService,
    },
};

const EXAMPLE_ORIGINAL_EVENT_ID: &str = "eventing-example-original-1";
const EXAMPLE_ORIGINAL_EVENT_TYPE: &str = "eventing.example.original";
const EXAMPLE_CORRELATION_ID: &str = "eventing-example-correlation-1";
const EXAMPLE_CUSTODY: &str = "eventing-example-custody";
const EXAMPLE_ROLE: &str = "eventing-example-role";
const EXAMPLE_SERVICE: &str = "eventing-example-service";
const EXAMPLE_COMPONENT: &str = "eventing-example-component";
const EXAMPLE_INSTANCE: &str = "eventing-example-instance";

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
            SourceComponent::parse(EXAMPLE_COMPONENT)?,
            RuntimeInstanceId::parse(EXAMPLE_INSTANCE)?,
        ),
        reason: DeadLetterReason::NoSubscriber,
        retry_state:
            ocentra_eventing::bus::reports::dead_letter::DeadLetterRetryState::NotAttempted,
        subscriber_id: None,
        target_handler: None,
    };

    let mut registry = EventContractRegistry::new();
    registry.register_event(&event)?;
    io::stdout()
        .lock()
        .write_all(registry.render_markdown().as_str().as_bytes())?;
    Ok(())
}
