use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityGamesReadModel, ActivityGamesReadModelRow, ActivityReadModelState,
    ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    AppGameServiceReadModel, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_STATE_UNAVAILABLE, APP_GAME_PRODUCT_LAUNCHER, APP_GAME_PRODUCT_NATIVE_GAME,
    APP_GAME_RUNTIME_NOT_CLAIMED,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::activity_surface_read_model_states::{
    offline_games_read_model, request_targets_remote_device, unavailable_games_read_model,
};

use super::shared::{
    app_game_boundary_row_counts, app_game_source_status_rows, push_app_game_boundary_evidence,
    push_evidence, row_device_id, row_state, CapabilityStatus,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct GameText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct GameMaybeText(Option<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClassificationText(String);

pub(crate) fn games_read_model(
    request: ActivitySurfaceRequest,
    model: Option<AppGameServiceReadModel>,
) -> ActivityGamesReadModel {
    if request_targets_remote_device(&request) {
        return offline_games_read_model(request);
    }

    let Some(model) = model else {
        return unavailable_games_read_model(request);
    };
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

fn game_rows(
    request: &ActivitySurfaceRequest,
    model: &AppGameServiceReadModel,
) -> Vec<ActivityGamesReadModelRow> {
    let (inventory, running, foreground, launcher, rollup) = game_sources(model);
    if !has_game_source(inventory, running, foreground, launcher, rollup) {
        return Vec::new();
    }

    let boundary_counts = app_game_boundary_row_counts(model);
    vec![ActivityGamesReadModelRow {
        row_id: game_row_id(inventory, running, foreground, launcher, rollup).0,
        display_name: game_label(inventory, running, foreground, launcher).0,
        device_id: row_device_id(request).0,
        state: row_state(&CapabilityStatus(model.capability_status.clone())),
        product_kind: game_product_kind(inventory, launcher).0,
        classification_state: game_classification(inventory, running, foreground, launcher, rollup)
            .0,
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
        last_observed_at: game_last_observed_at(inventory, running, foreground, launcher).0,
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
            .filter(|rollup| {
                is_game_classification(&ClassificationText(rollup.classification_state.clone()))
            })
            .count() as u64,
        evidence_claim_row_count: boundary_counts.evidence_claim_row_count,
        identity_row_count: boundary_counts.identity_row_count,
        approval_authority_row_count: boundary_counts.approval_authority_row_count,
        approval_action_result_row_count: boundary_counts.approval_action_result_row_count,
        platform_authority_matrix_count: boundary_counts.platform_authority_matrix_count,
        platform_authority_row_count: boundary_counts.platform_authority_row_count,
        ai_classifier_result_row_count: boundary_counts.ai_classifier_result_row_count,
        source_status_rows: game_source_status_rows(model),
        evidence: game_evidence(model),
    }]
}

fn has_game_source(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
    rollup: Option<&ocentra_parent_agent_protocol::app_game::AppGameSessionDailyRollup>,
) -> bool {
    inventory.is_some()
        || running.is_some()
        || foreground.is_some()
        || launcher.is_some()
        || rollup.is_some()
}

fn is_game_inventory(row: &AppGameInventoryEvidenceRow) -> bool {
    row.product_kind == APP_GAME_PRODUCT_NATIVE_GAME
        || row.product_kind == APP_GAME_PRODUCT_LAUNCHER
}

fn is_game_runtime(row: &AppGameRuntimeEvidenceRow) -> bool {
    is_game_classification(&ClassificationText(row.classification_state.clone()))
}

fn is_game_foreground(row: &AppGameForegroundEvidenceRow) -> bool {
    is_game_classification(&ClassificationText(row.classification_state.clone()))
}

fn is_game_classification(classification: &ClassificationText) -> bool {
    matches!(
        classification.0.as_str(),
        APP_GAME_CLASSIFICATION_KNOWN_GAME
            | APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
            | APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE
            | APP_GAME_CLASSIFICATION_POSSIBLY_GAME
    )
}

fn game_source_status_rows(
    model: &AppGameServiceReadModel,
) -> Vec<
    ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow,
> {
    app_game_source_status_rows(
        model,
        is_game_inventory,
        is_game_runtime,
        is_game_foreground,
        true,
    )
}

fn game_row_id(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
    rollup: Option<&ocentra_parent_agent_protocol::app_game::AppGameSessionDailyRollup>,
) -> GameText {
    GameText(
        rollup
            .and_then(|row| row.session_ids.first().cloned())
            .or_else(|| launcher.map(|row| row.launcher_evidence_id.clone()))
            .or_else(|| foreground.map(|row| row.foreground_evidence_id.clone()))
            .or_else(|| running.map(|row| row.runtime_evidence_id.clone()))
            .or_else(|| inventory.map(|row| row.inventory_entry_id.clone()))
            .unwrap_or_else(|| constants::activity_surface::READ_MODEL_GAMES.to_string()),
    )
}

fn game_label(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
) -> GameText {
    GameText(
        foreground
            .map(|row| row.process_name.clone())
            .or_else(|| running.map(|row| row.process_name.clone()))
            .or_else(|| launcher_label(launcher).0)
            .or_else(|| inventory.map(|row| row.display_label.clone()))
            .unwrap_or_else(|| constants::activity_surface::SECTION_GAMES.to_string()),
    )
}

fn launcher_label(
    launcher: Option<&ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
) -> GameMaybeText {
    GameMaybeText(
        launcher
            .and_then(|row| row.launcher_process_name.clone())
            .or_else(|| launcher.map(|row| row.launcher_ref.clone())),
    )
}

fn game_classification(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
    rollup: Option<&ocentra_parent_agent_protocol::app_game::AppGameSessionDailyRollup>,
) -> GameText {
    GameText(
        rollup
            .map(|row| row.classification_state.clone())
            .or_else(|| launcher.map(|row| row.classification_state.clone()))
            .or_else(|| foreground.map(|row| row.classification_state.clone()))
            .or_else(|| running.map(|row| row.classification_state.clone()))
            .or_else(|| inventory.map(|row| row.classification_state.clone()))
            .unwrap_or_else(|| APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string()),
    )
}

fn game_product_kind(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
) -> GameText {
    GameText(
        inventory
            .map(|row| row.product_kind.clone())
            .or_else(|| launcher.map(|_| APP_GAME_PRODUCT_LAUNCHER.to_string()))
            .unwrap_or_else(|| APP_GAME_PRODUCT_NATIVE_GAME.to_string()),
    )
}

fn game_last_observed_at(
    inventory: Option<&AppGameInventoryEvidenceRow>,
    running: Option<&AppGameRuntimeEvidenceRow>,
    foreground: Option<&AppGameForegroundEvidenceRow>,
    launcher: Option<&ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
) -> GameMaybeText {
    GameMaybeText(
        launcher
            .map(|row| row.observed_at.clone())
            .or_else(|| foreground.map(|row| row.observed_at.clone()))
            .or_else(|| running.map(|row| row.observed_at.clone()))
            .or_else(|| inventory.map(|row| row.observed_at.clone())),
    )
}

fn game_total_ms(model: &AppGameServiceReadModel) -> u64 {
    model
        .daily_rollups
        .iter()
        .filter(|rollup| {
            is_game_classification(&ClassificationText(rollup.classification_state.clone()))
        })
        .map(|rollup| rollup.running_duration_ms)
        .sum()
}

fn game_session_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .daily_rollups
        .iter()
        .filter(|rollup| {
            is_game_classification(&ClassificationText(rollup.classification_state.clone()))
        })
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
        .filter(|row| is_game_classification(&ClassificationText(row.classification_state.clone())))
    {
        push_evidence(&mut evidence, &row.evidence);
    }
    push_app_game_boundary_evidence(&mut evidence, model);
    evidence
}

type GameSources<'a> = (
    Option<&'a AppGameInventoryEvidenceRow>,
    Option<&'a AppGameRuntimeEvidenceRow>,
    Option<&'a AppGameForegroundEvidenceRow>,
    Option<&'a ocentra_parent_agent_protocol::app_game::AppGameLauncherEvidenceRow>,
    Option<&'a ocentra_parent_agent_protocol::app_game::AppGameSessionDailyRollup>,
);

fn game_sources(model: &AppGameServiceReadModel) -> GameSources<'_> {
    (
        model
            .inventory_rows
            .iter()
            .find(|row| is_game_inventory(row)),
        model
            .running_now_rows
            .iter()
            .find(|row| is_game_runtime(row)),
        model
            .foreground_now_rows
            .iter()
            .find(|row| is_game_foreground(row)),
        model.launcher_rows.first(),
        model.daily_rollups.iter().find(|rollup| {
            is_game_classification(&ClassificationText(rollup.classification_state.clone()))
        }),
    )
}
