use std::path::PathBuf;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityNetworkFlowReadModel, ActivityRecentSummary, AppGameSessionReport,
    BrowserEvidenceReadModel, ScreenEvidenceRecentSummary,
};

use crate::{activity_store_path::activity_db_path, time::timestamp_now};

pub(crate) struct ActivitySurfaceStoreSnapshot {
    pub(crate) recent_returned: u64,
    pub(crate) last_event_id: Option<String>,
    pub(crate) browser_returned: u64,
    pub(crate) network_returned: u64,
    pub(crate) games_returned: u64,
    pub(crate) screen_returned: u64,
}

pub(crate) async fn local_store_snapshot() -> Option<ActivitySurfaceStoreSnapshot> {
    local_store_snapshot_from_path(activity_db_path()).await
}

pub(crate) async fn local_store_snapshot_from_path(
    path: PathBuf,
) -> Option<ActivitySurfaceStoreSnapshot> {
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        let generated_at = timestamp_now();
        let recent = store
            .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .ok()?;
        let browser = store
            .browser_evidence_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()?;
        let network = store
            .network_flow_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()?;
        let games = store
            .app_game_session_report(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .ok()?;
        let screen = store
            .screen_evidence_recent_summary(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()?;
        Some(ActivitySurfaceStoreSnapshot {
            recent_returned: recent.returned,
            last_event_id: recent.last_event_id,
            browser_returned: browser.returned,
            network_returned: network.returned,
            games_returned: games.returned,
            screen_returned: screen.returned,
        })
    })
    .await
    .ok()
    .flatten()
}

pub(crate) async fn load_recent_summary() -> Option<ActivityRecentSummary> {
    load_recent_summary_from_path(activity_db_path()).await
}

pub(crate) async fn load_recent_summary_from_path(path: PathBuf) -> Option<ActivityRecentSummary> {
    with_store(path, |store| {
        store
            .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .ok()
    })
    .await
}

pub(crate) async fn load_browser_model() -> Option<BrowserEvidenceReadModel> {
    load_browser_model_from_path(activity_db_path()).await
}

pub(crate) async fn load_browser_model_from_path(
    path: PathBuf,
) -> Option<BrowserEvidenceReadModel> {
    with_store(path, |store| {
        let generated_at = timestamp_now();
        store
            .browser_evidence_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()
    })
    .await
}

pub(crate) async fn load_network_model() -> Option<ActivityNetworkFlowReadModel> {
    load_network_model_from_path(activity_db_path()).await
}

pub(crate) async fn load_network_model_from_path(
    path: PathBuf,
) -> Option<ActivityNetworkFlowReadModel> {
    with_store(path, |store| {
        let generated_at = timestamp_now();
        store
            .network_flow_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()
    })
    .await
}

pub(crate) async fn load_app_game_report() -> Option<AppGameSessionReport> {
    load_app_game_report_from_path(activity_db_path()).await
}

pub(crate) async fn load_app_game_report_from_path(path: PathBuf) -> Option<AppGameSessionReport> {
    with_store(path, |store| {
        store
            .app_game_session_report(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .ok()
    })
    .await
}

pub(crate) async fn load_screen_summary() -> Option<ScreenEvidenceRecentSummary> {
    load_screen_summary_from_path(activity_db_path()).await
}

pub(crate) async fn load_screen_summary_from_path(
    path: PathBuf,
) -> Option<ScreenEvidenceRecentSummary> {
    with_store(path, |store| {
        let generated_at = timestamp_now();
        store
            .screen_evidence_recent_summary(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()
    })
    .await
}

async fn with_store<T, F>(path: PathBuf, read: F) -> Option<T>
where
    T: Send + 'static,
    F: FnOnce(ActivityStore) -> Option<T> + Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        read(store)
    })
    .await
    .ok()
    .flatten()
}
