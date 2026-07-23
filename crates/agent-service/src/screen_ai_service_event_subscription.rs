use std::sync::{Arc, Mutex, MutexGuard};

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::subscriber::SubscriptionReport, bus::EventBus,
    envelope::DomainEvent, envelope::EventContract, envelope::EventMetadata, envelope::EventSource,
    error::EventingError, ids::AggregateKey, ids::CorrelationId, ids::EventCustody, ids::EventId,
    ids::EventType, ids::IdempotencyKey, ids::RecordedAt, ids::RuntimeInstanceId,
    ids::SchemaVersion, ids::SourceComponent, ids::SourceService, ids::SubscriberId,
    ids::TargetHandler,
};
use ocentra_parent_agent_core::screen_event_runtime::{ScreenRuntimeReport, ScreenRuntimeSpine};
use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::screen_ai_service_event_bridge::{
    publish_screen_degraded_event_chain, publish_screen_service_row_event_chain,
    screen_runtime_deletion_input_from_service_row, ScreenAiServiceEventBridgeError,
    ScreenAiServiceEventBridgeRefs,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActionRefText(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ObservedAtText(pub(crate) String);

#[path = "screen_ai_service_event_subscription/live_view_service_runtime.rs"]
pub(crate) mod live_view_service_runtime;

pub(crate) struct ScreenAiServiceEventRuntime {
    bus: EventBus,
}

impl ScreenAiServiceEventRuntime {
    pub(crate) async fn start() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        let state = ScreenAiServiceEventSubscriptionState::default();
        subscribe_screen_service_row_ready_events(&bus, state.clone()).await?;
        Ok(Self { bus })
    }

    pub(crate) async fn publish_row_ready(
        &self,
        row: ActivityScreenReadModelRow,
        action_ref: ActionRefText,
        observed_at: ObservedAtText,
    ) -> Result<ocentra_eventing::bus::reports::handler::PublishReport, EventingError> {
        publish_screen_service_row_ready_event(
            &self.bus,
            ScreenAiServiceRowReadyEvent::new(row, action_ref),
            observed_at,
        )
        .await
    }

    pub(crate) async fn publish_deletion_row(
        &self,
        row: ActivityScreenReadModelRow,
        observed_at: ObservedAtText,
    ) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
        let input = screen_runtime_deletion_input_from_service_row(row)?;
        ScreenRuntimeSpine::with_default_handlers()
            .await
            .map_err(|_start_error| ScreenAiServiceEventBridgeError::EventPublishFailed)?
            .publish_deletion_event(input, observed_at.0.as_str())
            .await
            .map_err(|_publish_error| ScreenAiServiceEventBridgeError::EventPublishFailed)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ScreenAiServiceRowReadyEvent {
    pub(crate) row: ActivityScreenReadModelRow,
    pub(crate) action_ref: String,
}

impl ScreenAiServiceRowReadyEvent {
    pub(crate) fn new(row: ActivityScreenReadModelRow, action_ref: ActionRefText) -> Self {
        Self {
            row,
            action_ref: action_ref.0,
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
    pub(crate) dispatches: Arc<Mutex<Vec<ScreenAiServiceEventSubscriptionDispatch>>>,
}

impl ScreenAiServiceEventSubscriptionState {
    fn record(&self, dispatch: ScreenAiServiceEventSubscriptionDispatch) {
        lock_recover(&self.dispatches).push(dispatch);
    }
}

fn lock_recover<T>(value: &Arc<Mutex<T>>) -> MutexGuard<'_, T> {
    value
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
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
                    ObservedAtText(context.envelope().observed_at.as_str().to_string()),
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
    observed_at: ObservedAtText,
) -> Result<ocentra_eventing::bus::reports::handler::PublishReport, EventingError> {
    bus.publish(event, screen_service_row_ready_metadata(&observed_at)?)
        .await
}

async fn handle_screen_service_row_ready_event(
    event: ScreenAiServiceRowReadyEvent,
    observed_at: ObservedAtText,
    state: ScreenAiServiceEventSubscriptionState,
) -> Result<(), EventingError> {
    let queue_job_id = event.row.queue_job_id.clone();
    let screen_analysis_result_id = event.row.row_id.clone();
    let result = publish_screen_runtime_chain_for_row(event, observed_at).await;

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

async fn publish_screen_runtime_chain_for_row(
    event: ScreenAiServiceRowReadyEvent,
    observed_at: ObservedAtText,
) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
    let ScreenAiServiceRowReadyEvent { row, action_ref } = event;
    if screen_service_row_is_degraded(&row) {
        return publish_screen_degraded_event_chain(row, observed_at).await;
    }
    publish_screen_service_row_event_chain(
        row,
        observed_at,
        ScreenAiServiceEventBridgeRefs {
            action_ref: ActionRefText(action_ref),
        },
    )
    .await
}

fn screen_service_row_is_degraded(row: &ActivityScreenReadModelRow) -> bool {
    row.capability_status == constants::activity_surface::SAVED_STATE_DEGRADED
        && !row.policy_eligible
}

fn screen_service_row_ready_metadata(
    observed_at: &ObservedAtText,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(constants::screen_flow::CORRELATION_SCREEN_RUNTIME_PREFIX)?,
        screen_service_row_ready_source()?,
        RecordedAt::parse(observed_at.0.as_str())?,
        Some(TargetHandler::parse(
            constants::screen_flow::TARGET_SCREEN_SERVICE_EVENT_SUBSCRIBER,
        )?),
    ))
}

fn screen_service_row_ready_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
        ocentra_eventing::ids::RuntimeRole::parse(constants::eventing_source::ROLE_AGENT)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(
            constants::screen_flow::RUNTIME_COMPONENT_SCREEN_SERVICE_SUBSCRIBER,
        )?,
        RuntimeInstanceId::parse(constants::screen_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
