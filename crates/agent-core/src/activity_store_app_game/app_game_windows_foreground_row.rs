use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED,
    APP_GAME_FOREGROUND_ADAPTER_ERROR, APP_GAME_FOREGROUND_BACKGROUND,
    APP_GAME_FOREGROUND_FOREGROUND, APP_GAME_FOREGROUND_PERMISSION_LIMITED,
    APP_GAME_FOREGROUND_UNKNOWN, APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW,
    APP_GAME_RUNTIME_RUNNING, APP_GAME_RUNTIME_UNKNOWN, APP_GAME_SCHEMA_VERSION,
    APP_GAME_TITLE_CAPTURE_ADAPTER_ERROR, APP_GAME_TITLE_CAPTURE_PERMISSION_LIMITED,
    APP_GAME_TITLE_CAPTURE_TITLE_OMITTED, APP_GAME_TITLE_CAPTURE_TITLE_REF,
};
use ocentra_parent_agent_protocol::constants;

use super::app_game_windows_foreground_classification::{
    capability_status_for_record, catalog_ready_state_for_record, classification_state_for_record,
    confidence_for_record,
};
use super::WindowsForegroundWindowRecord;

pub(super) fn row_from_record(
    record: &WindowsForegroundWindowRecord,
) -> AppGameForegroundEvidenceRow {
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
