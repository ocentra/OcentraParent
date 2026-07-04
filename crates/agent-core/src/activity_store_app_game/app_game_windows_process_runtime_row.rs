use ocentra_parent_agent_protocol::app_game::{
    AppGameRuntimeEvidenceRow, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_OBSERVATION_MODE_PROCESS_EXIT, APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
    APP_GAME_OBSERVATION_MODE_PROCESS_START, APP_GAME_RUNTIME_NOT_RUNNING,
    APP_GAME_RUNTIME_PERMISSION_LIMITED, APP_GAME_RUNTIME_RUNNING, APP_GAME_RUNTIME_UNKNOWN,
    APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::constants;

use super::app_game_windows_process_runtime_state::{
    capability_status_for_record, catalog_ready_state_for_record, classification_state_for_record,
    confidence_for_record,
};
use super::WindowsProcessRuntimeRecord;

pub(super) fn row_from_record(record: &WindowsProcessRuntimeRecord) -> AppGameRuntimeEvidenceRow {
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
    if record.capability_status == APP_GAME_RUNTIME_PERMISSION_LIMITED {
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
