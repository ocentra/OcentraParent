use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::subscriber::EventSubscriber,
    bus::subscriber::SubscriptionReport, envelope::DomainEvent, error::EventingError,
    ids::EventType, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;

use crate::tracking_dispatch::PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE;

use super::ParentTrackingConfigUpdateEventState;

const SUBSCRIBER_DISPATCH_RECORDER: &str =
    "subscriber.parent-runtime.tracking-config.dispatch-recorder";
const SUBSCRIBER_CHANGE_APPROVED_RECORDER: &str =
    "subscriber.parent-runtime.tracking-config.change-approved-recorder";
const SUBSCRIBER_CHANGE_REJECTED_RECORDER: &str =
    "subscriber.parent-runtime.tracking-config.change-rejected-recorder";
const SUBSCRIBER_AUDIT_RECORDER: &str = "subscriber.parent-runtime.tracking-config.audit-recorder";
const SUBSCRIBER_PORTAL_RECORDER: &str =
    "subscriber.parent-runtime.tracking-config.portal-recorder";
const TARGET_DISPATCH_RECORDER: &str = "target.parent-runtime.tracking-config.dispatch-recorder";
const TARGET_CHANGE_APPROVED_RECORDER: &str =
    "target.parent-runtime.tracking-config.change-approved-recorder";
const TARGET_CHANGE_REJECTED_RECORDER: &str =
    "target.parent-runtime.tracking-config.change-rejected-recorder";
const TARGET_AUDIT_RECORDER: &str = "target.parent-runtime.tracking-config.audit-recorder";
const TARGET_PORTAL_RECORDER: &str = "target.parent-runtime.tracking-config.portal-recorder";

#[derive(Clone)]
pub(super) struct TrackingConfigEventSinkSubscriptionReports {
    pub(super) dispatch: SubscriptionReport,
    pub(super) change_approved: SubscriptionReport,
    pub(super) change_rejected: SubscriptionReport,
    pub(super) audit: SubscriptionReport,
    pub(super) portal: SubscriptionReport,
}

pub(super) async fn subscribe_tracking_config_event_sinks(
    bus: &RootEventPublisher,
    state: ParentTrackingConfigUpdateEventState,
) -> Result<TrackingConfigEventSinkSubscriptionReports, EventingError> {
    Ok(TrackingConfigEventSinkSubscriptionReports {
        dispatch: subscribe_event(
            bus,
            state.clone(),
            SUBSCRIBER_DISPATCH_RECORDER,
            PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE,
            TARGET_DISPATCH_RECORDER,
            ParentTrackingConfigUpdateEventState::record_dispatch_event,
        )
        .await?,
        change_approved: subscribe_event(
            bus,
            state.clone(),
            SUBSCRIBER_CHANGE_APPROVED_RECORDER,
            constants::tracking_config_update::CHANGE_APPROVED_EVENT_TYPE,
            TARGET_CHANGE_APPROVED_RECORDER,
            ParentTrackingConfigUpdateEventState::record_change_approved_event,
        )
        .await?,
        change_rejected: subscribe_event(
            bus,
            state.clone(),
            SUBSCRIBER_CHANGE_REJECTED_RECORDER,
            constants::tracking_config_update::CHANGE_REJECTED_EVENT_TYPE,
            TARGET_CHANGE_REJECTED_RECORDER,
            ParentTrackingConfigUpdateEventState::record_change_rejected_event,
        )
        .await?,
        audit: subscribe_event(
            bus,
            state.clone(),
            SUBSCRIBER_AUDIT_RECORDER,
            constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED,
            TARGET_AUDIT_RECORDER,
            ParentTrackingConfigUpdateEventState::record_audit_event,
        )
        .await?,
        portal: subscribe_event(
            bus,
            state,
            SUBSCRIBER_PORTAL_RECORDER,
            constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED,
            TARGET_PORTAL_RECORDER,
            ParentTrackingConfigUpdateEventState::record_portal_event,
        )
        .await?,
    })
}

async fn subscribe_event<E>(
    bus: &RootEventPublisher,
    state: ParentTrackingConfigUpdateEventState,
    subscriber_id: &str,
    event_type: &str,
    target_handler: &str,
    record: fn(&ParentTrackingConfigUpdateEventState, E),
) -> Result<SubscriptionReport, EventingError>
where
    E: DomainEvent + Clone,
{
    bus.subscribe::<E, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(subscriber_id)?,
            EventType::parse(event_type)?,
            TargetHandler::parse(target_handler)?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                record(&state, context.payload().clone());
                Ok(())
            }
        },
    )
    .await
}
