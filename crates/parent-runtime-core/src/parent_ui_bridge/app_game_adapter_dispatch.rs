use super::*;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightRow;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultRow;

pub(super) fn app_game_adapter_dispatch_panel_snapshot(
    preflight_read_model: Option<&AppGameAdapterDispatchPreflightReadModel>,
    result_read_model: Option<&AppGameAdapterDispatchResultReadModel>,
    execute_result: Option<&Value>,
) -> ParentAppGameAdapterDispatchPanelSnapshot {
    ParentAppGameAdapterDispatchPanelSnapshot {
        eyebrow: "Runtime reference".to_string(),
        title: "App/game adapter dispatch".to_string(),
        body: "Service-backed adapter dispatch preflight and scoped command-result handoff."
            .to_string(),
        preflight_panel: app_game_adapter_dispatch_preflight_panel_snapshot(preflight_read_model),
        result_panel: app_game_adapter_dispatch_result_panel_snapshot(
            result_read_model,
            execute_result,
        ),
        execute_action_label: result_read_model.and_then(|read_model| {
            read_model
                .rows
                .iter()
                .any(|row| row.dispatch_command_result_decision == "command-accepted")
                .then(|| "Execute scoped adapter dispatch".to_string())
        }),
    }
}

fn app_game_adapter_dispatch_preflight_panel_snapshot(
    read_model: Option<&AppGameAdapterDispatchPreflightReadModel>,
) -> ParentAppGamePanelSnapshot {
    let product_claim = "Adapter dispatch preflight only marks the scoped Windows owned-process app/game timer row as dispatch eligible. Android and Linux host capability refs remain visibility-only and do not make dispatch eligible. Adapter execution, broad installed-app blocking, platform enforcement, provider delivery, child delivery, and private diagnostics remain unclaimed.";
    match read_model {
        None => app_game_panel_unavailable(
            "Runtime reference",
            "App/game adapter dispatch preflight",
            "Service-backed dispatch preflight derived from adapter execution readiness and policy dispatch.",
            "No app/game adapter dispatch preflight read model has been reported yet.",
            product_claim,
        ),
        Some(read_model) => ParentAppGamePanelSnapshot {
            eyebrow: "Runtime reference".to_string(),
            title: "App/game adapter dispatch preflight".to_string(),
            body: "Service-backed dispatch preflight derived from adapter execution readiness and policy dispatch.".to_string(),
            load_state: app_game_adapter_dispatch_preflight_load_state(read_model),
            summary_details: app_game_adapter_dispatch_preflight_summary_details(
                read_model,
                product_claim,
            ),
            rows: app_game_adapter_dispatch_preflight_rows(read_model, product_claim),
            empty_message:
                "No app/game adapter dispatch preflight read model has been reported yet."
                    .to_string(),
            product_claim: product_claim.to_string(),
        },
    }
}

fn app_game_adapter_dispatch_result_panel_snapshot(
    read_model: Option<&AppGameAdapterDispatchResultReadModel>,
    execute_result: Option<&Value>,
) -> ParentAppGamePanelSnapshot {
    let product_claim = "Adapter dispatch execution is reported only for the scoped Windows owned-process app/game timer row when real enforcement audit evidence is attached. Broad installed-app blocking, platform enforcement, provider delivery, child delivery, and private diagnostics remain unclaimed.";
    let execute_summary = app_game_adapter_dispatch_execute_summary_details(execute_result);
    match read_model {
        None => {
            let mut panel = app_game_panel_unavailable(
                "Runtime reference",
                "App/game adapter dispatch result",
                "Service-backed command-result handoff for scoped app/game adapter dispatch.",
                "No app/game adapter dispatch result read model has been reported yet.",
                product_claim,
            );
            if !execute_summary.is_empty() {
                panel.summary_details.extend(execute_summary);
            }
            panel
        }
        Some(read_model) => {
            let mut summary_details =
                app_game_adapter_dispatch_result_summary_details(read_model, product_claim);
            summary_details.extend(execute_summary);
            ParentAppGamePanelSnapshot {
                eyebrow: "Runtime reference".to_string(),
                title: "App/game adapter dispatch result".to_string(),
                body: "Service-backed command-result handoff for scoped app/game adapter dispatch."
                    .to_string(),
                load_state: app_game_adapter_dispatch_result_load_state(read_model),
                summary_details,
                rows: app_game_adapter_dispatch_result_rows(read_model, product_claim),
                empty_message:
                    "No app/game adapter dispatch result read model has been reported yet."
                        .to_string(),
                product_claim: product_claim.to_string(),
            }
        }
    }
}

fn app_game_adapter_dispatch_preflight_summary_details(
    read_model: &AppGameAdapterDispatchPreflightReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(
            "Status",
            app_game_adapter_dispatch_preflight_load_state(read_model),
        ),
        app_game_detail("Generated at", read_model.generated_at.as_str()),
        app_game_detail("Custody", read_model.custody_label.as_str()),
        app_game_detail("Capability", read_model.capability_status.as_str()),
        app_game_detail("Rows returned", read_model.returned.to_string()),
        app_game_detail(
            "Read model rows",
            read_model.dispatch_eligible_count.to_string(),
        ),
        app_game_detail(
            "Manual review",
            read_model.blocked_before_dispatch_count.to_string(),
        ),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(read_model.adapter_dispatch_eligible_count > 0),
        ),
        app_game_detail(
            "Execution state",
            app_game_claimed_value(read_model.adapter_dispatch_executed_claimed_count > 0),
        ),
        app_game_detail(
            "Host available rows",
            read_model.host_capability_available_count.to_string(),
        ),
        app_game_detail(
            "Host not-detected rows",
            read_model.host_capability_not_detected_count.to_string(),
        ),
        app_game_detail(
            "Host not-applicable rows",
            read_model.host_capability_not_applicable_count.to_string(),
        ),
        app_game_detail(
            "Host probe refs",
            read_model.host_capability_probe_ref_count.to_string(),
        ),
        app_game_detail(
            "Platform state",
            app_game_claimed_value(read_model.platform_enforcement_claimed),
        ),
        app_game_detail(
            "Child delivery",
            app_game_claimed_value(read_model.child_device_delivery_claimed),
        ),
        app_game_detail("Product claim", product_claim),
    ]
}

fn app_game_adapter_dispatch_preflight_rows(
    read_model: &AppGameAdapterDispatchPreflightReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelRowSnapshot> {
    read_model
        .rows
        .iter()
        .map(|row| app_game_adapter_dispatch_preflight_row_snapshot(row, product_claim))
        .collect()
}

fn app_game_adapter_dispatch_preflight_row_snapshot(
    row: &AppGameAdapterDispatchPreflightRow,
    product_claim: &str,
) -> ParentAppGamePanelRowSnapshot {
    app_game_panel_row(
        row.source_proof_entry_id.clone(),
        vec![
            app_game_detail("Platform", row.platform.as_str()),
            app_game_detail("Capability", row.adapter_capability.as_str()),
            app_game_detail(
                "Status",
                app_game_adapter_dispatch_preflight_row_status(
                    row.dispatch_preflight_state.as_str(),
                ),
            ),
            app_game_detail(
                "Adapter boundary",
                row.source_execution_readiness_row_id.as_str(),
            ),
            app_game_detail(
                "Preview status",
                app_game_adapter_dispatch_preflight_decision_label(row.dispatch_decision.as_str()),
            ),
            app_game_detail(
                "Dispatch intent",
                app_game_optional_string(row.dispatch_intent_id.as_deref()),
            ),
            app_game_detail(
                "Dispatch outcome",
                app_game_adapter_dispatch_preflight_outcome_label(
                    row.dispatch_outcome_state.as_str(),
                ),
            ),
            app_game_detail(
                "Evidence references",
                app_game_join_strings(&row.dispatch_evidence_refs),
            ),
            app_game_detail("Host capability state", row.host_capability_state.as_str()),
            app_game_detail(
                "Host capability evidence",
                app_game_join_strings(&row.host_capability_evidence_refs),
            ),
            app_game_detail(
                "Host capability probe",
                app_game_join_strings(&row.host_capability_probe_refs),
            ),
            app_game_detail(
                "Audit references",
                app_game_join_strings(&row.dispatch_audit_refs),
            ),
            app_game_detail(
                "Timer references",
                app_game_join_strings(&row.dispatch_timer_refs),
            ),
            app_game_detail(
                "Manual review",
                app_game_join_strings(&row.manual_proof_requirements),
            ),
            app_game_detail(
                "Adapter dispatch",
                app_game_claimed_value(row.adapter_dispatch_eligible),
            ),
            app_game_detail(
                "Execution state",
                app_game_claimed_value(row.adapter_dispatch_executed_claimed),
            ),
            app_game_detail(
                "Platform state",
                app_game_claimed_value(row.platform_enforcement_claimed),
            ),
            app_game_detail(
                "Child delivery",
                app_game_claimed_value(row.child_device_delivery_claimed),
            ),
            app_game_detail("Product claim", product_claim),
        ],
    )
}

fn app_game_adapter_dispatch_result_summary_details(
    read_model: &AppGameAdapterDispatchResultReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(
            "Status",
            app_game_adapter_dispatch_result_load_state(read_model),
        ),
        app_game_detail("Generated at", read_model.generated_at.as_str()),
        app_game_detail("Custody", read_model.custody_label.as_str()),
        app_game_detail("Capability", read_model.capability_status.as_str()),
        app_game_detail("Rows returned", read_model.returned.to_string()),
        app_game_detail(
            "Read model rows",
            read_model.command_accepted_count.to_string(),
        ),
        app_game_detail(
            "Manual review",
            read_model.blocked_before_command_count.to_string(),
        ),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(read_model.adapter_dispatch_command_result_claimed_count > 0),
        ),
        app_game_detail(
            "Execution audit",
            app_game_claimed_value(read_model.service_local_execution_audit_claimed_count > 0),
        ),
        app_game_detail(
            "Execution state",
            app_game_claimed_value(read_model.adapter_dispatch_executed_claimed_count > 0),
        ),
        app_game_detail(
            "Adapter execution",
            read_model.adapter_execution_reported_count.to_string(),
        ),
        app_game_detail(
            "Platform state",
            app_game_claimed_value(read_model.platform_enforcement_claimed),
        ),
        app_game_detail(
            "Child delivery",
            app_game_claimed_value(read_model.child_device_delivery_claimed),
        ),
        app_game_detail("Product claim", product_claim),
    ]
}

fn app_game_adapter_dispatch_result_rows(
    read_model: &AppGameAdapterDispatchResultReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelRowSnapshot> {
    read_model
        .rows
        .iter()
        .map(|row| app_game_adapter_dispatch_result_row_snapshot(row, product_claim))
        .collect()
}

fn app_game_adapter_dispatch_result_row_snapshot(
    row: &AppGameAdapterDispatchResultRow,
    product_claim: &str,
) -> ParentAppGamePanelRowSnapshot {
    app_game_panel_row(
        row.source_proof_entry_id.clone(),
        app_game_adapter_dispatch_result_row_details(row, product_claim),
    )
}

fn app_game_adapter_dispatch_result_row_details(
    row: &AppGameAdapterDispatchResultRow,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    let mut details = vec![
        app_game_detail("Platform", row.platform.as_str()),
        app_game_detail("Capability", row.adapter_capability.as_str()),
        app_game_detail(
            "Status",
            app_game_adapter_dispatch_result_row_status(row.dispatch_command_result_state.as_str()),
        ),
        app_game_detail(
            "Adapter boundary",
            row.source_dispatch_preflight_row_id.as_str(),
        ),
        app_game_detail(
            "Preview status",
            app_game_adapter_dispatch_result_decision_label(
                row.dispatch_command_result_decision.as_str(),
            ),
        ),
        app_game_detail(
            "Dispatch command",
            app_game_optional_string(row.enforcement_command_name.as_deref()),
        ),
        app_game_detail(
            "Dispatch event",
            app_game_optional_string(row.enforcement_event_name.as_deref()),
        ),
        app_game_detail(
            "Dispatch action",
            app_game_optional_string(row.enforcement_action_mode.as_deref()),
        ),
        app_game_detail(
            "Dispatch result",
            app_game_optional_string(row.dispatch_command_result_id.as_deref()),
        ),
        app_game_detail(
            "Audit references",
            app_game_join_strings(&row.dispatch_command_audit_refs),
        ),
        app_game_detail(
            "Timer references",
            app_game_join_strings(&row.dispatch_command_timer_refs),
        ),
    ];
    details.extend(app_game_adapter_dispatch_result_execution_details(row));
    details.push(app_game_detail("Product claim", product_claim));
    details
}

fn app_game_adapter_dispatch_result_execution_details(
    row: &AppGameAdapterDispatchResultRow,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(
            "Execution audit",
            app_game_adapter_dispatch_execution_audit_label(
                row.dispatch_execution_audit_state.as_str(),
            ),
        ),
        app_game_detail(
            "Execution audit refs",
            app_game_join_strings(&row.dispatch_execution_audit_refs),
        ),
        app_game_detail(
            "Adapter execution",
            app_game_adapter_dispatch_adapter_execution_label(
                row.dispatch_adapter_execution_state.as_str(),
            ),
        ),
        app_game_detail(
            "Adapter execution result",
            app_game_optional_string(row.dispatch_adapter_execution_result_id.as_deref()),
        ),
        app_game_detail(
            "Adapter execution status",
            app_game_optional_string(row.dispatch_adapter_execution_status.as_deref()),
        ),
        app_game_detail(
            "Adapter execution refs",
            app_game_join_strings(&row.dispatch_adapter_execution_refs),
        ),
        app_game_detail(
            "Manual review",
            app_game_join_strings(&row.manual_proof_requirements),
        ),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(row.adapter_dispatch_command_result_claimed),
        ),
        app_game_detail(
            "Execution audit",
            app_game_claimed_value(row.service_local_execution_audit_claimed),
        ),
        app_game_detail(
            "Execution state",
            app_game_claimed_value(row.adapter_dispatch_executed_claimed),
        ),
        app_game_detail(
            "Platform state",
            app_game_claimed_value(row.platform_enforcement_claimed),
        ),
        app_game_detail(
            "Child delivery",
            app_game_claimed_value(row.child_device_delivery_claimed),
        ),
    ]
}
