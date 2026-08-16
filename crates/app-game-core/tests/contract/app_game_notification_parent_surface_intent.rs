use ocentra_app_game_core::app_game_notification_parent_surface_intent::{
    app_game_notification_parent_surface_intent_typescript,
    build_app_game_notification_parent_surface_intent_read_model,
    AppGameNotificationFamilyReference, AppGameNotificationParentSurfaceIntentOptions,
    AppGameNotificationPreferenceStatusEntry, AppGameNotificationPreferenceStatusHandoffReadModel,
    AppGameNotificationPreferenceStatusHandoffRow, AppGameNotificationProviderStatusBoundaryEntry,
    AppGameNotificationProviderStatusHandoffReadModel, AppGameNotificationProviderStatusHandoffRow,
    APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_FAMILY_MISMATCH,
    APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_ROW_COUNT_MISMATCH,
};
use ocentra_eventing::expect_value::ExpectValue;

type AppGameNotificationText<'a> = &'a str;

#[test]
fn app_game_notification_parent_surface_intent_keeps_parent_surface_redacted_and_manual() {
    let read_model = build_app_game_notification_parent_surface_intent_read_model(
        &options(),
        &provider_status_read_model(),
        &preference_status_read_model(),
    )
    .expect_value("parent surface intent read model");

    assert_eq!(read_model.manual_action_required_count, 2);
    assert_eq!(read_model.unavailable_visible_count, 1);
    assert_eq!(read_model.history_visible_count, 3);
    assert_eq!(read_model.preference_setup_required_count, 2);
    assert_eq!(
        read_model
            .rows
            .iter()
            .map(|row| row.parent_surface_status.as_str())
            .collect::<Vec<_>>(),
        vec![
            "manual-action-required",
            "manual-action-required",
            "unavailable-visible",
        ]
    );

    let first_row = &read_model.rows[0];
    assert_eq!(
        first_row.source_scheduler_entry_ref.as_deref(),
        Some("scheduler-entry-app-game-time-limit")
    );
    assert_eq!(
        first_row.source_outbox_record_ref.as_deref(),
        Some("outbox-record-app-game-time-limit")
    );
    assert_eq!(
        first_row.drill_in_refs,
        vec![
            "app-game-provider-status-ref-time-limit".to_string(),
            "app-game-preference-status-result-time-limit".to_string(),
        ]
    );
    assert_eq!(
        first_row.audit_refs,
        vec![
            "app-game-provider-status-audit-time-limit".to_string(),
            "app-game-preference-status-audit-time-limit".to_string(),
        ]
    );
    assert_eq!(first_row.preference_visibility, "preference-setup-required");
    assert!(!first_row.sensitive_detail_included);
    assert!(!first_row.provider_delivery_claimed);
    assert!(!first_row.provider_receipt_claimed);
    assert!(!first_row.parent_preference_mutation_claimed);
    assert!(!first_row.child_delivery_claimed);

    let unavailable_row = &read_model.rows[2];
    assert_eq!(unavailable_row.parent_surface_status, "unavailable-visible");
    assert_eq!(
        unavailable_row.history_visibility,
        "unavailable-row-visible"
    );
    assert_eq!(
        unavailable_row.preference_visibility,
        "preference-disabled-visible"
    );
}

#[test]
fn app_game_notification_parent_surface_intent_rejects_family_and_row_count_mismatches() {
    let family_mismatch = build_app_game_notification_parent_surface_intent_read_model(
        &options(),
        &provider_status_read_model(),
        &preference_status_read_model_with_family("family-other"),
    );
    assert_eq!(
        family_mismatch,
        Err(APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_FAMILY_MISMATCH)
    );

    let row_count_mismatch = build_app_game_notification_parent_surface_intent_read_model(
        &options(),
        &provider_status_read_model(),
        &two_row_preference_status_read_model(),
    );
    assert_eq!(
        row_count_mismatch,
        Err(APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_ROW_COUNT_MISMATCH)
    );
}

#[test]
fn generated_app_game_notification_parent_surface_intent_helper_stays_checked_in() {
    let checked_in = include_str!("../generated/app-game-notification-parent-surface-intent.ts");
    let generated = app_game_notification_parent_surface_intent_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated
            .matches("buildGeneratedAppGameNotificationParentSurfaceIntentReadModel")
            .count(),
        1
    );
    assert_eq!(
        generated
            .matches("GeneratedRequiredAppGameNotificationParentSurfaceIntentNonClaims")
            .count(),
        1
    );
}

fn options() -> AppGameNotificationParentSurfaceIntentOptions {
    AppGameNotificationParentSurfaceIntentOptions {
        generated_at: "2026-06-05T09:12:00Z".to_string(),
        intent_id: "app-game-notification-parent-surface-intent-proof".to_string(),
        source_contract_refs: vec![
            "app-game-notification-provider-status-handoff".to_string(),
            "app-game-notification-preference-status-handoff".to_string(),
            "notifications-expectation-parent-surface-boundary".to_string(),
        ],
    }
}

fn family(family_id: AppGameNotificationText<'_>) -> AppGameNotificationFamilyReference {
    AppGameNotificationFamilyReference {
        family_id: family_id.to_string(),
    }
}

fn provider_status_read_model() -> AppGameNotificationProviderStatusHandoffReadModel {
    AppGameNotificationProviderStatusHandoffReadModel {
        handoff_id: "app-game-provider-status-handoff-parent-surface".to_string(),
        family: family("family-app-game-parent-surface"),
        rows: vec![
            provider_status_row("time-limit", false),
            provider_status_row("manual-required", false),
            provider_status_row("unavailable", true),
        ],
    }
}

fn provider_status_row(
    label: AppGameNotificationText<'_>,
    unavailable: bool,
) -> AppGameNotificationProviderStatusHandoffRow {
    let manual_ref = format!("manual-proof-provider-{label}");

    AppGameNotificationProviderStatusHandoffRow {
        handoff_row_id: format!("provider-status-handoff-{label}"),
        source_scheduler_entry_ref: (!unavailable)
            .then(|| format!("scheduler-entry-app-game-{label}")),
        source_outbox_record_ref: (!unavailable).then(|| format!("outbox-record-app-game-{label}")),
        provider_status_boundary_entry: AppGameNotificationProviderStatusBoundaryEntry {
            provider_status: if unavailable {
                "unavailable".to_string()
            } else {
                "manual-required".to_string()
            },
            notification_status_ref: format!("app-game-provider-status-ref-{label}"),
            audit_refs: vec![format!("app-game-provider-status-audit-{label}")],
            manual_proof_requirements: vec![manual_ref],
        },
    }
}

fn preference_status_read_model() -> AppGameNotificationPreferenceStatusHandoffReadModel {
    preference_status_read_model_with_family("family-app-game-parent-surface")
}

fn preference_status_read_model_with_family(
    family_id: AppGameNotificationText<'_>,
) -> AppGameNotificationPreferenceStatusHandoffReadModel {
    AppGameNotificationPreferenceStatusHandoffReadModel {
        handoff_id: "app-game-preference-status-handoff-parent-surface".to_string(),
        family: family(family_id),
        rows: vec![
            preference_status_row("time-limit", false),
            preference_status_row("manual-required", false),
            preference_status_row("unavailable", true),
        ],
    }
}

fn two_row_preference_status_read_model() -> AppGameNotificationPreferenceStatusHandoffReadModel {
    let mut read_model = preference_status_read_model();
    read_model.rows.truncate(2);
    read_model
}

fn preference_status_row(
    label: AppGameNotificationText<'_>,
    unavailable: bool,
) -> AppGameNotificationPreferenceStatusHandoffRow {
    let manual_ref = format!("manual-proof-preference-{label}");

    AppGameNotificationPreferenceStatusHandoffRow {
        handoff_row_id: format!("preference-status-handoff-{label}"),
        source_scheduler_entry_ref: (!unavailable)
            .then(|| format!("scheduler-entry-app-game-{label}")),
        source_outbox_record_ref: (!unavailable).then(|| format!("outbox-record-app-game-{label}")),
        notification_preference_status_entry: AppGameNotificationPreferenceStatusEntry {
            delivery_result_state: if unavailable {
                "not-sent".to_string()
            } else {
                "manual-required".to_string()
            },
            parent_preference_state: if unavailable {
                "channel-disabled".to_string()
            } else {
                "manual-setup-required".to_string()
            },
            quiet_hours_decision: if unavailable {
                "allow".to_string()
            } else {
                "manual-required".to_string()
            },
            provider_channel: "in-app".to_string(),
            delivery_result_ref: format!("app-game-preference-status-result-{label}"),
            audit_refs: vec![format!("app-game-preference-status-audit-{label}")],
            manual_proof_requirements: vec![manual_ref],
        },
    }
}
