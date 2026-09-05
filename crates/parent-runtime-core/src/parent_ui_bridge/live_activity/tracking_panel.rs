#[path = "tracking_panel/helpers.rs"]
mod helpers;
#[path = "tracking_panel/product_cards.rs"]
mod product_cards;
#[path = "tracking_panel/row_details.rs"]
mod row_details;

use self::helpers::*;
use self::product_cards::*;
use self::row_details::*;
use super::*;

pub(super) fn activity_tracking_panel_snapshot_impl(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
) -> ParentTrackingStatusPanelSnapshot {
    activity_tracking_panel_snapshot_for_surface(
        read_model_result,
        write_result,
        TrackingPanelSurface::Product,
    )
}

pub(super) fn activity_tracking_proof_panel_snapshot_impl(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
) -> ParentTrackingStatusPanelSnapshot {
    activity_tracking_panel_snapshot_for_surface(
        read_model_result,
        write_result,
        TrackingPanelSurface::Proof,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrackingPanelSurface {
    Product,
    Proof,
}

fn activity_tracking_panel_snapshot_for_surface(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
    surface: TrackingPanelSurface,
) -> ParentTrackingStatusPanelSnapshot {
    let product_claim = "Shows only records reported by the local Rust service. It does not prove child delivery, live device execution, provider delivery, or permission to take actions.".to_string();
    let latest_row = read_model_result
        .and_then(|result| result.value.as_ref())
        .and_then(|value| value.rows.first());
    let mut cards = match surface {
        TrackingPanelSurface::Product => {
            tracking_product_cards(read_model_result, write_result, latest_row, &product_claim)
        }
        TrackingPanelSurface::Proof => {
            tracking_static_cards(read_model_result, write_result, latest_row, &product_claim)
        }
    };
    cards.extend(tracking_citation_cards(read_model_result, &product_claim));

    ParentTrackingStatusPanelSnapshot {
        eyebrow: match surface {
            TrackingPanelSurface::Product => "Family tracking".to_string(),
            TrackingPanelSurface::Proof => "Tracking proof".to_string(),
        },
        title: "Tracking status".to_string(),
        body: match surface {
            TrackingPanelSurface::Product => "Current child tracking history, service coverage, custody, and honest connection gaps from the local Rust service.".to_string(),
            TrackingPanelSurface::Proof => "Rust-generated tracking proof surface.".to_string(),
        },
        summary_cards: vec![
            tracking_live_summary_card(read_model_result, &product_claim),
            tracking_service_coverage_card(read_model_result, &product_claim),
        ],
        cards,
        empty_message: "No tracking history has been reported by the local service yet.".to_string(),
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
    card.details.extend(
        read_model_result
            .filter(|result| !result.ok)
            .and_then(|result| result.reason.as_ref())
            .map(serialized_enum_label)
            .map(|reason| tracking_detail("Reason", reason)),
    );
    card
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
        tracking_evidence_row_details(latest_row, product_claim),
    )
}

fn tracking_static_cards(
    read_model_result: Option<&ParentActivityTrackingReadModelResultSnapshot>,
    write_result: Option<&Value>,
    latest_row: Option<&ParentActivityTrackingReadModelRowSnapshot>,
    product_claim: &str,
) -> Vec<ParentTrackingStatusPanelCardSnapshot> {
    let mut cards = tracking_overview_cards(read_model_result, product_claim);
    cards.push(tracking_retention_settings_card(
        write_result,
        product_claim,
    ));
    cards.push(tracking_evidence_drawer_card(latest_row, product_claim));
    cards.extend(tracking_device_action_cards(product_claim));
    cards
}
