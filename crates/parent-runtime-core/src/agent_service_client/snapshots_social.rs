use serde::de::DeserializeOwned;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentEventName};

use super::payload_fields::serialized_enum_label;
use super::snapshots_network::response_json_payload_field;
use super::transport::rejection_message;
use super::types::{
    AgentServiceCommandResult, SocialAlertReportAgentServiceSnapshot,
    SocialAlertReportParentSurfaceAgentServiceSnapshot, SocialAuditExplanationAgentServiceSnapshot,
    SocialDashboardAgentServiceSnapshot, SocialParentNotificationDeliveryAgentServiceSnapshot,
};

pub(crate) fn social_dashboard_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<SocialDashboardAgentServiceSnapshot, String> {
    social_read_model_from_result(
        result,
        AgentEventName::AgentBrowserSocialDashboardReadModelReported,
        constants::field::BROWSER_SOCIAL_DASHBOARD_READ_MODEL,
        "social dashboard",
    )
    .map(|read_model| SocialDashboardAgentServiceSnapshot { read_model })
}

pub(crate) fn social_audit_explanation_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<SocialAuditExplanationAgentServiceSnapshot, String> {
    social_read_model_from_result(
        result,
        AgentEventName::AgentBrowserSocialAuditExplanationReadModelReported,
        constants::field::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL,
        "social audit explanation",
    )
    .map(|read_model| SocialAuditExplanationAgentServiceSnapshot { read_model })
}

pub(crate) fn social_alert_report_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<SocialAlertReportAgentServiceSnapshot, String> {
    social_read_model_from_result(
        result,
        AgentEventName::AgentBrowserSocialAlertReportReadModelReported,
        constants::field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL,
        "social alert report",
    )
    .map(|read_model| SocialAlertReportAgentServiceSnapshot { read_model })
}

pub(crate) fn social_alert_report_parent_surface_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<SocialAlertReportParentSurfaceAgentServiceSnapshot, String> {
    social_read_model_from_result(
        result,
        AgentEventName::AgentBrowserSocialAlertReportParentSurfaceReadModelReported,
        constants::field::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL,
        "social alert report parent surface",
    )
    .map(|read_model| SocialAlertReportParentSurfaceAgentServiceSnapshot { read_model })
}

pub(crate) fn social_parent_notification_delivery_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<SocialParentNotificationDeliveryAgentServiceSnapshot, String> {
    social_read_model_from_result(
        result,
        AgentEventName::AgentBrowserSocialParentNotificationDeliveryReadModelReported,
        constants::field::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL,
        "social parent notification delivery",
    )
    .map(|read_model| SocialParentNotificationDeliveryAgentServiceSnapshot { read_model })
}

fn social_read_model_from_result<T: DeserializeOwned>(
    result: AgentServiceCommandResult,
    expected: AgentEventName,
    payload_field: &str,
    label: &str,
) -> Result<T, String> {
    let response_event = result.response_event;
    reject_failed_response(&response_event, &expected, label)?;
    serde_json::from_value(response_json_payload_field(&response_event, payload_field)?)
        .map_err(|error| format!("agent-service {label} parse failed: {error}"))
}

fn reject_failed_response(
    response_event: &AgentEventEnvelope,
    expected: &AgentEventName,
    label: &str,
) -> Result<(), String> {
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(response_event));
    }
    if &response_event.event == expected {
        return Ok(());
    }
    Err(format!(
        "agent-service expected {} for {label}, received {}",
        serialized_enum_label(expected),
        serialized_enum_label(&response_event.event)
    ))
}
