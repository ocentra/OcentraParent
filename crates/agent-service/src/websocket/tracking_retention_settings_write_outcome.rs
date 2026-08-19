use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::{
    constants, logging::LogFieldValue,
    tracking::retention_settings_write_command::TrackingRetentionSettingsWriteResult,
};

pub(super) fn tracking_write_result_field_value(
    result: Option<TrackingRetentionSettingsWriteResult>,
) -> Option<LogFieldValue> {
    result
        .and_then(|value| serde_json::to_string(&value).ok())
        .map(LogFieldValue::String)
}

pub(super) struct TrackingParentRuntimeObservability {
    pub(super) state: serde_json::Value,
    pub(super) error: serde_json::Value,
}

pub(super) fn tracking_parent_runtime_observability(
    error: Option<&EventingError>,
    reached_terminal_result: bool,
) -> TrackingParentRuntimeObservability {
    match error {
        Some(error) => TrackingParentRuntimeObservability {
            state: serde_json::Value::String(
                constants::tracking_retention_settings_write::FLOW_PARENT_RUNTIME_FAILED
                    .to_string(),
            ),
            error: serde_json::Value::String(error.to_string()),
        },
        None if reached_terminal_result => TrackingParentRuntimeObservability {
            state: serde_json::Value::String(
                constants::tracking_retention_settings_write::FLOW_PARENT_RUNTIME_COMPLETED
                    .to_string(),
            ),
            error: serde_json::Value::Null,
        },
        None => TrackingParentRuntimeObservability {
            state: serde_json::Value::String(
                constants::tracking_retention_settings_write::FLOW_PARENT_RUNTIME_NOT_STARTED
                    .to_string(),
            ),
            error: serde_json::Value::Null,
        },
    }
}
