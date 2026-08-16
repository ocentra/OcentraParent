use ocentra_parent_agent_protocol::app_game::{
    AppGameLauncherEvidenceRow, APP_GAME_CONFIDENCE_UNKNOWN,
    APP_GAME_FOREGROUND_PERMISSION_LIMITED, APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME,
    APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME, APP_GAME_RUNTIME_PERMISSION_LIMITED,
    APP_GAME_SCHEMA_VERSION,
};

use super::app_game_windows_launcher_classification::{
    capability_status_for_record, catalog_ready_state_for_record, classification_state_for_record,
    game_proof_state_for_record, has_launcher_reference,
};
use super::WindowsLauncherEvidenceRecord;

pub(super) fn row_from_record(
    record: &WindowsLauncherEvidenceRecord,
) -> AppGameLauncherEvidenceRow {
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
    if record.capability_status == APP_GAME_RUNTIME_PERMISSION_LIMITED {
        return APP_GAME_RUNTIME_PERMISSION_LIMITED.to_string();
    }
    record.runtime_state.clone()
}

fn foreground_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_FOREGROUND_PERMISSION_LIMITED {
        return APP_GAME_FOREGROUND_PERMISSION_LIMITED.to_string();
    }
    record.foreground_state.clone()
}

fn confidence_for_record(record: &WindowsLauncherEvidenceRecord) -> f64 {
    if record.capability_status == APP_GAME_RUNTIME_PERMISSION_LIMITED
        || record.capability_status == APP_GAME_FOREGROUND_PERMISSION_LIMITED
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
