use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType,
    ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent, ids::SourceService,
    ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::{
    ParentChildRuntimeEventPayload, ParentChildRuntimePhase, ParentChildRuntimeReport,
};

mod build;
mod refs;

use build::runtime_events_for_input;

pub(crate) type ParentChildRuntimeInput =
    ocentra_parent_agent_protocol::transport::parent_child_runtime_input::ParentChildRuntimeInput;

pub async fn publish_parent_child_runtime_for_validated_intent(
    input: ParentChildRuntimeInput,
) -> Result<ParentChildRuntimeReport, EventingError> {
    let spine = ParentChildRuntimeSpine::with_default_handlers().await?;
    spine.publish_validated_intent(input).await
}

struct ParentChildRuntimeSpine {
    bus: EventBus,
}

impl ParentChildRuntimeSpine {
    async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        for phase in ParentChildRuntimePhase::ordered_chain() {
            bus.subscribe::<ParentChildRuntimeEventPayload, _, _>(
                EventSubscriber::new(
                    SubscriberId::parse(phase.subscriber_id())?,
                    EventType::parse(phase.event_type())?,
                    TargetHandler::parse(phase.target_handler())?,
                ),
                |_| async { Ok(()) },
            )
            .await?;
        }
        Ok(Self { bus })
    }

    async fn publish_validated_intent(
        &self,
        input: ParentChildRuntimeInput,
    ) -> Result<ParentChildRuntimeReport, EventingError> {
        let mut reports = Vec::new();
        for payload in runtime_events_for_input(&input) {
            let phase = payload.phase();
            let metadata = parent_child_event_metadata(phase, &input)?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        Ok(ParentChildRuntimeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }
}

fn parent_child_event_metadata(
    phase: ParentChildRuntimePhase,
    input: &ParentChildRuntimeInput,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(refs::parent_child_correlation_id(input))?,
        parent_child_event_source(phase)?,
        RecordedAt::parse(&input.observed_at)?,
        Some(TargetHandler::parse(phase.target_handler())?),
    ))
}

fn parent_child_event_source(phase: ParentChildRuntimePhase) -> Result<EventSource, EventingError> {
    let component = if phase.is_child_agent_phase() {
        constants::child_agent::RUNTIME_COMPONENT_CHILD_AGENT
    } else {
        constants::parent_controller::RUNTIME_COMPONENT_PARENT_CHILD_SPINE
    };
    let instance = if phase.is_child_agent_phase() {
        constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT
    } else {
        constants::parent_controller::RUNTIME_INSTANCE_LOCAL_PARENT_CONTROLLER
    };

    Ok(EventSource::new(
        phase.custody()?,
        phase.runtime_role()?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(component)?,
        RuntimeInstanceId::parse(instance)?,
    ))
}
