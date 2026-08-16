use super::*;

pub(super) fn app_game_notification_parent_surface_panel_snapshot(
    read_model: Option<&AppGameNotificationReadinessReadModel>,
    status_read_models: Option<&AppGameNotificationStatusReadModels>,
) -> ParentAppGameNotificationParentSurfacePanelSnapshot {
    let product_claim = "Notification readiness rows are parent-visible intent readiness only. Provider delivery, receipt ingestion, runtime outbox, scheduler execution, adapter dispatch, and child delivery remain unclaimed.".to_string();
    match read_model {
        None => ParentAppGameNotificationParentSurfacePanelSnapshot {
            eyebrow: "Notification readiness".to_string(),
            title: "App/game notification parent surface".to_string(),
            body: "Parent-visible notification readiness for native app and game warning surfaces."
                .to_string(),
            state: "unavailable".to_string(),
            summary: "0 notification readiness rows".to_string(),
            product_claim: product_claim.clone(),
            metrics: vec![
                app_game_detail("Rows returned", "0"),
                app_game_detail("Status", "unavailable"),
                app_game_detail("Product claim", product_claim.as_str()),
            ],
            rows: Vec::new(),
            empty_message: "No app/game notification readiness has been reported yet.".to_string(),
        },
        Some(read_model) => ParentAppGameNotificationParentSurfacePanelSnapshot {
            eyebrow: "Notification readiness".to_string(),
            title: "App/game notification parent surface".to_string(),
            body: "Parent-visible notification readiness for native app and game warning surfaces."
                .to_string(),
            state: app_game_notification_load_state(read_model),
            summary: format!("{} notification readiness rows", read_model.returned),
            product_claim: product_claim.clone(),
            metrics: app_game_notification_metrics(read_model, &product_claim),
            rows: app_game_notification_rows(read_model, status_read_models, &product_claim),
            empty_message: "No app/game notification readiness rows were returned.".to_string(),
        },
    }
}

fn app_game_notification_metrics(
    read_model: &AppGameNotificationReadinessReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail("Generated at", read_model.generated_at.as_str()),
        app_game_detail("Custody", read_model.custody_label.as_str()),
        app_game_detail("Capability", read_model.capability_status.as_str()),
        app_game_detail("Rows returned", read_model.returned.to_string()),
        app_game_detail("Ready intents", read_model.ready_intent_count.to_string()),
        app_game_detail(
            "Manual required",
            read_model.manual_required_count.to_string(),
        ),
        app_game_detail("Unavailable", read_model.unavailable_count.to_string()),
        app_game_detail(
            "Provider delivery",
            app_game_claimed_value(read_model.provider_delivery_claimed),
        ),
        app_game_detail(
            "Receipt ingestion",
            app_game_claimed_value(read_model.provider_receipt_ingestion_claimed),
        ),
        app_game_detail(
            "Local outbox runtime",
            app_game_claimed_value(read_model.local_outbox_runtime_claimed),
        ),
        app_game_detail(
            "Scheduler runtime",
            app_game_claimed_value(read_model.scheduler_runtime_claimed),
        ),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(read_model.adapter_dispatch_claimed),
        ),
        app_game_detail(
            "Parent UI",
            app_game_claimed_value(read_model.parent_ui_claimed),
        ),
        app_game_detail(
            "Child delivery",
            app_game_claimed_value(read_model.child_delivery_claimed),
        ),
        app_game_detail("Product claim", product_claim),
    ]
}

fn app_game_notification_rows(
    read_model: &AppGameNotificationReadinessReadModel,
    status_read_models: Option<&AppGameNotificationStatusReadModels>,
    product_claim: &str,
) -> Vec<ParentAppGameNotificationParentSurfacePanelRowSnapshot> {
    let mut rows = read_model
        .rows
        .iter()
        .map(|row| app_game_notification_row_snapshot(row, product_claim))
        .collect::<Vec<_>>();
    if let Some(status_read_models) = status_read_models {
        append_notification_status_rows(&mut rows, status_read_models, product_claim);
    }
    rows
}

fn append_notification_status_rows(
    rows: &mut Vec<ParentAppGameNotificationParentSurfacePanelRowSnapshot>,
    status_read_models: &AppGameNotificationStatusReadModels,
    product_claim: &str,
) {
    rows.extend(
        status_read_models
            .provider_status_boundary
            .entries
            .iter()
            .map(|entry| app_game_provider_status_row_snapshot(entry, product_claim)),
    );
    rows.extend(
        status_read_models
            .preference_status
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                app_game_preference_status_row_snapshot(index, entry, product_claim)
            }),
    );
}

fn app_game_provider_status_row_snapshot(
    entry: &V08NotificationProviderStatusBoundaryEntry,
    product_claim: &str,
) -> ParentAppGameNotificationParentSurfacePanelRowSnapshot {
    ParentAppGameNotificationParentSurfacePanelRowSnapshot {
        key: entry.status_entry_id.clone(),
        title: "Provider delivery boundary".to_string(),
        details: vec![
            app_game_detail(
                "Provider status",
                serialized_enum_label(&entry.provider_status),
            ),
            app_game_detail(
                "Proof state",
                serialized_enum_label(&entry.status_proof_state),
            ),
            app_game_detail(
                "Delivery claim",
                serialized_enum_label(&entry.delivery_claim_state),
            ),
            app_game_detail(
                "Provider delivery implemented",
                app_game_claimed_value(entry.provider_delivery_implemented),
            ),
            app_game_detail(
                "Provider delivery observed",
                app_game_claimed_value(entry.provider_delivery_observed),
            ),
            app_game_detail("Manual proof", entry.manual_proof_requirements.join(", ")),
            app_game_detail("Product claim", product_claim),
        ],
    }
}

fn app_game_preference_status_row_snapshot(
    index: usize,
    entry: &AppGameNotificationPreferenceStatusEntry,
    product_claim: &str,
) -> ParentAppGameNotificationParentSurfacePanelRowSnapshot {
    ParentAppGameNotificationParentSurfacePanelRowSnapshot {
        key: format!("app-game-preference-status:{index}"),
        title: "Parent notification preference boundary".to_string(),
        details: vec![
            app_game_detail(
                "Delivery result",
                serialized_enum_label(&entry.delivery_result_state),
            ),
            app_game_detail(
                "Parent preference",
                serialized_enum_label(&entry.parent_preference_state),
            ),
            app_game_detail(
                "Quiet hours",
                serialized_enum_label(&entry.quiet_hours_decision),
            ),
            app_game_detail(
                "Provider channel",
                serialized_enum_label(&entry.provider_channel),
            ),
            app_game_detail("Manual proof", entry.manual_proof_requirements.join(", ")),
            app_game_detail("Product claim", product_claim),
        ],
    }
}

fn app_game_notification_row_snapshot(
    row: &AppGameNotificationReadinessRow,
    product_claim: &str,
) -> ParentAppGameNotificationParentSurfacePanelRowSnapshot {
    ParentAppGameNotificationParentSurfacePanelRowSnapshot {
        key: row.row_id.clone(),
        title: app_game_notification_reason_label(row.reason.as_str()),
        details: vec![
            app_game_detail("Status", row.readiness_state.as_str()),
            app_game_detail("Reason", row.reason.as_str()),
            app_game_detail("Row count", row.row_count.to_string()),
            app_game_detail("Minimal payload", row.minimal_payload_ref.as_str()),
            app_game_detail("Evidence references", app_game_join_notification_refs(row)),
            app_game_detail("Product claim", product_claim),
        ],
    }
}
