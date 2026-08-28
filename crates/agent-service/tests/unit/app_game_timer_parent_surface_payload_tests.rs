use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, AppGameIdentity, AppGameServiceReadModel,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE, APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED,
    APP_GAME_PRODUCT_LAUNCHER, APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE,
    APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_DISPLAY_LABEL,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameControlActionResult, AppGameControlApprovalAuthority, AppGameControlApprovalDecision,
    AppGameControlApprovalRequest, AppGameEnforcementCapabilityStatus, AppGameParentActorReference,
    AppGameParentDeviceReference, AppGameParentEvidenceReference, AppGamePlatformAuthorityMatrix,
    AppGamePolicyTarget, APP_GAME_CONTROL_ACTION_STATUS_ENFORCED,
    APP_GAME_CONTROL_AUTHORITY_ACTIVE, APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED,
    APP_GAME_CONTROL_DECISION_APPROVED, APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY,
    APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE, APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE,
    APP_GAME_CONTROL_POLICY_KIND_APP, APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY,
    APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL, APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED,
    APP_GAME_PARENT_ACTOR_ROLE_PARENT, APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
    APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT, APP_GAME_PARENT_PLATFORM_WINDOWS,
    APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH, APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED,
};
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::{
    AppGameTimerParentSurfaceReadModel,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS, APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};
use ocentra_parent_agent_protocol::constants;

use crate::test_invariants::{require_json_decode, require_log_string_field, require_ok};

use super::app_game_timer_parent_surface_payload::{
    app_game_timer_parent_surface_from_service_model_with_timer_state,
    app_game_timer_parent_surface_payload,
};

const APP_GAME_CONTROL_APPROVAL_STATE_APPROVED: &str = "approved";
const APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED: &str = "process-terminated";
const APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED: &str = "supported";
const APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY: &str = "inventory";
const APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC: &str = "deterministic";
const APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED: &str = "catalogMatched";
const APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN: &str = "inventoryScan";
const APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED: &str = "manual-required";
const APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER: &str = "windows-applocker-proof";
const APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED: &str = "manual-required";
const APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED: &str = "manual-required";
const APP_GAME_TEST_ACTION_REFERENCE_ID: &str = "parent-action-app-game-1";
const APP_GAME_TEST_ACTION_RESULT_ID: &str = "action-result-app-game-1";
const APP_GAME_TEST_AUTHORITY_ID: &str = "authority-app-game-1";
const APP_GAME_TEST_CHILD_PROFILE_ID: &str = "child-app-game-1";
const APP_GAME_TEST_DEVICE_ID: &str = "device-windows-app-game-1";
const APP_GAME_TEST_DEVICE_LABEL: &str = "Study PC";
const APP_GAME_TEST_ENFORCEMENT_ACTION_ID: &str = "enforcement-action-app-game-1";
const APP_GAME_TEST_ENFORCEMENT_RESULT_ID: &str = "enforcement-result-app-game-1";
const APP_GAME_TEST_EVIDENCE_CLAIM_ID: &str = "claim-ocentra-inventory";
const APP_GAME_TEST_EVIDENCE_REF_ID: &str = "evidence-app-game-session-1";
const APP_GAME_TEST_IDENTITY_ID: &str = "identity-ocentra-game";
const APP_GAME_TEST_PARENT_ACTOR_ID: &str = "parent-actor-app-game-1";
const APP_GAME_TEST_PLATFORM_MATRIX_ID: &str = "app-game-platform-authority-matrix";
const APP_GAME_TEST_POLICY_VERSION: &str = "policy-version-app-game-1";
const APP_GAME_TEST_REASON_PARENT_APPROVED: &str = "parent-approved";
const APP_GAME_TEST_TARGET_ID: &str = "target-app-game-1";
const APP_GAME_TEST_TARGET_VALUE: &str = "process:ocentra-game.exe";
const APP_GAME_TEST_TIMESTAMP: &str = "2026-06-03T22:15:00Z";
const APP_GAME_TEST_WINDOWS_LIMITATION: &str =
    "Broad installed-app blocking needs AppLocker or App Control proof before execution.";
const APP_GAME_TEST_WINDOWS_ROW_ID: &str = "windows-block-launch-row";

#[test]
fn app_game_timer_parent_surface_payload_reports_game_rows_without_runtime_claims() {
    let model = service_model();
    let read_model =
        app_game_timer_parent_surface_from_service_model_with_timer_state(&model, None);
    let payload = app_game_timer_parent_surface_payload(&read_model);
    let read_model_json = require_log_string_field(
        payload.get(constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let decoded = require_json_decode::<AppGameTimerParentSurfaceReadModel>(
        read_model_json,
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_parent_surface_counts_and_claim_boundaries(&decoded);
    assert_control_action_result_visibility(&decoded);
    assert_child_ux_local_artifact_visibility(&decoded);
    assert_eq!(
        decoded.rows[0].target_domain,
        APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME
    );
    assert_eq!(
        decoded.rows[0].timer_surface_state,
        APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE
    );
    assert_eq!(
        decoded.rows[0].evidence_reference_ids,
        vec![
            APP_GAME_TEST_IDENTITY_ID,
            APP_GAME_TEST_EVIDENCE_REF_ID,
            APP_GAME_TEST_EVIDENCE_CLAIM_ID,
            APP_GAME_TEST_WINDOWS_ROW_ID,
            APP_GAME_TEST_AUTHORITY_ID
        ]
    );
}

#[test]
fn app_game_timer_parent_surface_payload_fails_closed_without_source_rows() {
    let mut model = service_model();
    model.evidence_claim_rows.clear();
    model.identity_rows.clear();
    model.approval_authority_rows.clear();
    model.approval_action_result_rows.clear();
    model.platform_authority_matrices.clear();

    let read_model =
        app_game_timer_parent_surface_from_service_model_with_timer_state(&model, None);
    let payload = app_game_timer_parent_surface_payload(&read_model);
    let read_model_json = require_log_string_field(
        payload.get(constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let decoded = require_json_decode::<AppGameTimerParentSurfaceReadModel>(
        read_model_json,
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(decoded.schema_version, APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        decoded.capability_status,
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS
    );
    assert_eq!(decoded.returned, 0);
    assert_eq!(decoded.ready_for_parent_surface_count, 0);
    assert_eq!(decoded.blocked_by_source_freshness_count, 0);
    assert_eq!(decoded.blocked_by_compiler_decision_count, 0);
    assert_eq!(decoded.runtime_manual_required_count, 0);
    assert!(decoded.rows.is_empty());
    assert_eq!(decoded.control_action_result_count, 0);
    assert!(decoded.control_action_result_reference_ids.is_empty());
    assert!(decoded.child_ux_handoff_reference_ids.is_empty());
    assert_eq!(decoded.child_ux_handoff_ready_count, 0);
    assert_eq!(decoded.child_ux_handoff_blocked_count, 0);
    assert!(!decoded.timer_runtime_claimed);
    assert!(!decoded.scheduler_persistence_claimed);
    assert!(!decoded.durable_scheduler_storage_claimed);
    assert!(!decoded.audit_runtime_claimed);
    assert!(!decoded.rollback_runtime_claimed);
    assert!(!decoded.adapter_dispatch_claimed);
    assert!(!decoded.child_delivery_claimed);
    assert!(!decoded.platform_enforcement_claimed);
    assert!(!decoded.raw_private_source_rows_included);
}

#[test]
fn app_game_timer_parent_surface_payload_omits_unsupported_identity_rows() {
    for product_kind in [
        APP_GAME_PRODUCT_LAUNCHER,
        APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE,
        "malformed-product-kind",
    ] {
        let mut model = service_model();
        model.identity_rows[0].product_kind = product_kind.to_string();
        model.approval_action_result_rows.clear();

        let read_model =
            app_game_timer_parent_surface_from_service_model_with_timer_state(&model, None);

        assert_eq!(
            read_model.capability_status,
            APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS
        );
        assert_eq!(read_model.returned, 0);
        assert!(read_model.rows.is_empty());
        assert_eq!(read_model.control_action_result_count, 0);
        assert!(!read_model.adapter_dispatch_claimed);
        assert!(!read_model.platform_enforcement_claimed);
        assert!(!read_model.raw_private_source_rows_included);
    }
}

#[test]
fn app_game_timer_parent_surface_action_results_project_parent_visible_handoffs() {
    let results = super::app_game_timer_parent_surface_action_results::timer_parent_surface_control_action_results(
        &service_model(),
    );

    assert_eq!(
        results.reference_ids,
        vec![APP_GAME_TEST_ACTION_RESULT_ID.to_string()]
    );
    assert_eq!(
        results.statuses,
        vec![APP_GAME_CONTROL_ACTION_STATUS_ENFORCED.to_string()]
    );
    assert_eq!(
        results.capability_states,
        vec![APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string()]
    );
    assert_eq!(
        results.enforcement_statuses,
        vec![APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED.to_string()]
    );
    assert_eq!(
        results.child_reason_reference_ids,
        vec![APP_GAME_TEST_REASON_PARENT_APPROVED.to_string()]
    );
    assert_eq!(
        results.child_status_reference_ids,
        vec![APP_GAME_TEST_ACTION_REFERENCE_ID.to_string()]
    );
    assert_eq!(results.child_ux_handoff_ready_count, 1);
    assert_eq!(results.child_ux_handoff_blocked_count, 0);
    assert!(results.adapter_dispatch_claimed);
    assert!(results.platform_enforcement_claimed);
}

fn assert_parent_surface_counts_and_claim_boundaries(decoded: &AppGameTimerParentSurfaceReadModel) {
    assert_eq!(decoded.returned, 1);
    assert_eq!(
        decoded.capability_status,
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY
    );
    assert_eq!(decoded.ready_for_parent_surface_count, 1);
    assert_eq!(decoded.blocked_by_source_freshness_count, 0);
    assert_eq!(decoded.blocked_by_compiler_decision_count, 0);
    assert_eq!(decoded.runtime_manual_required_count, 0);
    assert!(!decoded.timer_runtime_claimed);
    assert!(!decoded.scheduler_persistence_claimed);
    assert!(!decoded.durable_scheduler_storage_claimed);
    assert!(!decoded.audit_runtime_claimed);
    assert!(!decoded.rollback_runtime_claimed);
    assert!(decoded.adapter_dispatch_claimed);
    assert!(!decoded.child_delivery_claimed);
    assert!(decoded.platform_enforcement_claimed);
    assert!(!decoded.raw_private_source_rows_included);
}

fn assert_control_action_result_visibility(decoded: &AppGameTimerParentSurfaceReadModel) {
    assert_eq!(decoded.control_action_result_count, 1);
    assert_eq!(
        decoded.control_action_result_reference_ids,
        vec![APP_GAME_TEST_ACTION_RESULT_ID]
    );
    assert_eq!(
        decoded.control_action_result_statuses,
        vec![APP_GAME_CONTROL_ACTION_STATUS_ENFORCED]
    );
    assert_eq!(
        decoded.control_action_result_capability_states,
        vec![APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED]
    );
    assert_eq!(
        decoded.control_action_result_enforcement_statuses,
        vec![APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED]
    );
}

fn assert_child_ux_local_artifact_visibility(decoded: &AppGameTimerParentSurfaceReadModel) {
    assert_eq!(
        decoded.child_facing_reason_reference_ids,
        vec![APP_GAME_TEST_REASON_PARENT_APPROVED.to_string()]
    );
    assert_eq!(
        decoded.child_facing_status_reference_ids,
        vec![APP_GAME_TEST_ACTION_REFERENCE_ID.to_string()]
    );
    assert_eq!(decoded.child_ux_handoff_ready_count, 1);
    assert_eq!(decoded.child_ux_handoff_blocked_count, 0);
    assert_eq!(
        decoded.child_ux_handoff_reference_ids,
        vec![APP_GAME_TEST_ACTION_RESULT_ID.to_string()]
    );
    assert_eq!(decoded.child_ux_local_handoff_artifact_record_count, 1);
    assert_eq!(decoded.child_ux_local_handoff_artifact_skipped_count, 0);
    assert_eq!(
        decoded.child_ux_local_handoff_artifact_reference_ids,
        vec![[
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TEST_ACTION_RESULT_ID
        ]
        .concat()]
    );
    assert_eq!(decoded.child_ux_local_handoff_artifact_records.len(), 1);
    assert_child_ux_parent_surface_intent_visibility(decoded);
    assert_child_ux_parent_preference_setup_visibility(decoded);
    assert_child_ux_local_artifact_record_visibility(decoded);
}

fn assert_child_ux_parent_surface_intent_visibility(decoded: &AppGameTimerParentSurfaceReadModel) {
    assert_eq!(
        decoded.child_ux_parent_surface_intent_manual_action_required_count,
        1
    );
    assert_eq!(
        decoded.child_ux_parent_surface_intent_unavailable_visible_count,
        0
    );
    assert_eq!(
        decoded.child_ux_parent_surface_intent_history_visible_count,
        1
    );
    assert_eq!(
        decoded.child_ux_parent_surface_intent_preference_setup_required_count,
        1
    );
    assert_eq!(
        decoded.child_ux_parent_surface_intent_reference_ids,
        vec![[
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX,
            APP_GAME_TEST_ACTION_RESULT_ID
        ]
        .concat()]
    );
    assert_eq!(decoded.child_ux_parent_surface_intent_records.len(), 1);
    let parent_surface_record = &decoded.child_ux_parent_surface_intent_records[0];
    assert_eq!(
        parent_surface_record.parent_surface_intent_reference_id,
        [
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX,
            APP_GAME_TEST_ACTION_RESULT_ID
        ]
        .concat()
    );
    assert_eq!(
        parent_surface_record.source_artifact_reference_id,
        [
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TEST_ACTION_RESULT_ID
        ]
        .concat()
    );
    assert!(!parent_surface_record.parent_notification_ui_rendered);
    assert!(!parent_surface_record.parent_preference_mutation_claimed);
    assert!(!parent_surface_record.provider_delivery_claimed);
    assert!(!parent_surface_record.child_delivery_claimed);
    assert!(parent_surface_record.adapter_dispatch_claimed);
    assert!(parent_surface_record.platform_enforcement_claimed);
    assert!(!parent_surface_record.raw_private_source_rows_included);
}

fn assert_child_ux_parent_preference_setup_visibility(
    decoded: &AppGameTimerParentSurfaceReadModel,
) {
    assert_eq!(
        decoded.child_ux_parent_preference_setup_draft_ready_count,
        1
    );
    assert_eq!(
        decoded.child_ux_parent_preference_setup_unavailable_visible_count,
        0
    );
    assert_eq!(
        decoded.child_ux_parent_preference_setup_reference_ids,
        vec![[
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX,
            APP_GAME_TEST_ACTION_RESULT_ID
        ]
        .concat()]
    );
    assert_eq!(decoded.child_ux_parent_preference_setup_records.len(), 1);
    let setup_record = &decoded.child_ux_parent_preference_setup_records[0];
    assert_eq!(
        setup_record.parent_preference_setup_reference_id,
        [
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX,
            APP_GAME_TEST_ACTION_RESULT_ID
        ]
        .concat()
    );
    assert_eq!(
        setup_record.source_parent_surface_intent_reference_id,
        [
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX,
            APP_GAME_TEST_ACTION_RESULT_ID
        ]
        .concat()
    );
    assert_eq!(
        setup_record.draft_status,
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_DRAFT_READY
    );
    assert!(!setup_record.parent_preference_ui_rendered);
    assert!(!setup_record.parent_frequency_control_ui_rendered);
    assert!(!setup_record.parent_preference_mutation_claimed);
    assert!(!setup_record.notification_rule_mutation_claimed);
    assert!(!setup_record.provider_delivery_claimed);
    assert!(!setup_record.child_delivery_claimed);
    assert!(setup_record.adapter_dispatch_claimed);
    assert!(setup_record.platform_enforcement_claimed);
    assert!(!setup_record.raw_private_source_rows_included);
}

fn assert_child_ux_local_artifact_record_visibility(decoded: &AppGameTimerParentSurfaceReadModel) {
    let artifact_record = &decoded.child_ux_local_handoff_artifact_records[0];
    assert_eq!(
        artifact_record.source_result_id,
        APP_GAME_TEST_ACTION_RESULT_ID
    );
    assert_eq!(
        artifact_record.target_domain,
        APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP
    );
    assert_eq!(
        artifact_record.child_reason_reference_ids,
        vec![APP_GAME_TEST_REASON_PARENT_APPROVED.to_string()]
    );
    assert_eq!(
        artifact_record.child_status_reference_ids,
        vec![APP_GAME_TEST_ACTION_REFERENCE_ID.to_string()]
    );
    assert!(!artifact_record.child_delivery_claimed);
    assert!(!artifact_record.notification_delivery_claimed);
    assert!(artifact_record.adapter_dispatch_claimed);
    assert!(artifact_record.platform_enforcement_claimed);
    assert!(!artifact_record.raw_private_source_rows_included);
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
        approval_action_result_returned: 1,
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
        approval_action_result_rows: vec![control_action_result()],
        platform_authority_matrices: vec![platform_matrix()],
        ai_classifier_result_rows: Vec::new(),
    }
}

fn control_action_result() -> AppGameControlActionResult {
    let capability = supported_capability();
    require_ok(
        serde_json::from_value(serde_json::json!({
            "schemaVersion": APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
            "resultId": APP_GAME_TEST_ACTION_RESULT_ID,
            "request": approval_request(),
            "decision": approval_decision(),
            "approvalState": APP_GAME_CONTROL_APPROVAL_STATE_APPROVED,
            "capabilityState": APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED,
            "capability": &capability,
            "evidenceProofKind": APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY,
            "resultStatus": APP_GAME_CONTROL_ACTION_STATUS_ENFORCED,
            "enforcementResult": {
                "schemaVersion": APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
                "resultId": APP_GAME_TEST_ENFORCEMENT_RESULT_ID,
                "actionId": APP_GAME_TEST_ENFORCEMENT_ACTION_ID,
                "status": APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED,
                "adapterResultCode": APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED,
                "startedAt": APP_GAME_TEST_TIMESTAMP,
                "completedAt": APP_GAME_TEST_TIMESTAMP,
                "rollbackToken": null,
                "rollbackState": "not-required",
                "unavailableReason": null,
                "unavailableStatus": null,
                "failedReason": null,
                "nextCheckAt": null,
                "capability": capability
            },
            "recordedAt": APP_GAME_TEST_TIMESTAMP
        })),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn approval_request() -> AppGameControlApprovalRequest {
    AppGameControlApprovalRequest {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        request_id: APP_GAME_TEST_AUTHORITY_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        device: AppGameParentDeviceReference {
            device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
            child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
            label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        },
        target: AppGamePolicyTarget {
            target_id: APP_GAME_TEST_TARGET_ID.to_string(),
            target_type: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
            target_value: APP_GAME_TEST_TARGET_VALUE.to_string(),
        },
        requested_action: APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH.to_string(),
        requested_mode: None,
        requested_setting_refs: Vec::new(),
        evidence_references: vec![parent_evidence_ref()],
        candidate: None,
        child_reason_state: APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED.to_string(),
        child_reason_references: vec![APP_GAME_TEST_REASON_PARENT_APPROVED.to_string()],
        child_status_references: vec![APP_GAME_TEST_ACTION_REFERENCE_ID.to_string()],
        expires_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        unanswered_fallback: APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY.to_string(),
    }
}

fn approval_decision() -> AppGameControlApprovalDecision {
    AppGameControlApprovalDecision {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        decision_id: APP_GAME_TEST_ACTION_REFERENCE_ID.to_string(),
        request_id: APP_GAME_TEST_AUTHORITY_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        decision_state: APP_GAME_CONTROL_DECISION_APPROVED.to_string(),
        parent_action: None,
        reason_codes: vec![APP_GAME_TEST_REASON_PARENT_APPROVED.to_string()],
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![parent_evidence_ref()],
        response_scope: Some(APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE.to_string()),
        decision_expires_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        audit_references: Vec::new(),
        persistence_state: APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE.to_string(),
        decided_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn supported_capability() -> AppGameEnforcementCapabilityStatus {
    AppGameEnforcementCapabilityStatus {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        adapter_kind: APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string(),
        permission_state: APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string(),
        dependency_state: APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED.to_string(),
        supported_actions: vec![APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH.to_string()],
        degraded_reason: None,
        last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn parent_evidence_ref() -> AppGameParentEvidenceReference {
    AppGameParentEvidenceReference {
        evidence_reference_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
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

fn parent_device() -> AppGameParentDeviceReference {
    AppGameParentDeviceReference {
        device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
        child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
        label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
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
        device: parent_device(),
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
    require_ok(
        serde_json::from_value(serde_json::json!({
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
                "capabilityState": ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED,
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
