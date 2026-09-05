use super::*;

pub(super) fn tracking_service_coverage_card(
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

pub(super) fn tracking_evidence_row_details(
    latest_row: Option<&ParentActivityTrackingReadModelRowSnapshot>,
    product_claim: &str,
) -> Vec<(&'static str, String)> {
    let mut details = match latest_row {
        Some(row) => tracking_reported_row_details(row),
        None => tracking_unreported_row_details(),
    };
    details.push(("Product claim", product_claim.to_string()));
    details
}

pub(super) fn tracking_citation_row_details(
    row: &ParentActivityTrackingReadModelRowSnapshot,
    product_claim: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("Observed at", row.observed_at.clone()),
        ("Device", row.device_id.to_string()),
        ("Platform", row.platform.clone()),
        ("Observer source", row.observer.clone()),
        ("Activity kind", row.kind.clone()),
        ("Subject kind", row.subject_kind.clone()),
        ("Subject ID", row.subject_id.to_string()),
        (
            "Subject name",
            tracking_option_value(row.subject_display_name.as_deref()),
        ),
        (
            "Capability status",
            tracking_option_value(row.capability_status.as_deref()),
        ),
        ("Query visibility", row.query_visibility.clone()),
        (
            "Deleted at",
            tracking_option_value(row.deleted_at.as_deref()),
        ),
        ("Evidence refs", tracking_refs(&row.evidence_reference_ids)),
        (
            "Deleted evidence refs",
            tracking_refs(&row.deleted_evidence_reference_ids),
        ),
        ("Product claim", product_claim.to_string()),
    ]
}

pub(super) fn tracking_citation_cards(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    read_model_result
        .and_then(|result| result.value.as_ref())
        .map(|read_model| {
            read_model
                .rows
                .iter()
                .take(6)
                .enumerate()
                .map(|(index, row)| {
                    tracking_card(
                        &format!("tracking-citation-{index}"),
                        &format!(
                            "{} · citation {}",
                            tracking_activity_label(&row.kind),
                            index + 1
                        ),
                        tracking_citation_row_details(row, product_claim),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn tracking_reported_row_details(
    row: &ParentActivityTrackingReadModelRowSnapshot,
) -> Vec<(&'static str, String)> {
    vec![
        ("Source event ID", row.event_id.to_string()),
        ("Observed at", row.observed_at.clone()),
        ("Device", row.device_id.to_string()),
        ("Observer source", row.observer.clone()),
        ("Subject kind", row.subject_kind.clone()),
        ("Subject ID", row.subject_id.to_string()),
        (
            "Subject name",
            tracking_option_value(row.subject_display_name.as_deref()),
        ),
        (
            "Capability status",
            tracking_option_value(row.capability_status.as_deref()),
        ),
        ("Query visibility", row.query_visibility.clone()),
        (
            "Deleted at",
            tracking_option_value(row.deleted_at.as_deref()),
        ),
        ("Evidence refs", tracking_refs(&row.evidence_reference_ids)),
        (
            "Deleted evidence refs",
            tracking_refs(&row.deleted_evidence_reference_ids),
        ),
    ]
}

fn tracking_unreported_row_details() -> Vec<(&'static str, String)> {
    vec![
        ("Source event ID", tracking_not_reported()),
        ("Observed at", tracking_not_reported()),
        ("Device", tracking_not_reported()),
        ("Observer source", tracking_not_reported()),
        ("Subject kind", tracking_not_reported()),
        ("Subject ID", tracking_not_reported()),
        ("Subject name", tracking_not_reported()),
        ("Capability status", tracking_not_reported()),
        ("Query visibility", tracking_not_reported()),
        ("Deleted at", tracking_not_reported()),
        ("Evidence refs", tracking_not_reported()),
        ("Deleted evidence refs", tracking_not_reported()),
    ]
}

fn tracking_not_reported() -> String {
    TRACKING_STATUS_NOT_REPORTED.to_string()
}
