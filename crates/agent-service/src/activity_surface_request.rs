use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReportDocument, ActivityReportFrequency, ActivityReportRequest, ActivitySurfaceRequest,
    ActivitySurfaceScope, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::time::timestamp_now;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PayloadFieldKey(&'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
struct PayloadText(String);

pub(crate) fn report_request_from_command(
    command: &AgentCommandEnvelope,
    frequency: ActivityReportFrequency,
) -> ActivityReportRequest {
    let request = surface_request_from_command(command);
    ActivityReportRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        frequency,
        scope: request.scope,
        requested_at: request.requested_at,
        range_start: request.range_start,
        range_end: request.range_end,
    }
}

pub(crate) fn report_document_from_command(
    command: &AgentCommandEnvelope,
) -> Option<ActivityReportDocument> {
    string_payload_field(
        command,
        PayloadFieldKey(constants::field::ACTIVITY_REPORT_DOCUMENT),
    )
    .and_then(|value| serde_json::from_str::<ActivityReportDocument>(&value.0).ok())
}

pub(crate) fn surface_request_from_command(
    command: &AgentCommandEnvelope,
) -> ActivitySurfaceRequest {
    let requested_at =
        string_payload_field(command, PayloadFieldKey(constants::field::REQUESTED_AT))
            .unwrap_or_else(|| PayloadText(timestamp_now()));
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: scope_from_command(command),
        requested_at: requested_at.0.clone(),
        range_start: string_payload_field(command, PayloadFieldKey(constants::field::RANGE_START))
            .map(|value| value.0)
            .unwrap_or_else(|| constants::activity_surface::DEFAULT_RANGE_START.to_string()),
        range_end: string_payload_field(command, PayloadFieldKey(constants::field::RANGE_END))
            .map(|value| value.0)
            .unwrap_or(requested_at.0),
    }
}

fn scope_from_command(command: &AgentCommandEnvelope) -> ActivitySurfaceScope {
    match string_payload_field(command, PayloadFieldKey(constants::field::SCOPE_KIND))
        .as_ref()
        .map(|value| value.0.as_str())
    {
        Some(constants::activity_surface::SCOPE_DEVICE) => ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Device,
            family_id: None,
            device_id: Some(
                string_payload_field(command, PayloadFieldKey(constants::field::DEVICE_ID))
                    .map(|value| value.0)
                    .unwrap_or_else(|| constants::activity_surface::DEFAULT_DEVICE_ID.to_string()),
            ),
        },
        _ => ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Family,
            family_id: Some(
                string_payload_field(command, PayloadFieldKey(constants::field::FAMILY_ID))
                    .map(|value| value.0)
                    .unwrap_or_else(|| constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            ),
            device_id: None,
        },
    }
}

fn string_payload_field(
    command: &AgentCommandEnvelope,
    field_key: PayloadFieldKey,
) -> Option<PayloadText> {
    match command.payload.get(field_key.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(PayloadText(value.trim().to_string()))
        }
        _ => None,
    }
}
