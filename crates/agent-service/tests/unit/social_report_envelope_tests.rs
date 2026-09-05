use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::social_alert_report_parent_surface_read_model_payload::build_browser_social_alert_report_parent_surface_read_model_report;
use crate::social_alert_report_read_model_payload::build_browser_social_alert_report_read_model_report;
use crate::social_audit_explanation_read_model_payload::build_browser_social_audit_explanation_read_model_report;
use crate::social_dashboard_read_model_payload::build_browser_social_dashboard_read_model_report;
use crate::social_parent_notification_delivery_read_model_payload::build_browser_social_parent_notification_delivery_read_model_report;
use crate::social_source_custody_mutation_payload::build_browser_social_source_custody_mutation_report;

#[tokio::test]
async fn social_report_builders_preserve_command_correlation_and_portal_target() {
    let dashboard = command(
        AgentCommandName::AgentBrowserSocialDashboardReadModelGet,
        "social-dashboard-command",
    );
    let dashboard_event = build_browser_social_dashboard_read_model_report(dashboard.clone()).await;
    assert_report_envelope(
        &dashboard_event,
        &dashboard,
        &AgentEventName::AgentBrowserSocialDashboardReadModelReported,
        constants::event_id::BROWSER_SOCIAL_DASHBOARD_READ_MODEL_REPORTED,
    );

    let audit = command(
        AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet,
        "social-audit-command",
    );
    let audit_event = build_browser_social_audit_explanation_read_model_report(audit.clone()).await;
    assert_report_envelope(
        &audit_event,
        &audit,
        &AgentEventName::AgentBrowserSocialAuditExplanationReadModelReported,
        constants::event_id::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL_REPORTED,
    );

    let alert = command(
        AgentCommandName::AgentBrowserSocialAlertReportReadModelGet,
        "social-alert-command",
    );
    let alert_event = build_browser_social_alert_report_read_model_report(alert.clone()).await;
    assert_report_envelope(
        &alert_event,
        &alert,
        &AgentEventName::AgentBrowserSocialAlertReportReadModelReported,
        constants::event_id::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL_REPORTED,
    );

    let parent_surface = command(
        AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet,
        "social-parent-surface-command",
    );
    let parent_surface_event =
        build_browser_social_alert_report_parent_surface_read_model_report(parent_surface.clone())
            .await;
    assert_report_envelope(
        &parent_surface_event,
        &parent_surface,
        &AgentEventName::AgentBrowserSocialAlertReportParentSurfaceReadModelReported,
        constants::event_id::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL_REPORTED,
    );

    let delivery = command(
        AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet,
        "social-delivery-command",
    );
    let delivery_event =
        build_browser_social_parent_notification_delivery_read_model_report(delivery.clone()).await;
    assert_report_envelope(
        &delivery_event,
        &delivery,
        &AgentEventName::AgentBrowserSocialParentNotificationDeliveryReadModelReported,
        constants::event_id::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL_REPORTED,
    );

    let mutation = command(
        AgentCommandName::AgentBrowserSocialSourceCustodyMutationApply,
        "social-custody-command",
    );
    let mutation_event =
        build_browser_social_source_custody_mutation_report(mutation.clone()).await;
    assert_report_envelope(
        &mutation_event,
        &mutation,
        &AgentEventName::AgentBrowserSocialSourceCustodyMutationApplied,
        constants::event_id::BROWSER_SOCIAL_SOURCE_CUSTODY_MUTATION_APPLIED,
    );
}

fn assert_report_envelope(
    event: &AgentEventEnvelope,
    command: &AgentCommandEnvelope,
    expected_event: &AgentEventName,
    expected_event_id_prefix: &str,
) {
    assert_eq!(event.schema_version, AGENT_PROTOCOL_SCHEMA_VERSION);
    assert_eq!(&event.event, expected_event);
    assert_eq!(event.correlation_id, command.message_id);
    assert_eq!(event.severity, LogLevel::Info);
    assert_eq!(event.source.peer_id, constants::peer::LOCAL_DEV_AGENT);
    assert_eq!(event.source.role, AgentPeerRole::AgentService);
    assert_eq!(event.target, command.source);
    assert_eq!(
        event.event_id,
        format!("{expected_event_id_prefix}-{}", std::process::id())
    );
    assert_eq!(event.snapshot, None);
}

fn command(command: AgentCommandName, message_id: &str) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: message_id.to_string(),
        sent_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command,
        payload: LogFields::new(),
    }
}
