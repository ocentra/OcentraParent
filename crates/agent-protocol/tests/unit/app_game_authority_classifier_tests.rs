use super::{constants, APP_GAME_SCHEMA_VERSION};
use crate::app_game_authority_classifier::*;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn app_game_control_authority_serializes_parent_approval_and_action_result_shape() {
    let authority = AppGameControlApprovalAuthority {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        authority_id: APP_GAME_TEST_AUTHORITY_ID.to_string(),
        actor: parent_actor(),
        device: child_device_windows(),
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        authority_state: APP_GAME_CONTROL_AUTHORITY_ACTIVE.to_string(),
        allowed_policy_kinds: vec![
            APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
            APP_GAME_CONTROL_POLICY_KIND_GAME.to_string(),
        ],
        can_approve: true,
        can_deny: true,
        can_extend: true,
        can_override: false,
        can_observe_only: true,
        checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    };
    let action_result = approved_action_result();

    let authority_json =
        serde_json::to_value(authority).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let result_json =
        serde_json::to_value(action_result).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        authority_json["schemaVersion"],
        APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION
    );
    assert_eq!(authority_json["authorityId"], APP_GAME_TEST_AUTHORITY_ID);
    assert_eq!(
        authority_json["allowedPolicyKinds"][0],
        APP_GAME_CONTROL_POLICY_KIND_APP
    );
    assert_eq!(
        result_json["request"]["candidate"]["candidateId"],
        APP_GAME_TEST_CANDIDATE_ID
    );
    assert_eq!(
        result_json["decision"]["responseScope"],
        APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE
    );
    assert_eq!(
        result_json["capability"]["supportedActions"][0],
        APP_GAME_ENFORCEMENT_MODE_TERMINATE_PROCESS
    );
    assert_eq!(
        result_json["enforcementResult"]["status"],
        APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED
    );
}

#[test]
fn app_game_platform_authority_matrix_serializes_proof_gated_rows() {
    let matrix = AppGamePlatformAuthorityMatrix {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        matrix_id: APP_GAME_TEST_PLATFORM_MATRIX_ID.to_string(),
        rows: vec![android_hide_row(), windows_manual_block_row()],
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    };

    let serialized =
        serde_json::to_value(matrix).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["matrixId"], APP_GAME_TEST_PLATFORM_MATRIX_ID);
    assert_eq!(
        serialized["rows"][0]["platform"],
        APP_GAME_PARENT_PLATFORM_ANDROID
    );
    assert_eq!(
        serialized["rows"][0]["authorityTier"],
        APP_GAME_PLATFORM_TIER_DEVICE_OWNER
    );
    assert_eq!(serialized["rows"][0]["canExecuteAdapter"], true);
    assert_eq!(
        serialized["rows"][0]["proofReferences"][0]["proofKind"],
        APP_GAME_PLATFORM_PROOF_KIND_DEVICE_OWNER
    );
    assert_eq!(
        serialized["rows"][1]["platform"],
        APP_GAME_PARENT_PLATFORM_WINDOWS
    );
    assert_eq!(
        serialized["rows"][1]["authorityTier"],
        APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED
    );
    assert_eq!(serialized["rows"][1]["canExecuteAdapter"], false);
    assert_eq!(
        serialized["rows"][1]["proofNeededToClaim"][0],
        APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER
    );
}

#[test]
fn app_game_ai_classifier_result_serializes_evidence_only_policy_handoff() {
    let candidate = classifier_result_candidate();
    let unavailable = classifier_result_unavailable();

    let candidate_json =
        serde_json::to_value(candidate).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let unavailable_json =
        serde_json::to_value(unavailable).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(candidate_json["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        candidate_json["sourceDigestKind"],
        APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY
    );
    assert_eq!(
        candidate_json["sourceEvidenceRefs"][0],
        APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF
    );
    assert_eq!(candidate_json["directActionRequested"], false);
    assert_eq!(candidate_json["rawScanIncluded"], false);
    assert_eq!(candidate_json["contentClaimIncluded"], false);
    assert_eq!(
        unavailable_json["fallbackState"],
        APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE
    );
    assert_eq!(
        APP_GAME_AI_CLASSIFIER_FORBIDDEN_KEYS,
        [
            "adapterAction",
            "block",
            "directAction",
            "durationMs",
            "enforcementAction",
            "fileScanRows",
            "foregroundDurationMs",
            "hide",
            "processScanRows",
            "rawOsScanResult",
            "runningDurationMs",
            "shield",
            "suspend",
            "terminate",
        ]
    );

    let round_tripped = serde_json::from_value::<AppGameAiClassifierResult>(candidate_json)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(round_tripped, classifier_result_candidate());
}

#[test]
fn app_game_ai_classifier_result_rejects_action_shaped_fields() {
    for forbidden_key in APP_GAME_AI_CLASSIFIER_FORBIDDEN_KEYS {
        let mut encoded = serde_json::to_value(classifier_result_candidate())
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
        encoded[forbidden_key] = serde_json::Value::Bool(true);

        let parsed = serde_json::from_value::<AppGameAiClassifierResult>(encoded);
        assert_eq!(
            parsed.err().map(|error| error.classify()),
            Some(serde_json::error::Category::Data)
        );
    }
}

fn approved_action_result() -> AppGameControlActionResult {
    let request = approval_request();
    let decision = approval_decision();
    let capability = supported_capability();

    AppGameControlActionResult {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        result_id: APP_GAME_TEST_ACTION_RESULT_ID.to_string(),
        request,
        decision,
        approval_state: APP_GAME_CONTROL_APPROVAL_STATE_APPROVED.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string(),
        capability: capability.clone(),
        evidence_proof_kind: APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY.to_string(),
        result_status: APP_GAME_CONTROL_ACTION_STATUS_ENFORCED.to_string(),
        enforcement_result: Some(AppGameEnforcementResult {
            schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
            result_id: APP_GAME_TEST_ENFORCEMENT_RESULT_ID.to_string(),
            action_id: APP_GAME_TEST_ENFORCEMENT_ACTION_ID.to_string(),
            status: APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED.to_string(),
            adapter_result_code: APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED.to_string(),
            started_at: APP_GAME_TEST_TIMESTAMP.to_string(),
            completed_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
            rollback_token: None,
            rollback_state: APP_GAME_ENFORCEMENT_ROLLBACK_NOT_REQUIRED.to_string(),
            unavailable_reason: None,
            unavailable_status: None,
            failed_reason: None,
            next_check_at: None,
            capability,
        }),
        recorded_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn approval_request() -> AppGameControlApprovalRequest {
    AppGameControlApprovalRequest {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        request_id: APP_GAME_TEST_REQUEST_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        device: child_device_windows(),
        target: AppGamePolicyTarget {
            target_id: APP_GAME_TEST_TARGET_ID.to_string(),
            target_type: APP_GAME_POLICY_TARGET_TYPE_APP.to_string(),
            target_value: APP_GAME_TEST_TARGET_VALUE.to_string(),
        },
        requested_action: APP_GAME_POLICY_ACTION_BLOCK.to_string(),
        requested_mode: Some(APP_GAME_ENFORCEMENT_MODE_TERMINATE_PROCESS.to_string()),
        requested_setting_refs: vec![AppGameControlSettingReference {
            setting_id: APP_GAME_TEST_SETTING_ID.to_string(),
            writes_to: APP_GAME_TEST_SETTING_PATH.to_string(),
        }],
        evidence_references: vec![evidence_ref()],
        candidate: Some(AppGameControlApprovalCandidate {
            candidate_id: APP_GAME_TEST_CANDIDATE_ID.to_string(),
            candidate_kind: APP_GAME_APPROVAL_CANDIDATE_NEW_INVENTORY_APP.to_string(),
            candidate_source: APP_GAME_APPROVAL_CANDIDATE_SOURCE_INVENTORY.to_string(),
            detected_at: APP_GAME_TEST_TIMESTAMP.to_string(),
            evidence_references: vec![evidence_ref()],
        }),
        child_reason_state: APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED.to_string(),
        child_reason_references: Vec::new(),
        child_status_references: Vec::new(),
        expires_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        unanswered_fallback: APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY.to_string(),
    }
}

fn approval_decision() -> AppGameControlApprovalDecision {
    AppGameControlApprovalDecision {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        decision_id: APP_GAME_TEST_DECISION_ID.to_string(),
        request_id: APP_GAME_TEST_REQUEST_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        decision_state: APP_GAME_CONTROL_DECISION_APPROVED.to_string(),
        parent_action: Some(AppGameParentActionReference {
            action_reference_id: APP_GAME_TEST_ACTION_REFERENCE_ID.to_string(),
            actor: parent_actor(),
            policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
            created_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }),
        reason_codes: vec![APP_GAME_TEST_REASON_PARENT_APPROVED.to_string()],
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![evidence_ref()],
        response_scope: Some(APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE.to_string()),
        decision_expires_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        audit_references: vec![APP_GAME_TEST_ACTION_REFERENCE_ID.to_string()],
        persistence_state: APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE.to_string(),
        decided_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn android_hide_row() -> AppGamePlatformAuthorityRow {
    AppGamePlatformAuthorityRow {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        row_id: APP_GAME_TEST_ANDROID_ROW_ID.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_ANDROID.to_string(),
        action: APP_GAME_PLATFORM_ACTION_HIDE_APP.to_string(),
        authority_tier: APP_GAME_PLATFORM_TIER_DEVICE_OWNER.to_string(),
        setup_state: APP_GAME_PLATFORM_SETUP_DEVICE_OWNER_REQUIRED.to_string(),
        proof_state: APP_GAME_PLATFORM_PROOF_STATE_RUNTIME_ATTACHED.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string(),
        parent_visible_state: APP_GAME_PLATFORM_PARENT_VISIBLE_MANAGED_DEVICE_REQUIRED.to_string(),
        parent_visible_limitation: APP_GAME_TEST_ANDROID_LIMITATION.to_string(),
        can_execute_adapter: true,
        supported_modes: vec![APP_GAME_ENFORCEMENT_MODE_BLOCK_PROCESS.to_string()],
        proof_references: vec![proof_device_owner(), proof_rollback()],
        proof_needed_to_claim: vec![
            APP_GAME_PLATFORM_PROOF_KIND_DEVICE_OWNER.to_string(),
            APP_GAME_PLATFORM_PROOF_KIND_ROLLBACK.to_string(),
        ],
        linux_mechanism: None,
        linux_distro: None,
        linux_session: None,
        last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn windows_manual_block_row() -> AppGamePlatformAuthorityRow {
    AppGamePlatformAuthorityRow {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        row_id: APP_GAME_TEST_WINDOWS_ROW_ID.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        action: APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH.to_string(),
        authority_tier: APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED.to_string(),
        setup_state: APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED.to_string(),
        proof_state: APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        parent_visible_state: APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED.to_string(),
        parent_visible_limitation: APP_GAME_TEST_WINDOWS_LIMITATION.to_string(),
        can_execute_adapter: false,
        supported_modes: Vec::new(),
        proof_references: Vec::new(),
        proof_needed_to_claim: vec![
            APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER.to_string(),
            APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APP_CONTROL.to_string(),
            APP_GAME_PLATFORM_PROOF_KIND_ROLLBACK.to_string(),
        ],
        linux_mechanism: None,
        linux_distro: None,
        linux_session: None,
        last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn classifier_result_candidate() -> AppGameAiClassifierResult {
    AppGameAiClassifierResult {
        schema_version: APP_GAME_SCHEMA_VERSION,
        classifier_run_id: APP_GAME_TEST_CLASSIFIER_RUN_ID.to_string(),
        product_kind: APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_GAME.to_string(),
        digest_ref: APP_GAME_TEST_CLASSIFIER_DIGEST_REF.to_string(),
        source_digest_kind: APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY.to_string(),
        source_evidence_refs: vec![APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF.to_string()],
        source_session_refs: vec![APP_GAME_TEST_CLASSIFIER_SESSION_REF.to_string()],
        candidate_kind: APP_GAME_AI_CLASSIFIER_CANDIDATE_GAME_CONTEXT.to_string(),
        candidate_label: APP_GAME_TEST_CLASSIFIER_LABEL.to_string(),
        classifier_state: APP_GAME_AI_CLASSIFIER_STATE_CANDIDATE.to_string(),
        confidence: 0.64,
        uncertainty_reason_codes: vec![APP_GAME_TEST_CLASSIFIER_REASON_CODE.to_string()],
        model_runtime_ref: APP_GAME_TEST_CLASSIFIER_RUNTIME_REF.to_string(),
        prompt_template_ref: APP_GAME_TEST_CLASSIFIER_PROMPT_REF.to_string(),
        prompt_version: APP_GAME_TEST_CLASSIFIER_PROMPT_REF.to_string(),
        fallback_state: APP_GAME_AI_CLASSIFIER_FALLBACK_NOT_NEEDED.to_string(),
        policy_handoff: APP_GAME_AI_CLASSIFIER_HANDOFF_PARENT_REVIEW.to_string(),
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        direct_action_requested: false,
        raw_scan_included: false,
        content_claim_included: false,
    }
}

fn classifier_result_unavailable() -> AppGameAiClassifierResult {
    AppGameAiClassifierResult {
        schema_version: APP_GAME_SCHEMA_VERSION,
        classifier_run_id: APP_GAME_TEST_CLASSIFIER_RUN_ID.to_string(),
        product_kind: APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP.to_string(),
        digest_ref: APP_GAME_TEST_CLASSIFIER_DIGEST_REF.to_string(),
        source_digest_kind: APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY.to_string(),
        source_evidence_refs: vec![APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF.to_string()],
        source_session_refs: vec![APP_GAME_TEST_CLASSIFIER_SESSION_REF.to_string()],
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
    }
}

fn supported_capability() -> AppGameEnforcementCapabilityStatus {
    AppGameEnforcementCapabilityStatus {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        adapter_kind: APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string(),
        permission_state: APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED.to_string(),
        dependency_state: APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED.to_string(),
        supported_actions: vec![APP_GAME_ENFORCEMENT_MODE_TERMINATE_PROCESS.to_string()],
        degraded_reason: None,
        last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn parent_actor() -> AppGameParentActorReference {
    AppGameParentActorReference {
        actor_id: APP_GAME_TEST_PARENT_ACTOR_ID.to_string(),
        role: APP_GAME_PARENT_ACTOR_ROLE_PARENT.to_string(),
    }
}

fn child_device_windows() -> AppGameParentDeviceReference {
    AppGameParentDeviceReference {
        device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
        child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
        label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
    }
}

fn evidence_ref() -> AppGameParentEvidenceReference {
    AppGameParentEvidenceReference {
        evidence_reference_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn proof_device_owner() -> AppGamePlatformProofReference {
    AppGamePlatformProofReference {
        proof_kind: APP_GAME_PLATFORM_PROOF_KIND_DEVICE_OWNER.to_string(),
        artifact_ref: APP_GAME_TEST_DEVICE_OWNER_PROOF_REF.to_string(),
    }
}

fn proof_rollback() -> AppGamePlatformProofReference {
    AppGamePlatformProofReference {
        proof_kind: APP_GAME_PLATFORM_PROOF_KIND_ROLLBACK.to_string(),
        artifact_ref: APP_GAME_TEST_ROLLBACK_PROOF_REF.to_string(),
    }
}
