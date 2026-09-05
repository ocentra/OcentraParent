use ocentra_parent_agent_protocol::transport::AgentCommandName;

use crate::agent_service_client::snapshots_social::{
    social_alert_report_parent_surface_snapshot_from_result,
    social_alert_report_snapshot_from_result, social_audit_explanation_snapshot_from_result,
    social_dashboard_snapshot_from_result,
    social_parent_notification_delivery_snapshot_from_result,
};
use crate::agent_service_client::types::AgentServiceCommandResult;
use crate::parent_ui_bridge::route_requirements::route_requires_browser_read_models;
use crate::parent_ui_bridge::route_snapshot::dependencies::ParentRouteSnapshotDependencies;
use crate::parent_ui_bridge::ParentRouteId;

use super::ParentAgentServiceProjection;

impl ParentAgentServiceProjection {
    pub(super) fn project_browser_social_dependencies(
        &mut self,
        route: &ParentRouteId,
        loaded: &mut ParentRouteSnapshotDependencies,
    ) {
        let required = route_requires_browser_read_models(route);
        loaded.social_dashboard_snapshot = self.project_optional_social(
            required,
            &AgentCommandName::AgentBrowserSocialDashboardReadModelGet,
            social_dashboard_snapshot_from_result,
        );
        loaded.social_audit_explanation_snapshot = self.project_optional_social(
            required,
            &AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet,
            social_audit_explanation_snapshot_from_result,
        );
        loaded.social_alert_report_snapshot = self.project_optional_social(
            required,
            &AgentCommandName::AgentBrowserSocialAlertReportReadModelGet,
            social_alert_report_snapshot_from_result,
        );
        loaded.social_alert_report_parent_surface_snapshot = self.project_optional_social(
            required,
            &AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet,
            social_alert_report_parent_surface_snapshot_from_result,
        );
        loaded.social_parent_notification_delivery_snapshot = self.project_optional_social(
            required,
            &AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet,
            social_parent_notification_delivery_snapshot_from_result,
        );
    }

    fn project_optional_social<T>(
        &mut self,
        required: bool,
        command: &AgentCommandName,
        project: impl FnOnce(AgentServiceCommandResult) -> Result<T, String>,
    ) -> Option<T> {
        required
            .then(|| {
                self.take_optional_social(command)
                    .and_then(|result| project(result).ok())
            })
            .flatten()
    }

    fn take_optional_social(
        &mut self,
        command: &AgentCommandName,
    ) -> Option<AgentServiceCommandResult> {
        let position = self
            .responses
            .iter()
            .position(|response| &response.result.command == command)?;
        Some(self.responses.remove(position).result)
    }
}
