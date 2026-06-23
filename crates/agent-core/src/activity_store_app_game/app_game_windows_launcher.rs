use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameLauncherEvidenceRow, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CATALOG_PERMISSION_LIMITED, APP_GAME_CATALOG_READY, APP_GAME_CATALOG_UNAVAILABLE,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
    APP_GAME_CLASSIFICATION_PERMISSION_LIMITED, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_CONFIDENCE_UNKNOWN, APP_GAME_FOREGROUND_PERMISSION_LIMITED,
    APP_GAME_LAUNCHER_PROOF_ADAPTER_ERROR, APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE,
    APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME,
    APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME, APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY,
    APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE, APP_GAME_LAUNCHER_PROOF_PERMISSION_LIMITED,
    APP_GAME_RUNTIME_PERMISSION_LIMITED, APP_GAME_SCHEMA_VERSION,
};

pub struct WindowsLauncherEvidenceRecord {
    pub launcher_evidence_id: String,
    pub observed_at: String,
    pub launcher_kind: String,
    pub launcher_ref: String,
    pub launcher_inventory_entry_id: Option<String>,
    pub launcher_manifest_id: Option<String>,
    pub launcher_app_id: Option<String>,
    pub launcher_process_identity: Option<String>,
    pub launcher_process_id: Option<u64>,
    pub launcher_process_name: Option<String>,
    pub child_process_identity: Option<String>,
    pub child_inventory_entry_id: Option<String>,
    pub child_game_evidence_claim_id: Option<String>,
    pub catalog_ref: Option<String>,
    pub runtime_state: String,
    pub foreground_state: String,
    pub observation_mode: String,
    pub classification_state: String,
    pub capability_status: String,
    pub game_proof_state: String,
    pub confidence: f64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn windows_launcher_rows_from_records(
    records: &[WindowsLauncherEvidenceRecord],
) -> Vec<AppGameLauncherEvidenceRow> {
    records.iter().map(row_from_record).collect()
}

fn row_from_record(record: &WindowsLauncherEvidenceRecord) -> AppGameLauncherEvidenceRow {
    AppGameLauncherEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        launcher_evidence_id: record.launcher_evidence_id.clone(),
        observed_at: record.observed_at.clone(),
        launcher_kind: record.launcher_kind.clone(),
        launcher_ref: record.launcher_ref.clone(),
        launcher_inventory_entry_id: record.launcher_inventory_entry_id.clone(),
        launcher_manifest_id: record.launcher_manifest_id.clone(),
        launcher_app_id: record.launcher_app_id.clone(),
        launcher_process_identity: record.launcher_process_identity.clone(),
        launcher_process_id: record.launcher_process_id,
        launcher_process_name: record.launcher_process_name.clone(),
        child_process_identity: record.child_process_identity.clone(),
        child_inventory_entry_id: record.child_inventory_entry_id.clone(),
        child_game_evidence_claim_id: child_game_evidence_claim_id_for_record(record),
        catalog_ref: record.catalog_ref.clone(),
        runtime_state: runtime_state_for_record(record),
        foreground_state: foreground_state_for_record(record),
        observation_mode: record.observation_mode.clone(),
        classification_state: classification_state_for_record(record),
        catalog_ready_state: catalog_ready_state_for_record(record),
        capability_status: capability_status_for_record(record),
        game_proof_state: game_proof_state_for_record(record),
        confidence: confidence_for_record(record),
        evidence: record.evidence.clone(),
    }
}

fn child_game_evidence_claim_id_for_record(
    record: &WindowsLauncherEvidenceRecord,
) -> Option<String> {
    if record_has_child_game_proof(record) {
        return record.child_game_evidence_claim_id.clone();
    }
    None
}

fn runtime_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_RUNTIME_PERMISSION_LIMITED.to_string();
    }
    record.runtime_state.clone()
}

fn foreground_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_FOREGROUND_PERMISSION_LIMITED.to_string();
    }
    record.foreground_state.clone()
}

fn classification_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CLASSIFICATION_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        return APP_GAME_CLASSIFICATION_ADAPTER_ERROR.to_string();
    }
    if record_has_child_game_proof(record) {
        return APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    }
    if record.game_proof_state == APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE
        || record.game_proof_state == APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE
    {
        return APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE.to_string();
    }
    if has_launcher_reference(record) {
        return APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string();
    }
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string()
}

fn catalog_ready_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_CATALOG_PERMISSION_LIMITED.to_string();
    }
    if record.catalog_ref.is_some() {
        return APP_GAME_CATALOG_READY.to_string();
    }
    APP_GAME_CATALOG_UNAVAILABLE.to_string()
}

fn capability_status_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string()
    } else if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR.to_string()
    } else {
        APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string()
    }
}

fn game_proof_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED {
        return APP_GAME_LAUNCHER_PROOF_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR {
        return APP_GAME_LAUNCHER_PROOF_ADAPTER_ERROR.to_string();
    }
    if record_has_child_game_proof(record) {
        return record.game_proof_state.clone();
    }
    if record.game_proof_state == APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE
        || record.game_proof_state == APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE
    {
        return record.game_proof_state.clone();
    }
    APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY.to_string()
}

fn confidence_for_record(record: &WindowsLauncherEvidenceRecord) -> f64 {
    if record.capability_status == APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED
        || record.capability_status == APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR
        || !has_launcher_reference(record)
    {
        return APP_GAME_CONFIDENCE_UNKNOWN;
    }
    record.confidence
}

fn record_has_child_game_proof(record: &WindowsLauncherEvidenceRecord) -> bool {
    (record.game_proof_state == APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME
        || record.game_proof_state == APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME)
        && record.child_game_evidence_claim_id.is_some()
}

fn has_launcher_reference(record: &WindowsLauncherEvidenceRecord) -> bool {
    !record.launcher_ref.is_empty()
}
