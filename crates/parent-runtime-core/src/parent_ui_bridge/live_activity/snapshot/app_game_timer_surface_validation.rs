use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::{
    AppGameTimerParentSurfaceReadModel, APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL, APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY,
};

mod artifacts;
mod rows;

pub(super) fn is_consumable(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    valid_header(read_model)
        && artifacts::valid(read_model)
        && artifacts::claims_are_clear(read_model)
        && rows::valid(read_model)
}

fn valid_header(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    read_model.schema_version == APP_GAME_SCHEMA_VERSION
        && !read_model.generated_at.trim().is_empty()
        && !read_model.custody_label.trim().is_empty()
        && !read_model.raw_private_source_rows_included
        && !read_model.timer_runtime_claimed
        && !read_model.scheduler_persistence_claimed
        && !read_model.durable_scheduler_storage_claimed
        && !read_model.adapter_dispatch_claimed
        && !read_model.platform_enforcement_claimed
        && !read_model.child_delivery_claimed
        && read_model.returned == read_model.rows.len() as u64
}

pub(super) fn status_matches(read_model: &AppGameTimerParentSurfaceReadModel, ready: u64) -> bool {
    let expected_status = match (read_model.returned, ready == read_model.returned) {
        (0, _) => APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS,
        (_, true) => APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY,
        (_, false) => APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL,
    };
    read_model.capability_status == expected_status
}
