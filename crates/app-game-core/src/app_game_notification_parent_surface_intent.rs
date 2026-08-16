pub const APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_FAMILY_MISMATCH: &str =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_FAMILY_MISMATCH;
pub const APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_ROW_COUNT_MISMATCH: &str =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_ROW_COUNT_MISMATCH;

const MINIMAL_SURFACE_PAYLOAD_BOUNDARY: &str = "Parent surface intent contains status refs and setup requirements only; sensitive app/game evidence stays behind authenticated drill-in.";

const REQUIRED_NON_CLAIMS: &[&str] = &[
    "no-parent-notification-ui-rendered",
    "no-parent-preference-ui-rendered",
    "no-parent-frequency-control-ui-rendered",
    "no-provider-delivery-execution",
    "no-provider-receipt-ingestion",
    "no-provider-credentials",
    "no-cloud-routing",
    "no-child-delivery",
    "no-production-runtime",
    "no-production-durable-outbox-storage",
    "no-adapter-dispatch",
];

pub type AppGameNotificationParentSurfaceIntentOptions =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationParentSurfaceIntentOptions;
pub type AppGameNotificationFamilyReference =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationFamilyReference;
pub type AppGameNotificationProviderStatusBoundaryEntry =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationProviderStatusBoundaryEntry;
pub type AppGameNotificationProviderStatusHandoffRow =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationProviderStatusHandoffRow;
pub type AppGameNotificationProviderStatusHandoffReadModel =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationProviderStatusHandoffReadModel;
pub type AppGameNotificationPreferenceStatusEntry =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationPreferenceStatusHandoffEntry;
pub type AppGameNotificationPreferenceStatusHandoffRow =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationPreferenceStatusHandoffRow;
pub type AppGameNotificationPreferenceStatusHandoffReadModel =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationPreferenceStatusHandoffReadModel;
pub type AppGameNotificationParentSurfaceIntentRow =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationParentSurfaceIntentRow;
pub type AppGameNotificationParentSurfaceIntentReadModel =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationParentSurfaceIntentReadModel;

pub fn build_app_game_notification_parent_surface_intent_read_model(
    options: &AppGameNotificationParentSurfaceIntentOptions,
    provider_read_model: &AppGameNotificationProviderStatusHandoffReadModel,
    preference_read_model: &AppGameNotificationPreferenceStatusHandoffReadModel,
) -> Result<AppGameNotificationParentSurfaceIntentReadModel, &'static str> {
    if provider_read_model.family.family_id != preference_read_model.family.family_id {
        return Err(APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_FAMILY_MISMATCH);
    }
    if provider_read_model.rows.len() != preference_read_model.rows.len() {
        return Err(APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_ROW_COUNT_MISMATCH);
    }

    let rows = provider_read_model
        .rows
        .iter()
        .zip(preference_read_model.rows.iter())
        .map(parent_surface_intent_row_for_status_rows)
        .collect::<Vec<_>>();

    Ok(AppGameNotificationParentSurfaceIntentReadModel {
        schema_version: "v0.6".to_string(),
        intent_id: options.intent_id.clone(),
        generated_at: options.generated_at.clone(),
        family: provider_read_model.family.clone(),
        source_provider_status_handoff_id: provider_read_model.handoff_id.clone(),
        source_preference_status_handoff_id: preference_read_model.handoff_id.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        manual_action_required_count: count_rows(&rows, "manual-action-required"),
        unavailable_visible_count: count_rows(&rows, "unavailable-visible"),
        history_visible_count: rows.len(),
        preference_setup_required_count: count_preference_rows(&rows, "preference-setup-required"),
        parent_surface_non_claims: REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
        parent_notification_ui_rendered: false,
        parent_preference_ui_rendered: false,
        parent_frequency_control_ui_rendered: false,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        provider_credentials_claimed: false,
        cloud_routing_claimed: false,
        child_delivery_claimed: false,
        production_runtime_claimed: false,
        production_durable_outbox_storage_claimed: false,
        adapter_dispatch_claimed: false,
        rows,
    })
}

fn parent_surface_intent_row_for_status_rows(
    (provider_row, preference_row): (
        &AppGameNotificationProviderStatusHandoffRow,
        &AppGameNotificationPreferenceStatusHandoffRow,
    ),
) -> AppGameNotificationParentSurfaceIntentRow {
    let provider_entry = &provider_row.provider_status_boundary_entry;
    let preference_entry = &preference_row.notification_preference_status_entry;

    AppGameNotificationParentSurfaceIntentRow {
        surface_row_id: format!(
            "app-game-notification-parent-surface-{}",
            provider_row.handoff_row_id
        ),
        source_provider_handoff_row_id: provider_row.handoff_row_id.clone(),
        source_preference_handoff_row_id: preference_row.handoff_row_id.clone(),
        source_scheduler_entry_ref: provider_row
            .source_scheduler_entry_ref
            .clone()
            .or_else(|| preference_row.source_scheduler_entry_ref.clone()),
        source_outbox_record_ref: provider_row
            .source_outbox_record_ref
            .clone()
            .or_else(|| preference_row.source_outbox_record_ref.clone()),
        provider_status: provider_entry.provider_status.clone(),
        delivery_result_state: preference_entry.delivery_result_state.clone(),
        parent_preference_state: preference_entry.parent_preference_state.clone(),
        quiet_hours_decision: preference_entry.quiet_hours_decision.clone(),
        provider_channel: preference_entry.provider_channel.clone(),
        parent_surface_status: parent_surface_status_for(&provider_entry.provider_status)
            .to_string(),
        history_visibility: history_visibility_for(&provider_entry.provider_status).to_string(),
        preference_visibility: preference_visibility_for(&preference_entry.parent_preference_state)
            .to_string(),
        drill_in_refs: vec![
            provider_entry.notification_status_ref.clone(),
            preference_entry.delivery_result_ref.clone(),
        ],
        audit_refs: provider_entry
            .audit_refs
            .iter()
            .chain(preference_entry.audit_refs.iter())
            .cloned()
            .collect(),
        manual_proof_requirements: provider_entry
            .manual_proof_requirements
            .iter()
            .chain(preference_entry.manual_proof_requirements.iter())
            .cloned()
            .collect(),
        minimal_surface_payload_boundary: MINIMAL_SURFACE_PAYLOAD_BOUNDARY.to_string(),
        sensitive_detail_included: false,
        provider_delivery_claimed: false,
        provider_receipt_claimed: false,
        parent_preference_mutation_claimed: false,
        child_delivery_claimed: false,
    }
}

fn parent_surface_status_for(provider_status: &str) -> &'static str {
    if provider_status == "unavailable" {
        "unavailable-visible"
    } else {
        "manual-action-required"
    }
}

fn history_visibility_for(provider_status: &str) -> &'static str {
    if provider_status == "unavailable" {
        "unavailable-row-visible"
    } else {
        "manual-review-only"
    }
}

fn preference_visibility_for(parent_preference_state: &str) -> &'static str {
    if parent_preference_state == "channel-disabled" {
        "preference-disabled-visible"
    } else {
        "preference-setup-required"
    }
}

fn count_rows(rows: &[AppGameNotificationParentSurfaceIntentRow], status: &str) -> usize {
    rows.iter()
        .filter(|row| row.parent_surface_status == status)
        .count()
}

fn count_preference_rows(
    rows: &[AppGameNotificationParentSurfaceIntentRow],
    visibility: &str,
) -> usize {
    rows.iter()
        .filter(|row| row.preference_visibility == visibility)
        .count()
}

pub fn app_game_notification_parent_surface_intent_typescript() -> String {
    include_str!("../tests/generated/app-game-notification-parent-surface-intent.ts").to_string()
}
