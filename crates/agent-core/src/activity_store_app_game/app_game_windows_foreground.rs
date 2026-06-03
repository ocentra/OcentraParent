use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, AppGameForegroundEvidenceRow, AppGameSessionSummary,
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CATALOG_PERMISSION_LIMITED,
    APP_GAME_CATALOG_READY, APP_GAME_CATALOG_UNAVAILABLE, APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CONFIDENCE_UNKNOWN,
    APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED, APP_GAME_FOREGROUND_ADAPTER_ERROR,
    APP_GAME_FOREGROUND_BACKGROUND, APP_GAME_FOREGROUND_FOREGROUND,
    APP_GAME_FOREGROUND_PERMISSION_LIMITED, APP_GAME_FOREGROUND_UNKNOWN,
    APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW, APP_GAME_RUNTIME_RUNNING,
    APP_GAME_RUNTIME_UNKNOWN, APP_GAME_SCHEMA_VERSION, APP_GAME_TITLE_CAPTURE_ADAPTER_ERROR,
    APP_GAME_TITLE_CAPTURE_PERMISSION_LIMITED, APP_GAME_TITLE_CAPTURE_TITLE_OMITTED,
    APP_GAME_TITLE_CAPTURE_TITLE_REF,
};

pub struct WindowsForegroundWindowRecord {
    pub foreground_evidence_id: String,
    pub observed_at: String,
    pub process_identity: Option<String>,
    pub process_id: u64,
    pub process_name: String,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub window_ref: Option<String>,
    pub window_title_ref: Option<String>,
    pub title_capture_state: String,
    pub foreground_started_at: Option<String>,
    pub foreground_ended_at: Option<String>,
    pub foreground_duration_ms: u64,
    pub foreground_state: String,
    pub classification_state: String,
    pub capability_status: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_foreground_rows_from_records(
    records: &[WindowsForegroundWindowRecord],
) -> Vec<AppGameForegroundEvidenceRow> {
    records.iter().map(row_from_record).collect()
}

pub fn apply_foreground_rows_to_runtime_summaries(
    summaries: &mut [AppGameSessionSummary],
    rows: &[AppGameForegroundEvidenceRow],
) {
    for row in rows {
        if let Some(summary) = summaries
            .iter_mut()
            .find(|summary| summary.primary_process_identity == row.process_identity)
        {
            apply_foreground_row(summary, row);
        }
    }
}

fn row_from_record(record: &WindowsForegroundWindowRecord) -> AppGameForegroundEvidenceRow {
    let process_identity = record
        .process_identity
        .clone()
        .unwrap_or_else(|| process_identity_from_pid(record.process_id));
    AppGameForegroundEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        foreground_evidence_id: record.foreground_evidence_id.clone(),
        observed_at: record.observed_at.clone(),
        process_identity,
        process_id: record.process_id,
        process_name: record.process_name.clone(),
        inventory_entry_id: record.inventory_entry_id.clone(),
        launcher_ref: record.launcher_ref.clone(),
        catalog_ref: record.catalog_ref.clone(),
        window_ref: record.window_ref.clone(),
        window_title_ref: window_title_ref_for_record(record),
        title_capture_state: title_capture_state_for_record(record),
        foreground_started_at: foreground_started_at_for_record(record),
        foreground_ended_at: foreground_ended_at_for_record(record),
        foreground_duration_ms: record.foreground_duration_ms,
        runtime_state: runtime_state_for_record(record),
        foreground_state: foreground_state_for_record(record),
        observation_mode: APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW.to_string(),
        classification_state: classification_state_for_record(record),
        catalog_ready_state: catalog_ready_state_for_record(record),
        capability_status: capability_status_for_record(record),
        content_knowledge_state: APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED.to_string(),
        confidence: confidence_for_record(record),
        evidence: record.evidence.clone(),
    }
}

fn process_identity_from_pid(process_id: u64) -> String {
    let mut process_identity = String::from(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX);
    process_identity.push_str(&process_id.to_string());
    process_identity
}

fn window_title_ref_for_record(record: &WindowsForegroundWindowRecord) -> Option<String> {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED
        || record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR
    {
        return None;
    }
    record.window_title_ref.clone()
}

fn title_capture_state_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_TITLE_CAPTURE_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        return APP_GAME_TITLE_CAPTURE_ADAPTER_ERROR.to_string();
    }
    if record.window_title_ref.is_some() {
        return APP_GAME_TITLE_CAPTURE_TITLE_REF.to_string();
    }
    if record.title_capture_state == APP_GAME_TITLE_CAPTURE_TITLE_OMITTED {
        return APP_GAME_TITLE_CAPTURE_TITLE_OMITTED.to_string();
    }
    APP_GAME_TITLE_CAPTURE_TITLE_OMITTED.to_string()
}

fn foreground_started_at_for_record(record: &WindowsForegroundWindowRecord) -> Option<String> {
    if foreground_state_for_record(record) == APP_GAME_FOREGROUND_FOREGROUND {
        return Some(
            record
                .foreground_started_at
                .clone()
                .unwrap_or_else(|| record.observed_at.clone()),
        );
    }
    record.foreground_started_at.clone()
}

fn foreground_ended_at_for_record(record: &WindowsForegroundWindowRecord) -> Option<String> {
    if foreground_state_for_record(record) == APP_GAME_FOREGROUND_BACKGROUND {
        return Some(
            record
                .foreground_ended_at
                .clone()
                .unwrap_or_else(|| record.observed_at.clone()),
        );
    }
    None
}

fn runtime_state_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED
        || record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR
    {
        return APP_GAME_RUNTIME_UNKNOWN.to_string();
    }
    APP_GAME_RUNTIME_RUNNING.to_string()
}

fn foreground_state_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_FOREGROUND_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        return APP_GAME_FOREGROUND_ADAPTER_ERROR.to_string();
    }
    if record.foreground_state == APP_GAME_FOREGROUND_FOREGROUND {
        return APP_GAME_FOREGROUND_FOREGROUND.to_string();
    }
    if record.foreground_state == APP_GAME_FOREGROUND_BACKGROUND {
        return APP_GAME_FOREGROUND_BACKGROUND.to_string();
    }
    APP_GAME_FOREGROUND_UNKNOWN.to_string()
}

fn classification_state_for_record(record: &WindowsForegroundWindowRecord) -> String {
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
    if has_deterministic_foreground_ref(record) {
        return record.classification_state.clone();
    }
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string()
}

fn catalog_ready_state_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CATALOG_PERMISSION_LIMITED.to_string();
    }
    if record.catalog_ref.is_some() {
        return APP_GAME_CATALOG_READY.to_string();
    }
    APP_GAME_CATALOG_UNAVAILABLE.to_string()
}

fn capability_status_for_record(record: &WindowsForegroundWindowRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string()
    } else if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR.to_string()
    } else {
        APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string()
    }
}

fn confidence_for_record(record: &WindowsForegroundWindowRecord) -> f64 {
    if has_deterministic_foreground_ref(record) {
        record.confidence
    } else {
        APP_GAME_CONFIDENCE_UNKNOWN
    }
}

fn has_deterministic_foreground_ref(record: &WindowsForegroundWindowRecord) -> bool {
    record.inventory_entry_id.is_some()
        || record.launcher_ref.is_some()
        || record.catalog_ref.is_some()
        || record.window_ref.is_some()
}

fn apply_foreground_row(summary: &mut AppGameSessionSummary, row: &AppGameForegroundEvidenceRow) {
    let foreground_duration = summary
        .foreground_duration_ms
        .max(row.foreground_duration_ms)
        .min(summary.running_duration_ms);
    summary.foreground_duration_ms = foreground_duration;
    summary.background_duration_ms = summary
        .running_duration_ms
        .saturating_sub(summary.foreground_duration_ms);
    if row.foreground_state == APP_GAME_FOREGROUND_FOREGROUND {
        summary.last_foreground_at = row.foreground_started_at.clone();
    }
    if row.foreground_state == APP_GAME_FOREGROUND_BACKGROUND {
        summary.last_background_at = row.foreground_ended_at.clone();
    }
    summary.last_observed_at = row.observed_at.clone();
    summary.evidence_count += row.evidence.len() as u64;
    summary.evidence.extend(row.evidence.clone());
    if is_stronger_foreground_classification(row, summary) {
        summary.display_name = row.process_name.clone();
        summary.classification_state = row.classification_state.clone();
        summary.catalog_ready_state = row.catalog_ready_state.clone();
        summary.inventory_entry_id = row.inventory_entry_id.clone();
        summary.launcher_ref = row.launcher_ref.clone();
        summary.catalog_ref = row.catalog_ref.clone();
        summary.confidence = row.confidence;
    }
}

fn is_stronger_foreground_classification(
    row: &AppGameForegroundEvidenceRow,
    summary: &AppGameSessionSummary,
) -> bool {
    foreground_classification_rank(&row.classification_state)
        > foreground_classification_rank(&summary.classification_state)
}

fn foreground_classification_rank(classification_state: &str) -> u8 {
    match classification_state {
        APP_GAME_CLASSIFICATION_KNOWN_APP => 4,
        APP_GAME_CLASSIFICATION_KNOWN_GAME => 4,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER => 3,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED => 1,
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR => 1,
        _ => 0,
    }
}
