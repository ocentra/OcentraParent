use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};

use ocentra_eventing::{
    bus::publisher::{EventContext, RootEventPublisher},
    bus::reports::handler::{HandlerOutcome, PublishReport},
    bus::subscriber::EventSubscriber,
    bus::subscriber::SubscriptionReport,
    bus::EventBus,
    envelope::DomainEvent,
    envelope::EventContract,
    error::EventingError,
    ids::AggregateKey,
    ids::EventType,
    ids::IdempotencyKey,
    ids::SchemaVersion,
    ids::SubscriberId,
    ids::TargetHandler,
    journal::ndjson::NdjsonEventJournal,
    journal::ndjson::NdjsonJournalOptions,
};
use ocentra_parent_agent_core::screen_event_runtime::{ScreenRuntimeReport, ScreenRuntimeSpine};
use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::screen_ai_service_event_bridge::{
    screen_runtime_degraded_input_from_service_row, screen_runtime_deletion_input_from_service_row,
    screen_runtime_input_from_service_row, ScreenAiServiceEventBridgeError,
    ScreenAiServiceEventBridgeRefs,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActionRefText(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ObservedAtText(pub(crate) String);

#[path = "screen_ai_service_event_subscription/live_view_service_runtime.rs"]
pub(crate) mod live_view_service_runtime;

pub(crate) struct ScreenAiServiceEventRuntime {
    deletion_journals: Arc<Mutex<BTreeMap<PathBuf, NdjsonEventJournal>>>,
}

impl ScreenAiServiceEventRuntime {
    pub(crate) async fn start() -> Result<Self, EventingError> {
        let bus = EventBus::root();
        let state = ScreenAiServiceEventSubscriptionState::default();
        subscribe_screen_service_row_ready_events(&bus, state.clone()).await?;
        Ok(Self {
            deletion_journals: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    pub(crate) async fn publish_row_ready(
        &self,
        _row: ActivityScreenReadModelRow,
        _action_ref: ActionRefText,
        _observed_at: ObservedAtText,
    ) -> Result<ocentra_eventing::bus::reports::handler::PublishReport, EventingError> {
        Err(EventingError::InvalidValue {
            field: constants::screen_flow::FIELD_SCREEN_SERVICE_ROW_READY,
            value: constants::screen_flow::ERROR_SCREEN_RUNTIME_OWNER_UNAVAILABLE_MANUAL_REQUIRED
                .to_string(),
        })
    }

    pub(crate) async fn publish_deletion_row(
        &self,
        row: ActivityScreenReadModelRow,
        observed_at: ObservedAtText,
        journal_path: &Path,
    ) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
        let input = screen_runtime_deletion_input_from_service_row(row)?;
        let deletion_spine =
            ScreenRuntimeSpine::with_durable_deletion_handler(self.deletion_journal(journal_path))
                .await
                .map_err(|_subscription_error| {
                    ScreenAiServiceEventBridgeError::EventPublishFailed
                })?;
        deletion_spine
            .publish_deletion_event(input, observed_at.0.as_str())
            .await
            .map_err(|_publish_error| ScreenAiServiceEventBridgeError::EventPublishFailed)
    }

    fn deletion_journal(&self, journal_path: &Path) -> NdjsonEventJournal {
        let mut journals = lock_recover(&self.deletion_journals);
        journals
            .entry(journal_path.to_path_buf())
            .or_insert_with(|| {
                NdjsonEventJournal::with_options(journal_path, NdjsonJournalOptions::hash_chain())
            })
            .clone()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ScreenAiServiceRowReadyEvent {
    pub(crate) row: ActivityScreenReadModelRow,
    pub(crate) action_ref: String,
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
    Rejected {
        queue_job_id: String,
        screen_analysis_result_id: String,
        reason: ScreenAiServiceEventBridgeError,
    },
}

pub(crate) async fn subscribe_screen_service_row_ready_events(
    bus: &RootEventPublisher,
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
            async move { handle_screen_service_row_ready_event(context, state).await }
        },
    )
    .await
}

async fn handle_screen_service_row_ready_event(
    context: EventContext<ScreenAiServiceRowReadyEvent>,
    state: ScreenAiServiceEventSubscriptionState,
) -> Result<(), EventingError> {
    let event = context.payload().clone();
    let queue_job_id = event.row.queue_job_id.clone();
    let screen_analysis_result_id = event.row.row_id.clone();
    let validation = if screen_service_row_is_degraded(&event.row) {
        screen_runtime_degraded_input_from_service_row(event.row.clone()).map(|_| ())
    } else {
        screen_runtime_input_from_service_row(
            event.row.clone(),
            ScreenAiServiceEventBridgeRefs {
                action_ref: ActionRefText(event.action_ref.clone()),
            },
        )
        .map(|_| ())
    };
    if let Err(reason) = validation {
        return Err(reject_screen_service_row(&state, &event, reason));
    }
    let error = EventingError::InvalidValue {
        field: constants::screen_flow::FIELD_SCREEN_SERVICE_ROW_READY,
        value: constants::screen_flow::ERROR_SCREEN_RUNTIME_OWNER_UNAVAILABLE_MANUAL_REQUIRED
            .to_string(),
    };
    state.record(ScreenAiServiceEventSubscriptionDispatch::Rejected {
        queue_job_id,
        screen_analysis_result_id,
        reason: ScreenAiServiceEventBridgeError::RuntimeOwnerUnavailable,
    });
    Err(error)
}

fn reject_screen_service_row(
    state: &ScreenAiServiceEventSubscriptionState,
    event: &ScreenAiServiceRowReadyEvent,
    reason: ScreenAiServiceEventBridgeError,
) -> EventingError {
    state.record(ScreenAiServiceEventSubscriptionDispatch::Rejected {
        queue_job_id: event.row.queue_job_id.clone(),
        screen_analysis_result_id: event.row.row_id.clone(),
        reason,
    });
    EventingError::InvalidValue {
        field: constants::screen_flow::FIELD_SCREEN_SERVICE_ROW_READY,
        value: constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_REJECTS.to_string(),
    }
}

pub(crate) fn publish_report_succeeded(report: &PublishReport) -> bool {
    report.subscriber_count > 0
        && report.handled_count == report.subscriber_count
        && report.dead_letter_count == 0
        && report.handler_reports.len() == report.subscriber_count
        && report
            .handler_reports
            .iter()
            .all(|handler| handler.outcome == HandlerOutcome::Handled)
}

fn screen_service_row_is_degraded(row: &ActivityScreenReadModelRow) -> bool {
    row.capability_status == constants::activity_surface::SAVED_STATE_DEGRADED
        && !row.policy_eligible
}
