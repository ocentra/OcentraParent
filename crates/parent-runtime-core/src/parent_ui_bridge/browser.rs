use super::*;
use crate::parent_ui_bridge::route_snapshot::dependencies::ParentRouteSnapshotDependencies;

#[path = "browser_panels_social.rs"]
mod social;

pub(super) fn browser_route_panels_snapshot(
    route: &ParentRouteId,
    loaded: &ParentRouteSnapshotDependencies,
) -> Option<ParentRouteBrowserPanelsSnapshot> {
    if !matches!(route, ParentRouteId::Browser) {
        return None;
    }

    Some(ParentRouteBrowserPanelsSnapshot {
        browser_parent_explanation: Some(browser_contract_only_panel_snapshot(
            "Browser parent explanations",
            "Schema-backed parent explanations show evidence, model, policy, action, child experience, fallback, and audit sections only when a validated browser AI explanation bundle is present.",
            "0 parent explanation rows",
            "No browser parent explanation bundle has been reported yet.",
            "Rendered parent explanation surface only; runtime service delivery, final policy authority, browser mutation, enforcement, remote AI, and raw page or prompt content remain unclaimed.",
        )),
        social_audit_explanation: Some(social::audit_explanation_panel(
            loaded.social_audit_explanation_snapshot.as_ref(),
        )),
        social_alert_report: Some(social::alert_report_panel(
            loaded.social_alert_report_snapshot.as_ref(),
        )),
        social_alert_report_parent_surface: Some(social::alert_report_parent_surface_panel(
            loaded.social_alert_report_parent_surface_snapshot.as_ref(),
        )),
        social_parent_notification_delivery: Some(social::parent_notification_delivery_panel(
            loaded.social_parent_notification_delivery_snapshot.as_ref(),
        )),
        social_dashboard: Some(social::dashboard_panel(
            loaded.social_dashboard_snapshot.as_ref(),
        )),
        browser_action_intent_stream_status: Some(browser_status_panel_snapshot(
            "Browser action-intent stream status",
            "Rust-owned route snapshot of browser action-intent stream status for the Browser route.",
            "0 action candidates",
            "No browser action-intent stream status has been reported yet.",
            "Browser runtime action-intent stream status only; local outbox handoff, child accepted refs, and parent read-model refs may be visible, while adapter dispatch, child intervention execution, browser mutation, final policy execution, unmanaged exact URL support, and enforcement remain unclaimed.",
        )),
        browser_social_provider_receipt_stream_status: Some(browser_status_panel_snapshot(
            "Social provider receipt stream status",
            "Rust-owned route snapshot of social provider receipt stream status for the Browser route.",
            "0 receipt boundary rows",
            "No social provider receipt stream status has been reported yet.",
            "Browser runtime social provider receipt stream status only; provider delivery, receipt ingestion, parent notification delivery, report delivery, final policy execution, connector/native runtime, and enforcement remain unclaimed.",
        )),
        browser_social_provider_receipt_ingestion_readiness_status: Some(
            browser_status_panel_snapshot(
                "Social provider receipt ingestion readiness",
                "Rust-owned route snapshot of receipt ingestion readiness for the Browser route.",
                "0 readiness rows",
                "No social provider receipt ingestion readiness status has been reported yet.",
                "Browser runtime social provider receipt ingestion readiness status only; provider delivery, receipt ingestion runtime, webhook runtime, credentials, observed provider receipts, parent notification delivery, report delivery, final policy execution, connector/native runtime, browser mutation, child intervention, unmanaged exact URL support, and enforcement remain unclaimed.",
            ),
        ),
    })
}

fn browser_contract_only_panel_snapshot(
    title: &str,
    body: &str,
    summary: &str,
    empty_message: &str,
    product_claim: &str,
) -> ParentBrowserPanelSnapshot {
    ParentBrowserPanelSnapshot {
        eyebrow: BROWSER_PANEL_EYEBROW.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        summary: summary.to_string(),
        summary_details: browser_panel_summary_details(summary, product_claim),
        rows: Vec::new(),
        empty_message: empty_message.to_string(),
        product_claim: product_claim.to_string(),
    }
}

fn browser_status_panel_snapshot(
    title: &str,
    body: &str,
    summary: &str,
    empty_message: &str,
    product_claim: &str,
) -> ParentBrowserPanelSnapshot {
    ParentBrowserPanelSnapshot {
        eyebrow: BROWSER_PANEL_EYEBROW.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        summary: summary.to_string(),
        summary_details: browser_panel_summary_details(summary, product_claim),
        rows: vec![ParentBrowserPanelRowSnapshot {
            key: title.to_string(),
            title: "Status projection".to_string(),
            details: vec![
                browser_panel_detail("Status", "unavailable"),
                browser_panel_detail("Summary", summary),
                browser_panel_detail("Product claim", product_claim),
            ],
        }],
        empty_message: empty_message.to_string(),
        product_claim: product_claim.to_string(),
    }
}

fn browser_panel_summary_details(
    summary: &str,
    product_claim: &str,
) -> Vec<ParentBrowserPanelDetailSnapshot> {
    vec![
        browser_panel_detail("Rows returned", "0"),
        browser_panel_detail("Status", BROWSER_PANEL_NOT_REPORTED),
        browser_panel_detail("Summary", summary),
        browser_panel_detail("Product claim", product_claim),
    ]
}

fn browser_panel_detail(label: &str, value: &str) -> ParentBrowserPanelDetailSnapshot {
    ParentBrowserPanelDetailSnapshot {
        label: label.to_string(),
        value: value.to_string(),
    }
}
