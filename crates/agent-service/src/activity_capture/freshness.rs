use std::path::Path;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{constants, ActivityIngestStatus, AppGameServiceReadModel};

use crate::activity_capture::{record_activity_capture_to_paths_at, ActivityCaptureError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActivityCaptureFreshnessStatus {
    pub capture_runs: u64,
    pub latest_ingest: ActivityIngestStatus,
    pub app_game_generated_at: String,
    pub app_game_last_observed_at: Option<String>,
    pub app_game_running_now_returned: u64,
    pub app_game_foreground_now_returned: u64,
}

pub(crate) fn record_activity_capture_freshness_to_paths(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    first_observed_at: &str,
    next_observed_ats: &[&str],
    generated_at: &str,
) -> Result<ActivityCaptureFreshnessStatus, ActivityCaptureError> {
    let mut capture_runs = 1;
    let mut latest_ingest = record_activity_capture_to_paths_at(
        journal_path,
        key_path,
        store_path,
        process_limit,
        network_limit,
        first_observed_at,
    )?;
    for observed_at in next_observed_ats {
        capture_runs += 1;
        latest_ingest = record_activity_capture_to_paths_at(
            journal_path,
            key_path,
            store_path,
            process_limit,
            network_limit,
            observed_at,
        )?;
    }

    let store = ActivityStore::open(store_path)?;
    let app_game = store.app_game_service_read_model(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        generated_at,
    )?;
    Ok(app_game_freshness_status(
        capture_runs,
        latest_ingest,
        app_game,
    ))
}

fn app_game_freshness_status(
    capture_runs: u64,
    latest_ingest: ActivityIngestStatus,
    app_game: AppGameServiceReadModel,
) -> ActivityCaptureFreshnessStatus {
    ActivityCaptureFreshnessStatus {
        capture_runs,
        latest_ingest,
        app_game_generated_at: app_game.generated_at,
        app_game_last_observed_at: app_game
            .running_now_rows
            .iter()
            .map(|row| row.observed_at.clone())
            .max(),
        app_game_running_now_returned: app_game.running_now_returned,
        app_game_foreground_now_returned: app_game.foreground_now_returned,
    }
}
