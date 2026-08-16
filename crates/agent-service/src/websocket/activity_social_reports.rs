use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};
use std::{future::Future, pin::Pin};

use crate::activity_api::{
    social_alert_report_parent_surface_read_model_payload::build_browser_social_alert_report_parent_surface_read_model_report,
    social_alert_report_read_model_payload::build_browser_social_alert_report_read_model_report,
    social_audit_explanation_read_model_payload::build_browser_social_audit_explanation_read_model_report,
    social_dashboard_read_model_payload::build_browser_social_dashboard_read_model_report,
    social_parent_notification_delivery_read_model_payload::build_browser_social_parent_notification_delivery_read_model_report,
};

use super::basic_reports::build_log_snapshot_report;

pub(super) fn build_activity_social_report(
    command: AgentCommandEnvelope,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        match command.command.clone() {
            AgentCommandName::AgentBrowserSocialDashboardReadModelGet => {
                build_browser_social_dashboard_read_model_report(command).await
            }
            AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet => {
                build_browser_social_audit_explanation_read_model_report(command).await
            }
            AgentCommandName::AgentBrowserSocialAlertReportReadModelGet => {
                build_browser_social_alert_report_read_model_report(command).await
            }
            AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet => {
                build_browser_social_alert_report_parent_surface_read_model_report(command).await
            }
            AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet => {
                build_browser_social_parent_notification_delivery_read_model_report(command).await
            }
            _ => build_log_snapshot_report(command),
        }
    })
}
