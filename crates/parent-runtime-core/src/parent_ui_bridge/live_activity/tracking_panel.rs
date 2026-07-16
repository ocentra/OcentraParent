#[path = "tracking_panel/helpers.rs"]
mod helpers;

use self::helpers::*;
use super::*;

pub(super) fn activity_tracking_panel_snapshot_impl(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
) -> ParentTrackingStatusPanelSnapshot {
    let product_claim = "Tracking status is Rust-read-model-backed UI only; provider delivery, child delivery, physical-device execution, and authority actions remain unclaimed unless an explicit proof row states otherwise.".to_string();
    let latest_row = read_model_result
        .and_then(|result| result.value.as_ref())
        .and_then(|value| value.rows.first());
    let mut cards = tracking_static_cards(write_result, latest_row, &product_claim);
    cards.extend(tracking_status_proof_cards(&product_claim));
    cards.extend(tracking_citation_cards(read_model_result, &product_claim));

    ParentTrackingStatusPanelSnapshot {
        eyebrow: "First target".to_string(),
        title: "Tracking status".to_string(),
        body: "Rust-generated tracking proof surface.".to_string(),
        summary_cards: vec![
            tracking_live_summary_card(read_model_result, &product_claim),
            tracking_service_coverage_card(read_model_result, &product_claim),
        ],
        cards,
        empty_message: "No tracking activity is available yet.".to_string(),
        product_claim,
    }
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

fn tracking_static_cards(
    write_result: Option<&Value>,
    latest_row: Option<&ParentActivityTrackingReadModelRowSnapshot>,
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    let mut cards = tracking_overview_cards(product_claim);
    cards.push(tracking_retention_settings_card(
        write_result,
        product_claim,
    ));
    cards.push(tracking_evidence_drawer_card(latest_row, product_claim));
    cards.extend(tracking_device_action_cards(product_claim));
    cards
}
