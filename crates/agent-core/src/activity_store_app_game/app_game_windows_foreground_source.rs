use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_CONFIDENCE_UNKNOWN, APP_GAME_FOREGROUND_EVIDENCE_ID_PREFIX,
    APP_GAME_FOREGROUND_FOREGROUND, APP_GAME_TITLE_CAPTURE_TITLE_OMITTED,
    APP_GAME_TITLE_CAPTURE_TITLE_REF, APP_GAME_WINDOW_REF_PREFIX, APP_GAME_WINDOW_TITLE_REF_PREFIX,
};
use sha2::{Digest, Sha256};

use super::{
    app_game_journal_sqlite_ingest::{
        app_game_foreground_journal_event, AppGameJournalSqliteIngestError,
    },
    app_game_windows_foreground::{
        windows_foreground_rows_from_records, WindowsForegroundWindowRecord,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum AppGameLiveForegroundWindowError {
    ForegroundJournalEventRejected,
}

impl From<AppGameJournalSqliteIngestError> for AppGameLiveForegroundWindowError {
    fn from(_: AppGameJournalSqliteIngestError) -> Self {
        Self::ForegroundJournalEventRejected
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveWindowsForegroundWindowSnapshot {
    pub process_id: u64,
    pub process_name: String,
    pub window_id: String,
    pub window_title: String,
}

pub fn live_windows_foreground_window_record(
    observed_at: &str,
) -> Option<WindowsForegroundWindowRecord> {
    active_window_snapshot()
        .map(|snapshot| live_windows_foreground_window_record_from_snapshot(observed_at, &snapshot))
}

pub fn live_windows_foreground_window_record_from_snapshot(
    observed_at: &str,
    snapshot: &LiveWindowsForegroundWindowSnapshot,
) -> WindowsForegroundWindowRecord {
    foreground_record_from_snapshot(observed_at, snapshot)
}

pub fn live_windows_foreground_window_journal_event(
    device_id: &str,
    platform: &str,
    observed_at: &str,
) -> Result<Option<ActivityEvent>, AppGameLiveForegroundWindowError> {
    let Some(record) = live_windows_foreground_window_record(observed_at) else {
        return Ok(None);
    };
    Ok(Some(foreground_journal_event_from_record(
        device_id, platform, record,
    )?))
}

pub fn live_windows_foreground_window_journal_event_from_snapshot(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    snapshot: &LiveWindowsForegroundWindowSnapshot,
) -> Result<ActivityEvent, AppGameLiveForegroundWindowError> {
    foreground_journal_event_from_record(
        device_id,
        platform,
        live_windows_foreground_window_record_from_snapshot(observed_at, snapshot),
    )
}

fn foreground_journal_event_from_record(
    device_id: &str,
    platform: &str,
    record: WindowsForegroundWindowRecord,
) -> Result<ActivityEvent, AppGameLiveForegroundWindowError> {
    let rows = windows_foreground_rows_from_records(&[record]);
    Ok(app_game_foreground_journal_event(
        device_id, platform, &rows[0],
    )?)
}

fn foreground_record_from_snapshot(
    observed_at: &str,
    snapshot: &LiveWindowsForegroundWindowSnapshot,
) -> WindowsForegroundWindowRecord {
    let window_title_ref = opaque_ref(APP_GAME_WINDOW_TITLE_REF_PREFIX, &snapshot.window_title);
    let title_capture_state = title_capture_state(window_title_ref.as_ref());
    WindowsForegroundWindowRecord {
        foreground_evidence_id: foreground_evidence_id(snapshot.process_id, observed_at),
        observed_at: observed_at.to_string(),
        process_identity: None,
        process_id: snapshot.process_id,
        process_name: snapshot.process_name.clone(),
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: None,
        window_ref: opaque_ref(APP_GAME_WINDOW_REF_PREFIX, &snapshot.window_id),
        window_title_ref,
        title_capture_state,
        foreground_started_at: Some(observed_at.to_string()),
        foreground_ended_at: None,
        foreground_duration_ms: 0,
        foreground_state: APP_GAME_FOREGROUND_FOREGROUND.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: APP_GAME_CONFIDENCE_UNKNOWN,
        evidence: Vec::new(),
    }
}

fn foreground_evidence_id(process_id: u64, observed_at: &str) -> String {
    let mut evidence_id = String::from(APP_GAME_FOREGROUND_EVIDENCE_ID_PREFIX);
    evidence_id.push_str(&process_id.to_string());
    evidence_id.push_str(&observed_at_suffix(observed_at));
    evidence_id
}

fn opaque_ref(prefix: &str, value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    let digest = Sha256::digest(value.as_bytes());
    let mut reference = String::from(prefix);
    reference.push_str(&BASE64_URL_SAFE_NO_PAD.encode(digest));
    Some(reference)
}

fn observed_at_suffix(observed_at: &str) -> String {
    let digest = Sha256::digest(observed_at.as_bytes());
    let mut suffix = String::from(ocentra_parent_agent_protocol::constants::delimiter::HYPHEN);
    suffix.push_str(&BASE64_URL_SAFE_NO_PAD.encode(digest));
    suffix
}

fn title_capture_state(window_title_ref: Option<&String>) -> String {
    if window_title_ref.is_some() {
        return APP_GAME_TITLE_CAPTURE_TITLE_REF.to_string();
    }
    APP_GAME_TITLE_CAPTURE_TITLE_OMITTED.to_string()
}

#[cfg(windows)]
fn active_window_snapshot() -> Option<LiveWindowsForegroundWindowSnapshot> {
    active_win_pos_rs::get_active_window()
        .ok()
        .map(|window| LiveWindowsForegroundWindowSnapshot {
            process_id: window.process_id,
            process_name: window.app_name,
            window_id: window.window_id,
            window_title: window.title,
        })
}

#[cfg(not(windows))]
fn active_window_snapshot() -> Option<LiveWindowsForegroundWindowSnapshot> {
    None
}
