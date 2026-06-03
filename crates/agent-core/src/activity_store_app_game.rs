use ocentra_parent_agent_protocol::{
    AppGameSessionReport, AppGameSessionSummary, APP_GAME_CATALOG_NOT_LOADED,
    APP_GAME_SCHEMA_VERSION,
};
use rusqlite::Connection;

mod app_game_session_rollups;
mod app_game_session_time;
mod app_game_sessionization;
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
// WP09 stages foreground-window evidence before live window capture calls it.
#[allow(dead_code)]
pub(crate) mod app_game_windows_foreground;
#[cfg(test)]
mod app_game_windows_foreground_tests;
// WP10 stages launcher evidence before live launcher manifest readers call it.
#[cfg(test)]
mod app_game_sessionization_tests;
#[allow(dead_code)]
pub(crate) mod app_game_windows_launcher;
#[cfg(test)]
mod app_game_windows_launcher_tests;

use crate::{activity_store_app_game_rows::app_game_rows, ActivityStoreError};

pub(crate) fn app_game_session_report(
    connection: &Connection,
    limit: u64,
) -> Result<AppGameSessionReport, ActivityStoreError> {
    let rows = app_game_rows(connection, limit)?;
    let summaries = app_game_sessionization::session_summaries_from_rows(rows, limit);
    Ok(report_from_summaries(limit, &summaries))
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
