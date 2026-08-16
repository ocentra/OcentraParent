#[path = "shared_boundary.rs"]
mod shared_boundary;
#[path = "shared_rows.rs"]
mod shared_rows;

use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow;
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    AppGameServiceReadModel,
};

pub(super) fn push_app_game_boundary_evidence(
    target: &mut Vec<ActivityEvidenceRef>,
    model: &AppGameServiceReadModel,
) {
    shared_boundary::push_app_game_boundary_evidence(target, model);
}

pub(super) fn app_game_source_status_rows(
    model: &AppGameServiceReadModel,
    inventory_filter: fn(&AppGameInventoryEvidenceRow) -> bool,
    runtime_filter: fn(&AppGameRuntimeEvidenceRow) -> bool,
    foreground_filter: fn(&AppGameForegroundEvidenceRow) -> bool,
    include_launcher_rows: bool,
) -> Vec<ActivityAppGameSourceStatusRow> {
    shared_rows::app_game_source_status_rows(
        model,
        inventory_filter,
        runtime_filter,
        foreground_filter,
        include_launcher_rows,
    )
}
