use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameRuntimeEvidenceRow, AppGameSessionSummary, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CATALOG_PERMISSION_LIMITED, APP_GAME_CATALOG_READY, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CONFIDENCE_UNKNOWN,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_OBSERVATION_MODE_PROCESS_EXIT,
    APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT, APP_GAME_OBSERVATION_MODE_PROCESS_START,
    APP_GAME_RUNTIME_NOT_RUNNING, APP_GAME_RUNTIME_PERMISSION_LIMITED, APP_GAME_RUNTIME_RUNNING,
    APP_GAME_RUNTIME_UNKNOWN, APP_GAME_SCHEMA_VERSION, APP_GAME_SESSION_END_REASON_PROCESS_EXIT,
    APP_GAME_SESSION_ID_PREFIX,
};
use ocentra_parent_agent_protocol::constants;

pub struct WindowsProcessRuntimeRecord {
    pub runtime_evidence_id: String,
    pub observed_at: String,
    pub process_identity: Option<String>,
    pub process_id: u64,
    pub parent_process_id: Option<u64>,
    pub process_name: String,
    pub executable_path_ref: Option<String>,
    pub publisher_signature_ref: Option<String>,
    pub file_hash_ref: Option<String>,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub started_at: Option<String>,
    pub exited_at: Option<String>,
    pub running_duration_ms: u64,
    pub observation_mode: String,
    pub classification_state: String,
    pub capability_status: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_process_runtime_rows_from_records(
    records: &[WindowsProcessRuntimeRecord],
) -> Vec<AppGameRuntimeEvidenceRow> {
    records.iter().map(row_from_record).collect()
}

pub fn runtime_session_summaries_from_rows(
    rows: &[AppGameRuntimeEvidenceRow],
) -> Vec<AppGameSessionSummary> {
    let mut summaries = Vec::new();
    for row in rows {
        upsert_runtime_summary(&mut summaries, row);
    }
    summaries
}

fn row_from_record(record: &WindowsProcessRuntimeRecord) -> AppGameRuntimeEvidenceRow {
    let process_identity = record
        .process_identity
        .clone()
        .unwrap_or_else(|| process_identity_from_pid(record.process_id));
    AppGameRuntimeEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        runtime_evidence_id: record.runtime_evidence_id.clone(),
        observed_at: record.observed_at.clone(),
        process_identity,
        process_id: record.process_id,
        parent_process_id: record.parent_process_id,
        process_name: record.process_name.clone(),
        executable_path_ref: record.executable_path_ref.clone(),
        publisher_signature_ref: record.publisher_signature_ref.clone(),
        file_hash_ref: record.file_hash_ref.clone(),
        inventory_entry_id: record.inventory_entry_id.clone(),
        launcher_ref: record.launcher_ref.clone(),
        catalog_ref: record.catalog_ref.clone(),
        started_at: started_at_for_record(record),
        exited_at: exited_at_for_record(record),
        running_duration_ms: record.running_duration_ms,
        runtime_state: runtime_state_for_record(record),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: record.observation_mode.clone(),
        classification_state: classification_state_for_record(record),
        catalog_ready_state: catalog_ready_state_for_record(record),
        capability_status: capability_status_for_record(record),
        confidence: confidence_for_record(record),
        evidence: record.evidence.clone(),
    }
}

fn process_identity_from_pid(process_id: u64) -> String {
    let mut process_identity = String::from(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX);
    process_identity.push_str(&process_id.to_string());
    process_identity
}

fn started_at_for_record(record: &WindowsProcessRuntimeRecord) -> Option<String> {
    if record.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_START {
        return Some(
            record
                .started_at
                .clone()
                .unwrap_or_else(|| record.observed_at.clone()),
        );
    }
    record.started_at.clone()
}

fn exited_at_for_record(record: &WindowsProcessRuntimeRecord) -> Option<String> {
    if record.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_EXIT {
        return Some(
            record
                .exited_at
                .clone()
                .unwrap_or_else(|| record.observed_at.clone()),
        );
    }
    None
}

fn runtime_state_for_record(record: &WindowsProcessRuntimeRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_RUNTIME_PERMISSION_LIMITED.to_string();
    }
    if record.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_EXIT {
        return APP_GAME_RUNTIME_NOT_RUNNING.to_string();
    }
    if record.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_START
        || record.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT
    {
        return APP_GAME_RUNTIME_RUNNING.to_string();
    }
    APP_GAME_RUNTIME_UNKNOWN.to_string()
}

fn classification_state_for_record(record: &WindowsProcessRuntimeRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CLASSIFICATION_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        return APP_GAME_CLASSIFICATION_ADAPTER_ERROR.to_string();
    }
    if record.classification_state == APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
        && record.launcher_ref.is_some()
    {
        return APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string();
    }
    if has_deterministic_runtime_ref(record) {
        return record.classification_state.clone();
    }
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string()
}

fn catalog_ready_state_for_record(record: &WindowsProcessRuntimeRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CATALOG_PERMISSION_LIMITED.to_string();
    }
    if record.catalog_ref.is_some() {
        return APP_GAME_CATALOG_READY.to_string();
    }
    APP_GAME_CATALOG_UNAVAILABLE.to_string()
}

fn capability_status_for_record(record: &WindowsProcessRuntimeRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string()
    } else if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR.to_string()
    } else {
        APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string()
    }
}

fn confidence_for_record(record: &WindowsProcessRuntimeRecord) -> f64 {
    if has_deterministic_runtime_ref(record) {
        record.confidence
    } else {
        APP_GAME_CONFIDENCE_UNKNOWN
    }
}

fn has_deterministic_runtime_ref(record: &WindowsProcessRuntimeRecord) -> bool {
    record.inventory_entry_id.is_some()
        || record.executable_path_ref.is_some()
        || record.publisher_signature_ref.is_some()
        || record.file_hash_ref.is_some()
        || record.launcher_ref.is_some()
        || record.catalog_ref.is_some()
}

fn upsert_runtime_summary(
    summaries: &mut Vec<AppGameSessionSummary>,
    row: &AppGameRuntimeEvidenceRow,
) {
    match summaries
        .iter_mut()
        .find(|summary| summary.primary_process_identity == row.process_identity)
    {
        Some(summary) => update_runtime_summary(summary, row),
        None => summaries.push(summary_from_runtime_row(row)),
    }
}

fn summary_from_runtime_row(row: &AppGameRuntimeEvidenceRow) -> AppGameSessionSummary {
    AppGameSessionSummary {
        schema_version: APP_GAME_SCHEMA_VERSION,
        session_id: session_id(&row.process_identity),
        primary_process_identity: row.process_identity.clone(),
        display_name: row.process_name.clone(),
        classification_state: row.classification_state.clone(),
        catalog_ready_state: row.catalog_ready_state.clone(),
        inventory_entry_id: row.inventory_entry_id.clone(),
        launcher_ref: row.launcher_ref.clone(),
        catalog_ref: row.catalog_ref.clone(),
        started_at: row
            .started_at
            .clone()
            .unwrap_or_else(|| row.observed_at.clone()),
        last_observed_at: row.observed_at.clone(),
        ended_at: row.exited_at.clone(),
        end_reason: row
            .exited_at
            .as_ref()
            .map(|_| APP_GAME_SESSION_END_REASON_PROCESS_EXIT.to_string()),
        running_duration_ms: row.running_duration_ms,
        foreground_duration_ms: 0,
        background_duration_ms: row.running_duration_ms,
        last_foreground_at: None,
        last_background_at: if row.running_duration_ms > 0 {
            Some(row.observed_at.clone())
        } else {
            None
        },
        observation_gap_ms: 0,
        observation_count: 1,
        evidence_count: row.evidence.len() as u64,
        evidence: row.evidence.clone(),
        ai_digest_ref: None,
        confidence: row.confidence,
    }
}

fn update_runtime_summary(summary: &mut AppGameSessionSummary, row: &AppGameRuntimeEvidenceRow) {
    summary.last_observed_at = row.observed_at.clone();
    if row.exited_at.is_some() {
        summary.ended_at = row.exited_at.clone();
        summary.end_reason = Some(APP_GAME_SESSION_END_REASON_PROCESS_EXIT.to_string());
    }
    summary.running_duration_ms = summary.running_duration_ms.max(row.running_duration_ms);
    summary.background_duration_ms = summary.running_duration_ms;
    summary.last_background_at = if row.running_duration_ms > 0 {
        Some(row.observed_at.clone())
    } else {
        summary.last_background_at.clone()
    };
    summary.observation_count += 1;
    summary.evidence_count += row.evidence.len() as u64;
    summary.evidence.extend(row.evidence.clone());
    if is_stronger_runtime_classification(row, summary) {
        summary.display_name = row.process_name.clone();
        summary.classification_state = row.classification_state.clone();
        summary.catalog_ready_state = row.catalog_ready_state.clone();
        summary.inventory_entry_id = row.inventory_entry_id.clone();
        summary.launcher_ref = row.launcher_ref.clone();
        summary.catalog_ref = row.catalog_ref.clone();
        summary.confidence = row.confidence;
    }
}

fn session_id(process_identity: &str) -> String {
    let mut session_id = String::from(APP_GAME_SESSION_ID_PREFIX);
    session_id.push_str(process_identity);
    session_id
}

fn is_stronger_runtime_classification(
    row: &AppGameRuntimeEvidenceRow,
    summary: &AppGameSessionSummary,
) -> bool {
    runtime_classification_rank(&row.classification_state)
        > runtime_classification_rank(&summary.classification_state)
}

fn runtime_classification_rank(classification_state: &str) -> u8 {
    match classification_state {
        APP_GAME_CLASSIFICATION_KNOWN_APP => 3,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER => 2,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED => 1,
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR => 1,
        _ => 0,
    }
}
