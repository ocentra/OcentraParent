#[path = "app_game_timer/summary_details.rs"]
mod summary_details;

use self::summary_details::app_game_timer_parent_surface_summary_details_impl;
use super::*;

pub(super) fn app_game_timer_parent_surface_panel_snapshot(
    read_model: Option<&AppGameTimerParentSurfaceReadModel>,
) -> ParentAppGameTimerParentSurfacePanelSnapshot {
    match read_model {
        None => app_game_timer_parent_surface_empty_snapshot(),
        Some(read_model) => app_game_timer_parent_surface_read_model_snapshot(read_model),
    }
}

fn app_game_timer_parent_surface_empty_snapshot() -> ParentAppGameTimerParentSurfacePanelSnapshot {
    let empty_product_claim = "Parent-surface rendering only; active timer state-store is shown only when reported by the service. Live scheduling execution, durable audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.";
    ParentAppGameTimerParentSurfacePanelSnapshot {
        eyebrow: "Runtime reference".to_string(),
        title: "App/game timer parent surface".to_string(),
        body: "Service-backed parent-surface timer rows only; no runtime scheduling or enforcement is claimed.".to_string(),
        load_state: "unavailable".to_string(),
        summary_details: vec![
            app_game_detail("Status", "unavailable"),
            app_game_detail("Product claim", empty_product_claim),
        ],
        parent_action_rows: Vec::new(),
        parent_preference_setup_rows: Vec::new(),
        rows: Vec::new(),
        empty_message: "No app/game timer parent-surface read model has been reported yet.".to_string(),
        product_claim: empty_product_claim.to_string(),
    }
}

fn app_game_timer_parent_surface_read_model_snapshot(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> ParentAppGameTimerParentSurfacePanelSnapshot {
    let product_claim = app_game_timer_parent_surface_product_claim(read_model);
    ParentAppGameTimerParentSurfacePanelSnapshot {
        eyebrow: "Runtime reference".to_string(),
        title: "App/game timer parent surface".to_string(),
        body: "Service-backed parent-surface timer rows only; no runtime scheduling or enforcement is claimed.".to_string(),
        load_state: app_game_timer_parent_surface_load_state(read_model),
        summary_details: app_game_timer_parent_surface_summary_details(read_model, &product_claim),
        parent_action_rows: app_game_timer_parent_surface_parent_action_rows(read_model, &product_claim),
        parent_preference_setup_rows: app_game_timer_parent_surface_parent_preference_setup_rows(read_model, &product_claim),
        rows: app_game_timer_parent_surface_rows(read_model, &product_claim),
        empty_message: "No app/game timer parent-surface read model has been reported yet.".to_string(),
        product_claim,
    }
}

fn app_game_timer_parent_surface_summary_details(
    read_model: &AppGameTimerParentSurfaceReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    app_game_timer_parent_surface_summary_details_impl(read_model, product_claim)
}

fn app_game_timer_parent_surface_summary_claim_details(
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail("Capture scope", "parent-surface read model"),
        app_game_detail("Claim focus", "read-only runtime timer state"),
        app_game_detail("Product claim", product_claim),
    ]
}

fn app_game_timer_parent_surface_parent_action_rows(
    read_model: &AppGameTimerParentSurfaceReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelRowSnapshot> {
    read_model
        .child_ux_parent_surface_intent_records
        .iter()
        .map(|record| {
            app_game_panel_row(
                record.parent_surface_intent_reference_id.clone(),
                vec![
                    app_game_detail(
                        "Target",
                        app_game_timer_target_label(record.target_domain.as_str()),
                    ),
                    app_game_detail("Status", record.parent_surface_status.as_str()),
                    app_game_detail("History visibility", record.history_visibility.as_str()),
                    app_game_detail(
                        "Preference visibility",
                        record.preference_visibility.as_str(),
                    ),
                    app_game_detail(
                        "Drill-in refs",
                        app_game_join_strings(&record.drill_in_reference_ids),
                    ),
                    app_game_detail(
                        "Manual proof refs",
                        app_game_join_strings(&record.manual_proof_reference_ids),
                    ),
                    app_game_detail(
                        "Adapter dispatch",
                        app_game_claimed_value(record.adapter_dispatch_claimed),
                    ),
                    app_game_detail(
                        "Child delivery",
                        app_game_claimed_value(record.child_delivery_claimed),
                    ),
                    app_game_detail(
                        "Platform state",
                        app_game_claimed_value(record.platform_enforcement_claimed),
                    ),
                    app_game_detail("Product claim", product_claim),
                ],
            )
        })
        .collect()
}

fn app_game_timer_parent_surface_parent_preference_setup_rows(
    read_model: &AppGameTimerParentSurfaceReadModel,
    product_claim: &str,
) -> Vec<ParentAppGameActionRowSnapshot> {
    read_model
        .child_ux_parent_preference_setup_records
        .iter()
        .map(|record| ParentAppGameActionRowSnapshot {
            title: record.parent_preference_setup_reference_id.clone(),
            details: vec![
                app_game_detail(
                    "Target",
                    app_game_timer_target_label(record.target_domain.as_str()),
                ),
                app_game_detail("Draft status", record.draft_status.as_str()),
                app_game_detail(
                    "Parent preference setup request status",
                    record.parent_preference_setup_request_status.as_str(),
                ),
                app_game_detail(
                    "Parent preference setup request refs",
                    app_game_join_strings(&record.parent_preference_setup_request_reference_ids),
                ),
                app_game_detail(
                    "Drill-in refs",
                    app_game_join_strings(&record.drill_in_reference_ids),
                ),
                app_game_detail(
                    "Manual proof refs",
                    app_game_join_strings(&record.manual_proof_reference_ids),
                ),
                app_game_detail(
                    "Adapter dispatch",
                    app_game_claimed_value(record.adapter_dispatch_claimed),
                ),
                app_game_detail(
                    "Child delivery",
                    app_game_claimed_value(record.child_delivery_claimed),
                ),
                app_game_detail(
                    "Platform state",
                    app_game_claimed_value(record.platform_enforcement_claimed),
                ),
                app_game_detail("Product claim", product_claim),
            ],
            action_label: (record.parent_preference_setup_request_status == "request-ready")
                .then(|| "Request parent setup".to_string()),
            action_payload: (record.parent_preference_setup_request_status == "request-ready")
                .then(|| {
                    app_game_timer_parent_preference_setup_payload(
                        record,
                        read_model.generated_at.as_str(),
                    )
                }),
        })
        .collect()
}

fn app_game_timer_parent_surface_rows(
    read_model: &AppGameTimerParentSurfaceReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelRowSnapshot> {
    read_model
        .rows
        .iter()
        .map(|row| {
            app_game_panel_row(
                row.row_id.clone(),
                vec![
                    app_game_detail(
                        "Target",
                        app_game_timer_target_label(row.target_domain.as_str()),
                    ),
                    app_game_detail(
                        "Status",
                        app_game_timer_surface_state_label(row.timer_surface_state.as_str()),
                    ),
                    app_game_detail("Row count", row.row_count.to_string()),
                    app_game_detail(
                        "Evidence references",
                        app_game_join_strings(&row.evidence_reference_ids),
                    ),
                    app_game_detail("Product claim", product_claim),
                ],
            )
        })
        .collect()
}
