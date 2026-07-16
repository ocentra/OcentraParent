use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER, APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE,
    APP_GAME_CLASSIFICATION_PERMISSION_LIMITED, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_LAUNCHER_PROOF_ADAPTER_ERROR, APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE,
    APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME,
    APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME, APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY,
    APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE, APP_GAME_LAUNCHER_PROOF_PERMISSION_LIMITED,
};

use super::has_launcher_reference;
use crate::activity_store_app_game::app_game_windows_launcher::WindowsLauncherEvidenceRecord;

pub(crate) fn classification_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_CLASSIFICATION_PERMISSION_LIMITED {
        return APP_GAME_CLASSIFICATION_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_CLASSIFICATION_ADAPTER_ERROR {
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

pub(crate) fn game_proof_state_for_record(record: &WindowsLauncherEvidenceRecord) -> String {
    if record.capability_status == APP_GAME_LAUNCHER_PROOF_PERMISSION_LIMITED {
        return APP_GAME_LAUNCHER_PROOF_PERMISSION_LIMITED.to_string();
    }
    if record.capability_status == APP_GAME_LAUNCHER_PROOF_ADAPTER_ERROR {
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

pub(crate) fn record_has_child_game_proof(record: &WindowsLauncherEvidenceRecord) -> bool {
    (record.game_proof_state == APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME
        || record.game_proof_state == APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME)
        && record.child_game_evidence_claim_id.is_some()
}
