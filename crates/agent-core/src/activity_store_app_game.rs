use ocentra_parent_agent_protocol::{
    AppGameSessionReport, AppGameSessionSummary, APP_GAME_CATALOG_NOT_LOADED,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_SCHEMA_VERSION,
};
use rusqlite::Connection;

// WP06 stages the typed parser before live Windows source readers call it.
#[allow(dead_code)]
pub(crate) mod app_game_windows_inventory;
#[cfg(test)]
mod app_game_windows_inventory_tests;
// WP07 stages Store/UWP package parsing before live package readers call it.
#[allow(dead_code)]
pub(crate) mod app_game_windows_store_inventory;
#[cfg(test)]
mod app_game_windows_store_inventory_tests;
// WP08 stages process runtime evidence before live process capture calls it.
#[allow(dead_code)]
pub(crate) mod app_game_windows_process_runtime;
#[cfg(test)]
mod app_game_windows_process_runtime_tests;

use crate::{
    activity_store_app_game_observation::AppGameObservation,
    activity_store_app_game_rows::app_game_rows, ActivityStoreError,
};

pub(crate) fn app_game_session_report(
    connection: &Connection,
    limit: u64,
) -> Result<AppGameSessionReport, ActivityStoreError> {
    let rows = app_game_rows(connection, limit)?;
    let summaries = session_summaries_from_rows(rows, limit);
    Ok(report_from_summaries(limit, &summaries))
}

fn session_summaries_from_rows(
    rows: Vec<crate::activity_store_app_game_rows::AppGameStoreRow>,
    limit: u64,
) -> Vec<AppGameSessionSummary> {
    let mut summaries = Vec::new();
    for row in rows {
        let observation = AppGameObservation::from_row(row);
        upsert_summary(&mut summaries, observation);
        if summaries.len() >= limit as usize {
            break;
        }
    }
    summaries
}

fn upsert_summary(summaries: &mut Vec<AppGameSessionSummary>, observation: AppGameObservation) {
    match summaries
        .iter_mut()
        .find(|summary| summary.primary_process_identity == observation.process_identity)
    {
        Some(summary) => update_summary(summary, observation),
        None => summaries.push(observation.into_summary()),
    }
}

fn update_summary(summary: &mut AppGameSessionSummary, observation: AppGameObservation) {
    summary.started_at = observation.observed_at;
    summary.observation_count += 1;
    summary.evidence_count += observation.evidence.len() as u64;
    summary.evidence.extend(observation.evidence);
    if is_stronger_classification(
        &observation.classification_state,
        &summary.classification_state,
    ) {
        summary.classification_state = observation.classification_state;
        summary.display_name = observation.display_name;
        summary.confidence = observation.confidence;
    }
}

fn report_from_summaries(limit: u64, summaries: &[AppGameSessionSummary]) -> AppGameSessionReport {
    let most_recent = summaries.first();
    AppGameSessionReport {
        schema_version: APP_GAME_SCHEMA_VERSION,
        limit,
        returned: summaries.len() as u64,
        catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
        first_observed_at: summaries.last().map(|summary| summary.started_at.clone()),
        last_observed_at: most_recent.map(|summary| summary.last_observed_at.clone()),
        most_recent_session_id: most_recent.map(|summary| summary.session_id.clone()),
        most_recent_classification_state: most_recent
            .map(|summary| summary.classification_state.clone()),
        most_recent_process_identity: most_recent
            .map(|summary| summary.primary_process_identity.clone()),
        most_recent_display_name: most_recent.map(|summary| summary.display_name.clone()),
        most_recent_running_duration_ms: most_recent.map(|summary| summary.running_duration_ms),
        most_recent_foreground_duration_ms: most_recent
            .map(|summary| summary.foreground_duration_ms),
        most_recent_evidence_count: most_recent.map(|summary| summary.evidence_count),
    }
}

fn is_stronger_classification(candidate: &str, current: &str) -> bool {
    classification_rank(candidate) > classification_rank(current)
}

fn classification_rank(value: &str) -> u8 {
    match value {
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME => 2,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED => 1,
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR => 1,
        _ => 0,
    }
}
