#[path = "app_game_windows_launcher_classification_state.rs"]
mod app_game_windows_launcher_classification_state;
#[path = "app_game_windows_launcher_status.rs"]
mod app_game_windows_launcher_status;

pub(super) use app_game_windows_launcher_classification_state::{
    classification_state_for_record, game_proof_state_for_record,
};
pub(super) use app_game_windows_launcher_status::{
    capability_status_for_record, catalog_ready_state_for_record, has_launcher_reference,
};
