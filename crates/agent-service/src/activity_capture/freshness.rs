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

pub(crate) struct ActivityCaptureFreshnessRequest<'a> {
    pub journal_path: &'a Path,
    pub key_path: &'a Path,
    pub store_path: &'a Path,
    pub process_limit: usize,
    pub network_limit: usize,
    pub first_observed_at: &'a str,
    pub next_observed_ats: &'a [&'a str],
    pub generated_at: &'a str,
}

pub(crate) fn record_activity_capture_freshness_to_paths(
    request: ActivityCaptureFreshnessRequest<'_>,
) -> Result<ActivityCaptureFreshnessStatus, ActivityCaptureError> {
    let mut capture_runs = 1;
    let mut latest_ingest = record_activity_capture_to_paths_at(
        request.journal_path,
        request.key_path,
        request.store_path,
        request.process_limit,
        request.network_limit,
        request.first_observed_at,
    )?;
    for observed_at in request.next_observed_ats {
        capture_runs += 1;
        latest_ingest = record_activity_capture_to_paths_at(
            request.journal_path,
            request.key_path,
            request.store_path,
            request.process_limit,
            request.network_limit,
            observed_at,
        )?;
    }

    let store = ActivityStore::open(request.store_path)?;
    let app_game = store.app_game_service_read_model(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        request.generated_at,
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
