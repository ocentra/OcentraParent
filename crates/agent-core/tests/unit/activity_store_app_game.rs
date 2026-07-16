use ocentra_parent_agent_core::activity_store_app_game as real;
use ocentra_parent_agent_core::activity_store_app_game::{
    app_game_session_rollups, app_game_sessionization, app_game_windows_foreground,
    app_game_windows_foreground_source, app_game_windows_inventory,
    app_game_windows_inventory_source, app_game_windows_launcher, app_game_windows_process_runtime,
    app_game_windows_process_source, app_game_windows_registry_source,
    app_game_windows_store_inventory, app_game_windows_store_package_manifest,
    app_game_windows_store_package_source,
};

use real::*;

#[path = "../support/app_game_windows_registry_source_support.rs"]
mod app_game_windows_registry_source_support;

#[path = "activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs"]
mod app_game_journal_sqlite_ingest_tests;
#[path = "activity_store_app_game/app_game_journal_sqlite_protocol_rows_tests.rs"]
mod app_game_journal_sqlite_protocol_rows_tests;
#[path = "activity_store_app_game/app_game_sessionization_tests.rs"]
mod app_game_sessionization_tests;
#[path = "activity_store_app_game/app_game_windows_foreground_source_tests.rs"]
mod app_game_windows_foreground_source_tests;
#[path = "activity_store_app_game/app_game_windows_foreground_tests.rs"]
mod app_game_windows_foreground_tests;
#[path = "activity_store_app_game/app_game_windows_inventory_source_tests.rs"]
mod app_game_windows_inventory_source_tests;
#[path = "activity_store_app_game/app_game_windows_inventory_tests.rs"]
mod app_game_windows_inventory_tests;
#[path = "activity_store_app_game/app_game_windows_launcher_tests.rs"]
mod app_game_windows_launcher_tests;
#[path = "activity_store_app_game/app_game_windows_process_runtime_tests.rs"]
mod app_game_windows_process_runtime_tests;
#[path = "activity_store_app_game/app_game_windows_process_source_tests.rs"]
mod app_game_windows_process_source_tests;
#[path = "activity_store_app_game/app_game_windows_registry_source_tests.rs"]
mod app_game_windows_registry_source_tests;
#[path = "activity_store_app_game/app_game_windows_store_inventory_tests.rs"]
mod app_game_windows_store_inventory_tests;
#[path = "activity_store_app_game/app_game_windows_store_package_source_tests.rs"]
mod app_game_windows_store_package_source_tests;
