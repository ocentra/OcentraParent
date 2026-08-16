#[path = "shared_rows/logic.rs"]
mod logic;

use ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow;
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    AppGameServiceReadModel,
};

pub(super) fn app_game_source_status_rows(
    model: &AppGameServiceReadModel,
    inventory_filter: fn(&AppGameInventoryEvidenceRow) -> bool,
    runtime_filter: fn(&AppGameRuntimeEvidenceRow) -> bool,
    foreground_filter: fn(&AppGameForegroundEvidenceRow) -> bool,
    include_launcher_rows: bool,
) -> Vec<ActivityAppGameSourceStatusRow> {
    logic::app_game_source_status_rows(
        model,
        inventory_filter,
        runtime_filter,
        foreground_filter,
        include_launcher_rows,
    )
}
