#[path = "app_game_windows_process_runtime_summary_create.rs"]
mod app_game_windows_process_runtime_summary_create;
#[path = "app_game_windows_process_runtime_summary_update.rs"]
mod app_game_windows_process_runtime_summary_update;

pub(super) use app_game_windows_process_runtime_summary_create::upsert_runtime_summary;
