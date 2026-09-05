use crate::agent_service_client::social_loaders::{
    load_social_alert_report, load_social_alert_report_parent_surface,
    load_social_audit_explanation, load_social_dashboard, load_social_parent_notification_delivery,
};
use crate::agent_service_client::types::{
    SocialAlertReportAgentServiceSnapshot, SocialAlertReportParentSurfaceAgentServiceSnapshot,
    SocialAuditExplanationAgentServiceSnapshot, SocialDashboardAgentServiceSnapshot,
    SocialParentNotificationDeliveryAgentServiceSnapshot,
};
use crate::parent_ui_bridge::route_requirements::route_requires_browser_read_models;
use crate::parent_ui_bridge::ParentRouteId;

#[derive(Default)]
pub(super) struct BrowserSocialDependencies {
    pub(super) dashboard: Option<SocialDashboardAgentServiceSnapshot>,
    pub(super) audit_explanation: Option<SocialAuditExplanationAgentServiceSnapshot>,
    pub(super) alert_report: Option<SocialAlertReportAgentServiceSnapshot>,
    pub(super) alert_report_parent_surface:
        Option<SocialAlertReportParentSurfaceAgentServiceSnapshot>,
    pub(super) parent_notification_delivery:
        Option<SocialParentNotificationDeliveryAgentServiceSnapshot>,
}

pub(super) fn load(route: &ParentRouteId) -> BrowserSocialDependencies {
    if !route_requires_browser_read_models(route) {
        return BrowserSocialDependencies::default();
    }
    BrowserSocialDependencies {
        dashboard: load_social_dashboard().ok(),
        audit_explanation: load_social_audit_explanation().ok(),
        alert_report: load_social_alert_report().ok(),
        alert_report_parent_surface: load_social_alert_report_parent_surface().ok(),
        parent_notification_delivery: load_social_parent_notification_delivery().ok(),
    }
}
