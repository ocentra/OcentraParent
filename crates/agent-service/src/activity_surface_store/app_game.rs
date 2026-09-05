use std::path::PathBuf;

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::constants;

use crate::{activity_store_path::activity_db_path, time::timestamp_now};

pub(crate) struct AppGameStorePath(pub(crate) PathBuf);

pub(crate) async fn load_app_game_model() -> Option<AppGameServiceReadModel> {
    load_app_game_model_from_path(AppGameStorePath(activity_db_path().into())).await
}

pub(crate) async fn load_app_game_model_from_path(
    path: AppGameStorePath,
) -> Option<AppGameServiceReadModel> {
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path.0).ok()?;
        let generated_at: String = timestamp_now();
        store
            .app_game_service_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}
