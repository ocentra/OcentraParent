#[cfg(windows)]
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    AppGameLauncherEvidenceRow, APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
    APP_GAME_CONFIDENCE_UNKNOWN, APP_GAME_FOREGROUND_UNKNOWN, APP_GAME_LAUNCHER_EVIDENCE_ID_PREFIX,
    APP_GAME_LAUNCHER_KIND_BATTLE_NET, APP_GAME_LAUNCHER_KIND_EA, APP_GAME_LAUNCHER_KIND_EPIC,
    APP_GAME_LAUNCHER_KIND_GOG, APP_GAME_LAUNCHER_KIND_ITCH_IO, APP_GAME_LAUNCHER_KIND_MINECRAFT,
    APP_GAME_LAUNCHER_KIND_RIOT, APP_GAME_LAUNCHER_KIND_ROBLOX, APP_GAME_LAUNCHER_KIND_STEAM,
    APP_GAME_LAUNCHER_KIND_UBISOFT, APP_GAME_LAUNCHER_KIND_XBOX,
    APP_GAME_LAUNCHER_PROCESS_NAME_BATTLE_NET, APP_GAME_LAUNCHER_PROCESS_NAME_EA_DESKTOP,
    APP_GAME_LAUNCHER_PROCESS_NAME_GALAXY, APP_GAME_LAUNCHER_PROCESS_NAME_GAMING_SERVICES,
    APP_GAME_LAUNCHER_PROCESS_NAME_ITCH, APP_GAME_LAUNCHER_PROCESS_NAME_ITCH_IO,
    APP_GAME_LAUNCHER_PROCESS_NAME_MINECRAFT, APP_GAME_LAUNCHER_PROCESS_NAME_ORIGIN,
    APP_GAME_LAUNCHER_PROCESS_NAME_RIOT, APP_GAME_LAUNCHER_PROCESS_NAME_RIOT_UI,
    APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX, APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX_PLAYER,
    APP_GAME_LAUNCHER_PROCESS_NAME_STEAM, APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT,
    APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT_CONNECT, APP_GAME_LAUNCHER_PROCESS_NAME_XBOX,
    APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE, APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY,
    APP_GAME_LAUNCHER_REF_PREFIX, APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
    APP_GAME_RUNTIME_RUNNING, APP_GAME_SCHEMA_VERSION,
};
#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
#[cfg(windows)]
use sysinfo::{Process, System};

use super::app_game_journal_sqlite_ingest::{
    app_game_launcher_journal_event, AppGameJournalSqliteIngestError,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AppGameLiveLauncherSourceError {
    LauncherJournalEventRejected,
}

impl From<AppGameJournalSqliteIngestError> for AppGameLiveLauncherSourceError {
    fn from(_: AppGameJournalSqliteIngestError) -> Self {
        Self::LauncherJournalEventRejected
    }
}

pub fn live_windows_launcher_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveLauncherSourceError> {
    #[cfg(windows)]
    {
        let system = super::app_game_windows_process_source::live_windows_process_snapshot_system();
        return launcher_journal_events_from_process_snapshot(
            device_id,
            platform,
            observed_at,
            limit,
            &system,
        );
    }
    #[cfg(not(windows))]
    {
        let _ = (device_id, platform, observed_at, limit);
        Ok(Vec::new())
    }
}

#[cfg(windows)]
pub(crate) fn launcher_journal_events_from_process_snapshot(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
    system: &System,
) -> Result<Vec<ActivityEvent>, AppGameLiveLauncherSourceError> {
    let rows = windows_launcher_rows_from_process_snapshot(system, observed_at, limit);
    rows.iter()
        .map(|row| app_game_launcher_journal_event(device_id, platform, row).map_err(Into::into))
        .collect()
}

#[cfg(windows)]
fn windows_launcher_rows_from_process_snapshot(
    system: &System,
    observed_at: &str,
    limit: usize,
) -> Vec<AppGameLauncherEvidenceRow> {
    let children = child_processes_by_parent(system);
    system
        .processes()
        .values()
        .filter_map(|process| record_from_process(&children, process, observed_at))
        .take(limit)
        .collect()
}

#[cfg(windows)]
fn record_from_process(
    children: &HashMap<u32, ChildProcessIdentity>,
    process: &Process,
    observed_at: &str,
) -> Option<AppGameLauncherEvidenceRow> {
    let canonical_path = canonical_executable_path(process.exe())?;
    let launcher_kind = launcher_kind(process, &canonical_path)?;
    let process_id = u64::from(process.pid().as_u32());
    let start_time = process.start_time();
    let child = children.get(&process.pid().as_u32()).copied();
    let child_proof = child.is_some();
    Some(AppGameLauncherEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        launcher_evidence_id: evidence_id(process_id, start_time, observed_at),
        observed_at: observed_at.to_string(),
        launcher_kind: launcher_kind.to_string(),
        launcher_ref: launcher_ref(&canonical_path),
        launcher_inventory_entry_id: None,
        launcher_manifest_id: None,
        launcher_app_id: None,
        launcher_process_identity: Some(process_identity(process_id, start_time)),
        launcher_process_id: Some(process_id),
        launcher_process_name: Some(process.name().to_string_lossy().into_owned()),
        child_process_identity: child
            .map(|child| process_identity(child.process_id, child.start_time)),
        child_inventory_entry_id: None,
        child_game_evidence_claim_id: None,
        catalog_ref: None,
        catalog_ready_state: APP_GAME_CATALOG_UNAVAILABLE.to_string(),
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_UNKNOWN.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: classification_state(child_proof),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        game_proof_state: game_proof_state(child_proof),
        confidence: APP_GAME_CONFIDENCE_UNKNOWN,
        evidence: Vec::new(),
    })
}

#[cfg(windows)]
#[derive(Clone, Copy)]
struct ChildProcessIdentity {
    process_id: u64,
    start_time: u64,
}

#[cfg(windows)]
fn child_processes_by_parent(system: &System) -> HashMap<u32, ChildProcessIdentity> {
    let mut children = HashMap::new();
    for process in system.processes().values() {
        let Some(parent_id) = process.parent() else {
            continue;
        };
        children
            .entry(parent_id.as_u32())
            .or_insert(ChildProcessIdentity {
                process_id: u64::from(process.pid().as_u32()),
                start_time: process.start_time(),
            });
    }
    children
}

#[cfg(windows)]
fn classification_state(child_proof: bool) -> String {
    if child_proof {
        return APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE.to_string();
    }
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string()
}

#[cfg(windows)]
fn game_proof_state(child_proof: bool) -> String {
    if child_proof {
        return APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE.to_string();
    }
    APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY.to_string()
}

#[cfg(windows)]
const KNOWN_LAUNCHER_PROCESSES: [(&str, &str); 17] = [
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_STEAM,
        APP_GAME_LAUNCHER_KIND_STEAM,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_EPIC,
        APP_GAME_LAUNCHER_KIND_EPIC,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_XBOX,
        APP_GAME_LAUNCHER_KIND_XBOX,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_GAMING_SERVICES,
        APP_GAME_LAUNCHER_KIND_XBOX,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_RIOT,
        APP_GAME_LAUNCHER_KIND_RIOT,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_RIOT_UI,
        APP_GAME_LAUNCHER_KIND_RIOT,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_BATTLE_NET,
        APP_GAME_LAUNCHER_KIND_BATTLE_NET,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_EA_DESKTOP,
        APP_GAME_LAUNCHER_KIND_EA,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_ORIGIN,
        APP_GAME_LAUNCHER_KIND_EA,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT,
        APP_GAME_LAUNCHER_KIND_UBISOFT,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT_CONNECT,
        APP_GAME_LAUNCHER_KIND_UBISOFT,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_GALAXY,
        APP_GAME_LAUNCHER_KIND_GOG,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX,
        APP_GAME_LAUNCHER_KIND_ROBLOX,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX_PLAYER,
        APP_GAME_LAUNCHER_KIND_ROBLOX,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_MINECRAFT,
        APP_GAME_LAUNCHER_KIND_MINECRAFT,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_ITCH,
        APP_GAME_LAUNCHER_KIND_ITCH_IO,
    ),
    (
        APP_GAME_LAUNCHER_PROCESS_NAME_ITCH_IO,
        APP_GAME_LAUNCHER_KIND_ITCH_IO,
    ),
];

#[cfg(windows)]
fn launcher_kind(process: &Process, canonical_path: &Path) -> Option<&'static str> {
    let process_name = process.name().to_string_lossy().to_ascii_lowercase();
    let name = process_name.strip_suffix(".exe").unwrap_or(&process_name);
    let executable_name = canonical_path
        .file_stem()?
        .to_string_lossy()
        .to_ascii_lowercase();
    if executable_name != name {
        return None;
    }
    KNOWN_LAUNCHER_PROCESSES
        .iter()
        .find_map(|(known_name, kind)| (*known_name == name).then_some(*kind))
}

#[cfg(windows)]
fn canonical_executable_path(path: Option<&Path>) -> Option<PathBuf> {
    path.and_then(|path| fs::canonicalize(path).ok())
}

#[cfg(windows)]
fn launcher_ref(executable: &Path) -> String {
    let mut digest = Sha256::new();
    digest.update(executable.to_string_lossy().as_bytes());
    format!(
        "{}{}",
        APP_GAME_LAUNCHER_REF_PREFIX,
        BASE64_URL_SAFE_NO_PAD.encode(digest.finalize())
    )
}

#[cfg(windows)]
fn evidence_id(process_id: u64, start_time: u64, observed_at: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(observed_at.as_bytes());
    format!(
        "{}{}-{}-{}",
        APP_GAME_LAUNCHER_EVIDENCE_ID_PREFIX,
        process_id,
        start_time,
        BASE64_URL_SAFE_NO_PAD.encode(digest.finalize())
    )
}

#[cfg(windows)]
fn process_identity(process_id: u64, start_time: u64) -> String {
    format!(
        "{}{}-{}",
        ocentra_parent_agent_protocol::constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX,
        process_id,
        start_time,
    )
}
