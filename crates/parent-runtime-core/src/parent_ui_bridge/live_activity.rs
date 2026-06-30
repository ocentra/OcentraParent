#[path = "live_activity/snapshot.rs"]
mod snapshot;
#[path = "live_activity/tracking_panel.rs"]
mod tracking_panel;

use self::snapshot::live_activity_snapshot_impl;
use self::tracking_panel::activity_tracking_panel_snapshot_impl;
use super::*;
use crate::parent_ui_bridge::ParentRouteLiveActivitySnapshotInput;
use std::fmt::Display;

pub(super) fn live_activity_snapshot(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
) -> Option<ParentRouteLiveActivitySnapshot> {
    live_activity_snapshot_impl(input)
}

fn empty_live_activity_snapshot() -> ParentRouteLiveActivitySnapshot {
    ParentRouteLiveActivitySnapshot {
        recent_summary: None,
        ingest_status: None,
        activity_screen_read_model: None,
        screen_summary_panel: None,
        browser_managed_event: None,
        browser_managed_status: None,
        local_ai_runtime_status_event: None,
        lan_ai_job_event: None,
        parent_assistant_boundary_event: None,
        activity_memory_graph_read_model: None,
        network_flow_event: None,
        network_flow_read_model: None,
        network_evidence_summary: None,
        network_runtime_event_chain_stream: None,
        lan_pairing_browser_discovery_event: None,
        lan_add_device_read_model: None,
        policy_preview_panel: None,
        app_game_notification_parent_surface_panel: None,
        app_game_policy_readiness_panel: None,
        app_game_platform_proof_status_panel: None,
        app_game_child_runtime_transport_receipt_panel: None,
        app_game_adapter_dispatch_panel: None,
        app_game_timer_parent_surface_panel: None,
        browser_intervention_event: None,
        browser_intervention_read_model: None,
        activity_tracking_read_model_event: None,
        activity_tracking_read_model: None,
        activity_tracking_panel: None,
        activity_tracking_retention_settings_write_result: None,
    }
}

const TRACKING_STATUS_NOT_REPORTED: &str = "Not reported";
const TRACKING_STATUS_UNAVAILABLE: &str = "Unavailable";

fn activity_tracking_panel_snapshot(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
) -> ParentTrackingStatusPanelSnapshot {
    activity_tracking_panel_snapshot_impl(read_model_result, write_result)
}

fn tracking_live_summary_card(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    let load_state = tracking_read_model_state(read_model_result);
    let latest_row = read_model_result
        .and_then(|result| result.value.as_ref())
        .and_then(|value| value.rows.first());
    let details = match read_model_result.and_then(|result| result.value.as_ref()) {
        Some(read_model) => vec![
            ("Status", load_state),
            ("Rows returned", read_model.rows.len().to_string()),
            (
                "Last observed",
                tracking_option_value(read_model.latest_observed_at.as_deref()),
            ),
            (
                "Event ID",
                tracking_option_value(read_model.latest_event_id.as_ref()),
            ),
            ("Capability", read_model.capability_status.clone()),
            ("Custody", read_model.custody_label.clone()),
            (
                "Evidence refs",
                latest_row
                    .map(|row| tracking_refs(&row.evidence_reference_ids))
                    .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string()),
            ),
            ("Product claim", product_claim.to_string()),
        ],
        None => vec![
            ("Status", load_state),
            ("Rows returned", "0".to_string()),
            ("Last observed", TRACKING_STATUS_NOT_REPORTED.to_string()),
            ("Event ID", TRACKING_STATUS_NOT_REPORTED.to_string()),
            ("Capability", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Custody", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Evidence refs", TRACKING_STATUS_NOT_REPORTED.to_string()),
            ("Product claim", product_claim.to_string()),
        ],
    };
    let mut card = tracking_card("tracking-live-summary", "Tracking live summary", details);
    if let Some(reason) = read_model_result
        .filter(|result| !result.ok)
        .and_then(|result| result.reason.as_ref())
        .map(serialized_enum_label)
    {
        card.details.push(tracking_detail("Reason", reason));
    }
    card
}

fn tracking_service_coverage_card(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    let load_state = tracking_read_model_state(read_model_result);
    let details = match read_model_result.and_then(|result| result.value.as_ref()) {
        Some(read_model) => vec![
            ("Status", load_state),
            ("Rows returned", read_model.rows.len().to_string()),
            (
                "Deleted evidence",
                read_model.deleted_evidence_reference_ids.len().to_string(),
            ),
            (
                "Activity kinds",
                tracking_count_summary(&read_model.active_kind_counts),
            ),
            (
                "Devices",
                tracking_count_summary(&read_model.active_device_counts),
            ),
            (
                "Capability",
                tracking_count_summary(&read_model.active_capability_status_counts),
            ),
            ("Product claim", product_claim.to_string()),
        ],
        None => vec![
            ("Status", load_state),
            ("Rows returned", "0".to_string()),
            ("Deleted evidence", "0".to_string()),
            ("Activity kinds", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Devices", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Capability", TRACKING_STATUS_UNAVAILABLE.to_string()),
            ("Product claim", product_claim.to_string()),
        ],
    };
    tracking_card(
        "tracking-service-data-coverage",
        "Tracking service data coverage",
        details,
    )
}

fn tracking_retention_settings_card(
    write_result: Option<&Value>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    tracking_card(
        "tracking-retention-settings-ui",
        "Tracking retention settings UI",
        vec![
            (
                "Command ID",
                tracking_json_string(write_result, "commandId"),
            ),
            (
                "Write state",
                tracking_json_string(write_result, "writeState"),
            ),
            (
                "Local service snapshot",
                tracking_json_string(write_result, "localServiceStateSnapshotRef"),
            ),
            ("Product claim", product_claim.to_string()),
        ],
    )
}

fn tracking_evidence_drawer_card(
    latest_row: Option<&ParentActivityTrackingReadModelRowSnapshot>,
    product_claim: &str,
) -> ParentTrackingStatusPanelCardSnapshot {
    tracking_card(
        "tracking-evidence-drawer-ui",
        "Tracking evidence drawer UI",
        vec![
            (
                "Source event ID",
                latest_row
                    .map(|row| row.event_id.to_string())
                    .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string()),
            ),
            (
                "Evidence refs",
                latest_row
                    .map(|row| tracking_refs(&row.evidence_reference_ids))
                    .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string()),
            ),
            (
                "Deleted evidence refs",
                latest_row
                    .map(|row| tracking_refs(&row.deleted_evidence_reference_ids))
                    .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string()),
            ),
            ("Product claim", product_claim.to_string()),
        ],
    )
}

fn tracking_citation_cards(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    read_model_result
        .and_then(|result| result.value.as_ref())
        .map(|read_model| {
            read_model
                .rows
                .iter()
                .take(3)
                .enumerate()
                .map(|(index, row)| {
                    tracking_card(
                        &format!("tracking-citation-{index}"),
                        &format!("Tracking citation {}", index + 1),
                        vec![
                            ("Observed at", row.observed_at.clone()),
                            ("Device", row.device_id.to_string()),
                            ("Platform", row.platform.clone()),
                            ("Observer", row.observer.clone()),
                            ("Activity kind", row.kind.clone()),
                            ("Evidence refs", tracking_refs(&row.evidence_reference_ids)),
                            ("Product claim", product_claim.to_string()),
                        ],
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn tracking_status_proof_cards(product_claim: &str) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    [
        "Tracking off",
        "Permission required",
        "Stale location",
        "Offline",
        "Low accuracy",
        "Nearby place ambiguous",
        "Alert active",
        "Acknowledged",
        "Exception active",
        "Temporary live mode",
        "Missing device mode",
        "Retention deleted",
    ]
    .into_iter()
    .enumerate()
    .map(|(index, title)| {
        tracking_card(
            &format!("tracking-proof-row-{index}"),
            title,
            vec![
                ("Status", "read-only".to_string()),
                ("Proof tier", "fixture".to_string()),
                ("Evidence refs", "ui-fixture".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        )
    })
    .collect()
}

fn tracking_card(
    key: &str,
    title: &str,
    details: Vec<(&str, String)>,
) -> ParentTrackingStatusPanelCardSnapshot {
    ParentTrackingStatusPanelCardSnapshot {
        key: key.to_string(),
        title: title.to_string(),
        details: details
            .into_iter()
            .map(|(label, value)| tracking_detail(label, value))
            .collect(),
    }
}

fn tracking_detail(
    label: &str,
    value: impl Into<String>,
) -> ParentTrackingStatusPanelDetailSnapshot {
    ParentTrackingStatusPanelDetailSnapshot {
        label: label.to_string(),
        value: value.into(),
    }
}

fn tracking_read_model_state(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
) -> String {
    match read_model_result {
        Some(result) if result.ok => "ready".to_string(),
        Some(result) => result
            .reason
            .as_ref()
            .map(serialized_enum_label)
            .unwrap_or_else(|| TRACKING_STATUS_UNAVAILABLE.to_string()),
        None => TRACKING_STATUS_UNAVAILABLE.to_string(),
    }
}

fn tracking_count_summary(
    counts: &[ocentra_schema::parent_ui_bridge::ParentActivityTrackingReadModelCountSnapshot],
) -> String {
    if counts.is_empty() {
        return TRACKING_STATUS_UNAVAILABLE.to_string();
    }
    counts
        .iter()
        .map(|count| format!("{} ({})", count.value, count.count))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn tracking_refs<T>(values: &[T]) -> String
where
    T: Display,
{
    if values.is_empty() {
        TRACKING_STATUS_NOT_REPORTED.to_string()
    } else {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

fn tracking_option_value<T>(value: Option<&T>) -> String
where
    T: Display + ?Sized,
{
    value
        .map(ToString::to_string)
        .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string())
}

fn tracking_json_string(value: Option<&Value>, field: &str) -> String {
    value
        .and_then(|value| value.get(field))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(|| TRACKING_STATUS_NOT_REPORTED.to_string())
}

pub(super) const SCREEN_SUMMARY_DETAIL_SEPARATOR: &str = " | ";
pub(super) const SCREEN_SUMMARY_NOT_REPORTED: &str = "Not reported";
pub(super) const SCREEN_SUMMARY_UNAVAILABLE: &str = "Unavailable";
