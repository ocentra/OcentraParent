use std::path::{Path as TestPath, PathBuf as TestPathBuf};
use std::string::String as TestString;

use crate::test_invariants::require_ok;
use crate::test_text::TestText;
#[cfg(windows)]
use ocentra_parent_agent_core::activity_store_app_game::{
    live_windows_foreground_window_journal_event, live_windows_inventory_journal_events_from_roots,
    live_windows_process_snapshot_journal_events_with_limit,
    live_windows_registry_inventory_journal_events_from_roots,
    live_windows_store_package_journal_events_from_roots,
};
use ocentra_parent_agent_core::{
    activity_store::ActivityStore, network_capture_event::network_snapshot_events,
    process_capture::process_snapshot_events, window_capture_event::foreground_window_event,
};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityCaptureFreshnessStatusForTest {
    pub capture_runs: u64,
    pub latest_ingest: ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus,
    pub app_game_generated_at: TestString,
    pub app_game_last_observed_at: Option<TestString>,
    pub app_game_running_now_returned: u64,
    pub app_game_foreground_now_returned: u64,
}

pub fn startup_activity_capture_enabled_for_value_for_test(value: Option<TestText>) -> bool {
    let value = value.map(|value| value.0);
    crate::activity_capture::startup_activity_capture_enabled_for_value(
        &crate::activity_capture::StartupActivityCaptureDisabledValue(value.as_deref()),
    )
}

pub fn record_activity_events_to_paths_for_test(
    journal_path: impl AsRef<TestPath>,
    key_path: impl AsRef<TestPath>,
    store_path: impl AsRef<TestPath>,
    events: &[ActivityEvent],
) -> Result<ActivityIngestStatus, crate::activity_capture::ActivityCaptureError> {
    crate::activity_capture::record_activity_events_to_paths(
        journal_path.as_ref(),
        key_path.as_ref(),
        store_path.as_ref(),
        events,
    )
}

pub fn record_activity_capture_to_paths_for_test(
    journal_path: impl AsRef<TestPath>,
    key_path: impl AsRef<TestPath>,
    store_path: impl AsRef<TestPath>,
    process_limit: usize,
    network_limit: usize,
) -> Result<ActivityIngestStatus, crate::activity_capture::ActivityCaptureError> {
    crate::activity_capture::record_activity_capture_to_paths(
        journal_path.as_ref(),
        key_path.as_ref(),
        store_path.as_ref(),
        process_limit,
        network_limit,
    )
}

pub fn record_activity_capture_freshness_to_paths_for_test<J, K, S>(
    paths: (J, K, S),
    limits: (usize, usize),
    freshness: (TestText, &[TestText], TestText),
) -> Result<ActivityCaptureFreshnessStatusForTest, crate::activity_capture::ActivityCaptureError>
where
    J: AsRef<TestPath>,
    K: AsRef<TestPath>,
    S: AsRef<TestPath>,
{
    let _: () = require_ok(
        Ok::<(), crate::activity_capture::ActivityCaptureError>(()),
        "activity_capture linkage ok",
    );
    let (journal_path, key_path, store_path) = paths;
    let (process_limit, network_limit) = limits;
    let (first_observed_at, next_observed_ats, generated_at) = freshness;
    let mut capture_runs = 1;
    let mut latest_ingest = crate::activity_capture::record_activity_capture_to_paths_at(
        journal_path.as_ref(),
        key_path.as_ref(),
        store_path.as_ref(),
        process_limit,
        network_limit,
        &crate::activity_capture::ActivityCaptureObservedAt(first_observed_at.0.as_str()),
    )?;
    for observed_at in next_observed_ats {
        capture_runs += 1;
        latest_ingest = crate::activity_capture::record_activity_capture_to_paths_at(
            journal_path.as_ref(),
            key_path.as_ref(),
            store_path.as_ref(),
            process_limit,
            network_limit,
            &crate::activity_capture::ActivityCaptureObservedAt(observed_at.0.as_str()),
        )?;
    }

    let store = ActivityStore::open(store_path)?;
    let app_game = store.app_game_service_read_model(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        generated_at.0.as_str(),
    )?;

    Ok(ActivityCaptureFreshnessStatusForTest {
        capture_runs,
        latest_ingest,
        app_game_generated_at: app_game.generated_at.clone(),
        app_game_last_observed_at: app_game
            .running_now_rows
            .iter()
            .map(|row| row.observed_at.clone())
            .max(),
        app_game_running_now_returned: app_game.running_now_returned,
        app_game_foreground_now_returned: app_game.foreground_now_returned,
    })
}

pub fn record_activity_capture_to_paths_at_with_inventory_roots_for_test<P>(
    journal_path: impl AsRef<TestPath>,
    key_path: impl AsRef<TestPath>,
    store_path: impl AsRef<TestPath>,
    process_limit: usize,
    network_limit: usize,
    observed_at: TestText,
    inventory_roots: &[P],
) -> Result<ActivityIngestStatus, crate::activity_capture::ActivityCaptureError>
where
    P: AsRef<TestPath>,
{
    let observed_at = observed_at.0;
    let events = activity_capture_events_with_inventory_sources(
        TestText(observed_at.clone()),
        process_limit,
        network_limit,
        live_inventory_events_from_roots_for_test(TestText(observed_at), inventory_roots)?,
        Vec::new(),
        Vec::new(),
    )?;
    crate::activity_capture::record_activity_events_to_paths(
        journal_path.as_ref(),
        key_path.as_ref(),
        store_path.as_ref(),
        &events,
    )
}

pub fn record_activity_capture_to_paths_at_with_store_package_roots_for_test<P>(
    journal_path: impl AsRef<TestPath>,
    key_path: impl AsRef<TestPath>,
    store_path: impl AsRef<TestPath>,
    process_limit: usize,
    network_limit: usize,
    observed_at: TestText,
    store_package_roots: &[P],
) -> Result<ActivityIngestStatus, crate::activity_capture::ActivityCaptureError>
where
    P: AsRef<TestPath>,
{
    let observed_at = observed_at.0;
    let events = activity_capture_events_with_inventory_sources(
        TestText(observed_at.clone()),
        process_limit,
        network_limit,
        Vec::new(),
        live_store_package_events_from_roots_for_test(TestText(observed_at), store_package_roots)?,
        Vec::new(),
    )?;
    crate::activity_capture::record_activity_events_to_paths(
        journal_path.as_ref(),
        key_path.as_ref(),
        store_path.as_ref(),
        &events,
    )
}

pub fn record_activity_capture_to_paths_at_with_registry_inventory_roots_for_test<P>(
    journal_path: impl AsRef<TestPath>,
    key_path: impl AsRef<TestPath>,
    store_path: impl AsRef<TestPath>,
    process_limit: usize,
    network_limit: usize,
    observed_at: TestText,
    registry_roots: &[P],
) -> Result<ActivityIngestStatus, crate::activity_capture::ActivityCaptureError>
where
    P: AsRef<TestPath>,
{
    let observed_at = observed_at.0;
    let events = activity_capture_events_with_inventory_sources(
        TestText(observed_at.clone()),
        process_limit,
        network_limit,
        Vec::new(),
        Vec::new(),
        live_registry_inventory_events_from_roots_for_test(TestText(observed_at), registry_roots)?,
    )?;
    crate::activity_capture::record_activity_events_to_paths(
        journal_path.as_ref(),
        key_path.as_ref(),
        store_path.as_ref(),
        &events,
    )
}

fn activity_capture_events_with_inventory_sources(
    observed_at: TestText,
    process_limit: usize,
    network_limit: usize,
    inventory_events: Vec<ActivityEvent>,
    store_package_events: Vec<ActivityEvent>,
    registry_inventory_events: Vec<ActivityEvent>,
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let observed_at = observed_at.0;
    let mut events = process_snapshot_events(&observed_at, process_limit);
    events.push(foreground_window_event(&observed_at));
    events.extend(network_snapshot_events(&observed_at, network_limit));
    events.extend(live_process_events_for_test(
        TestText(observed_at.clone()),
        process_limit,
    )?);
    events.extend(inventory_events);
    events.extend(store_package_events);
    events.extend(registry_inventory_events);
    if let Some(event) = live_foreground_event_for_test(TestText(observed_at))? {
        events.push(event);
    }
    Ok(events)
}

#[cfg(windows)]
fn live_process_events_for_test(
    observed_at: TestText,
    limit: usize,
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let observed_at = observed_at.0;
    Ok(live_windows_process_snapshot_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.as_str(),
        limit,
    )?)
}

#[cfg(not(windows))]
fn live_process_events_for_test(
    observed_at: TestText,
    _limit: usize,
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let _ = observed_at.0;
    Ok(Vec::new())
}

#[cfg(windows)]
fn live_foreground_event_for_test(
    observed_at: TestText,
) -> Result<Option<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let observed_at = observed_at.0;
    Ok(live_windows_foreground_window_journal_event(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.as_str(),
    )?)
}

#[cfg(not(windows))]
fn live_foreground_event_for_test(
    observed_at: TestText,
) -> Result<Option<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let _ = observed_at.0;
    Ok(None)
}

#[cfg(windows)]
fn live_inventory_events_from_roots_for_test<P>(
    observed_at: TestText,
    roots: &[P],
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError>
where
    P: AsRef<TestPath>,
{
    let observed_at = observed_at.0;
    let roots: Vec<TestPathBuf> = roots
        .iter()
        .map(|root| root.as_ref().to_path_buf())
        .collect();
    Ok(live_windows_inventory_journal_events_from_roots(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.as_str(),
        &roots,
        constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
    )?)
}

#[cfg(not(windows))]
fn live_inventory_events_from_roots_for_test<P>(
    observed_at: TestText,
    _roots: &[P],
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let _ = observed_at.0;
    Ok(Vec::new())
}

#[cfg(windows)]
fn live_store_package_events_from_roots_for_test<P>(
    observed_at: TestText,
    roots: &[P],
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError>
where
    P: AsRef<TestPath>,
{
    let observed_at = observed_at.0;
    let roots: Vec<TestPathBuf> = roots
        .iter()
        .map(|root| root.as_ref().to_path_buf())
        .collect();
    Ok(live_windows_store_package_journal_events_from_roots(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.as_str(),
        &roots,
        constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
    )?)
}

#[cfg(not(windows))]
fn live_store_package_events_from_roots_for_test<P>(
    observed_at: TestText,
    _roots: &[P],
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let _ = observed_at.0;
    Ok(Vec::new())
}

#[cfg(windows)]
fn live_registry_inventory_events_from_roots_for_test<P>(
    observed_at: TestText,
    roots: &[P],
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError>
where
    P: AsRef<TestPath>,
{
    let observed_at = observed_at.0;
    let roots: Vec<TestPathBuf> = roots
        .iter()
        .map(|root| root.as_ref().to_path_buf())
        .collect();
    Ok(live_windows_registry_inventory_journal_events_from_roots(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.as_str(),
        &roots,
        constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
    )?)
}

#[cfg(not(windows))]
fn live_registry_inventory_events_from_roots_for_test<P>(
    observed_at: TestText,
    _roots: &[P],
) -> Result<Vec<ActivityEvent>, crate::activity_capture::ActivityCaptureError> {
    let _ = observed_at.0;
    Ok(Vec::new())
}
