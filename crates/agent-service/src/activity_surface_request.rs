use ocentra_parent_agent_protocol::{
    constants, ActivityReportFrequency, ActivityReportRequest, ActivitySurfaceRequest,
    ActivitySurfaceScope, ActivitySurfaceScopeKind, AgentCommandEnvelope, LogFieldValue,
    ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::time::timestamp_now;

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

pub(crate) fn surface_request_from_command(
    command: &AgentCommandEnvelope,
) -> ActivitySurfaceRequest {
    let requested_at =
        string_payload_field(command, constants::field::REQUESTED_AT).unwrap_or_else(timestamp_now);
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: scope_from_command(command),
        requested_at: requested_at.clone(),
        range_start: string_payload_field(command, constants::field::RANGE_START)
            .unwrap_or_else(|| constants::activity_surface::DEFAULT_RANGE_START.to_string()),
        range_end: string_payload_field(command, constants::field::RANGE_END)
            .unwrap_or(requested_at),
    }
}

fn scope_from_command(command: &AgentCommandEnvelope) -> ActivitySurfaceScope {
    match string_payload_field(command, constants::field::SCOPE_KIND).as_deref() {
        Some(constants::activity_surface::SCOPE_DEVICE) => ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Device,
            family_id: None,
            device_id: Some(
                string_payload_field(command, constants::field::DEVICE_ID)
                    .unwrap_or_else(|| constants::activity_surface::DEFAULT_DEVICE_ID.to_string()),
            ),
        },
        _ => ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Family,
            family_id: Some(
                string_payload_field(command, constants::field::FAMILY_ID)
                    .unwrap_or_else(|| constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            ),
            device_id: None,
        },
    }
}

fn string_payload_field(command: &AgentCommandEnvelope, key: &str) -> Option<String> {
    match command.payload.get(key) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}
