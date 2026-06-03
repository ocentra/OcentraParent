use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, ActivityGamesReadModel, ActivityGamesReadModelRow,
    ActivityReadModelState, ActivitySurfaceRequest, AppGameForegroundEvidenceRow,
    AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow, AppGameServiceReadModel,
    ACTIVITY_SURFACE_SCHEMA_VERSION, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_STATE_UNAVAILABLE, APP_GAME_PRODUCT_LAUNCHER, APP_GAME_PRODUCT_NATIVE_GAME,
    APP_GAME_RUNTIME_NOT_CLAIMED,
};

use crate::activity_surface_read_model_states::{
    offline_games_read_model, request_targets_remote_device, unavailable_games_read_model,
};

use super::shared::{push_evidence, row_device_id, row_state};

pub(crate) fn games_read_model(
    request: ActivitySurfaceRequest,
    model: Option<AppGameServiceReadModel>,
) -> ActivityGamesReadModel {
    if request_targets_remote_device(&request) {
        return offline_games_read_model(request);
    }

    match model {
        Some(model) => {
            let generated_at = model.generated_at.clone();
            let rows = game_rows(&request, &model);
            if rows.is_empty() {
                return ActivityGamesReadModel {
                    schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
                    request,
                    state: ActivityReadModelState::Empty,
                    generated_at,
                    summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
                    rows,
                };
            }
            ActivityGamesReadModel {
                schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
                request,
                state: ActivityReadModelState::Ready,
                generated_at,
                summary: constants::activity_surface::SUMMARY_READY.to_string(),
                rows,
            }
        }
        None => unavailable_games_read_model(request),
    }
}

fn game_rows(
    request: &ActivitySurfaceRequest,
    model: &AppGameServiceReadModel,
) -> Vec<ActivityGamesReadModelRow> {
    let inventory = model
        .inventory_rows
        .iter()
        .find(|row| is_game_inventory(row));
    let running = model
        .running_now_rows
        .iter()
        .find(|row| is_game_runtime(row));
    let foreground = model
        .foreground_now_rows
        .iter()
        .find(|row| is_game_foreground(row));
    let launcher = model.launcher_rows.first();
    let rollup = model
        .daily_rollups
        .iter()
        .find(|rollup| is_game_classification(&rollup.classification_state));
    if inventory.is_none()
        && running.is_none()
        && foreground.is_none()
        && launcher.is_none()
        && rollup.is_none()
    {
        return Vec::new();
    }

    vec![ActivityGamesReadModelRow {
        row_id: game_row_id(inventory, running, foreground, launcher, rollup),
        display_name: game_label(inventory, running, foreground, launcher),
        device_id: row_device_id(request),
        state: row_state(&model.capability_status),
        product_kind: game_product_kind(inventory, launcher),
        classification_state: game_classification(inventory, running, foreground, launcher, rollup),
        inventory_state: inventory
            .map(|row| row.inventory_state.clone())
            .unwrap_or_else(|| APP_GAME_INVENTORY_STATE_UNAVAILABLE.to_string()),
        runtime_state: running
            .map(|row| row.runtime_state.clone())
            .or_else(|| foreground.map(|row| row.runtime_state.clone()))
            .or_else(|| launcher.map(|row| row.runtime_state.clone()))
            .unwrap_or_else(|| APP_GAME_RUNTIME_NOT_CLAIMED.to_string()),
        foreground_state: foreground
            .map(|row| row.foreground_state.clone())
            .or_else(|| running.map(|row| row.foreground_state.clone()))
            .or_else(|| launcher.map(|row| row.foreground_state.clone()))
            .or_else(|| inventory.map(|row| row.foreground_state.clone()))
            .unwrap_or_else(|| APP_GAME_FOREGROUND_NOT_CLAIMED.to_string()),
        capability_status: model.capability_status.clone(),
        last_observed_at: game_last_observed_at(inventory, running, foreground, launcher),
        total_ms: game_total_ms(model),
        session_count: game_session_count(model),
        launcher_row_count: model.launcher_returned,
        running_row_count: model
            .running_now_rows
            .iter()
            .filter(|row| is_game_runtime(row))
            .count() as u64,
        foreground_row_count: model
            .foreground_now_rows
            .iter()
            .filter(|row| is_game_foreground(row))
            .count() as u64,
        daily_rollup_count: model
            .daily_rollups
            .iter()
            .filter(|rollup| is_game_classification(&rollup.classification_state))
            .count() as u64,
        evidence: game_evidence(model),
    }]
}

fn is_game_inventory(row: &AppGameInventoryEvidenceRow) -> bool {
    row.product_kind == APP_GAME_PRODUCT_NATIVE_GAME
        || row.product_kind == APP_GAME_PRODUCT_LAUNCHER
}

fn is_game_runtime(row: &AppGameRuntimeEvidenceRow) -> bool {
    is_game_classification(&row.classification_state)
}

fn is_game_foreground(row: &AppGameForegroundEvidenceRow) -> bool {
    is_game_classification(&row.classification_state)
}

fn is_game_classification(classification: &str) -> bool {
    matches!(
        classification,
        APP_GAME_CLASSIFICATION_KNOWN_GAME
            | APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
            | APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE
            | APP_GAME_CLASSIFICATION_POSSIBLY_GAME
    )
}

fn game_row_id(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::AppGameLauncherEvidenceRow>,
    rollup: Option<&ocentra_parent_agent_protocol::AppGameSessionDailyRollup>,
) -> String {
    rollup
        .and_then(|row| row.session_ids.first().cloned())
        .or_else(|| launcher.map(|row| row.launcher_evidence_id.clone()))
        .or_else(|| foreground.map(|row| row.foreground_evidence_id.clone()))
        .or_else(|| running.map(|row| row.runtime_evidence_id.clone()))
        .or_else(|| inventory.map(|row| row.inventory_entry_id.clone()))
        .unwrap_or_else(|| constants::activity_surface::READ_MODEL_GAMES.to_string())
}

fn game_label(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::AppGameLauncherEvidenceRow>,
) -> String {
    foreground
        .map(|row| row.process_name.clone())
        .or_else(|| running.map(|row| row.process_name.clone()))
        .or_else(|| {
            launcher
                .and_then(|row| row.launcher_process_name.clone())
                .or_else(|| launcher.map(|row| row.launcher_ref.clone()))
        })
        .or_else(|| inventory.map(|row| row.display_label.clone()))
        .unwrap_or_else(|| constants::activity_surface::SECTION_GAMES.to_string())
}

fn game_classification(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::AppGameLauncherEvidenceRow>,
    rollup: Option<&ocentra_parent_agent_protocol::AppGameSessionDailyRollup>,
) -> String {
    rollup
        .map(|row| row.classification_state.clone())
        .or_else(|| launcher.map(|row| row.classification_state.clone()))
        .or_else(|| foreground.map(|row| row.classification_state.clone()))
        .or_else(|| running.map(|row| row.classification_state.clone()))
        .or_else(|| inventory.map(|row| row.classification_state.clone()))
        .unwrap_or_else(|| APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string())
}

fn game_product_kind(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::AppGameLauncherEvidenceRow>,
) -> String {
    inventory
        .map(|row| row.product_kind.clone())
        .or_else(|| launcher.map(|_| APP_GAME_PRODUCT_LAUNCHER.to_string()))
        .unwrap_or_else(|| APP_GAME_PRODUCT_NATIVE_GAME.to_string())
}

fn game_last_observed_at(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::AppGameLauncherEvidenceRow>,
) -> Option<String> {
    launcher
        .map(|row| row.observed_at.clone())
        .or_else(|| foreground.map(|row| row.observed_at.clone()))
        .or_else(|| running.map(|row| row.observed_at.clone()))
        .or_else(|| inventory.map(|row| row.observed_at.clone()))
}

fn game_total_ms(model: &AppGameServiceReadModel) -> u64 {
    model
        .daily_rollups
        .iter()
        .filter(|rollup| is_game_classification(&rollup.classification_state))
        .map(|rollup| rollup.running_duration_ms)
        .sum()
}

fn game_session_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .daily_rollups
        .iter()
        .filter(|rollup| is_game_classification(&rollup.classification_state))
        .map(|rollup| rollup.session_count)
        .sum::<u64>()
        .max(model.launcher_returned)
}

fn game_evidence(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in model
        .inventory_rows
        .iter()
        .filter(|row| is_game_inventory(row))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    for row in model
        .running_now_rows
        .iter()
        .filter(|row| is_game_runtime(row))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    for row in model
        .foreground_now_rows
        .iter()
        .filter(|row| is_game_foreground(row))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    for row in &model.launcher_rows {
        push_evidence(&mut evidence, &row.evidence);
    }
    for row in model
        .daily_rollups
        .iter()
        .filter(|row| is_game_classification(&row.classification_state))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    evidence
}
