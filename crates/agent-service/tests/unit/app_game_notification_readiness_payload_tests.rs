use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, AppGameIdentity, AppGameInventoryEvidenceRow, AppGameServiceReadModel,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT,
    APP_GAME_INVENTORY_SOURCE_UNKNOWN, APP_GAME_INVENTORY_STATE_DETECTABLE,
    APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE, APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED,
    APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_DISPLAY_LABEL,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameControlActionResult, AppGameControlApprovalAuthority, AppGameControlApprovalDecision,
    AppGameControlApprovalRequest, AppGameEnforcementCapabilityStatus, AppGameParentActorReference,
    AppGameParentDeviceReference, AppGameParentEvidenceReference, AppGamePlatformAuthorityMatrix,
    AppGamePolicyTarget, APP_GAME_CONTROL_ACTION_STATUS_ENFORCED,
    APP_GAME_CONTROL_AUTHORITY_ACTIVE, APP_GAME_CONTROL_DECISION_APPROVED,
    APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY, APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE,
    APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE, APP_GAME_CONTROL_POLICY_KIND_APP,
    APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY, APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL,
    APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED, APP_GAME_PARENT_ACTOR_ROLE_PARENT,
    APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION, APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT,
    APP_GAME_PARENT_PLATFORM_WINDOWS, APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH,
    APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED, APP_GAME_POLICY_TARGET_TYPE_APP,
};
use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessReadModel, AppGameNotificationReadinessRow,
    APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST,
    APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
    APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL,
};
use ocentra_parent_agent_protocol::constants;
use std::primitive::str as TestStr;

use crate::test_require_json_decode::require_json_decode;
use crate::test_require_log_string_field::require_log_string_field;
use crate::test_require_ok::require_ok;
use crate::test_require_some::require_some;

use super::app_game_notification_readiness_payload::{
    app_game_notification_readiness_from_service_model, app_game_notification_readiness_payload,
};

const APP_GAME_CONTROL_APPROVAL_STATE_APPROVED: &TestStr = "approved";
const APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED: &TestStr = "process-terminated";
const APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED: &TestStr = "supported";
const APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED: &TestStr = "installed";
const APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED: &TestStr = "allowed";
const APP_GAME_ENFORCEMENT_ROLLBACK_NOT_REQUIRED: &TestStr = "not-required";
const APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY: &TestStr = "inventory";
const APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC: &TestStr = "deterministic";
const APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED: &TestStr = "catalogMatched";
const APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN: &TestStr = "inventoryScan";
const APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER: &TestStr = "windows-applocker-proof";
const APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED: &TestStr = "manual-required";
const APP_GAME_POLICY_ACTION_BLOCK: &TestStr = "block";
const APP_GAME_TEST_ACTION_RESULT_ID: &TestStr = "action-result-app-game-1";
const APP_GAME_TEST_AUTHORITY_ID: &TestStr = "authority-app-game-1";
const APP_GAME_TEST_CHILD_PROFILE_ID: &TestStr = "child-app-game-1";
const APP_GAME_TEST_DECISION_ID: &TestStr = "approval-decision-app-game-1";
const APP_GAME_TEST_DEVICE_ID: &TestStr = "device-windows-app-game-1";
const APP_GAME_TEST_DEVICE_LABEL: &TestStr = "Study PC";
const APP_GAME_TEST_ENFORCEMENT_ACTION_ID: &TestStr = "enforcement-action-app-game-1";
const APP_GAME_TEST_ENFORCEMENT_RESULT_ID: &TestStr = "enforcement-result-app-game-1";
const APP_GAME_TEST_EVIDENCE_CLAIM_ID: &TestStr = "claim-ocentra-inventory";
const APP_GAME_TEST_EVIDENCE_REF_ID: &TestStr = "evidence-app-game-session-1";
const APP_GAME_TEST_IDENTITY_ID: &TestStr = "identity-ocentra-game";
const APP_GAME_TEST_PARENT_ACTOR_ID: &TestStr = "parent-actor-app-game-1";
const APP_GAME_TEST_PLATFORM_MATRIX_ID: &TestStr = "app-game-platform-authority-matrix";
const APP_GAME_TEST_POLICY_VERSION: &TestStr = "policy-version-app-game-1";
const APP_GAME_TEST_REQUEST_ID: &TestStr = "approval-request-app-game-1";
const APP_GAME_TEST_TARGET_ID: &TestStr = "target-app-game-1";
const APP_GAME_TEST_TARGET_VALUE: &TestStr = "process:ocentra-game.exe";
const APP_GAME_TEST_TIMESTAMP: &TestStr = "2026-06-03T22:15:00Z";
const APP_GAME_TEST_UNKNOWN_SOURCE_REF: &TestStr = "source-display-only-unknown";
const APP_GAME_TEST_WINDOWS_LIMITATION: &TestStr =
    "Broad installed-app blocking needs AppLocker or App Control proof before execution.";
const APP_GAME_TEST_WINDOWS_ROW_ID: &TestStr = "windows-block-launch-row";

#[test]
fn app_game_notification_readiness_payload_does_not_fabricate_intents_without_signals() {
    let read_model = app_game_notification_readiness_from_service_model(service_model());
    let payload = app_game_notification_readiness_payload(&read_model);
    let read_model_json = string_payload(
        &payload,
        constants::field::APP_GAME_NOTIFICATION_READINESS_READ_MODEL,
    );
    let decoded = require_json_decode::<AppGameNotificationReadinessReadModel>(
        read_model_json,
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(decoded.returned, 1);
    assert_eq!(
        decoded.capability_status,
        APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL
    );
    assert_eq!(decoded.ready_intent_count, 0);
    assert_eq!(decoded.manual_required_count, 1);
    assert_eq!(decoded.unavailable_count, 0);
    assert!(!decoded.provider_delivery_claimed);
    assert!(!decoded.provider_receipt_ingestion_claimed);
    assert!(!decoded.local_outbox_runtime_claimed);
    assert!(!decoded.scheduler_runtime_claimed);
    assert!(!decoded.adapter_dispatch_claimed);
    assert!(!decoded.parent_ui_claimed);
    assert!(!decoded.child_delivery_claimed);
    assert!(!decoded
        .rows
        .iter()
        .any(|row| row.reason == APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED));
    assert!(!decoded
        .rows
        .iter()
        .any(|row| row.reason == APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST));
    assert!(!decoded
        .rows
        .iter()
        .any(|row| row.reason == APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN));
    assert_eq!(
        readiness_row(
            &decoded.rows,
            APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED
        )
        .readiness_state,
        APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED
    );
}

#[test]
fn app_game_notification_readiness_marks_explicit_unknown_classification_for_manual_review() {
    let mut model = service_model();
    model.inventory_rows.push(unknown_inventory_row());

    let read_model = app_game_notification_readiness_from_service_model(model);

    assert_eq!(read_model.returned, 2);
    assert_eq!(
        read_model.capability_status,
        APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL
    );
    assert_eq!(read_model.ready_intent_count, 0);
    assert_eq!(read_model.manual_required_count, 2);
    assert_eq!(read_model.unavailable_count, 0);
    let unknown_row = readiness_row(
        &read_model.rows,
        APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
    );
    assert_eq!(
        unknown_row.readiness_state,
        APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED
    );
    assert_eq!(unknown_row.row_count, 1);
    assert_eq!(
        unknown_row.evidence_reference_ids,
        vec![
            constants::value::APP_GAME_TEST_POLICY_READINESS_UNKNOWN_EVIDENCE_ID,
            constants::value::APP_GAME_TEST_POLICY_READINESS_UNKNOWN_INVENTORY_ENTRY_ID,
        ]
    );
}

pub(super) fn service_model() -> AppGameServiceReadModel {
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
                "rollbackState": APP_GAME_ENFORCEMENT_ROLLBACK_NOT_REQUIRED,
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
        request_id: APP_GAME_TEST_REQUEST_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        device: parent_device(),
        target: AppGamePolicyTarget {
            target_id: APP_GAME_TEST_TARGET_ID.to_string(),
            target_type: APP_GAME_POLICY_TARGET_TYPE_APP.to_string(),
            target_value: APP_GAME_TEST_TARGET_VALUE.to_string(),
        },
        requested_action: APP_GAME_POLICY_ACTION_BLOCK.to_string(),
        requested_mode: None,
        requested_setting_refs: Vec::new(),
        evidence_references: vec![parent_evidence_ref()],
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
        parent_action: None,
        reason_codes: Vec::new(),
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![parent_evidence_ref()],
        response_scope: Some(APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE.to_string()),
        decision_expires_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        audit_references: vec![APP_GAME_TEST_EVIDENCE_REF_ID.to_string()],
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
        permission_state: APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED.to_string(),
        dependency_state: APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED.to_string(),
        supported_actions: vec![APP_GAME_POLICY_ACTION_BLOCK.to_string()],
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

fn parent_device() -> AppGameParentDeviceReference {
    AppGameParentDeviceReference {
        device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
        child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
        label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
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

fn local_db_ref(evidence_id: &TestStr) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn readiness_row<'a>(
    rows: &'a [AppGameNotificationReadinessRow],
    reason: &TestStr,
) -> &'a AppGameNotificationReadinessRow {
    require_some(
        rows.iter().find(|row| row.reason == reason),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
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
