use ocentra_eventing::{
    envelope::EventMetadata, envelope::EventSource, error::EventingError, ids::CorrelationId,
    ids::EventCustody, ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::RuntimeRole,
    ids::SourceComponent, ids::SourceService, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    tracking_config_update_applied_event_from_child, ChildTrackingConfigUpdatedEvent,
    ParentTrackingConfigUpdatedEvent, TrackingConfigUpdateAppliedEvent,
    TrackingConfigUpdateEventName, TrackingConfigUpdateResponse, TrackingConfigUpdateResponseState,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_tracking_core::retention_settings::TrackingConfigUpdateAppliedState;

use super::TrackingConfigUpdateAppliedReport;

pub(super) fn tracking_config_update_response(
    parent_event: &ParentTrackingConfigUpdatedEvent,
    applied_report: TrackingConfigUpdateAppliedReport,
) -> TrackingConfigUpdateResponse {
    TrackingConfigUpdateResponse {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        source_command_id: parent_event.source_command_id.clone(),
        response_state: applied_report.response_state,
        effective_tracking_state: applied_report
            .applied_state
            .effective_tracking_state
            .clone(),
        child_event_type: applied_report.child_event_type,
        target: parent_event.target.clone(),
        local_service_state_revision: Some(
            applied_report.applied_state.local_service_state_revision,
        ),
        durable_settings_persistence_state: applied_report
            .applied_state
            .durable_settings_persistence_state,
    }
}

pub(super) fn apply_child_tracking_config_updated_event(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> TrackingConfigUpdateAppliedReport {
    let applied_state = on_child_tracking_config_updated_event(child_event);
    let effective_tracking_state = applied_state.effective_tracking_state.clone();

    TrackingConfigUpdateAppliedReport {
        parent_event_type: child_event.parent_event_type,
        child_event_type: TrackingConfigUpdateEventName::Child,
        applied_event_type: TrackingConfigUpdateEventName::Applied,
        target_scope: child_event.target.scope,
        response_state: TrackingConfigUpdateResponseState::Applied,
        effective_tracking_state,
        applied_state,
    }
}

pub(super) fn tracking_config_update_applied_event_from_report(
    child_event: &ChildTrackingConfigUpdatedEvent,
    applied_report: &TrackingConfigUpdateAppliedReport,
) -> TrackingConfigUpdateAppliedEvent {
    tracking_config_update_applied_event_from_child(
        child_event,
        applied_report.response_state.clone(),
        applied_report.effective_tracking_state.clone(),
        applied_report.applied_state.local_service_state_revision,
        applied_report
            .applied_state
            .durable_settings_persistence_state,
    )
}

pub(super) fn tracking_config_update_applied_report(
    applied_event: &TrackingConfigUpdateAppliedEvent,
) -> TrackingConfigUpdateAppliedReport {
    TrackingConfigUpdateAppliedReport {
        parent_event_type: applied_event.parent_event_type,
        child_event_type: applied_event.child_event_type,
        applied_event_type: TrackingConfigUpdateEventName::Applied,
        target_scope: applied_event.target.scope,
        response_state: applied_event.response_state.clone(),
        effective_tracking_state: applied_event.effective_tracking_state.clone(),
        applied_state: TrackingConfigUpdateAppliedState {
            local_service_state_revision: applied_event.local_service_state_revision,
            durable_settings_persistence_state: applied_event.durable_settings_persistence_state,
            effective_tracking_state: applied_event.effective_tracking_state.clone(),
        },
    }
}

pub(super) fn parent_tracking_config_updated_metadata(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        parent_event.source_command_id.as_str(),
        tracking_parent_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_RELAY,
    )
}

pub(super) fn child_tracking_config_updated_metadata(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        child_event.source_command_id.as_str(),
        tracking_child_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIER,
    )
}

pub(super) fn child_tracking_config_applied_metadata(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        child_event.source_command_id.as_str(),
        tracking_child_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER,
    )
}

fn on_child_tracking_config_updated_event(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> TrackingConfigUpdateAppliedState {
    ocentra_tracking_core::retention_settings::apply_tracking_config_update(&child_event.config)
}

fn tracking_config_update_metadata(
    source_command_id: &str,
    source: EventSource,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_config_update_correlation_id(source_command_id)?,
        source,
        RecordedAt::parse(constants::tracking_retention_settings_write::ACCEPTED_AT)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn tracking_parent_event_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::eventing_source::CUSTODY_LOCAL_JOURNAL)?,
        RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(
            constants::tracking_config_update::SOURCE_COMPONENT_PARENT_AGENT_SERVICE,
        )?,
        RuntimeInstanceId::parse(constants::peer::PORTAL_DEV)?,
    ))
}

fn tracking_child_event_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
        RuntimeRole::parse(constants::eventing_source::ROLE_AGENT)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(
            constants::tracking_config_update::SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME,
        )?,
        RuntimeInstanceId::parse(constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}

fn tracking_config_update_correlation_id(
    source_command_id: &str,
) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_config_update::CORRELATION_PREFIX);
    value.push_str(source_command_id);
    CorrelationId::parse(value)
}
