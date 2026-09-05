use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    self, AppGameEvidenceClaim, AppGameServiceReadModel, AppGameSessionDailyRollup,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    self, AppGamePlatformAuthorityMatrix,
};
use ocentra_parent_agent_protocol::app_game_boundary_read_model::{self, AppGameBoundaryReadModel};
use ocentra_parent_agent_protocol::constants;
use std::primitive::str as TestStr;

use crate::test_require_json_decode::require_json_decode;
use crate::test_require_log_string_field::require_log_string_field;
use crate::test_require_ok::require_ok;

use super::app_game_boundary_read_model_payload::{
    app_game_boundary_read_model_from_service_model, app_game_boundary_read_model_payload,
};

const APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY: &TestStr = "inventory";
const APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED: &TestStr = "catalogMatched";
const APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN: &TestStr = "inventoryScan";
const APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER: &TestStr = "windows-applocker-proof";
const APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_TEST_EVIDENCE_CLAIM_ID: &TestStr = "claim-ocentra-inventory";
const APP_GAME_TEST_EVIDENCE_REF_ID: &TestStr = "evidence-app-game-session-1";
const APP_GAME_TEST_PLATFORM_MATRIX_ID: &TestStr = "app-game-platform-authority-matrix";
const APP_GAME_TEST_TIMESTAMP: &TestStr = "2026-06-03T22:15:00Z";
const APP_GAME_TEST_WINDOWS_LIMITATION: &TestStr =
    "Broad installed-app blocking needs AppLocker or App Control proof before execution.";
const APP_GAME_TEST_WINDOWS_ROW_ID: &TestStr = "windows-block-launch-row";

#[test]
fn app_game_boundary_payload_contains_dedicated_counts_and_citations() {
    let read_model = app_game_boundary_read_model_from_service_model(service_model());
    let payload = app_game_boundary_read_model_payload(&read_model);
    let read_model_json = string_payload(&payload, constants::field::APP_GAME_BOUNDARY_READ_MODEL);
    let decoded = require_json_decode::<AppGameBoundaryReadModel>(
        read_model_json,
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(decoded.returned, 3);
    assert_eq!(
        decoded.performance_health.limit,
        constants::activity_store::DEFAULT_RECENT_LIMIT
    );
    assert_eq!(
        decoded.performance_health.status,
        app_game_boundary_read_model::AppGameHealthStatus::Healthy
    );
    assert_eq!(decoded.performance_health.daily_rollup_returned, 1);
    assert_eq!(decoded.performance_health.returned, 1);
    assert_eq!(
        decoded.performance_health.custody_label,
        app_game::APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE
    );
    assert_eq!(
        decoded.performance_health.replay_state,
        app_game::APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED
    );
    assert_eq!(decoded.evidence_claim_row_count, 1);
    assert_eq!(decoded.platform_authority_matrix_count, 1);
    assert_eq!(decoded.platform_authority_row_count, 1);
    assert_eq!(
        decoded.rows[0].boundary_kind,
        app_game_boundary_read_model::APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM
    );
    assert_eq!(
        decoded.rows[0].evidence_reference_ids,
        vec![
            APP_GAME_TEST_EVIDENCE_REF_ID,
            APP_GAME_TEST_EVIDENCE_CLAIM_ID
        ]
    );
    assert_eq!(
        decoded.rows[2].boundary_kind,
        app_game_boundary_read_model::APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_ROW
    );
    assert_eq!(
        decoded.rows[2].evidence_reference_ids,
        vec![APP_GAME_TEST_WINDOWS_ROW_ID]
    );
}

fn service_model() -> AppGameServiceReadModel {
    AppGameServiceReadModel {
        schema_version: app_game::APP_GAME_SCHEMA_VERSION,
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        custody_label: app_game::APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        replay_state: app_game::APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.to_string(),
        capability_status: app_game::APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        inventory_returned: 0,
        running_now_returned: 0,
        foreground_now_returned: 0,
        launcher_returned: 0,
        daily_rollup_returned: 1,
        evidence_claim_returned: 1,
        identity_returned: 0,
        approval_authority_returned: 0,
        approval_action_result_returned: 0,
        platform_authority_matrix_returned: 1,
        ai_classifier_result_returned: 0,
        inventory_rows: Vec::new(),
        running_now_rows: Vec::new(),
        foreground_now_rows: Vec::new(),
        launcher_rows: Vec::new(),
        daily_rollups: vec![AppGameSessionDailyRollup {
            schema_version: app_game::APP_GAME_SCHEMA_VERSION,
            rollup_date: APP_GAME_TEST_TIMESTAMP.to_string(),
            classification_state: app_game::APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
            session_count: 1,
            running_duration_ms: 1_000,
            foreground_duration_ms: 500,
            background_duration_ms: 500,
            evidence_count: 1,
            session_ids: vec!["session-1".to_string()],
            evidence: vec![local_db_ref(APP_GAME_TEST_EVIDENCE_REF_ID)],
        }],
        evidence_claim_rows: vec![evidence_claim()],
        identity_rows: Vec::new(),
        approval_authority_rows: Vec::new(),
        approval_action_result_rows: Vec::new(),
        platform_authority_matrices: vec![platform_matrix()],
        ai_classifier_result_rows: Vec::new(),
    }
}

fn evidence_claim() -> AppGameEvidenceClaim {
    AppGameEvidenceClaim {
        schema_version: app_game::APP_GAME_SCHEMA_VERSION,
        claim_id: APP_GAME_TEST_EVIDENCE_CLAIM_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        claim_kind: APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN.to_string(),
        display_name: app_game::APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_strength: APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED.to_string(),
        classification_state: app_game::APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: app_game::APP_GAME_CATALOG_READY.to_string(),
        runtime_state: app_game::APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: app_game::APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        inventory_entry_id: None,
        process_identity: None,
        launcher_ref: None,
        catalog_ref: None,
        confidence: 1.0,
        evidence: vec![local_db_ref(APP_GAME_TEST_EVIDENCE_REF_ID)],
    }
}

fn platform_matrix() -> AppGamePlatformAuthorityMatrix {
    require_ok(
        serde_json::from_value(serde_json::json!({
            "schemaVersion": app_game_authority_classifier::APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
            "matrixId": APP_GAME_TEST_PLATFORM_MATRIX_ID,
            "rows": [{
                "schemaVersion": app_game_authority_classifier::APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
                "rowId": APP_GAME_TEST_WINDOWS_ROW_ID,
                "platform": app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_WINDOWS,
                "action": app_game_authority_classifier::APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH,
                "authorityTier": app_game_authority_classifier::APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED,
                "setupState": APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED,
                "proofState": APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED,
                "capabilityState": app_game_authority_classifier::APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED,
                "parentVisibleState": APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED,
                "parentVisibleLimitation": APP_GAME_TEST_WINDOWS_LIMITATION,
                "canExecuteAdapter": false,
                "supportedModes": [],
                "proofReferences": [],
                "proofNeededToClaim": [APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER],
                "linuxMechanism": null,
                "linuxDistro": null,
                "linuxSession": null,
                "lastCheckedAt": APP_GAME_TEST_TIMESTAMP
            }],
            "generatedAt": APP_GAME_TEST_TIMESTAMP
        })),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn local_db_ref(evidence_id: &TestStr) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn string_payload<'a>(
    payload: &'a ocentra_parent_agent_protocol::logging::LogFields,
    field_name: &TestStr,
) -> &'a TestStr {
    require_log_string_field(
        payload.get(field_name),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}
