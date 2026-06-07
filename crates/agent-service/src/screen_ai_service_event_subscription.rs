use std::sync::{Arc, Mutex};

use ocentra_eventing::{
    AggregateKey, CorrelationId, DomainEvent, EventBus, EventContract, EventCustody, EventId,
    EventMetadata, EventSource, EventSubscriber, EventType, EventingError, IdempotencyKey,
    RecordedAt, RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    SubscriptionReport, TargetHandler,
};
use ocentra_parent_agent_protocol::{constants, ActivityScreenReadModelRow};
use serde::{Deserialize, Serialize};

use crate::screen_ai_service_event_bridge::{
    publish_screen_service_row_event_chain, ScreenAiServiceEventBridgeError,
    ScreenAiServiceEventBridgeRefs,
};

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod live_view_runtime;
#[cfg(test)]
mod live_view_runtime_tests;

pub(crate) struct ScreenAiServiceEventRuntime {
    bus: EventBus,
    #[cfg(test)]
    state: ScreenAiServiceEventSubscriptionState,
}

impl ScreenAiServiceEventRuntime {
    pub(crate) async fn start() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        let state = ScreenAiServiceEventSubscriptionState::default();
        subscribe_screen_service_row_ready_events(&bus, state.clone()).await?;
        Ok(Self {
            bus,
            #[cfg(test)]
            state,
        })
    }

    pub(crate) async fn publish_row_ready(
        &self,
        row: ActivityScreenReadModelRow,
        action_ref: impl Into<String>,
        observed_at: &str,
    ) -> Result<ocentra_eventing::PublishReport, EventingError> {
        publish_screen_service_row_ready_event(
            &self.bus,
            ScreenAiServiceRowReadyEvent::new(row, action_ref),
            observed_at,
        )
        .await
    }

    #[cfg(test)]
    pub(crate) fn dispatches(&self) -> Vec<ScreenAiServiceEventSubscriptionDispatch> {
        self.state.dispatches()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ScreenAiServiceRowReadyEvent {
    pub(crate) row: ActivityScreenReadModelRow,
    pub(crate) action_ref: String,
}

impl ScreenAiServiceRowReadyEvent {
    pub(crate) fn new(row: ActivityScreenReadModelRow, action_ref: impl Into<String>) -> Self {
        Self {
            row,
            action_ref: action_ref.into(),
        }
    }
}

impl DomainEvent for ScreenAiServiceRowReadyEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::screen_flow::EVENT_SCREEN_SERVICE_ROW_READY)?,
            SchemaVersion::new(constants::screen_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        let mut value = String::from(constants::screen_flow::AGGREGATE_SCREEN_QUEUE_PREFIX);
        value.push_str(&self.row.queue_job_id);
        AggregateKey::parse(value)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value =
            String::from(constants::screen_flow::IDEMPOTENCY_SCREEN_SERVICE_ROW_READY_PREFIX);
        value.push_str(&self.row.queue_job_id);
        value.push(ocentra_parent_agent_protocol::constants::delimiter::HYPHEN);
        value.push_str(&self.row.row_id);
        IdempotencyKey::parse(value)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ScreenAiServiceEventSubscriptionState {
    dispatches: Arc<Mutex<Vec<ScreenAiServiceEventSubscriptionDispatch>>>,
}

impl ScreenAiServiceEventSubscriptionState {
    #[cfg(test)]
    pub(crate) fn dispatches(&self) -> Vec<ScreenAiServiceEventSubscriptionDispatch> {
        self.dispatches
            .lock()
            .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_RECORDS)
            .clone()
    }

    fn record(&self, dispatch: ScreenAiServiceEventSubscriptionDispatch) {
        self.dispatches
            .lock()
            .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_RECORDS)
            .push(dispatch);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScreenAiServiceEventSubscriptionDispatch {
    Published {
        queue_job_id: String,
        screen_analysis_result_id: String,
        downstream_event_count: usize,
        raw_image_escaped: bool,
    },
    Rejected {
        queue_job_id: String,
        screen_analysis_result_id: String,
        reason: ScreenAiServiceEventBridgeError,
    },
}

pub(crate) async fn subscribe_screen_service_row_ready_events(
    bus: &EventBus,
    state: ScreenAiServiceEventSubscriptionState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ScreenAiServiceRowReadyEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::screen_flow::SUBSCRIBER_SCREEN_SERVICE_ROW_READY)?,
            EventType::parse(constants::screen_flow::EVENT_SCREEN_SERVICE_ROW_READY)?,
            TargetHandler::parse(constants::screen_flow::TARGET_SCREEN_SERVICE_EVENT_SUBSCRIBER)?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                handle_screen_service_row_ready_event(
                    context.payload().clone(),
                    context.envelope().observed_at.as_str(),
                    state,
                )
                .await
            }
        },
    )
    .await
}

pub(crate) async fn publish_screen_service_row_ready_event(
    bus: &EventBus,
    event: ScreenAiServiceRowReadyEvent,
    observed_at: &str,
) -> Result<ocentra_eventing::PublishReport, EventingError> {
    bus.publish(event, screen_service_row_ready_metadata(observed_at)?)
        .await
}

async fn handle_screen_service_row_ready_event(
    event: ScreenAiServiceRowReadyEvent,
    observed_at: &str,
    state: ScreenAiServiceEventSubscriptionState,
) -> Result<(), EventingError> {
    let queue_job_id = event.row.queue_job_id.clone();
    let screen_analysis_result_id = event.row.row_id.clone();
    let result = publish_screen_service_row_event_chain(
        event.row,
        observed_at,
        ScreenAiServiceEventBridgeRefs {
            action_ref: event.action_ref,
        },
    )
    .await;

    match result {
        Ok(report) => {
            let downstream_event_count = report.stored_events.len();
            let raw_image_escaped = report.raw_image_escaped();
            state.record(ScreenAiServiceEventSubscriptionDispatch::Published {
                queue_job_id,
                screen_analysis_result_id,
                downstream_event_count,
                raw_image_escaped,
            });
            Ok(())
        }
        Err(reason) => {
            state.record(ScreenAiServiceEventSubscriptionDispatch::Rejected {
                queue_job_id,
                screen_analysis_result_id,
                reason,
            });
            Err(EventingError::InvalidValue {
                field: constants::screen_flow::FIELD_SCREEN_SERVICE_ROW_READY,
                value: constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_REJECTS
                    .to_string(),
            })
        }
    }
}

fn screen_service_row_ready_metadata(observed_at: &str) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(constants::screen_flow::CORRELATION_SCREEN_RUNTIME_PREFIX)?,
        screen_service_row_ready_source()?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(
            constants::screen_flow::TARGET_SCREEN_SERVICE_EVENT_SUBSCRIBER,
        )?),
    ))
}

fn screen_service_row_ready_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
        ocentra_eventing::RuntimeRole::parse(constants::eventing_source::ROLE_AGENT)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(
            constants::screen_flow::RUNTIME_COMPONENT_SCREEN_SERVICE_SUBSCRIBER,
        )?,
        RuntimeInstanceId::parse(constants::screen_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
