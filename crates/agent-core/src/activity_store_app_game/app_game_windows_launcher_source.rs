#[cfg(windows)]
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER,
    APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE, APP_GAME_CONFIDENCE_UNKNOWN,
    APP_GAME_FOREGROUND_UNKNOWN, APP_GAME_LAUNCHER_EVIDENCE_ID_PREFIX,
    APP_GAME_LAUNCHER_KIND_BATTLE_NET, APP_GAME_LAUNCHER_KIND_EA, APP_GAME_LAUNCHER_KIND_EPIC,
    APP_GAME_LAUNCHER_KIND_GOG, APP_GAME_LAUNCHER_KIND_ITCH_IO, APP_GAME_LAUNCHER_KIND_MINECRAFT,
    APP_GAME_LAUNCHER_KIND_RIOT, APP_GAME_LAUNCHER_KIND_ROBLOX, APP_GAME_LAUNCHER_KIND_STEAM,
    APP_GAME_LAUNCHER_KIND_UBISOFT, APP_GAME_LAUNCHER_KIND_XBOX,
    APP_GAME_LAUNCHER_PROCESS_NAME_BATTLE_NET, APP_GAME_LAUNCHER_PROCESS_NAME_BATTLE_NET_AGENT,
    APP_GAME_LAUNCHER_PROCESS_NAME_EA_DESKTOP, APP_GAME_LAUNCHER_PROCESS_NAME_EPIC,
    APP_GAME_LAUNCHER_PROCESS_NAME_GALAXY, APP_GAME_LAUNCHER_PROCESS_NAME_GAMING_SERVICES,
    APP_GAME_LAUNCHER_PROCESS_NAME_ITCH, APP_GAME_LAUNCHER_PROCESS_NAME_ITCH_IO,
    APP_GAME_LAUNCHER_PROCESS_NAME_MINECRAFT, APP_GAME_LAUNCHER_PROCESS_NAME_ORIGIN,
    APP_GAME_LAUNCHER_PROCESS_NAME_RIOT, APP_GAME_LAUNCHER_PROCESS_NAME_RIOT_UI,
    APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX, APP_GAME_LAUNCHER_PROCESS_NAME_ROBLOX_PLAYER,
    APP_GAME_LAUNCHER_PROCESS_NAME_STEAM, APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT,
    APP_GAME_LAUNCHER_PROCESS_NAME_UBISOFT_CONNECT, APP_GAME_LAUNCHER_PROCESS_NAME_XBOX,
    APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE, APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY,
    APP_GAME_LAUNCHER_REF_PREFIX, APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
    APP_GAME_RUNTIME_RUNNING,
};
#[cfg(windows)]
use sha2::{Digest, Sha256};
#[cfg(windows)]
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

use super::{
    app_game_journal_sqlite_ingest::{
        app_game_launcher_journal_event, AppGameJournalSqliteIngestError,
    },
    app_game_windows_launcher::{
        windows_launcher_rows_from_records, WindowsLauncherEvidenceRecord,
    },
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
    let rows =
        windows_launcher_rows_from_records(&live_windows_launcher_records(observed_at, limit));
    rows.iter()
        .map(|row| app_game_launcher_journal_event(device_id, platform, row).map_err(Into::into))
        .collect()
}

#[cfg(windows)]
fn live_windows_launcher_records(
    observed_at: &str,
    limit: usize,
) -> Vec<WindowsLauncherEvidenceRecord> {
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything()
            .without_cmd()
            .without_cpu()
            .without_cwd()
            .without_disk_usage()
            .without_environ()
            .without_memory()
            .without_root()
            .without_user()
            .with_exe(UpdateKind::OnlyIfNotSet),
    );
    let mut launchers = Vec::new();
    for process in system.processes().values() {
        if launchers.len() >= limit {
            break;
        }
        let Some(kind) = launcher_kind(process.name().to_string_lossy().as_ref()) else {
            continue;
        };
        launchers.push(record_from_process(&system, process, kind, observed_at));
    }
    launchers
}

#[cfg(windows)]
fn record_from_process(
    system: &System,
    process: &sysinfo::Process,
    launcher_kind: &str,
    observed_at: &str,
) -> WindowsLauncherEvidenceRecord {
    let process_id = u64::from(process.pid().as_u32());
    let child_process_id = system
        .processes()
        .values()
        .find(|candidate| {
            candidate.parent().map(|parent| parent.as_u32()) == Some(process.pid().as_u32())
        })
        .map(|child| u64::from(child.pid().as_u32()));
    let child_proof = child_process_id.is_some();
    WindowsLauncherEvidenceRecord {
        launcher_evidence_id: evidence_id(process_id, observed_at),
        observed_at: observed_at.to_string(),
        launcher_kind: launcher_kind.to_string(),
        launcher_ref: launcher_ref(process.exe(), process.name().to_string_lossy().as_ref()),
        launcher_inventory_entry_id: None,
        launcher_manifest_id: None,
        launcher_app_id: None,
        launcher_process_identity: Some(process_identity(process_id)),
        launcher_process_id: Some(process_id),
        launcher_process_name: Some(process.name().to_string_lossy().into_owned()),
        child_process_identity: child_process_id.map(process_identity),
        child_inventory_entry_id: None,
        child_game_evidence_claim_id: None,
        catalog_ref: None,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_UNKNOWN.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: classification_state(child_proof),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        game_proof_state: game_proof_state(child_proof),
        confidence: APP_GAME_CONFIDENCE_UNKNOWN,
        evidence: Vec::new(),
    }
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

#[cfg(not(windows))]
fn live_windows_launcher_records(
    _observed_at: &str,
    _limit: usize,
) -> Vec<WindowsLauncherEvidenceRecord> {
    Vec::new()
}

#[cfg(windows)]
const KNOWN_LAUNCHER_PROCESSES: [(&str, &str); 18] = [
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
        APP_GAME_LAUNCHER_PROCESS_NAME_BATTLE_NET_AGENT,
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
fn launcher_kind(process_name: &str) -> Option<&'static str> {
    let process_name = process_name.to_ascii_lowercase();
    let name = process_name.strip_suffix(".exe").unwrap_or(&process_name);
    KNOWN_LAUNCHER_PROCESSES
        .iter()
        .find_map(|(known_name, kind)| (*known_name == name).then_some(*kind))
}

#[cfg(windows)]
fn launcher_ref(executable: Option<&std::path::Path>, process_name: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(
        executable
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| process_name.to_string()),
    );
    format!(
        "{}{}",
        APP_GAME_LAUNCHER_REF_PREFIX,
        BASE64_URL_SAFE_NO_PAD.encode(digest.finalize())
    )
}

#[cfg(windows)]
fn evidence_id(process_id: u64, observed_at: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(observed_at.as_bytes());
    format!(
        "{}{}-{}",
        APP_GAME_LAUNCHER_EVIDENCE_ID_PREFIX,
        process_id,
        BASE64_URL_SAFE_NO_PAD.encode(digest.finalize())
    )
}

#[cfg(windows)]
fn process_identity(process_id: u64) -> String {
    format!(
        "{}{}",
        ocentra_parent_agent_protocol::constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX,
        process_id
    )
}
