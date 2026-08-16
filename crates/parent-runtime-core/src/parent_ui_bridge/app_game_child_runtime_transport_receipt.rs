use super::*;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptRow;

pub(super) fn app_game_child_runtime_transport_receipt_panel_snapshot(
    read_model: Option<&AppGameChildRuntimeTransportReceiptReadModel>,
) -> ParentAppGamePanelSnapshot {
    let product_claim = "Child runtime transport receipt rows are parent-visible readiness only. Runtime transport execution, receipt ingestion, provider delivery, platform channel delivery, adapter dispatch, platform enforcement, and raw private rows remain unclaimed.".to_string();
    match read_model {
        None => app_game_panel_unavailable(
            "Runtime reference",
            "App/game child runtime transport receipts",
            "Parent-visible child runtime transport and receipt readiness for native app and native game warning and request delivery.",
            "No app/game child runtime transport receipt read model has been reported yet.",
            product_claim.as_str(),
        ),
        Some(read_model) => ParentAppGamePanelSnapshot {
            eyebrow: "Runtime reference".to_string(),
            title: "App/game child runtime transport receipts".to_string(),
            body: "Parent-visible child runtime transport and receipt readiness for native app and native game warning and request delivery.".to_string(),
            load_state: if read_model.transport_required_count > 0
                || read_model.manual_required_count > 0
            {
                "warn".to_string()
            } else {
                "ready".to_string()
            },
            summary_details: app_game_child_runtime_transport_receipt_summary_details(
                read_model,
                &product_claim,
            ),
            rows: app_game_child_runtime_transport_receipt_rows(read_model, &product_claim),
            empty_message:
                "No app/game child runtime transport receipt rows were returned.".to_string(),
            product_claim,
        },
    }
}

fn app_game_child_runtime_transport_receipt_summary_details(
    read_model: &AppGameChildRuntimeTransportReceiptReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    let status = if read_model.transport_required_count > 0 || read_model.manual_required_count > 0
    {
        "warn"
    } else {
        "ready"
    };
    vec![
        app_game_detail("Status", status),
        app_game_detail("Generated at", read_model.generated_at.as_str()),
        app_game_detail("Transport rows", read_model.returned.to_string()),
        app_game_detail(
            "Transport-required rows",
            read_model.transport_required_count.to_string(),
        ),
        app_game_detail(
            "Manual-required rows",
            read_model.manual_required_count.to_string(),
        ),
        app_game_detail("Unavailable rows", read_model.unavailable_count.to_string()),
        app_game_detail(
            "Runtime transport",
            app_game_claimed_value(read_model.runtime_transport_executed),
        ),
        app_game_detail(
            "Runtime receipt",
            app_game_claimed_value(read_model.runtime_receipt_ingested),
        ),
        app_game_detail(
            "Provider delivery",
            app_game_claimed_value(read_model.provider_delivery_executed),
        ),
        app_game_detail(
            "Platform delivery",
            app_game_claimed_value(read_model.platform_delivery_channel_claimed),
        ),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(read_model.adapter_dispatch_claimed),
        ),
        app_game_detail(
            "Platform enforcement",
            app_game_claimed_value(read_model.platform_enforcement_claimed),
        ),
        app_game_detail(
            "Raw private rows",
            app_game_claimed_value(read_model.raw_private_source_rows_included),
        ),
        app_game_detail("Product claim", product_claim),
    ]
}

fn app_game_child_runtime_transport_receipt_rows(
    read_model: &AppGameChildRuntimeTransportReceiptReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelRowSnapshot> {
    read_model
        .rows
        .iter()
        .map(|row| app_game_child_runtime_transport_receipt_row_snapshot(row, product_claim))
        .collect()
}

fn app_game_child_runtime_transport_receipt_row_snapshot(
    row: &AppGameChildRuntimeTransportReceiptRow,
    product_claim: &str,
) -> ParentAppGamePanelRowSnapshot {
    app_game_panel_row(
        row.row_id.clone(),
        vec![
            app_game_detail("Status", row.boundary_state.as_str()),
            app_game_detail(
                "Source runtime writer",
                row.source_runtime_writer_row_id.as_str(),
            ),
            app_game_detail(
                "Product meanings",
                app_game_join_strings(&row.product_meanings),
            ),
            app_game_detail(
                "Required transport refs",
                app_game_join_strings(&row.required_transport_refs),
            ),
            app_game_detail(
                "Required receipt refs",
                app_game_join_strings(&row.required_receipt_refs),
            ),
            app_game_detail("Open gaps", app_game_join_strings(&row.open_gaps)),
            app_game_detail(
                "Runtime transport",
                app_game_claimed_value(row.runtime_transport_executed),
            ),
            app_game_detail(
                "Runtime receipt",
                app_game_claimed_value(row.runtime_receipt_ingested),
            ),
            app_game_detail(
                "Provider delivery",
                app_game_claimed_value(row.provider_delivery_executed),
            ),
            app_game_detail(
                "Platform delivery",
                app_game_claimed_value(row.platform_delivery_channel_claimed),
            ),
            app_game_detail("Product claim", product_claim),
        ],
    )
}
