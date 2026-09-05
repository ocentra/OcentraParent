use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{AgentCommandName, AgentRoute};

use super::snapshots_social::{
    social_alert_report_parent_surface_snapshot_from_result,
    social_alert_report_snapshot_from_result, social_audit_explanation_snapshot_from_result,
    social_dashboard_snapshot_from_result,
    social_parent_notification_delivery_snapshot_from_result,
};
use super::transport::send_agent_command;
use super::types::{
    AgentServiceError, AgentServiceResult, SocialAlertReportAgentServiceSnapshot,
    SocialAlertReportParentSurfaceAgentServiceSnapshot, SocialAuditExplanationAgentServiceSnapshot,
    SocialDashboardAgentServiceSnapshot, SocialParentNotificationDeliveryAgentServiceSnapshot,
};

pub(crate) fn load_social_dashboard() -> AgentServiceResult<SocialDashboardAgentServiceSnapshot> {
    send_social_command(AgentCommandName::AgentBrowserSocialDashboardReadModelGet)
        .and_then(social_dashboard_snapshot_from_result)
        .map_err(AgentServiceError::from_display)
}

pub(crate) fn load_social_audit_explanation(
) -> AgentServiceResult<SocialAuditExplanationAgentServiceSnapshot> {
    send_social_command(AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet)
        .and_then(social_audit_explanation_snapshot_from_result)
        .map_err(AgentServiceError::from_display)
}

pub(crate) fn load_social_alert_report() -> AgentServiceResult<SocialAlertReportAgentServiceSnapshot>
{
    send_social_command(AgentCommandName::AgentBrowserSocialAlertReportReadModelGet)
        .and_then(social_alert_report_snapshot_from_result)
        .map_err(AgentServiceError::from_display)
}

pub(crate) fn load_social_alert_report_parent_surface(
) -> AgentServiceResult<SocialAlertReportParentSurfaceAgentServiceSnapshot> {
    send_social_command(AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet)
        .and_then(social_alert_report_parent_surface_snapshot_from_result)
        .map_err(AgentServiceError::from_display)
}

pub(crate) fn load_social_parent_notification_delivery(
) -> AgentServiceResult<SocialParentNotificationDeliveryAgentServiceSnapshot> {
    send_social_command(AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet)
        .and_then(social_parent_notification_delivery_snapshot_from_result)
        .map_err(AgentServiceError::from_display)
}

fn send_social_command(
    command: AgentCommandName,
) -> Result<super::types::AgentServiceCommandResult, String> {
    send_agent_command(command, LogFields::new(), None, AgentRoute::Localhost)
}
