use ocentra_parent_agent_protocol::social_alert_report_parent_surface_read_model::SocialAlertReportParentSurfaceReadModelRow;
use ocentra_parent_agent_protocol::social_alert_report_read_model::{
    SocialAlertReportIntent, SocialAlertReportProviderStatusRow,
};
use ocentra_parent_agent_protocol::social_parent_notification_delivery_read_model::SocialParentNotificationDeliveryReadinessRow;
use ocentra_parent_agent_protocol::{SocialAuditExplanationEntry, SocialDashboardPanel};

use crate::agent_service_client::types::{
    SocialAlertReportAgentServiceSnapshot, SocialAlertReportParentSurfaceAgentServiceSnapshot,
    SocialAuditExplanationAgentServiceSnapshot, SocialDashboardAgentServiceSnapshot,
    SocialParentNotificationDeliveryAgentServiceSnapshot,
};
use ocentra_schema::parent_ui_bridge::{
    ParentBrowserPanelDetailSnapshot, ParentBrowserPanelRowSnapshot, ParentBrowserPanelSnapshot,
};

use super::{browser_contract_only_panel_snapshot, BROWSER_PANEL_EYEBROW};

const DASHBOARD_CLAIM: &str = "Rendered parent surface only; social runtime data fetch, notifications, connector authorization, native app control, policy execution, and enforcement remain unclaimed.";
const AUDIT_CLAIM: &str = "Rendered parent explanation surface only; runtime audit-store delivery, notifications, connector authorization, native app control, final policy execution, and enforcement remain unclaimed.";
const ALERT_CLAIM: &str = "Rendered parent alert/report intent surface only; provider delivery, report delivery, notification UI delivery, final policy execution, and enforcement remain unclaimed.";
const PARENT_SURFACE_CLAIM: &str = "Parent-surface status projection only; notification UI delivery, provider delivery, receipt ingestion, final policy execution, and enforcement remain unclaimed.";
const NOTIFICATION_CLAIM: &str = "Parent report readiness projection only; parent notification UI delivery, external runtime report delivery, provider delivery, final policy execution, and enforcement remain unclaimed.";

pub(super) fn dashboard_panel(
    snapshot: Option<&SocialDashboardAgentServiceSnapshot>,
) -> ParentBrowserPanelSnapshot {
    let Some(snapshot) = snapshot else {
        return unavailable_dashboard_panel();
    };
    let read_model = &snapshot.read_model;
    reported_panel(
        "Social dashboard",
        "Service-backed social review rows expose status, evidence references, and manual boundaries without raw social content.",
        "social dashboard rows",
        &read_model.generated_at,
        read_model.panels.iter().map(dashboard_row).collect(),
        "No social dashboard rows were reported.",
        DASHBOARD_CLAIM,
    )
}

pub(super) fn audit_explanation_panel(
    snapshot: Option<&SocialAuditExplanationAgentServiceSnapshot>,
) -> ParentBrowserPanelSnapshot {
    let Some(snapshot) = snapshot else {
        return browser_contract_only_panel_snapshot(
            "Social explanations",
            "Schema-backed social explanations show evidence, policy, approval, memory, connector, native, manual, and audit refs without raw social content.",
            "0 social explanation rows",
            "No social audit explanation snapshot has been reported yet.",
            AUDIT_CLAIM,
        );
    };
    let read_model = &snapshot.read_model;
    reported_panel(
        "Social explanations",
        "Service-backed social explanations expose parent-visible decisions and evidence references without raw account, video, or message content.",
        "social explanation rows",
        &read_model.captured_at,
        read_model.entries.iter().map(audit_row).collect(),
        "No social explanation rows were reported.",
        AUDIT_CLAIM,
    )
}

pub(super) fn alert_report_panel(
    snapshot: Option<&SocialAlertReportAgentServiceSnapshot>,
) -> ParentBrowserPanelSnapshot {
    let Some(snapshot) = snapshot else {
        return browser_contract_only_panel_snapshot(
            "Social alerts and reports",
            "Schema-backed social alert and report intents show ref-only local outbox or manual-required rows without provider delivery or enforcement claims.",
            "0 social alert/report rows",
            "No social alert/report read model has been reported yet.",
            ALERT_CLAIM,
        );
    };
    let read_model = &snapshot.read_model;
    let rows = read_model
        .intents
        .iter()
        .map(alert_intent_row)
        .chain(
            read_model
                .provider_status_rows
                .iter()
                .map(alert_provider_row),
        )
        .collect();
    reported_panel(
        "Social alerts and reports",
        "Service-backed social alert intents and provider readiness are ref-only; raw social content is excluded.",
        "social alert/report rows",
        &read_model.generated_at,
        rows,
        "No social alert or provider readiness rows were reported.",
        ALERT_CLAIM,
    )
}

pub(super) fn alert_report_parent_surface_panel(
    snapshot: Option<&SocialAlertReportParentSurfaceAgentServiceSnapshot>,
) -> ParentBrowserPanelSnapshot {
    let Some(snapshot) = snapshot else {
        return browser_contract_only_panel_snapshot(
            "Social parent surface status",
            "Service-backed parent-surface status shows provider and preference handoff state without rendering notification, preference, history, or delivery UI.",
            "0 parent surface rows",
            "No parent-surface status snapshot has been reported yet.",
            PARENT_SURFACE_CLAIM,
        );
    };
    let read_model = &snapshot.read_model;
    reported_panel(
        "Social parent surface status",
        "Service-backed status exposes provider and preference handoff readiness without claiming notification or delivery UI.",
        "parent surface rows",
        &read_model.generated_at,
        read_model.rows.iter().map(parent_surface_row).collect(),
        "No social parent-surface rows were reported.",
        PARENT_SURFACE_CLAIM,
    )
}

pub(super) fn parent_notification_delivery_panel(
    snapshot: Option<&SocialParentNotificationDeliveryAgentServiceSnapshot>,
) -> ParentBrowserPanelSnapshot {
    let Some(snapshot) = snapshot else {
        return browser_contract_only_panel_snapshot(
            "Social parent notification delivery readiness",
            "Service-backed readiness projection shows parent-owned report status and manual gaps without claiming notification UI delivery, provider delivery, final policy execution, or enforcement.",
            "0 parent notification readiness rows",
            "No parent notification delivery readiness snapshot has been reported yet.",
            NOTIFICATION_CLAIM,
        );
    };
    let read_model = &snapshot.read_model;
    reported_panel(
        "Social parent notification delivery readiness",
        "Service-backed readiness exposes parent-owned report state and manual gaps without claiming external delivery.",
        "parent notification readiness rows",
        &read_model.generated_at,
        read_model.rows.iter().map(notification_row).collect(),
        "No social parent-notification readiness rows were reported.",
        NOTIFICATION_CLAIM,
    )
}

fn unavailable_dashboard_panel() -> ParentBrowserPanelSnapshot {
    browser_contract_only_panel_snapshot(
        "Social dashboard",
        "Schema-backed social rows show parent-review and manual-required status only; runtime fetch, connector, native app, policy execution, and enforcement remain unclaimed.",
        "0 social dashboard rows",
        "No social dashboard snapshot has been reported yet.",
        DASHBOARD_CLAIM,
    )
}

fn reported_panel(
    title: &str,
    body: &str,
    row_label: &str,
    generated_at: &str,
    rows: Vec<ParentBrowserPanelRowSnapshot>,
    empty_message: &str,
    product_claim: &str,
) -> ParentBrowserPanelSnapshot {
    let summary = format!("{} {row_label}", rows.len());
    ParentBrowserPanelSnapshot {
        eyebrow: BROWSER_PANEL_EYEBROW.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        summary: summary.clone(),
        summary_details: vec![
            detail("Rows returned", &rows.len().to_string()),
            detail("Status", "reported"),
            detail("Generated at", generated_at),
            detail("Product claim", product_claim),
        ],
        rows,
        empty_message: empty_message.to_string(),
        product_claim: product_claim.to_string(),
    }
}

fn dashboard_row(row: &SocialDashboardPanel) -> ParentBrowserPanelRowSnapshot {
    panel_row(
        &row.panel_id,
        &row.panel_kind,
        vec![
            detail("Status", &row.status),
            detail("Primary action", &row.primary_action),
            detail("Severity", &row.severity),
            detail("Reasons", &joined(&row.reasons)),
            detail("Evidence references", &joined(&row.source_evidence_refs)),
        ],
    )
}

fn audit_row(row: &SocialAuditExplanationEntry) -> ParentBrowserPanelRowSnapshot {
    let evidence_refs = row
        .evidence_links
        .iter()
        .map(|link| link.evidence_ref.as_str())
        .collect::<Vec<_>>();
    panel_row(
        &row.event_id,
        &row.subject_kind,
        vec![
            detail("Status", &row.status),
            detail("Decision state", &row.decision_state),
            detail("Action candidate", &row.action_candidate),
            detail("Policy reasons", &joined(&row.policy_reason_codes)),
            detail("Explanation reasons", &joined(&row.explanation_reasons)),
            detail("Evidence references", &joined_refs(&evidence_refs)),
            detail("Audit references", &joined(&row.audit_refs)),
        ],
    )
}

fn alert_intent_row(row: &SocialAlertReportIntent) -> ParentBrowserPanelRowSnapshot {
    let evidence_refs = row
        .evidence_references
        .iter()
        .map(|reference| reference.evidence_reference_id.as_str())
        .collect::<Vec<_>>();
    panel_row(
        &row.alert_report_intent_id,
        &row.intent_kind,
        vec![
            detail("Status", &row.intent_status),
            detail("Priority", &row.priority),
            detail("Severity", &row.severity),
            detail("Reason", &row.notification_reason_code),
            detail("Delivery claim", &row.delivery_claim_state),
            detail("Evidence references", &joined_refs(&evidence_refs)),
        ],
    )
}

fn alert_provider_row(row: &SocialAlertReportProviderStatusRow) -> ParentBrowserPanelRowSnapshot {
    panel_row(
        &row.status_entry_id,
        "Provider readiness",
        vec![
            detail("Source intent", &row.source_intent_ref),
            detail("Preflight", &row.source_preflight_status),
            detail("Provider status", &row.provider_status),
            detail("Proof state", &row.status_proof_state),
            detail("Delivery claim", &row.delivery_claim_state),
        ],
    )
}

fn parent_surface_row(
    row: &SocialAlertReportParentSurfaceReadModelRow,
) -> ParentBrowserPanelRowSnapshot {
    panel_row(
        &row.surface_row_id,
        "Parent surface readiness",
        vec![
            detail("Source intent", &row.source_intent_ref),
            detail("Surface status", &row.parent_surface_status),
            detail("History visibility", &row.history_visibility),
            detail("Preference visibility", &row.preference_visibility),
            detail("Audit references", &joined(&row.audit_refs)),
        ],
    )
}

fn notification_row(
    row: &SocialParentNotificationDeliveryReadinessRow,
) -> ParentBrowserPanelRowSnapshot {
    panel_row(
        &row.notification_delivery_readiness_row_id,
        "Parent notification readiness",
        vec![
            detail("Source intent", &row.source_intent_ref),
            detail("Readiness", &row.notification_delivery_readiness_state),
            detail("Report delivery", &row.report_delivery_execution_state),
            detail("Evidence references", &joined(&row.source_evidence_refs)),
            detail("Audit references", &joined(&row.source_audit_refs)),
        ],
    )
}

fn panel_row(
    key: &str,
    title: &str,
    details: Vec<ParentBrowserPanelDetailSnapshot>,
) -> ParentBrowserPanelRowSnapshot {
    ParentBrowserPanelRowSnapshot {
        key: key.to_string(),
        title: title.to_string(),
        details,
    }
}

fn detail(label: &str, value: &str) -> ParentBrowserPanelDetailSnapshot {
    ParentBrowserPanelDetailSnapshot {
        label: label.to_string(),
        value: value.to_string(),
    }
}

fn joined(values: &[String]) -> String {
    let refs = values.iter().map(String::as_str).collect::<Vec<_>>();
    joined_refs(&refs)
}

fn joined_refs(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}
