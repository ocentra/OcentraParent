use std::path::PathBuf;

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::screen_evidence::ScreenEvidenceRecentSummary;

use crate::{activity_store_path::activity_db_path, time::timestamp_now};

#[path = "activity_surface_store/app_game.rs"]
pub(crate) mod app_game;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActivitySurfaceDeviceRefText(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActivityStorePath(pub(crate) PathBuf);

pub(crate) struct ActivitySurfaceStoreSnapshot {
    pub(crate) device_id: ActivitySurfaceDeviceRefText,
    pub(crate) recent_returned: u64,
    pub(crate) last_event_id: Option<String>,
    pub(crate) last_observed_at: Option<String>,
    pub(crate) browser_returned: u64,
    pub(crate) network_returned: u64,
    pub(crate) games_returned: u64,
    pub(crate) screen_returned: u64,
}

pub(crate) async fn local_store_snapshot() -> Option<ActivitySurfaceStoreSnapshot> {
    local_store_snapshot_from_path(ActivityStorePath(activity_db_path().into())).await
}

pub(crate) async fn local_store_snapshot_from_path(
    path: ActivityStorePath,
) -> Option<ActivitySurfaceStoreSnapshot> {
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path.0).ok()?;
        let generated_at: String = timestamp_now();
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
            .app_game_service_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()?;
        let screen = store
            .screen_evidence_recent_summary(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()?;
        Some(ActivitySurfaceStoreSnapshot {
            device_id: ActivitySurfaceDeviceRefText(
                constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            ),
            recent_returned: recent.returned,
            last_event_id: recent.last_event_id,
            last_observed_at: recent.last_observed_at,
            browser_returned: browser.returned,
            network_returned: network.returned,
            games_returned: games.daily_rollup_returned + games.launcher_returned,
            screen_returned: screen.returned,
        })
    })
    .await
    .ok()
    .flatten()
}

pub(crate) async fn load_browser_model() -> Option<BrowserEvidenceReadModel> {
    load_browser_model_from_path(ActivityStorePath(activity_db_path().into())).await
}

pub(crate) async fn load_browser_model_from_path(
    path: ActivityStorePath,
) -> Option<BrowserEvidenceReadModel> {
    with_store(path, |store| {
        let generated_at: String = timestamp_now();
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
    load_network_model_from_path(ActivityStorePath(activity_db_path().into())).await
}

pub(crate) async fn load_network_model_from_path(
    path: ActivityStorePath,
) -> Option<ActivityNetworkFlowReadModel> {
    with_store(path, |store| {
        let generated_at: String = timestamp_now();
        store
            .network_flow_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()
    })
    .await
}

pub(crate) async fn load_screen_summary() -> Option<ScreenEvidenceRecentSummary> {
    load_screen_summary_from_path(ActivityStorePath(activity_db_path().into())).await
}

pub(crate) async fn load_screen_summary_from_path(
    path: ActivityStorePath,
) -> Option<ScreenEvidenceRecentSummary> {
    with_store(path, |store| {
        let generated_at: String = timestamp_now();
        store
            .screen_evidence_recent_summary(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()
    })
    .await
}

async fn with_store<T, F>(path: ActivityStorePath, read: F) -> Option<T>
where
    T: Send + 'static,
    F: FnOnce(ActivityStore) -> Option<T> + Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path.0).ok()?;
        read(store)
    })
    .await
    .ok()
    .flatten()
}
