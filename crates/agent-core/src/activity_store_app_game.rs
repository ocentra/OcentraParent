use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, AppGameSessionDailyRollup, AppGameSessionReport,
    AppGameSessionSummary, APP_GAME_CATALOG_NOT_LOADED, APP_GAME_SCHEMA_VERSION,
};
use rusqlite::Connection;
use sysinfo::System;

pub mod app_game_journal_sqlite_ingest;
pub mod app_game_session_rollups;
mod app_game_session_time;
pub mod app_game_sessionization;
// WP06 stages the typed parser before live Windows source readers call it.
pub mod app_game_windows_inventory;
// WP41 adds a bounded live Windows shortcut inventory source.
pub mod app_game_windows_inventory_source;
// WP45 adds a bounded live Windows installed-app registry source.
mod app_game_windows_registry_export;
#[cfg(windows)]
mod app_game_windows_registry_live;
mod app_game_windows_registry_record;
pub mod app_game_windows_registry_source;
// WP07 stages Store/UWP package parsing before live package readers call it.
pub mod app_game_windows_store_inventory;
// WP43 adds a bounded live Windows packaged-app manifest source.
pub mod app_game_windows_store_package_manifest;
pub mod app_game_windows_store_package_source;
// WP08 stages process runtime evidence before live process capture calls it.
pub mod app_game_windows_process_runtime;
// WP32 adds a real process snapshot source that feeds the staged runtime rows.
pub mod app_game_windows_process_source;
// WP09 stages foreground-window evidence before live window capture calls it.
pub mod app_game_windows_foreground;
// WP36 adds a real foreground-window source that feeds the staged rows.
pub mod app_game_windows_foreground_source;
// WP10 stages launcher evidence before live launcher manifest readers call it.
pub mod app_game_windows_launcher;
// WP10 live Windows process source feeds launcher evidence into the journal.
pub mod app_game_windows_launcher_source;

use crate::{activity_store_app_game_rows::app_game_rows, ActivityStoreError};

use app_game_windows_foreground_source::{
    live_windows_foreground_window_journal_event as live_windows_foreground_window_journal_event_impl,
    live_windows_foreground_window_journal_event_from_system as live_windows_foreground_window_journal_event_from_system_impl,
    AppGameLiveForegroundWindowError as AppGameLiveForegroundWindowErrorImpl,
};
use app_game_windows_inventory_source::{
    live_windows_inventory_journal_events_from_roots as live_windows_inventory_journal_events_from_roots_impl,
    live_windows_inventory_journal_events_with_limit as live_windows_inventory_journal_events_with_limit_impl,
    AppGameLiveInventorySourceError as AppGameLiveInventorySourceErrorImpl,
};
use app_game_windows_launcher_source::live_windows_launcher_journal_events_with_limit as live_windows_launcher_journal_events_with_limit_impl;
use app_game_windows_process_source::{
    live_windows_process_and_launcher_snapshot_journal_events_from_system as live_windows_process_and_launcher_snapshot_journal_events_from_system_impl,
    live_windows_process_and_launcher_snapshot_journal_events_with_limit as live_windows_process_and_launcher_snapshot_journal_events_with_limit_impl,
    live_windows_process_snapshot_journal_events_with_limit as live_windows_process_snapshot_journal_events_with_limit_impl,
    AppGameLiveProcessSnapshotError as AppGameLiveProcessSnapshotErrorImpl,
};
use app_game_windows_registry_source::{
    live_windows_registry_inventory_journal_events_from_roots as live_windows_registry_inventory_journal_events_from_roots_impl,
    live_windows_registry_inventory_journal_events_with_limit as live_windows_registry_inventory_journal_events_with_limit_impl,
    AppGameLiveRegistryInventorySourceError as AppGameLiveRegistryInventorySourceErrorImpl,
};
use app_game_windows_store_package_source::{
    live_windows_store_package_journal_events_from_roots as live_windows_store_package_journal_events_from_roots_impl,
    live_windows_store_package_journal_events_with_limit as live_windows_store_package_journal_events_with_limit_impl,
    AppGameLiveStorePackageSourceError as AppGameLiveStorePackageSourceErrorImpl,
};

pub type AppGameLiveForegroundWindowError = AppGameLiveForegroundWindowErrorImpl;
pub type AppGameLiveInventorySourceError = AppGameLiveInventorySourceErrorImpl;
pub type AppGameLiveProcessSnapshotError = AppGameLiveProcessSnapshotErrorImpl;
pub type AppGameLiveRegistryInventorySourceError = AppGameLiveRegistryInventorySourceErrorImpl;
pub type AppGameLiveStorePackageSourceError = AppGameLiveStorePackageSourceErrorImpl;
pub type AppGameLiveLauncherSourceError =
    app_game_windows_launcher_source::AppGameLiveLauncherSourceError;

pub fn live_windows_foreground_window_journal_event(
    device_id: &str,
    platform: &str,
    observed_at: &str,
) -> Result<Option<ActivityEvent>, AppGameLiveForegroundWindowError> {
    live_windows_foreground_window_journal_event_impl(device_id, platform, observed_at)
}

pub fn live_windows_foreground_window_journal_event_from_system(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    system: &System,
) -> Result<Option<ActivityEvent>, AppGameLiveForegroundWindowError> {
    live_windows_foreground_window_journal_event_from_system_impl(
        device_id,
        platform,
        observed_at,
        system,
    )
}

pub fn live_windows_inventory_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveInventorySourceError> {
    live_windows_inventory_journal_events_with_limit_impl(device_id, platform, observed_at, limit)
}

pub fn live_windows_inventory_journal_events_from_roots(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    roots: &[std::path::PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveInventorySourceError> {
    live_windows_inventory_journal_events_from_roots_impl(
        device_id,
        platform,
        observed_at,
        roots,
        limit,
    )
}

pub fn live_windows_process_snapshot_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveProcessSnapshotError> {
    live_windows_process_snapshot_journal_events_with_limit_impl(
        device_id,
        platform,
        observed_at,
        limit,
    )
}

pub fn live_windows_process_and_launcher_snapshot_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveProcessSnapshotError> {
    live_windows_process_and_launcher_snapshot_journal_events_with_limit_impl(
        device_id,
        platform,
        observed_at,
        limit,
    )
}

pub fn live_windows_process_and_launcher_snapshot_journal_events_from_system(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
    system: &System,
) -> Result<Vec<ActivityEvent>, AppGameLiveProcessSnapshotError> {
    live_windows_process_and_launcher_snapshot_journal_events_from_system_impl(
        device_id,
        platform,
        observed_at,
        limit,
        system,
    )
}

pub fn live_windows_launcher_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveLauncherSourceError> {
    live_windows_launcher_journal_events_with_limit_impl(device_id, platform, observed_at, limit)
}

pub fn live_windows_registry_inventory_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveRegistryInventorySourceError> {
    live_windows_registry_inventory_journal_events_with_limit_impl(
        device_id,
        platform,
        observed_at,
        limit,
    )
}

pub fn live_windows_registry_inventory_journal_events_from_roots(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    roots: &[std::path::PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveRegistryInventorySourceError> {
    live_windows_registry_inventory_journal_events_from_roots_impl(
        device_id,
        platform,
        observed_at,
        roots,
        limit,
    )
}

pub fn live_windows_store_package_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveStorePackageSourceError> {
    live_windows_store_package_journal_events_with_limit_impl(
        device_id,
        platform,
        observed_at,
        limit,
    )
}

pub fn live_windows_store_package_journal_events_from_roots(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    roots: &[std::path::PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveStorePackageSourceError> {
    live_windows_store_package_journal_events_from_roots_impl(
        device_id,
        platform,
        observed_at,
        roots,
        limit,
    )
}

pub(crate) fn app_game_session_report(
    connection: &Connection,
    limit: u64,
) -> Result<AppGameSessionReport, ActivityStoreError> {
    let summaries = app_game_session_summaries(connection, limit)?;
    Ok(report_from_summaries(limit, &summaries))
}

pub(crate) fn app_game_session_daily_rollups(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<AppGameSessionDailyRollup>, ActivityStoreError> {
    let summaries = app_game_session_summaries(connection, limit)?;
    Ok(app_game_session_rollups::daily_rollups_from_summaries(
        &summaries,
    ))
}

pub(crate) fn app_game_service_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<AppGameServiceReadModel, ActivityStoreError> {
    app_game_journal_sqlite_ingest::read_model::app_game_journal_sqlite_read_model(
        connection,
        limit,
        generated_at,
    )
}

pub(crate) fn app_game_session_summaries(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<AppGameSessionSummary>, ActivityStoreError> {
    let rows = app_game_rows(connection, limit)?;
    Ok(app_game_sessionization::session_summaries_from_rows(
        rows, limit,
    ))
}

fn report_from_summaries(limit: u64, summaries: &[AppGameSessionSummary]) -> AppGameSessionReport {
    let most_recent = summaries.first();
    AppGameSessionReport {
        schema_version: APP_GAME_SCHEMA_VERSION,
        limit,
        returned: summaries.len() as u64,
        catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
        first_observed_at: summaries.last().map(|summary| summary.started_at.clone()),
        last_observed_at: most_recent.map(|summary| summary.last_observed_at.clone()),
        most_recent_session_id: most_recent.map(|summary| summary.session_id.clone()),
        most_recent_classification_state: most_recent
            .map(|summary| summary.classification_state.clone()),
        most_recent_process_identity: most_recent
            .map(|summary| summary.primary_process_identity.clone()),
        most_recent_display_name: most_recent.map(|summary| summary.display_name.clone()),
        most_recent_running_duration_ms: most_recent.map(|summary| summary.running_duration_ms),
        most_recent_foreground_duration_ms: most_recent
            .map(|summary| summary.foreground_duration_ms),
        most_recent_evidence_count: most_recent.map(|summary| summary.evidence_count),
    }
}
