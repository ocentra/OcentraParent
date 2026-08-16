use super::*;

pub(super) fn app_game_policy_readiness_panel_snapshot(
    read_model: Option<&AppGamePolicyReadinessReadModel>,
) -> ParentAppGamePanelSnapshot {
    let product_claim = "Policy readiness rows remain parent-visible decision input only. Native enforcement, adapter dispatch, child delivery, provider delivery, and raw private rows remain unclaimed.".to_string();
    match read_model {
        None => app_game_panel_unavailable(
            "Policy readiness",
            "App/game policy readiness",
            "Parent-visible policy readiness for native app and game decision inputs.",
            "No app/game policy readiness has been reported yet.",
            product_claim.as_str(),
        ),
        Some(read_model) => ParentAppGamePanelSnapshot {
            eyebrow: "Policy readiness".to_string(),
            title: "App/game policy readiness".to_string(),
            body: "Parent-visible policy readiness for native app and game decision inputs."
                .to_string(),
            load_state: app_game_policy_load_state(read_model),
            summary_details: app_game_policy_summary_details(read_model, &product_claim),
            rows: app_game_policy_rows(read_model, &product_claim),
            empty_message: "No app/game policy readiness rows were returned.".to_string(),
            product_claim,
        },
    }
}

fn app_game_policy_summary_details(
    read_model: &AppGamePolicyReadinessReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail("Status", app_game_policy_load_state(read_model)),
        app_game_detail("Generated at", read_model.generated_at.as_str()),
        app_game_detail("Custody", read_model.custody_label.as_str()),
        app_game_detail("Capability", read_model.capability_status.as_str()),
        app_game_detail("Rows returned", read_model.returned.to_string()),
        app_game_detail(
            "Evidence claim rows",
            read_model.evidence_claim_row_count.to_string(),
        ),
        app_game_detail(
            "Approval authority rows",
            read_model.approval_authority_row_count.to_string(),
        ),
        app_game_detail(
            "Approval action result rows",
            read_model.approval_action_result_row_count.to_string(),
        ),
        app_game_detail(
            "Platform authority rows",
            read_model.platform_authority_row_count.to_string(),
        ),
        app_game_detail(
            "AI classifier rows",
            read_model.ai_classifier_result_row_count.to_string(),
        ),
        app_game_detail(
            "Category candidate rows",
            read_model.category_candidate_row_count.to_string(),
        ),
        app_game_detail(
            "Unknown review rows",
            read_model.unknown_review_row_count.to_string(),
        ),
        app_game_detail(
            "Policy evaluation",
            app_game_ready_warn_value(read_model.policy_evaluation_ready),
        ),
        app_game_detail(
            "Category routing",
            app_game_ready_warn_value(read_model.category_routing_ready),
        ),
        app_game_detail(
            "Manual review",
            app_game_manual_required_value(read_model.manual_review_required),
        ),
        app_game_detail(
            "Unknown review required",
            app_game_manual_required_value(read_model.unknown_review_required),
        ),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(read_model.adapter_dispatch_claimed),
        ),
        app_game_detail("Product claim", product_claim),
    ]
}

fn app_game_policy_rows(
    read_model: &AppGamePolicyReadinessReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelRowSnapshot> {
    read_model
        .rows
        .iter()
        .map(|row| app_game_policy_row_snapshot(row, product_claim))
        .collect()
}

fn app_game_policy_row_snapshot(
    row: &AppGamePolicyReadinessRow,
    product_claim: &str,
) -> ParentAppGamePanelRowSnapshot {
    app_game_panel_row(
        app_game_policy_kind_label(row.readiness_kind.as_str()),
        vec![
            app_game_detail(
                "Readiness kind",
                app_game_policy_kind_label(row.readiness_kind.as_str()),
            ),
            app_game_detail("Status", row.readiness_state.as_str()),
            app_game_detail("Row count", row.row_count.to_string()),
            app_game_detail("Reason", app_game_policy_row_reason(row)),
            app_game_detail("Evidence references", app_game_join_policy_refs(row)),
            app_game_detail("Product claim", product_claim),
        ],
    )
}
