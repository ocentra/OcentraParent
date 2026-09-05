use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;

use super::app_game_windows_launcher::{
    windows_launcher_rows_from_records, WindowsLauncherEvidenceRecord,
};

#[test]
fn launcher_process_stays_launcher_without_game_claim() {
    let rows = windows_launcher_rows_from_records(&[launcher_only_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].launcher_evidence_id,
        APP_GAME_TEST_LAUNCHER_EVIDENCE_ID
    );
    assert_eq!(rows[0].launcher_kind, APP_GAME_LAUNCHER_KIND_STEAM);
    assert_eq!(rows[0].launcher_ref, APP_GAME_TEST_LAUNCHER_REF);
    assert_eq!(
        rows[0].launcher_process_identity,
        Some(APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY.to_string())
    );
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
    );
    assert_eq!(
        rows[0].game_proof_state,
        APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY
    );
    assert_eq!(rows[0].child_game_evidence_claim_id, None);
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
}

#[test]
fn launcher_foreground_remains_launcher_foreground_only() {
    let mut record = launcher_only_record();
    record.foreground_state = APP_GAME_FOREGROUND_FOREGROUND.to_string();

    let rows = windows_launcher_rows_from_records(&[record]);

    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
    );
    assert_eq!(
        rows[0].game_proof_state,
        APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY
    );
    assert_eq!(rows[0].child_game_evidence_claim_id, None);
}

#[test]
fn launcher_game_candidate_is_not_known_game_without_child_proof() {
    let mut record = launcher_only_record();
    record.launcher_evidence_id = APP_GAME_TEST_LAUNCHER_CANDIDATE_EVIDENCE_ID.to_string();
    record.launcher_app_id = Some(APP_GAME_TEST_LAUNCHER_APP_ID.to_string());
    record.child_process_identity = Some(APP_GAME_TEST_LAUNCHER_CHILD_PROCESS_IDENTITY.to_string());
    record.classification_state = APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    record.game_proof_state = APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE.to_string();
    record.confidence = 0.52;

    let rows = windows_launcher_rows_from_records(&[record]);

    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE
    );
    assert_eq!(
        rows[0].game_proof_state,
        APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE
    );
    assert_eq!(rows[0].child_game_evidence_claim_id, None);
}

#[test]
fn launcher_manifest_candidate_is_not_known_game_without_child_proof() {
    let mut record = launcher_only_record();
    record.launcher_evidence_id = APP_GAME_TEST_LAUNCHER_CANDIDATE_EVIDENCE_ID.to_string();
    record.launcher_app_id = Some(APP_GAME_TEST_LAUNCHER_APP_ID.to_string());
    record.game_proof_state = APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE.to_string();
    record.classification_state = APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    record.confidence = 0.67;

    let rows = windows_launcher_rows_from_records(&[record]);

    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE
    );
    assert_eq!(
        rows[0].game_proof_state,
        APP_GAME_LAUNCHER_PROOF_MANIFEST_CANDIDATE
    );
    assert_eq!(rows[0].child_process_identity, None);
    assert_eq!(rows[0].child_game_evidence_claim_id, None);
}

#[test]
fn deterministic_child_game_proof_can_promote_known_game() {
    let mut record = launcher_only_record();
    record.launcher_evidence_id = APP_GAME_TEST_LAUNCHER_KNOWN_GAME_EVIDENCE_ID.to_string();
    record.launcher_app_id = Some(APP_GAME_TEST_LAUNCHER_APP_ID.to_string());
    record.child_process_identity = Some(APP_GAME_TEST_LAUNCHER_CHILD_PROCESS_IDENTITY.to_string());
    record.child_inventory_entry_id =
        Some(APP_GAME_TEST_LAUNCHER_CHILD_INVENTORY_ENTRY_ID.to_string());
    record.child_game_evidence_claim_id =
        Some(APP_GAME_TEST_LAUNCHER_CHILD_GAME_CLAIM_ID.to_string());
    record.catalog_ref = Some(APP_GAME_TEST_STORE_GAME_CATALOG_REF.to_string());
    record.game_proof_state = APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME.to_string();
    record.confidence = 0.91;

    let rows = windows_launcher_rows_from_records(&[record]);

    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_GAME
    );
    assert_eq!(
        rows[0].game_proof_state,
        APP_GAME_LAUNCHER_PROOF_DETERMINISTIC_CHILD_GAME
    );
    assert_eq!(
        rows[0].child_game_evidence_claim_id,
        Some(APP_GAME_TEST_LAUNCHER_CHILD_GAME_CLAIM_ID.to_string())
    );
    assert_eq!(rows[0].catalog_ready_state, APP_GAME_CATALOG_READY);
}

#[test]
fn classifier_backed_child_game_proof_can_promote_known_game() {
    let mut record = launcher_only_record();
    record.launcher_evidence_id = APP_GAME_TEST_LAUNCHER_KNOWN_GAME_EVIDENCE_ID.to_string();
    record.launcher_app_id = Some(APP_GAME_TEST_LAUNCHER_APP_ID.to_string());
    record.child_process_identity = Some(APP_GAME_TEST_LAUNCHER_CHILD_PROCESS_IDENTITY.to_string());
    record.child_inventory_entry_id =
        Some(APP_GAME_TEST_LAUNCHER_CHILD_INVENTORY_ENTRY_ID.to_string());
    record.child_game_evidence_claim_id =
        Some(APP_GAME_TEST_LAUNCHER_CHILD_GAME_CLAIM_ID.to_string());
    record.catalog_ref = Some(APP_GAME_TEST_STORE_GAME_CATALOG_REF.to_string());
    record.game_proof_state = APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME.to_string();
    record.confidence = 0.89;

    let rows = windows_launcher_rows_from_records(&[record]);

    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_GAME
    );
    assert_eq!(
        rows[0].game_proof_state,
        APP_GAME_LAUNCHER_PROOF_CLASSIFIER_BACKED_CHILD_GAME
    );
    assert_eq!(
        rows[0].child_game_evidence_claim_id,
        Some(APP_GAME_TEST_LAUNCHER_CHILD_GAME_CLAIM_ID.to_string())
    );
    assert_eq!(rows[0].catalog_ready_state, APP_GAME_CATALOG_READY);
}

#[test]
fn permission_limited_launcher_state_remains_explicit() {
    let mut record = launcher_only_record();
    record.capability_status = APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string();
    record.launcher_inventory_entry_id = None;
    record.launcher_manifest_id = None;
    record.launcher_process_identity = None;
    record.launcher_process_id = None;
    record.launcher_process_name = None;

    let rows = windows_launcher_rows_from_records(&[record]);

    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED
    );
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_PERMISSION_LIMITED);
    assert_eq!(
        rows[0].foreground_state,
        APP_GAME_FOREGROUND_PERMISSION_LIMITED
    );
    assert_eq!(
        rows[0].catalog_ready_state,
        APP_GAME_CATALOG_PERMISSION_LIMITED
    );
    assert_eq!(
        rows[0].game_proof_state,
        APP_GAME_LAUNCHER_PROOF_PERMISSION_LIMITED
    );
    assert_eq!(rows[0].confidence, 0.0);
}

fn launcher_only_record() -> WindowsLauncherEvidenceRecord {
    WindowsLauncherEvidenceRecord {
        launcher_evidence_id: APP_GAME_TEST_LAUNCHER_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        launcher_kind: APP_GAME_LAUNCHER_KIND_STEAM.to_string(),
        launcher_ref: APP_GAME_TEST_LAUNCHER_REF.to_string(),
        launcher_inventory_entry_id: Some(APP_GAME_TEST_LAUNCHER_SOURCE_REF.to_string()),
        launcher_manifest_id: Some(APP_GAME_TEST_LAUNCHER_MANIFEST_ID.to_string()),
        launcher_app_id: None,
        launcher_process_identity: Some(APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY.to_string()),
        launcher_process_id: Some(APP_GAME_TEST_LAUNCHER_PROCESS_ID),
        launcher_process_name: Some(APP_GAME_TEST_LAUNCHER_PROCESS_NAME.to_string()),
        child_process_identity: None,
        child_inventory_entry_id: None,
        child_game_evidence_claim_id: None,
        catalog_ref: None,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        game_proof_state: APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY.to_string(),
        confidence: 0.74,
        evidence: Vec::new(),
    }
}
