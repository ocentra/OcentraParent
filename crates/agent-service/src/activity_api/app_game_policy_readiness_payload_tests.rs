use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, AppGameControlActionResult,
    AppGameControlApprovalAuthority, AppGameControlApprovalDecision, AppGameControlApprovalRequest,
    AppGameEnforcementCapabilityStatus, AppGameEnforcementResult, AppGameEvidenceClaim,
    AppGameIdentity, AppGameInventoryCategoryCandidate, AppGameInventoryEvidenceRow,
    AppGameParentActionReference, AppGameParentActorReference, AppGameParentDeviceReference,
    AppGameParentEvidenceReference, AppGamePlatformAuthorityMatrix, AppGamePlatformAuthorityRow,
    AppGamePolicyReadinessReadModel, AppGamePolicyReadinessRow, AppGamePolicyTarget,
    AppGameServiceReadModel, LogFieldValue, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_CONTROL_ACTION_STATUS_ENFORCED, APP_GAME_CONTROL_APPROVAL_STATE_APPROVED,
    APP_GAME_CONTROL_AUTHORITY_ACTIVE, APP_GAME_CONTROL_DECISION_APPROVED,
    APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY, APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE,
    APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE, APP_GAME_CONTROL_POLICY_KIND_APP,
    APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY, APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL,
    APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED,
    APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED, APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED,
    APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED, APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED,
    APP_GAME_ENFORCEMENT_ROLLBACK_NOT_REQUIRED, APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC,
    APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED, APP_GAME_INVENTORY_CATEGORY_GAME,
    APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT, APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD,
    APP_GAME_INVENTORY_SOURCE_UNKNOWN, APP_GAME_INVENTORY_STATE_DETECTABLE,
    APP_GAME_INVENTORY_STATE_INSTALLED, APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE,
    APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED, APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN,
    APP_GAME_PARENT_ACTOR_ROLE_PARENT, APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
    APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT, APP_GAME_PARENT_PLATFORM_WINDOWS,
    APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH, APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED,
    APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER, APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED,
    APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED, APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED,
    APP_GAME_POLICY_ACTION_BLOCK, APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT,
    APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT,
    APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE,
    APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY,
    APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE, APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW,
    APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED, APP_GAME_POLICY_READINESS_STATE_READY,
    APP_GAME_POLICY_READINESS_STATUS_READY, APP_GAME_POLICY_TARGET_TYPE_APP,
    APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_ACTION_REFERENCE_ID, APP_GAME_TEST_ACTION_RESULT_ID,
    APP_GAME_TEST_AUTHORITY_ID, APP_GAME_TEST_CATALOG_REF, APP_GAME_TEST_CHILD_PROFILE_ID,
    APP_GAME_TEST_DECISION_ID, APP_GAME_TEST_DEVICE_ID, APP_GAME_TEST_DEVICE_LABEL,
    APP_GAME_TEST_DISPLAY_LABEL, APP_GAME_TEST_ENFORCEMENT_ACTION_ID,
    APP_GAME_TEST_ENFORCEMENT_RESULT_ID, APP_GAME_TEST_EVIDENCE_CLAIM_ID,
    APP_GAME_TEST_EVIDENCE_REF_ID, APP_GAME_TEST_IDENTITY_ID, APP_GAME_TEST_PARENT_ACTOR_ID,
    APP_GAME_TEST_PLATFORM_MATRIX_ID, APP_GAME_TEST_POLICY_VERSION,
    APP_GAME_TEST_REASON_PARENT_APPROVED, APP_GAME_TEST_REQUEST_ID, APP_GAME_TEST_TARGET_ID,
    APP_GAME_TEST_TARGET_VALUE, APP_GAME_TEST_TIMESTAMP, APP_GAME_TEST_UNKNOWN_SOURCE_REF,
    APP_GAME_TEST_WINDOWS_LIMITATION, APP_GAME_TEST_WINDOWS_ROW_ID,
};

use super::app_game_policy_readiness_payload::{
    app_game_policy_readiness_from_service_model, app_game_policy_readiness_payload,
};

#[test]
fn app_game_policy_readiness_payload_reports_service_counts_with_source_dispatch() {
    let read_model = app_game_policy_readiness_from_service_model(service_model());
    let payload = app_game_policy_readiness_payload(&read_model);
    let read_model_json = string_payload(
        &payload,
        constants::field::APP_GAME_POLICY_READINESS_READ_MODEL,
    );
    let decoded: AppGamePolicyReadinessReadModel =
        serde_json::from_str(read_model_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(decoded.returned, 7);
    assert_eq!(
        decoded.capability_status,
        APP_GAME_POLICY_READINESS_STATUS_READY
    );
    assert!(decoded.policy_evaluation_ready);
    assert!(decoded.category_routing_ready);
    assert!(decoded.unknown_review_required);
    assert!(decoded.manual_review_required);
    assert!(decoded.adapter_dispatch_claimed);
    assert_eq!(decoded.evidence_claim_row_count, 1);
    assert_eq!(decoded.identity_row_count, 1);
    assert_eq!(decoded.approval_authority_row_count, 1);
    assert_eq!(decoded.approval_action_result_row_count, 1);
    assert_eq!(decoded.platform_authority_row_count, 1);
    assert_eq!(decoded.category_candidate_row_count, 1);
    assert_eq!(decoded.unknown_review_row_count, 1);
    assert_policy_evidence_row(&decoded);
    assert_policy_authority_rows(&decoded);
    assert_category_and_unknown_rows(&decoded);
}

fn assert_policy_evidence_row(decoded: &AppGamePolicyReadinessReadModel) {
    assert_eq!(
        readiness_row(
            &decoded.rows,
            APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE
        )
        .readiness_state,
        APP_GAME_POLICY_READINESS_STATE_READY
    );
    assert_eq!(
        readiness_row(
            &decoded.rows,
            APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE
        )
        .evidence_reference_ids,
        vec![
            APP_GAME_TEST_EVIDENCE_REF_ID,
            APP_GAME_TEST_EVIDENCE_CLAIM_ID,
            APP_GAME_TEST_IDENTITY_ID
        ]
    );
}

fn assert_policy_authority_rows(decoded: &AppGamePolicyReadinessReadModel) {
    assert_eq!(
        readiness_row(
            &decoded.rows,
            APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY
        )
        .evidence_reference_ids,
        vec![APP_GAME_TEST_WINDOWS_ROW_ID]
    );
    assert_eq!(
        readiness_row(
            &decoded.rows,
            APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT
        )
        .readiness_state,
        APP_GAME_POLICY_READINESS_STATE_READY
    );
    assert_eq!(
        readiness_row(
            &decoded.rows,
            APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT
        )
        .readiness_state,
        APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED
    );
}

fn assert_category_and_unknown_rows(decoded: &AppGamePolicyReadinessReadModel) {
    assert_eq!(
        readiness_row(
            &decoded.rows,
            APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE
        )
        .evidence_reference_ids,
        vec![
            constants::value::APP_GAME_TEST_POLICY_READINESS_CATEGORY_EVIDENCE_ID,
            APP_GAME_TEST_CATALOG_REF
        ]
    );
    assert_eq!(
        readiness_row(&decoded.rows, APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW).readiness_state,
        APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED
    );
}

fn service_model() -> AppGameServiceReadModel {
    AppGameServiceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        custody_label: APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED.to_string(),
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
        ai_classifier_result_returned: 0,
        inventory_rows: vec![categorized_inventory_row(), unknown_inventory_row()],
        running_now_rows: Vec::new(),
        foreground_now_rows: Vec::new(),
        launcher_rows: Vec::new(),
        daily_rollups: Vec::new(),
        evidence_claim_rows: vec![evidence_claim()],
        identity_rows: vec![identity()],
        approval_authority_rows: vec![approval_authority()],
        approval_action_result_rows: vec![action_result()],
        platform_authority_matrices: vec![platform_matrix()],
        ai_classifier_result_rows: Vec::new(),
    }
}

fn action_result() -> AppGameControlActionResult {
    let capability = enforcement_capability();
    AppGameControlActionResult {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        result_id: APP_GAME_TEST_ACTION_RESULT_ID.to_string(),
        request: approval_request(),
        decision: approval_decision(),
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
        device: AppGameParentDeviceReference {
            device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
            child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
            label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        },
        target: AppGamePolicyTarget {
            target_id: APP_GAME_TEST_TARGET_ID.to_string(),
            target_type: APP_GAME_POLICY_TARGET_TYPE_APP.to_string(),
            target_value: APP_GAME_TEST_TARGET_VALUE.to_string(),
        },
        requested_action: APP_GAME_POLICY_ACTION_BLOCK.to_string(),
        requested_mode: None,
        requested_setting_refs: Vec::new(),
        evidence_references: vec![AppGameParentEvidenceReference {
            evidence_reference_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
            kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
            observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }],
        candidate: None,
        child_reason_state: APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY.to_string(),
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
            actor: AppGameParentActorReference {
                actor_id: APP_GAME_TEST_PARENT_ACTOR_ID.to_string(),
                role: APP_GAME_PARENT_ACTOR_ROLE_PARENT.to_string(),
            },
            policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
            created_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }),
        reason_codes: vec![APP_GAME_TEST_REASON_PARENT_APPROVED.to_string()],
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![AppGameParentEvidenceReference {
            evidence_reference_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
            kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
            observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }],
        response_scope: Some(APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE.to_string()),
        decision_expires_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        audit_references: vec![APP_GAME_TEST_EVIDENCE_REF_ID.to_string()],
        persistence_state: APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE.to_string(),
        decided_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn enforcement_capability() -> AppGameEnforcementCapabilityStatus {
    AppGameEnforcementCapabilityStatus {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        adapter_kind: APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string(),
        permission_state: APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED.to_string(),
        dependency_state: APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED.to_string(),
        supported_actions: vec![APP_GAME_POLICY_ACTION_BLOCK.to_string()],
        degraded_reason: None,
        last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn categorized_inventory_row() -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id:
            constants::value::APP_GAME_TEST_POLICY_READINESS_CATEGORY_INVENTORY_ENTRY_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD.to_string(),
        source_ref: constants::value::APP_GAME_TEST_POLICY_READINESS_CATEGORY_SOURCE_REF
            .to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
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
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.94,
        category_candidates: vec![AppGameInventoryCategoryCandidate {
            category_kind: APP_GAME_INVENTORY_CATEGORY_GAME.to_string(),
            confidence: 0.94,
            catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
            evidence: vec![local_db_ref(
                constants::value::APP_GAME_TEST_POLICY_READINESS_CATEGORY_EVIDENCE_ID,
            )],
        }],
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: vec![local_db_ref(
            constants::value::APP_GAME_TEST_POLICY_READINESS_CATEGORY_INVENTORY_ENTRY_ID,
        )],
    }
}

fn unknown_inventory_row() -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id:
            constants::value::APP_GAME_TEST_POLICY_READINESS_UNKNOWN_INVENTORY_ENTRY_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_UNKNOWN.to_string(),
        source_ref: APP_GAME_TEST_UNKNOWN_SOURCE_REF.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_APP.to_string(),
        display_label: constants::value::APP_GAME_TEST_POLICY_READINESS_UNKNOWN_DISPLAY_LABEL
            .to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: None,
        inventory_state: APP_GAME_INVENTORY_STATE_DETECTABLE.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED.to_string(),
        confidence: 0.22,
        category_candidates: Vec::new(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: vec![local_db_ref(
            constants::value::APP_GAME_TEST_POLICY_READINESS_UNKNOWN_EVIDENCE_ID,
        )],
    }
}

fn evidence_claim() -> AppGameEvidenceClaim {
    AppGameEvidenceClaim {
        schema_version: APP_GAME_SCHEMA_VERSION,
        claim_id: APP_GAME_TEST_EVIDENCE_CLAIM_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        claim_kind: APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN.to_string(),
        display_name: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_strength: APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        inventory_entry_id: None,
        process_identity: None,
        launcher_ref: None,
        catalog_ref: None,
        confidence: 1.0,
        evidence: vec![local_db_ref(APP_GAME_TEST_EVIDENCE_REF_ID)],
    }
}

fn identity() -> AppGameIdentity {
    AppGameIdentity {
        schema_version: APP_GAME_SCHEMA_VERSION,
        identity_id: APP_GAME_TEST_IDENTITY_ID.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
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
        catalog_ref: None,
        child_game_evidence_claim_id: None,
        evidence: Vec::new(),
    }
}

fn approval_authority() -> AppGameControlApprovalAuthority {
    AppGameControlApprovalAuthority {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        authority_id: APP_GAME_TEST_AUTHORITY_ID.to_string(),
        actor: AppGameParentActorReference {
            actor_id: APP_GAME_TEST_PARENT_ACTOR_ID.to_string(),
            role: APP_GAME_PARENT_ACTOR_ROLE_PARENT.to_string(),
        },
        device: AppGameParentDeviceReference {
            device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
            child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
            label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        },
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        authority_state: APP_GAME_CONTROL_AUTHORITY_ACTIVE.to_string(),
        allowed_policy_kinds: vec![APP_GAME_CONTROL_POLICY_KIND_APP.to_string()],
        can_approve: true,
        can_deny: true,
        can_extend: true,
        can_override: true,
        can_observe_only: false,
        checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn platform_matrix() -> AppGamePlatformAuthorityMatrix {
    AppGamePlatformAuthorityMatrix {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        matrix_id: APP_GAME_TEST_PLATFORM_MATRIX_ID.to_string(),
        rows: vec![AppGamePlatformAuthorityRow {
            schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
            row_id: APP_GAME_TEST_WINDOWS_ROW_ID.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
            action: APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH.to_string(),
            authority_tier: APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED.to_string(),
            setup_state: APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED.to_string(),
            proof_state: APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED.to_string(),
            capability_state:
                ocentra_parent_agent_protocol::APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED
                    .to_string(),
            parent_visible_state: APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED.to_string(),
            parent_visible_limitation: APP_GAME_TEST_WINDOWS_LIMITATION.to_string(),
            can_execute_adapter: false,
            supported_modes: Vec::new(),
            proof_references: Vec::new(),
            proof_needed_to_claim: vec![APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER.to_string()],
            linux_mechanism: None,
            linux_distro: None,
            linux_session: None,
            last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }],
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn local_db_ref(evidence_id: &str) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn readiness_row<'a>(
    rows: &'a [AppGamePolicyReadinessRow],
    readiness_kind: &str,
) -> &'a AppGamePolicyReadinessRow {
    rows.iter()
        .find(|row| row.readiness_kind == readiness_kind)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn string_payload<'a>(payload: &'a ocentra_parent_agent_protocol::LogFields, key: &str) -> &'a str {
    match payload.get(key) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
