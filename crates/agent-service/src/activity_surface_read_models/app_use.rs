use ocentra_parent_agent_protocol::{
    constants, ActivityAppUseReadModel, ActivityAppUseReadModelRow, ActivityEvidenceRef,
    ActivityReadModelState, ActivityRecentSummary, ActivitySurfaceRequest,
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    AppGameServiceReadModel, ACTIVITY_SURFACE_SCHEMA_VERSION,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_STALE, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_STATE_UNAVAILABLE, APP_GAME_PRODUCT_NATIVE_APP,
    APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE, APP_GAME_RUNTIME_NOT_CLAIMED,
};

use crate::activity_surface_read_model_states::{
    empty_app_use_read_model, offline_app_use_read_model, request_targets_remote_device,
    unavailable_app_use_read_model,
};
use crate::time::timestamp_now;

use super::shared::{
    app_game_boundary_row_counts, app_game_source_status_rows, push_app_game_boundary_evidence,
    push_evidence, row_device_id, row_state,
};
use source::AppUseReadModelSource;

mod source;

pub(crate) fn app_use_read_model<T>(
    request: ActivitySurfaceRequest,
    source: T,
) -> ActivityAppUseReadModel
where
    T: Into<AppUseReadModelSource>,
{
    if request_targets_remote_device(&request) {
        return offline_app_use_read_model(request);
    }

    match source.into() {
        AppUseReadModelSource::AppGame(model) => app_use_model_from_app_game(request, *model),
        AppUseReadModelSource::Recent(summary) => {
            app_use_model_from_recent_summary(request, summary)
        }
    }
}

fn app_use_model_from_app_game(
    request: ActivitySurfaceRequest,
    model: Option<AppGameServiceReadModel>,
) -> ActivityAppUseReadModel {
    match model {
        Some(model) => app_use_model_from_rows(request, model),
        None => unavailable_app_use_read_model(request),
    }
}

fn app_use_model_from_rows(
    request: ActivitySurfaceRequest,
    model: AppGameServiceReadModel,
) -> ActivityAppUseReadModel {
    let generated_at = model.generated_at.clone();
    let rows = app_use_rows(&request, &model);
    if rows.is_empty() {
        return ActivityAppUseReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Empty,
            generated_at,
            summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
            rows,
        };
    }
    ActivityAppUseReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Ready,
        generated_at,
        summary: constants::activity_surface::SUMMARY_READY.to_string(),
        rows,
    }
}

fn app_use_model_from_recent_summary(
    request: ActivitySurfaceRequest,
    summary: Option<ActivityRecentSummary>,
) -> ActivityAppUseReadModel {
    match summary {
        Some(summary) if summary.returned > 0 => {
            let rows = vec![app_use_recent_row(&request, summary)];
            ActivityAppUseReadModel {
                schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
                request,
                state: ActivityReadModelState::Ready,
                generated_at: timestamp_now(),
                summary: constants::activity_surface::SUMMARY_READY.to_string(),
                rows,
            }
        }
        Some(_) => empty_app_use_read_model(request),
        None => unavailable_app_use_read_model(request),
    }
}

fn app_use_rows(
    request: &ActivitySurfaceRequest,
    model: &AppGameServiceReadModel,
) -> Vec<ActivityAppUseReadModelRow> {
    let inventory = model
        .inventory_rows
        .iter()
        .find(|row| is_app_inventory(row));
    let running = model
        .running_now_rows
        .iter()
        .find(|row| is_app_runtime(row));
    let foreground = model
        .foreground_now_rows
        .iter()
        .find(|row| is_app_foreground(row));
    let rollup = model
        .daily_rollups
        .iter()
        .find(|rollup| is_app_classification(&rollup.classification_state));
    if inventory.is_none() && running.is_none() && foreground.is_none() && rollup.is_none() {
        return Vec::new();
    }

    let boundary_counts = app_game_boundary_row_counts(model);
    vec![ActivityAppUseReadModelRow {
        row_id: app_row_id(inventory, running, foreground, rollup),
        app_name: app_label(inventory, running, foreground),
        device_id: row_device_id(request),
        state: row_state(&model.capability_status),
        product_kind: APP_GAME_PRODUCT_NATIVE_APP.to_string(),
        classification_state: app_classification(inventory, running, foreground, rollup),
        inventory_state: inventory
            .map(|row| row.inventory_state.clone())
            .unwrap_or_else(|| APP_GAME_INVENTORY_STATE_UNAVAILABLE.to_string()),
        runtime_state: running
            .map(|row| row.runtime_state.clone())
            .or_else(|| foreground.map(|row| row.runtime_state.clone()))
            .unwrap_or_else(|| APP_GAME_RUNTIME_NOT_CLAIMED.to_string()),
        foreground_state: foreground
            .map(|row| row.foreground_state.clone())
            .or_else(|| running.map(|row| row.foreground_state.clone()))
            .or_else(|| inventory.map(|row| row.foreground_state.clone()))
            .unwrap_or_else(|| APP_GAME_FOREGROUND_NOT_CLAIMED.to_string()),
        capability_status: model.capability_status.clone(),
        last_observed_at: app_last_observed_at(inventory, running, foreground),
        total_ms: app_total_ms(model),
        launch_count: app_session_count(model),
        inventory_row_count: model
            .inventory_rows
            .iter()
            .filter(|row| is_app_inventory(row))
            .count() as u64,
        running_row_count: model
            .running_now_rows
            .iter()
            .filter(|row| is_app_runtime(row))
            .count() as u64,
        foreground_row_count: model
            .foreground_now_rows
            .iter()
            .filter(|row| is_app_foreground(row))
            .count() as u64,
        daily_rollup_count: model
            .daily_rollups
            .iter()
            .filter(|rollup| is_app_classification(&rollup.classification_state))
            .count() as u64,
        evidence_claim_row_count: boundary_counts.evidence_claim_row_count,
        identity_row_count: boundary_counts.identity_row_count,
        approval_authority_row_count: boundary_counts.approval_authority_row_count,
        approval_action_result_row_count: boundary_counts.approval_action_result_row_count,
        platform_authority_matrix_count: boundary_counts.platform_authority_matrix_count,
        platform_authority_row_count: boundary_counts.platform_authority_row_count,
        ai_classifier_result_row_count: boundary_counts.ai_classifier_result_row_count,
        source_status_rows: app_use_source_status_rows(model),
        evidence: app_evidence(model),
    }]
}

fn app_use_recent_row(
    request: &ActivitySurfaceRequest,
    summary: ActivityRecentSummary,
) -> ActivityAppUseReadModelRow {
    ActivityAppUseReadModelRow {
        row_id: summary
            .last_event_id
            .unwrap_or_else(|| constants::activity_surface::READ_MODEL_APP_USE.to_string()),
        app_name: summary
            .most_recent_subject_name
            .unwrap_or_else(|| constants::activity_surface::SECTION_APP_USE.to_string()),
        device_id: row_device_id(request),
        state: ActivityReadModelState::Ready,
        product_kind: APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string(),
        inventory_state: APP_GAME_INVENTORY_STATE_UNAVAILABLE.to_string(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED.to_string(),
        last_observed_at: summary.last_observed_at,
        total_ms: 0,
        launch_count: summary.returned,
        inventory_row_count: 0,
        running_row_count: 0,
        foreground_row_count: 0,
        daily_rollup_count: 0,
        evidence_claim_row_count: 0,
        identity_row_count: 0,
        approval_authority_row_count: 0,
        approval_action_result_row_count: 0,
        platform_authority_matrix_count: 0,
        platform_authority_row_count: 0,
        ai_classifier_result_row_count: 0,
        source_status_rows: Vec::new(),
        evidence: Vec::new(),
    }
}

fn is_app_inventory(row: &AppGameInventoryEvidenceRow) -> bool {
    row.product_kind == APP_GAME_PRODUCT_NATIVE_APP
        || row.product_kind == APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE
}

fn is_app_runtime(row: &AppGameRuntimeEvidenceRow) -> bool {
    is_app_classification(&row.classification_state)
}

fn is_app_foreground(row: &AppGameForegroundEvidenceRow) -> bool {
    is_app_classification(&row.classification_state)
}

fn is_app_classification(classification: &str) -> bool {
    matches!(
        classification,
        APP_GAME_CLASSIFICATION_KNOWN_APP
            | APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
            | APP_GAME_CLASSIFICATION_PERMISSION_LIMITED
            | APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM
            | APP_GAME_CLASSIFICATION_STALE
            | APP_GAME_CLASSIFICATION_ADAPTER_ERROR
    )
}

fn app_use_source_status_rows(
    model: &AppGameServiceReadModel,
) -> Vec<
    ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow,
> {
    app_game_source_status_rows(
        model,
        is_app_inventory,
        is_app_runtime,
        is_app_foreground,
        false,
    )
}

fn app_row_id(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    rollup: Option<&ocentra_parent_agent_protocol::AppGameSessionDailyRollup>,
) -> String {
    rollup
        .and_then(|row| row.session_ids.first().cloned())
        .or_else(|| foreground.map(|row| row.foreground_evidence_id.clone()))
        .or_else(|| running.map(|row| row.runtime_evidence_id.clone()))
        .or_else(|| inventory.map(|row| row.inventory_entry_id.clone()))
        .unwrap_or_else(|| constants::activity_surface::READ_MODEL_APP_USE.to_string())
}

fn app_label(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
) -> String {
    foreground
        .map(|row| row.process_name.clone())
        .or_else(|| running.map(|row| row.process_name.clone()))
        .or_else(|| inventory.map(|row| row.display_label.clone()))
        .unwrap_or_else(|| constants::activity_surface::SECTION_APP_USE.to_string())
}

fn app_classification(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    rollup: Option<&ocentra_parent_agent_protocol::AppGameSessionDailyRollup>,
) -> String {
    rollup
        .map(|row| row.classification_state.clone())
        .or_else(|| foreground.map(|row| row.classification_state.clone()))
        .or_else(|| running.map(|row| row.classification_state.clone()))
        .or_else(|| inventory.map(|row| row.classification_state.clone()))
        .unwrap_or_else(|| APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string())
}

fn app_last_observed_at(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
) -> Option<String> {
    foreground
        .map(|row| row.observed_at.clone())
        .or_else(|| running.map(|row| row.observed_at.clone()))
        .or_else(|| inventory.map(|row| row.observed_at.clone()))
}

fn app_total_ms(model: &AppGameServiceReadModel) -> u64 {
    model
        .daily_rollups
        .iter()
        .filter(|rollup| is_app_classification(&rollup.classification_state))
        .map(|rollup| rollup.running_duration_ms)
        .sum()
}

fn app_session_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .daily_rollups
        .iter()
        .filter(|rollup| is_app_classification(&rollup.classification_state))
        .map(|rollup| rollup.session_count)
        .sum::<u64>()
        .max(
            model
                .running_now_rows
                .iter()
                .filter(|row| is_app_runtime(row))
                .count() as u64,
        )
}

fn app_evidence(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in model
        .inventory_rows
        .iter()
        .filter(|row| is_app_inventory(row))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    for row in model
        .running_now_rows
        .iter()
        .filter(|row| is_app_runtime(row))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    for row in model
        .foreground_now_rows
        .iter()
        .filter(|row| is_app_foreground(row))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    for row in model
        .daily_rollups
        .iter()
        .filter(|row| is_app_classification(&row.classification_state))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    push_app_game_boundary_evidence(&mut evidence, model);
    evidence
}
