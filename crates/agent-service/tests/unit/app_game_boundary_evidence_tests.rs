use std::error::Error;
use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivitySurfaceRequest, ActivitySurfaceScope, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, AppGameIdentity, AppGameInventoryEvidenceRow, AppGameServiceReadModel,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_INVENTORY_STATE_INSTALLED, APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE,
    APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED, APP_GAME_PRODUCT_NATIVE_APP,
    APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION,
    APP_GAME_TEST_DISPLAY_LABEL,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameAiClassifierResult, AppGameControlActionResult, AppGameControlApprovalAuthority,
    AppGameControlApprovalDecision, AppGameControlApprovalRequest,
    AppGameEnforcementCapabilityStatus, AppGameParentActionReference, AppGameParentActorReference,
    AppGameParentDeviceReference, AppGameParentEvidenceReference, AppGamePlatformAuthorityMatrix,
    APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED,
    APP_GAME_CONTROL_APPROVAL_STATE_MANUAL_REQUIRED, APP_GAME_CONTROL_AUTHORITY_ACTIVE,
    APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED, APP_GAME_CONTROL_POLICY_KIND_APP,
    APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY, APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL,
    APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED, APP_GAME_PARENT_ACTOR_ROLE_PARENT,
    APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION, APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT,
    APP_GAME_PARENT_PLATFORM_WINDOWS, APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH,
    APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED, APP_GAME_POLICY_TARGET_TYPE_APP,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use ocentra_parent_agent_service::test_support::{
    build_activity_app_use_read_model_from_app_game_for_test,
    build_activity_games_read_model_for_test,
};

const APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY: &TestStr = "unknownIdentityCandidate";
const APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY: &TestStr = "inventoryEvidence";
const APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE: &TestStr = "localModelUnavailable";
const APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW: &TestStr = "manualReview";
const APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP: &TestStr = "unknownApp";
const APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE: &TestStr = "providerUnavailable";
const APP_GAME_CONTROL_DECISION_DENIED: &TestStr = "denied";
const APP_GAME_CONTROL_EVIDENCE_PROOF_LAUNCHER_ONLY: &TestStr = "launcher-only";
const APP_GAME_CONTROL_PERSISTENCE_NOT_PERSISTED: &TestStr = "not-persisted";
const APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED: &TestStr = "installed";
const APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED: &TestStr = "allowed";
const APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY: &TestStr = "inventory";
const APP_GAME_FOREGROUND_NOT_CLAIMED: &TestStr = "notClaimed";
const APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC: &TestStr = "deterministic";
const APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED: &TestStr = "catalogMatched";
const APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN: &TestStr = "inventoryScan";
const APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_PLATFORM_PROOF_KIND_ROLLBACK: &TestStr = "rollback-proof";
const APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER: &TestStr = "windows-applocker-proof";
const APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_POLICY_ACTION_BLOCK: &TestStr = "block";
const APP_GAME_TEST_ACTION_REFERENCE_ID: &TestStr = "parent-action-app-game-1";
const APP_GAME_TEST_ACTION_RESULT_ID: &TestStr = "action-result-app-game-1";
const APP_GAME_TEST_AUTHORITY_ID: &TestStr = "authority-app-game-1";
const APP_GAME_TEST_CATALOG_REF: &TestStr = "catalog-ref-ocentra-game";
const APP_GAME_TEST_CHILD_PROFILE_ID: &TestStr = "child-app-game-1";
const APP_GAME_TEST_CLASSIFIER_DIGEST_REF: &TestStr = "classifier-digest-app-game-1";
const APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF: &TestStr = "classifier-evidence-app-game-1";
const APP_GAME_TEST_CLASSIFIER_LABEL: &TestStr = "Possible native game";
const APP_GAME_TEST_CLASSIFIER_PROMPT_REF: &TestStr = "prompt-app-game-classifier";
const APP_GAME_TEST_CLASSIFIER_REASON_CODE: &TestStr = "unknown-game-like";
const APP_GAME_TEST_CLASSIFIER_RUNTIME_REF: &TestStr = "local-ai-runtime-app-game";
const APP_GAME_TEST_CLASSIFIER_RUN_ID: &TestStr = "classifier-run-app-game-1";
const APP_GAME_TEST_DECISION_ID: &TestStr = "approval-decision-app-game-1";
const APP_GAME_TEST_DEVICE_ID: &TestStr = "device-windows-app-game-1";
const APP_GAME_TEST_DEVICE_LABEL: &TestStr = "Study PC";
const APP_GAME_TEST_EVIDENCE_CLAIM_ID: &TestStr = "claim-ocentra-inventory";
const APP_GAME_TEST_EVIDENCE_REF_ID: &TestStr = "evidence-app-game-session-1";
const APP_GAME_TEST_GAME_DISPLAY_LABEL: &TestStr = "Ocentra Game Fixture";
const APP_GAME_TEST_IDENTITY_ID: &TestStr = "identity-ocentra-game";
const APP_GAME_TEST_PARENT_ACTOR_ID: &TestStr = "parent-actor-app-game-1";
const APP_GAME_TEST_PLATFORM_MATRIX_ID: &TestStr = "app-game-platform-authority-matrix";
const APP_GAME_TEST_POLICY_VERSION: &TestStr = "policy-version-app-game-1";
const APP_GAME_TEST_REQUEST_ID: &TestStr = "approval-request-app-game-1";
const APP_GAME_TEST_SETTING_ID: &TestStr = "app.enforcement.allowedActions";
const APP_GAME_TEST_SETTING_PATH: &TestStr = "/appPolicy/enforcement/allowedActions";
const APP_GAME_TEST_TARGET_ID: &TestStr = "target-app-game-1";
const APP_GAME_TEST_TARGET_VALUE: &TestStr = "process:ocentra-game.exe";
const APP_GAME_TEST_TIMESTAMP: &TestStr = "2026-06-03T22:15:00Z";
const APP_GAME_TEST_WINDOWS_LIMITATION: &TestStr =
    "Broad installed-app blocking needs AppLocker or App Control proof before execution.";
const APP_GAME_TEST_WINDOWS_ROW_ID: &TestStr = "windows-block-launch-row";

#[test]
fn app_use_read_model_preserves_app_game_boundary_evidence_refs() -> Result<(), Box<dyn Error>> {
    let read_model = build_activity_app_use_read_model_from_app_game_for_test(
        surface_request(),
        Some(service_model()?),
    );

    assert_eq!(read_model.state, ActivityReadModelState::Ready);
    assert_eq!(read_model.rows[0].evidence_claim_row_count, 1);
    assert_eq!(read_model.rows[0].identity_row_count, 1);
    assert_eq!(read_model.rows[0].approval_authority_row_count, 1);
    assert_eq!(read_model.rows[0].approval_action_result_row_count, 1);
    assert_eq!(read_model.rows[0].platform_authority_matrix_count, 1);
    assert_eq!(read_model.rows[0].platform_authority_row_count, 1);
    assert_eq!(read_model.rows[0].ai_classifier_result_row_count, 1);
    assert_boundary_refs(&read_model.rows[0].evidence);
    Ok(())
}

#[test]
fn games_read_model_preserves_app_game_boundary_evidence_refs() -> Result<(), Box<dyn Error>> {
    let read_model =
        build_activity_games_read_model_for_test(surface_request(), Some(service_model()?));

    assert_eq!(read_model.state, ActivityReadModelState::Ready);
    assert_eq!(read_model.rows[0].evidence_claim_row_count, 1);
    assert_eq!(read_model.rows[0].identity_row_count, 1);
    assert_eq!(read_model.rows[0].approval_authority_row_count, 1);
    assert_eq!(read_model.rows[0].approval_action_result_row_count, 1);
    assert_eq!(read_model.rows[0].platform_authority_matrix_count, 1);
    assert_eq!(read_model.rows[0].platform_authority_row_count, 1);
    assert_eq!(read_model.rows[0].ai_classifier_result_row_count, 1);
    assert_boundary_refs(&read_model.rows[0].evidence);
    Ok(())
}

fn assert_boundary_refs(evidence: &[ActivityEvidenceRef]) {
    for expected in [
        APP_GAME_TEST_EVIDENCE_REF_ID,
        APP_GAME_TEST_EVIDENCE_CLAIM_ID,
        APP_GAME_TEST_IDENTITY_ID,
        APP_GAME_TEST_AUTHORITY_ID,
        APP_GAME_TEST_ACTION_RESULT_ID,
        APP_GAME_TEST_PLATFORM_MATRIX_ID,
        APP_GAME_TEST_WINDOWS_ROW_ID,
        APP_GAME_TEST_CLASSIFIER_RUN_ID,
        APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF,
    ] {
        assert!(
            evidence
                .iter()
                .any(|row| row.evidence_id == expected
                    && row.kind == ActivityEvidenceKind::LocalDbRow)
        );
    }
}

fn service_model() -> Result<AppGameServiceReadModel, Box<dyn Error>> {
    Ok(AppGameServiceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        custody_label: APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        inventory_returned: 2,
        running_now_returned: 0,
        foreground_now_returned: 0,
        launcher_returned: 0,
        daily_rollup_returned: 0,
        evidence_claim_returned: 1,
        identity_returned: 1,
        approval_authority_returned: 1,
        approval_action_result_returned: 1,
        platform_authority_matrix_returned: 1,
        ai_classifier_result_returned: 1,
        inventory_rows: vec![
            inventory_row(
                APP_GAME_PRODUCT_NATIVE_APP,
                APP_GAME_CLASSIFICATION_KNOWN_APP,
            ),
            inventory_row(
                APP_GAME_PRODUCT_NATIVE_GAME,
                APP_GAME_CLASSIFICATION_KNOWN_GAME,
            ),
        ],
        running_now_rows: Vec::new(),
        foreground_now_rows: Vec::new(),
        launcher_rows: Vec::new(),
        daily_rollups: Vec::new(),
        evidence_claim_rows: vec![evidence_claim()],
        identity_rows: vec![identity()],
        approval_authority_rows: vec![approval_authority()],
        approval_action_result_rows: vec![manual_action_result()],
        platform_authority_matrices: vec![platform_matrix()?],
        ai_classifier_result_rows: vec![classifier_result()?],
    })
}

fn inventory_row(
    product_kind: &TestStr,
    classification_state: &TestStr,
) -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source_kind: APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN.to_string(),
        source_ref: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        custody_state: APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        product_kind: product_kind.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_id: Some(APP_GAME_TEST_IDENTITY_ID.to_string()),
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        classification_state: classification_state.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.82,
        category_candidates: Vec::new(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: Vec::new(),
    }
}

fn evidence_claim() -> AppGameEvidenceClaim {
    AppGameEvidenceClaim {
        schema_version: APP_GAME_SCHEMA_VERSION,
        claim_id: APP_GAME_TEST_EVIDENCE_CLAIM_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        claim_kind: APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN.to_string(),
        display_name: APP_GAME_TEST_GAME_DISPLAY_LABEL.to_string(),
        identity_strength: APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        inventory_entry_id: Some(APP_GAME_TEST_EVIDENCE_REF_ID.to_string()),
        process_identity: None,
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        confidence: 0.82,
        evidence: vec![ActivityEvidenceRef {
            evidence_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    }
}

fn identity() -> AppGameIdentity {
    AppGameIdentity {
        schema_version: APP_GAME_SCHEMA_VERSION,
        identity_id: APP_GAME_TEST_IDENTITY_ID.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
        display_label: APP_GAME_TEST_GAME_DISPLAY_LABEL.to_string(),
        parent_label: None,
        confidence: APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        application_token_ref: None,
        executable_path_ref: None,
        publisher_signature_ref: None,
        file_hash_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        child_game_evidence_claim_id: Some(APP_GAME_TEST_EVIDENCE_CLAIM_ID.to_string()),
        evidence: Vec::new(),
    }
}

fn approval_authority() -> AppGameControlApprovalAuthority {
    AppGameControlApprovalAuthority {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        authority_id: APP_GAME_TEST_AUTHORITY_ID.to_string(),
        actor: parent_actor(),
        device: child_device(),
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        authority_state: APP_GAME_CONTROL_AUTHORITY_ACTIVE.to_string(),
        allowed_policy_kinds: vec![APP_GAME_CONTROL_POLICY_KIND_APP.to_string()],
        can_approve: true,
        can_deny: true,
        can_extend: false,
        can_override: false,
        can_observe_only: true,
        checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn manual_action_result() -> AppGameControlActionResult {
    AppGameControlActionResult {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        result_id: APP_GAME_TEST_ACTION_RESULT_ID.to_string(),
        request: approval_request()?,
        decision: approval_decision(),
        approval_state: APP_GAME_CONTROL_APPROVAL_STATE_MANUAL_REQUIRED.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        capability: capability(),
        evidence_proof_kind: APP_GAME_CONTROL_EVIDENCE_PROOF_LAUNCHER_ONLY.to_string(),
        result_status: APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED.to_string(),
        enforcement_result: None,
        recorded_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn approval_request() -> Result<AppGameControlApprovalRequest, Box<dyn Error>> {
    Ok(serde_json::from_value(serde_json::json!({
        "schemaVersion": APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
        "requestId": APP_GAME_TEST_REQUEST_ID,
        "policyKind": APP_GAME_CONTROL_POLICY_KIND_APP,
        "device": child_device(),
        "target": {
            "targetId": APP_GAME_TEST_TARGET_ID,
            "targetType": APP_GAME_POLICY_TARGET_TYPE_APP,
            "targetValue": APP_GAME_TEST_TARGET_VALUE
        },
        "requestedAction": APP_GAME_POLICY_ACTION_BLOCK,
        "requestedMode": null,
        "requestedSettingRefs": [{
            "settingId": APP_GAME_TEST_SETTING_ID,
            "writesTo": APP_GAME_TEST_SETTING_PATH
        }],
        "evidenceReferences": [parent_evidence()],
        "candidate": null,
        "childReasonState": APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED,
        "childReasonReferences": [],
        "childStatusReferences": [],
        "expiresAt": APP_GAME_TEST_TIMESTAMP,
        "unansweredFallback": APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY
    }))?)
}

fn approval_decision() -> AppGameControlApprovalDecision {
    AppGameControlApprovalDecision {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        decision_id: APP_GAME_TEST_DECISION_ID.to_string(),
        request_id: APP_GAME_TEST_REQUEST_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        decision_state: APP_GAME_CONTROL_DECISION_DENIED.to_string(),
        parent_action: Some(AppGameParentActionReference {
            action_reference_id: APP_GAME_TEST_ACTION_REFERENCE_ID.to_string(),
            actor: parent_actor(),
            policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
            created_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }),
        reason_codes: Vec::new(),
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![parent_evidence()],
        response_scope: None,
        decision_expires_at: None,
        audit_references: Vec::new(),
        persistence_state: APP_GAME_CONTROL_PERSISTENCE_NOT_PERSISTED.to_string(),
        decided_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn platform_matrix() -> Result<AppGamePlatformAuthorityMatrix, Box<dyn Error>> {
    Ok(serde_json::from_value(serde_json::json!({
        "schemaVersion": APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
        "matrixId": APP_GAME_TEST_PLATFORM_MATRIX_ID,
        "rows": [{
            "schemaVersion": APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
            "rowId": APP_GAME_TEST_WINDOWS_ROW_ID,
            "platform": APP_GAME_PARENT_PLATFORM_WINDOWS,
            "action": APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH,
            "authorityTier": APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED,
            "setupState": APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED,
            "proofState": APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED,
            "capabilityState": APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED,
            "parentVisibleState": APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED,
            "parentVisibleLimitation": APP_GAME_TEST_WINDOWS_LIMITATION,
            "canExecuteAdapter": false,
            "supportedModes": [],
            "proofReferences": [],
            "proofNeededToClaim": [
                APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER,
                APP_GAME_PLATFORM_PROOF_KIND_ROLLBACK
            ],
            "linuxMechanism": null,
            "linuxDistro": null,
            "linuxSession": null,
            "lastCheckedAt": APP_GAME_TEST_TIMESTAMP
        }],
        "generatedAt": APP_GAME_TEST_TIMESTAMP
    }))?)
}

fn classifier_result() -> Result<AppGameAiClassifierResult, Box<dyn Error>> {
    Ok(AppGameAiClassifierResult {
        schema_version: APP_GAME_SCHEMA_VERSION,
        classifier_run_id: APP_GAME_TEST_CLASSIFIER_RUN_ID.to_string(),
        product_kind: APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP.to_string(),
        digest_ref: APP_GAME_TEST_CLASSIFIER_DIGEST_REF.to_string(),
        source_digest_kind: APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY.to_string(),
        source_evidence_refs: vec![APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF.to_string()],
        source_session_refs: Vec::new(),
        candidate_kind: APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY.to_string(),
        candidate_label: APP_GAME_TEST_CLASSIFIER_LABEL.to_string(),
        classifier_state: APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE.to_string(),
        confidence: 0.0,
        uncertainty_reason_codes: vec![APP_GAME_TEST_CLASSIFIER_REASON_CODE.to_string()],
        model_runtime_ref: APP_GAME_TEST_CLASSIFIER_RUNTIME_REF.to_string(),
        prompt_template_ref: APP_GAME_TEST_CLASSIFIER_PROMPT_REF.to_string(),
        prompt_version: APP_GAME_TEST_CLASSIFIER_PROMPT_REF.to_string(),
        fallback_state: APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE.to_string(),
        policy_handoff: APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW.to_string(),
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        direct_action_requested: false,
        raw_scan_included: false,
        content_claim_included: false,
    })
}

fn capability() -> AppGameEnforcementCapabilityStatus {
    AppGameEnforcementCapabilityStatus {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        adapter_kind: APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        permission_state: APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED.to_string(),
        dependency_state: APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED.to_string(),
        supported_actions: Vec::new(),
        degraded_reason: Some(APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string()),
        last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn parent_actor() -> AppGameParentActorReference {
    AppGameParentActorReference {
        actor_id: APP_GAME_TEST_PARENT_ACTOR_ID.to_string(),
        role: APP_GAME_PARENT_ACTOR_ROLE_PARENT.to_string(),
    }
}

fn child_device() -> AppGameParentDeviceReference {
    AppGameParentDeviceReference {
        device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
        child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
        label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
    }
}

fn parent_evidence() -> AppGameParentEvidenceReference {
    AppGameParentEvidenceReference {
        evidence_reference_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn surface_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Family,
            family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            device_id: None,
        },
        requested_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        range_start: APP_GAME_TEST_TIMESTAMP.to_string(),
        range_end: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}
