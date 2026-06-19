use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::DomainEvent,
    envelope::EventContract, envelope::EventMetadata, envelope::EventSource, error::EventingError,
    ids::AggregateKey, ids::CorrelationId, ids::EventId, ids::EventType, ids::IdempotencyKey,
    ids::RecordedAt, ids::RuntimeInstanceId, ids::SchemaVersion, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentActionReceivedEvent, ParentChildCommandForwardRequestedEvent,
    ParentChildCommandForwardedEvent, ParentCommandValidatedEvent, ParentControllerActionKind,
    ParentControllerSource, ParentReadModelProjectedEvent,
};
use ocentra_parent_agent_protocol::{
    constants, ChildCapabilityStateUpdatedEvent, ChildCommandAcceptedEvent, ChildCommandKind,
    ChildCommandReceivedEvent, ChildRuntimeHealthUpdatedEvent,
};
use serde::{Deserialize, Serialize};

use crate::ParentChildRuntimePhase;

mod build;
mod refs;

use build::runtime_events_for_input;
use refs::{parent_child_aggregate_key, parent_child_idempotency_key};

#[derive(Clone, Debug, PartialEq)]
pub struct ParentChildRuntimeInput {
    pub parent_intent_ref: String,
    pub parent_profile_ref: String,
    pub device_ref: String,
    pub observed_at: String,
    pub action_kind: ParentControllerActionKind,
    pub source: ParentControllerSource,
    pub child_command_kind: ChildCommandKind,
}

impl ParentChildRuntimeInput {
    pub fn validated_review_fixture() -> Self {
        Self {
            parent_intent_ref: constants::parent_controller::TEST_PARENT_INTENT_REF.to_string(),
            parent_profile_ref: constants::parent_controller::TEST_PARENT_PROFILE_REF.to_string(),
            device_ref: constants::parent_controller::TEST_DEVICE_REF.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            action_kind: ParentControllerActionKind::Review,
            source: ParentControllerSource::PortalTypedIntent,
            child_command_kind: ChildCommandKind::ObserveNetwork,
        }
    }

    pub fn browser_action_intent_handoff_fixture() -> Self {
        Self {
            parent_intent_ref: constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
                .to_string(),
            child_command_kind: ChildCommandKind::BrowserActionIntentHandoff,
            ..Self::validated_review_fixture()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParentChildRuntimeEventPayload {
    ParentActionReceived(ParentActionReceivedEvent),
    ParentCommandValidated(ParentCommandValidatedEvent),
    ParentChildCommandForwardRequested(ParentChildCommandForwardRequestedEvent),
    ParentChildCommandForwarded(ParentChildCommandForwardedEvent),
    ChildCommandReceived(ChildCommandReceivedEvent),
    ChildCommandAccepted(ChildCommandAcceptedEvent),
    ChildCapabilityStateUpdated(ChildCapabilityStateUpdatedEvent),
    ChildRuntimeHealthUpdated(ChildRuntimeHealthUpdatedEvent),
    ParentReadModelProjected(ParentReadModelProjectedEvent),
}

impl ParentChildRuntimeEventPayload {
    pub fn phase(&self) -> ParentChildRuntimePhase {
        match self {
            Self::ParentActionReceived(_) => ParentChildRuntimePhase::ParentActionReceived,
            Self::ParentCommandValidated(_) => ParentChildRuntimePhase::ParentCommandValidated,
            Self::ParentChildCommandForwardRequested(_) => {
                ParentChildRuntimePhase::ParentChildCommandForwardRequested
            }
            Self::ParentChildCommandForwarded(_) => {
                ParentChildRuntimePhase::ParentChildCommandForwarded
            }
            Self::ChildCommandReceived(_) => ParentChildRuntimePhase::ChildCommandReceived,
            Self::ChildCommandAccepted(_) => ParentChildRuntimePhase::ChildCommandAccepted,
            Self::ChildCapabilityStateUpdated(_) => {
                ParentChildRuntimePhase::ChildCapabilityStateUpdated
            }
            Self::ChildRuntimeHealthUpdated(_) => {
                ParentChildRuntimePhase::ChildRuntimeHealthUpdated
            }
            Self::ParentReadModelProjected(_) => ParentChildRuntimePhase::ParentReadModelProjected,
        }
    }

    fn event_ref(&self) -> &str {
        match self {
            Self::ParentActionReceived(event) => &event.parent_action_event_ref,
            Self::ParentCommandValidated(event) => &event.command_validated_event_ref,
            Self::ParentChildCommandForwardRequested(event) => &event.forward_requested_event_ref,
            Self::ParentChildCommandForwarded(event) => &event.forwarded_event_ref,
            Self::ChildCommandReceived(event) => &event.command_received_event_ref,
            Self::ChildCommandAccepted(event) => &event.command_accepted_event_ref,
            Self::ChildCapabilityStateUpdated(event) => &event.capability_state_event_ref,
            Self::ChildRuntimeHealthUpdated(event) => &event.runtime_health_event_ref,
            Self::ParentReadModelProjected(event) => &event.read_model_projected_event_ref,
        }
    }
}

impl DomainEvent for ParentChildRuntimeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase().event_type())?,
            SchemaVersion::new(self.phase().schema_version())?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(parent_child_aggregate_key(self.event_ref()))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(parent_child_idempotency_key(self.event_ref()))
    }
}

#[derive(Clone, Debug)]
pub struct ParentChildRuntimeReport {
    pub publish_reports: Vec<ocentra_eventing::bus::reports::PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
}

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
        phase.custody(),
        phase.runtime_role(),
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(component)?,
        RuntimeInstanceId::parse(instance)?,
    ))
}
