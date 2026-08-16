use std::path::Path;

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_CONFIDENCE_UNKNOWN, APP_GAME_EXECUTABLE_PATH_REF_PREFIX,
    APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT, APP_GAME_RUNTIME_EVIDENCE_ID_PREFIX,
};
use sha2::{Digest, Sha256};
use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

use super::{
    app_game_journal_sqlite_ingest::{
        app_game_runtime_journal_event, AppGameJournalSqliteIngestError,
    },
    app_game_windows_process_runtime::{
        windows_process_runtime_rows_from_records, WindowsProcessRuntimeRecord,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum AppGameLiveProcessSnapshotError {
    RuntimeJournalEventRejected,
}

impl From<AppGameJournalSqliteIngestError> for AppGameLiveProcessSnapshotError {
    fn from(_: AppGameJournalSqliteIngestError) -> Self {
        Self::RuntimeJournalEventRejected
    }
}

pub fn live_windows_process_snapshot_records(
    observed_at: &str,
) -> Vec<WindowsProcessRuntimeRecord> {
    let mut system = System::new();
    refresh_processes(&mut system, ProcessesToUpdate::All);
    records_from_system(&system, observed_at)
}

pub fn live_windows_process_snapshot_record_for_pid(
    observed_at: &str,
    process_id: u32,
) -> Option<WindowsProcessRuntimeRecord> {
    let pid = Pid::from_u32(process_id);
    let pids = [pid];
    let mut system = System::new();
    refresh_processes(&mut system, ProcessesToUpdate::Some(&pids));
    system
        .process(pid)
        .map(|process| record_from_process(process, observed_at))
}

pub fn live_windows_process_snapshot_journal_events(
    device_id: &str,
    platform: &str,
    observed_at: &str,
) -> Result<Vec<ActivityEvent>, AppGameJournalSqliteIngestError> {
    let records = live_windows_process_snapshot_records(observed_at);
    journal_events_from_records(device_id, platform, &records)
}

pub fn live_windows_process_snapshot_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveProcessSnapshotError> {
    let records = live_windows_process_snapshot_records(observed_at)
        .into_iter()
        .take(limit)
        .collect::<Vec<_>>();
    Ok(journal_events_from_records(device_id, platform, &records)?)
}

pub fn live_windows_process_snapshot_journal_event_for_pid(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    process_id: u32,
) -> Result<Option<ActivityEvent>, AppGameJournalSqliteIngestError> {
    let Some(record) = live_windows_process_snapshot_record_for_pid(observed_at, process_id) else {
        return Ok(None);
    };
    let events = journal_events_from_records(device_id, platform, &[record])?;
    Ok(events.into_iter().next())
}

fn journal_events_from_records(
    device_id: &str,
    platform: &str,
    records: &[WindowsProcessRuntimeRecord],
) -> Result<Vec<ActivityEvent>, AppGameJournalSqliteIngestError> {
    let rows = windows_process_runtime_rows_from_records(records);
    rows.iter()
        .map(|row| app_game_runtime_journal_event(device_id, platform, row))
        .collect()
}

fn records_from_system(system: &System, observed_at: &str) -> Vec<WindowsProcessRuntimeRecord> {
    system
        .processes()
        .values()
        .map(|process| record_from_process(process, observed_at))
        .collect()
}

fn refresh_processes(system: &mut System, processes_to_update: ProcessesToUpdate<'_>) {
    system.refresh_processes_specifics(processes_to_update, true, process_refresh_kind());
}

fn process_refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::everything()
        .without_cmd()
        .without_cpu()
        .without_cwd()
        .without_disk_usage()
        .without_environ()
        .without_memory()
        .without_root()
        .without_user()
        .with_exe(UpdateKind::OnlyIfNotSet)
}

fn record_from_process(process: &Process, observed_at: &str) -> WindowsProcessRuntimeRecord {
    let process_id = pid_to_u64(process.pid());
    WindowsProcessRuntimeRecord {
        runtime_evidence_id: runtime_evidence_id(process_id, observed_at),
        observed_at: observed_at.to_string(),
        process_identity: None,
        process_id,
        parent_process_id: process.parent().map(pid_to_u64),
        process_name: process.name().to_string_lossy().into_owned(),
        executable_path_ref: executable_path_ref(process.exe()),
        publisher_signature_ref: None,
        file_hash_ref: None,
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: None,
        started_at: timestamp_from_epoch_seconds(process.start_time()),
        exited_at: None,
        running_duration_ms: milliseconds_from_seconds(process.run_time()),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: APP_GAME_CONFIDENCE_UNKNOWN,
        evidence: Vec::new(),
    }
}

fn runtime_evidence_id(process_id: u64, observed_at: &str) -> String {
    let mut evidence_id = String::from(APP_GAME_RUNTIME_EVIDENCE_ID_PREFIX);
    evidence_id.push_str(&process_id.to_string());
    evidence_id.push_str(&observed_at_suffix(observed_at));
    evidence_id
}

fn observed_at_suffix(observed_at: &str) -> String {
    let digest = Sha256::digest(observed_at.as_bytes());
    let mut suffix = String::from(ocentra_parent_agent_protocol::constants::delimiter::HYPHEN);
    suffix.push_str(&BASE64_URL_SAFE_NO_PAD.encode(digest));
    suffix
}

fn executable_path_ref(path: Option<&Path>) -> Option<String> {
    path.map(|path| {
        let canonical_path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let digest = Sha256::digest(canonical_path.to_string_lossy().as_bytes());
        let mut path_ref = String::from(APP_GAME_EXECUTABLE_PATH_REF_PREFIX);
        path_ref.push_str(&BASE64_URL_SAFE_NO_PAD.encode(digest));
        path_ref
    })
}

fn timestamp_from_epoch_seconds(seconds: u64) -> Option<String> {
    let Ok(seconds) = i64::try_from(seconds) else {
        return None;
    };
    DateTime::<Utc>::from_timestamp(seconds, 0).map(|timestamp| timestamp.to_rfc3339())
}

fn milliseconds_from_seconds(seconds: u64) -> u64 {
    seconds.saturating_mul(1000)
}

fn pid_to_u64(pid: Pid) -> u64 {
    u64::from(pid.as_u32())
}
