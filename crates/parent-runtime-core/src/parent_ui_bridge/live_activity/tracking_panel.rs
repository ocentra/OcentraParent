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

fn tracking_overview_cards(product_claim: &str) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    vec![
        tracking_card(
            "family-dashboard-rollup",
            "Family dashboard tracking rollup",
            vec![
                ("Status", "read-only".to_string()),
                ("Visible children", "3".to_string()),
                ("Attention items", "2".to_string()),
                ("Retained audit items", "2".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "report-policy-consumer-ui",
            "Tracking report policy consumer UI",
            vec![
                ("Status", "ready".to_string()),
                (
                    "Stored journal refs",
                    "tracking-report-journal | tracking-policy-journal | tracking-retention-journal"
                        .to_string(),
                ),
                (
                    "Stored read-model refs",
                    "tracking-report-read-model | tracking-policy-read-model | tracking-retention-read-model"
                        .to_string(),
                ),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "report-export-ui",
            "Tracking report export UI",
            vec![
                ("Status", "ready".to_string()),
                ("Exported rows", "4".to_string()),
                ("Redacted evidence refs", "4".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "notification-history-intent-ui",
            "Notification history intent UI",
            vec![
                ("Status", "ready".to_string()),
                ("Rows returned", "3".to_string()),
                ("Provider delivery claimed rows", "0".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "parent-action-readiness-ui",
            "Parent action readiness UI",
            vec![
                ("Status", "ready".to_string()),
                ("Rows returned", "9".to_string()),
                ("Action dispatch claimed rows", "0".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "missing-device-ui",
            "Missing device UI",
            vec![
                ("Status", "ready".to_string()),
                ("Rows returned", "4".to_string()),
                ("Manual required rows", "1".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
    ]
}

fn tracking_device_action_cards(product_claim: &str) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    vec![
        tracking_card(
            "child-check-in-request",
            "Child check-in request",
            vec![
                ("Status", "ready".to_string()),
                ("Safe action", "Mark safe".to_string()),
                ("Help action", "Ask for help".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "child-runtime-ui",
            "Child runtime UI",
            vec![
                ("Status", "ready".to_string()),
                ("Disclosure", "Read-only surface".to_string()),
                ("Location consent", "Required".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
        tracking_card(
            "unsupported-manual-platform",
            "Unsupported manual platform",
            vec![
                ("Status", "manual-required".to_string()),
                ("Rows returned", "5".to_string()),
                ("Product claim", product_claim.to_string()),
            ],
        ),
    ]
}
