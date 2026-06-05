use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, AppGameControlApprovalAuthority,
    AppGameEvidenceClaim, AppGameIdentity, AppGameParentActorReference,
    AppGameParentDeviceReference, AppGamePlatformAuthorityMatrix, AppGamePlatformAuthorityRow,
    AppGamePolicyEvaluationReadModel, AppGamePolicyEvaluationRow, AppGameServiceReadModel,
    LogFieldValue, APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CONTROL_AUTHORITY_ACTIVE,
    APP_GAME_CONTROL_POLICY_KIND_APP, APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC,
    APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED, APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE,
    APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED, APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN,
    APP_GAME_PARENT_ACTOR_ROLE_PARENT, APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
    APP_GAME_PARENT_PLATFORM_WINDOWS, APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH,
    APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED,
    APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER, APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED,
    APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED, APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED,
    APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED,
    APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY, APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED,
    APP_GAME_POLICY_EVALUATION_KIND_APPROVAL_REQUEST, APP_GAME_POLICY_EVALUATION_KIND_BLOCK_LAUNCH,
    APP_GAME_POLICY_EVALUATION_KIND_CATEGORY_RISK_REVIEW,
    APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT,
    APP_GAME_POLICY_EVALUATION_REJECTION_BLOCK_LAUNCH_MANUAL_REQUIRED,
    APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_CLASSIFIER_CONTEXT,
    APP_GAME_POLICY_EVALUATION_REJECTION_NONE, APP_GAME_POLICY_EVALUATION_STATUS_READY,
    APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION,
    APP_GAME_TEST_AUTHORITY_ID, APP_GAME_TEST_CHILD_PROFILE_ID, APP_GAME_TEST_DEVICE_ID,
    APP_GAME_TEST_DEVICE_LABEL, APP_GAME_TEST_DISPLAY_LABEL, APP_GAME_TEST_EVIDENCE_CLAIM_ID,
    APP_GAME_TEST_EVIDENCE_REF_ID, APP_GAME_TEST_IDENTITY_ID, APP_GAME_TEST_PARENT_ACTOR_ID,
    APP_GAME_TEST_PLATFORM_MATRIX_ID, APP_GAME_TEST_POLICY_VERSION, APP_GAME_TEST_TIMESTAMP,
    APP_GAME_TEST_WINDOWS_LIMITATION, APP_GAME_TEST_WINDOWS_ROW_ID,
};

use super::app_game_policy_evaluation_payload::{
    app_game_policy_evaluation_from_service_model, app_game_policy_evaluation_payload,
};

#[test]
fn app_game_policy_evaluation_payload_reports_dry_run_rows_without_adapter_dispatch() {
    let read_model = app_game_policy_evaluation_from_service_model(service_model());
    let payload = app_game_policy_evaluation_payload(&read_model);
    let read_model_json = string_payload(
        &payload,
        constants::field::APP_GAME_POLICY_EVALUATION_READ_MODEL,
    );
    let decoded: AppGamePolicyEvaluationReadModel =
        serde_json::from_str(read_model_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(decoded.returned, 4);
    assert_eq!(
        decoded.capability_status,
        APP_GAME_POLICY_EVALUATION_STATUS_READY
    );
    assert!(decoded.policy_evaluation_ready);
    assert!(decoded.manual_review_required);
    assert!(decoded.dry_run);
    assert_eq!(
        decoded.enforcement_handoff_state,
        APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED
    );
    assert!(!decoded.adapter_dispatch_claimed);
    assert_eq!(decoded.readiness_row_count, 5);
    assert_eq!(decoded.evaluated_row_count, 4);
    assert_eq!(decoded.evidence_claim_row_count, 1);
    assert_eq!(decoded.identity_row_count, 1);
    assert_eq!(decoded.approval_authority_row_count, 1);
    assert_eq!(decoded.platform_authority_row_count, 1);
    assert_eq!(
        evaluation_row(&decoded.rows, APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT).decision_state,
        APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY
    );
    assert_eq!(
        evaluation_row(&decoded.rows, APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT).rejection_reason,
        APP_GAME_POLICY_EVALUATION_REJECTION_NONE
    );
    assert_eq!(
        evaluation_row(
            &decoded.rows,
            APP_GAME_POLICY_EVALUATION_KIND_APPROVAL_REQUEST
        )
        .decision_state,
        APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY
    );
    assert_eq!(
        evaluation_row(
            &decoded.rows,
            APP_GAME_POLICY_EVALUATION_KIND_CATEGORY_RISK_REVIEW
        )
        .rejection_reason,
        APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_CLASSIFIER_CONTEXT
    );
    assert_eq!(
        evaluation_row(&decoded.rows, APP_GAME_POLICY_EVALUATION_KIND_BLOCK_LAUNCH)
            .rejection_reason,
        APP_GAME_POLICY_EVALUATION_REJECTION_BLOCK_LAUNCH_MANUAL_REQUIRED
    );
    assert!(decoded.rows.iter().all(|row| row.dry_run
        && row.adapter_dispatch_state == APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED));
}

fn service_model() -> AppGameServiceReadModel {
    AppGameServiceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        custody_label: APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED.to_string(),
        inventory_returned: 0,
        running_now_returned: 0,
        foreground_now_returned: 0,
        launcher_returned: 0,
        daily_rollup_returned: 0,
        evidence_claim_returned: 1,
        identity_returned: 1,
        approval_authority_returned: 1,
        approval_action_result_returned: 0,
        platform_authority_matrix_returned: 1,
        ai_classifier_result_returned: 0,
        inventory_rows: Vec::new(),
        running_now_rows: Vec::new(),
        foreground_now_rows: Vec::new(),
        launcher_rows: Vec::new(),
        daily_rollups: Vec::new(),
        evidence_claim_rows: vec![evidence_claim()],
        identity_rows: vec![identity()],
        approval_authority_rows: vec![approval_authority()],
        approval_action_result_rows: Vec::new(),
        platform_authority_matrices: vec![platform_matrix()],
        ai_classifier_result_rows: Vec::new(),
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

fn evaluation_row<'a>(
    rows: &'a [AppGamePolicyEvaluationRow],
    evaluation_kind: &str,
) -> &'a AppGamePolicyEvaluationRow {
    rows.iter()
        .find(|row| row.evaluation_kind == evaluation_kind)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn string_payload<'a>(payload: &'a ocentra_parent_agent_protocol::LogFields, key: &str) -> &'a str {
    match payload.get(key) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
