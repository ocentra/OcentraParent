use super::*;
use crate::agent_service_client::payload_fields::serialized_enum_label;
use crate::agent_service_client::snapshots_network::activity_surface_read_model_from_response;
use crate::agent_service_client::transport::rejection_message;
use crate::parent_ui_bridge::route_metadata::tracking_read_model_snapshot;

pub(super) fn tracking_read_model_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<TrackingReadModelAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    let event = events.last().cloned().ok_or_else(|| {
        "agent-service tracking read model result did not include a response event".to_string()
    })?;

    Ok(TrackingReadModelAgentServiceSnapshot {
        event,
        read_model: tracking_read_model_result_from_response(&response_event),
    })
}

fn tracking_read_model_result_from_response(
    response_event: &AgentEventEnvelope,
) -> ParentActivityTrackingReadModelResultSnapshot {
    if response_event.event != AgentEventName::AgentActivityTrackingReadModelReported {
        return tracking_read_model_failure(
            ParentActivityTrackingReadModelFailureReason::WrongEvent,
        );
    }
    let Some(read_model_json) = response_event
        .payload
        .get(constants::field::ACTIVITY_TRACKING_READ_MODEL)
        .and_then(log_field_string)
    else {
        return tracking_read_model_failure(
            ParentActivityTrackingReadModelFailureReason::MissingJsonField,
        );
    };
    let Ok(decoded) = serde_json::from_str::<Value>(read_model_json) else {
        return tracking_read_model_failure(
            ParentActivityTrackingReadModelFailureReason::InvalidJson,
        );
    };
    let Ok(read_model) = serde_json::from_value::<TrackingReadModel>(decoded) else {
        return tracking_read_model_failure(
            ParentActivityTrackingReadModelFailureReason::InvalidPayload,
        );
    };

    ParentActivityTrackingReadModelResultSnapshot {
        ok: true,
        reason: None,
        value: Some(tracking_read_model_snapshot(&read_model)),
    }
}

fn tracking_read_model_failure(
    reason: ParentActivityTrackingReadModelFailureReason,
) -> ParentActivityTrackingReadModelResultSnapshot {
    ParentActivityTrackingReadModelResultSnapshot {
        ok: false,
        reason: Some(reason),
        value: None,
    }
}

pub(super) fn activity_screen_read_model_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<ScreenReadModelAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event != AgentEventName::AgentActivityScreenReadModelReported {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(&AgentEventName::AgentActivityScreenReadModelReported),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service screen read model result did not include a response event".to_string()
    })?;
    let read_model = activity_surface_read_model_from_response::<ActivityScreenReadModel>(
        &response_event,
        constants::activity_surface::READ_MODEL_SCREEN,
        "screen",
    )?;
    Ok(ScreenReadModelAgentServiceSnapshot { read_model })
}
