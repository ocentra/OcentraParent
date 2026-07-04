#[path = "app_game_windows_inventory_state_category.rs"]
mod app_game_windows_inventory_state_category;
#[path = "app_game_windows_inventory_state_classification.rs"]
mod app_game_windows_inventory_state_classification;
#[path = "app_game_windows_inventory_state_status.rs"]
mod app_game_windows_inventory_state_status;

pub(super) use app_game_windows_inventory_state_category::category_candidates_for_record;
pub(super) use app_game_windows_inventory_state_classification::classification_state_for_record;
pub(super) use app_game_windows_inventory_state_status::{
    capability_status_for_record, catalog_ready_state_for_record,
};
