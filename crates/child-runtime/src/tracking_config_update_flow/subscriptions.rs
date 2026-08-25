use super::state::TrackingConfigUpdateEventState;
use super::support::{
    apply_child_tracking_config_updated_event, child_tracking_config_applied_metadata,
    child_tracking_config_updated_metadata, tracking_config_update_applied_event_from_report,
    tracking_config_update_applied_report, tracking_config_update_response,
};
use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::subscriber::EventSubscriber,
    bus::subscriber::SubscriptionReport, error::EventingError, ids::EventType, ids::SubscriberId,
    ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    child_tracking_config_updated_event_from_parent, ChildTrackingConfigUpdatedEvent,
    ParentTrackingConfigUpdatedEvent, TrackingConfigUpdateAppliedEvent,
};

pub(super) async fn subscribe_parent_tracking_config_updated_events(
    bus: &RootEventPublisher,
    state: TrackingConfigUpdateEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ParentTrackingConfigUpdatedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_PARENT_TRACKING_CONFIG_RELAY,
            )?,
            EventType::parse(constants::tracking_config_update::PARENT_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_RELAY,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let child_event =
                    child_tracking_config_updated_event_from_parent(context.payload());
                state.record_child_event(child_event.clone());
                let child_event_metadata = child_tracking_config_updated_metadata(&child_event)?;
                context
                    .publisher()
                    .publish(child_event, child_event_metadata)
                    .await?;
                context
                    .complete_request(tracking_config_update_response(
                        context.payload(),
                        state.applied_report()?,
                    ))
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

pub(super) async fn subscribe_child_tracking_config_updated_events(
    bus: &RootEventPublisher,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildTrackingConfigUpdatedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIER,
            )?,
            EventType::parse(constants::tracking_config_update::CHILD_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIER,
            )?,
        ),
        move |context| async move {
            let applied_report = apply_child_tracking_config_updated_event(context.payload());
            let applied_event = tracking_config_update_applied_event_from_report(
                context.payload(),
                &applied_report,
            );
            context
                .publisher()
                .publish(
                    applied_event,
                    child_tracking_config_applied_metadata(context.payload())?,
                )
                .await?;
            Ok(())
        },
    )
    .await
}

pub(super) async fn subscribe_child_tracking_config_applied_events(
    bus: &RootEventPublisher,
    state: TrackingConfigUpdateEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingConfigUpdateAppliedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER,
            )?,
            EventType::parse(constants::tracking_config_update::APPLIED_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_applied_event(context.payload().clone());
                state.record_applied_report(tracking_config_update_applied_report(context.payload()));
                Ok(())
            }
        },
    )
    .await
}
