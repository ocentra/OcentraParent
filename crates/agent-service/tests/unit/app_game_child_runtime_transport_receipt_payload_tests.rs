#[path = "../support/test_invariants.rs"]
mod test_invariants;

use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED,
    APP_GAME_FOREGROUND_FOREGROUND, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT, APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD,
    APP_GAME_INVENTORY_STATE_INSTALLED, APP_GAME_INVENTORY_STATE_UNAVAILABLE,
    APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE, APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED,
    APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW, APP_GAME_OBSERVATION_MODE_PROCESS_START,
    APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_RUNTIME_PERMISSION_LIMITED,
    APP_GAME_RUNTIME_RUNNING, APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_DISPLAY_LABEL,
    APP_GAME_TITLE_CAPTURE_TITLE_REF,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameControlActionResult, AppGameControlApprovalDecision, AppGameControlApprovalRequest,
    AppGameEnforcementCapabilityStatus, AppGameParentActionReference, AppGameParentActorReference,
    AppGameParentDeviceReference, AppGameParentEvidenceReference, AppGamePolicyTarget,
    APP_GAME_CONTROL_ACTION_STATUS_ENFORCED, APP_GAME_CONTROL_DECISION_APPROVED,
    APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY, APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE,
    APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE, APP_GAME_CONTROL_POLICY_KIND_APP,
    APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY, APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL,
    APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED, APP_GAME_PARENT_ACTOR_ROLE_PARENT,
    APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION, APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT,
    APP_GAME_PARENT_PLATFORM_WINDOWS, APP_GAME_POLICY_TARGET_TYPE_APP,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::AppGameChildRuntimeTransportReceiptReadModel;
use ocentra_parent_agent_protocol::AppGameChildRuntimeTransportReceiptRow;
use ocentra_parent_agent_protocol::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID;
use ocentra_parent_agent_protocol::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use crate::test_invariants::{require_json_decode, require_log_string_field, require_ok};

use super::app_game_child_runtime_transport_receipt_payload::{
    app_game_child_runtime_transport_receipt_payload,
    app_game_child_runtime_transport_receipt_read_model,
    app_game_child_runtime_transport_receipt_read_model_from_service_model,
};

const GENERATED_AT: &TestStr =
    constants::value::APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_GENERATED_AT;
const TEST_INVENTORY_ENTRY_ID: &TestStr =
    constants::value::APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_INVENTORY_ENTRY_ID;
const TEST_PERMISSION_LIMITED_RUNTIME_ID: &TestStr =
    constants::value::APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_PERMISSION_LIMITED_RUNTIME_ID;
const TEST_PERMISSION_LIMITED_FOREGROUND_ID: &TestStr =
    constants::value::APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_PERMISSION_LIMITED_FOREGROUND_ID;
const TEST_UNAVAILABLE_INVENTORY_ENTRY_ID: &TestStr =
    constants::value::APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_UNAVAILABLE_INVENTORY_ENTRY_ID;
const APP_GAME_CONTROL_APPROVAL_STATE_APPROVED: &TestStr = "approved";
const APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED: &TestStr = "process-terminated";
const APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED: &TestStr = "supported";
const APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED: &TestStr = "installed";
const APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED: &TestStr = "allowed";
const APP_GAME_ENFORCEMENT_ROLLBACK_NOT_REQUIRED: &TestStr = "not-required";
const APP_GAME_POLICY_ACTION_BLOCK: &TestStr = "block";
const APP_GAME_TEST_ACTION_REFERENCE_ID: &TestStr = "parent-action-app-game-1";
const APP_GAME_TEST_ACTION_RESULT_ID: &TestStr = "action-result-app-game-1";
const APP_GAME_TEST_CATALOG_REF: &TestStr = "catalog-ref-ocentra-game";
const APP_GAME_TEST_CHILD_PROFILE_ID: &TestStr = "child-app-game-1";
const APP_GAME_TEST_DECISION_ID: &TestStr = "approval-decision-app-game-1";
const APP_GAME_TEST_DEVICE_ID: &TestStr = "device-windows-app-game-1";
const APP_GAME_TEST_DEVICE_LABEL: &TestStr = "Study PC";
const APP_GAME_TEST_ENFORCEMENT_ACTION_ID: &TestStr = "enforcement-action-app-game-1";
const APP_GAME_TEST_ENFORCEMENT_RESULT_ID: &TestStr = "enforcement-result-app-game-1";
const APP_GAME_TEST_EVIDENCE_REF_ID: &TestStr = "evidence-app-game-session-1";
const APP_GAME_TEST_EXECUTABLE_PATH_REF: &TestStr = "path-ref-ocentra-fixture";
const APP_GAME_TEST_FOREGROUND_EVIDENCE_ID: &TestStr = "foreground-evidence-window-4242";
const APP_GAME_TEST_PARENT_ACTOR_ID: &TestStr = "parent-actor-app-game-1";
const APP_GAME_TEST_PARENT_PROCESS_ID: u64 = 1000;
const APP_GAME_TEST_POLICY_VERSION: &TestStr = "policy-version-app-game-1";
const APP_GAME_TEST_PROCESS_ID: u64 = 4242;
const APP_GAME_TEST_PROCESS_IDENTITY: &TestStr = "process-4242";
const APP_GAME_TEST_PROCESS_NAME: &TestStr = "ocentra-fixture.exe";
const APP_GAME_TEST_REASON_PARENT_APPROVED: &TestStr = "parent-approved";
const APP_GAME_TEST_REQUEST_ID: &TestStr = "approval-request-app-game-1";
const APP_GAME_TEST_RUNTIME_EVIDENCE_ID: &TestStr = "runtime-evidence-process-4242";
const APP_GAME_TEST_TARGET_ID: &TestStr = "target-app-game-1";
const APP_GAME_TEST_TARGET_VALUE: &TestStr = "process:ocentra-game.exe";
const APP_GAME_TEST_TIMESTAMP: &TestStr = "2026-06-03T22:15:00Z";
const APP_GAME_TEST_WINDOW_REF: &TestStr = "window-ref-4242";
const APP_GAME_TEST_WINDOW_TITLE_REF: &TestStr = "title-ref-4242";

#[test]
fn child_runtime_transport_receipt_payload_serializes_parent_safe_status_model() {
    let read_model = app_game_child_runtime_transport_receipt_read_model(GENERATED_AT);
    let payload = app_game_child_runtime_transport_receipt_payload(&read_model);

    let reparsed = require_json_decode::<AppGameChildRuntimeTransportReceiptReadModel>(
        string_payload(
            &payload,
            constants::field::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(
        reparsed.read_model_id,
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID
    );
    assert_eq!(reparsed.returned, 0);
    assert_eq!(reparsed.transport_required_count, 0);
    assert_eq!(reparsed.manual_required_count, 0);
    assert_eq!(reparsed.unavailable_count, 0);
    assert!(!reparsed.runtime_transport_executed);
    assert!(!reparsed.runtime_receipt_ingested);
    assert!(!reparsed.provider_delivery_executed);
    assert!(!reparsed.platform_delivery_channel_claimed);
    assert_eq!(
        transport_required_rows(&reparsed).len(),
        reparsed.transport_required_count as usize
    );
}

#[test]
fn child_runtime_transport_receipt_rows_come_from_service_model_runtime_sources() {
    let read_model =
        app_game_child_runtime_transport_receipt_read_model_from_service_model(service_model());

    assert_eq!(read_model.generated_at, APP_GAME_TEST_TIMESTAMP);
    assert_eq!(read_model.returned, 5);
    assert_eq!(read_model.transport_required_count, 2);
    assert_eq!(read_model.manual_required_count, 2);
    assert_eq!(read_model.unavailable_count, 1);
    assert_eq!(
        source_row_ids(&read_model),
        vec![
            APP_GAME_TEST_RUNTIME_EVIDENCE_ID.to_string(),
            TEST_PERMISSION_LIMITED_RUNTIME_ID.to_string(),
            APP_GAME_TEST_FOREGROUND_EVIDENCE_ID.to_string(),
            TEST_PERMISSION_LIMITED_FOREGROUND_ID.to_string(),
            TEST_UNAVAILABLE_INVENTORY_ENTRY_ID.to_string()
        ]
    );
    assert!(read_model.rows[0]
        .required_transport_refs
        .contains(&APP_GAME_TEST_EVIDENCE_REF_ID.to_string()));
    assert!(!read_model.runtime_transport_executed);
    assert!(!read_model.runtime_receipt_ingested);
    assert!(!read_model.provider_delivery_executed);
    assert!(!read_model.platform_delivery_channel_claimed);
    assert!(read_model.adapter_dispatch_claimed);
    assert!(read_model.platform_enforcement_claimed);
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

fn transport_required_rows(
    read_model: &AppGameChildRuntimeTransportReceiptReadModel,
) -> Vec<&AppGameChildRuntimeTransportReceiptRow> {
    read_model
        .rows
        .iter()
        .filter(|row| {
            row.boundary_state == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED
        })
        .collect()
}

fn source_row_ids(read_model: &AppGameChildRuntimeTransportReceiptReadModel) -> Vec<TestString> {
    read_model
        .rows
        .iter()
        .map(|row| row.source_runtime_writer_row_id.clone())
        .collect()
}

fn service_model() -> AppGameServiceReadModel {
    AppGameServiceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        limit: 10,
        custody_label: APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        inventory_returned: 2,
        running_now_returned: 2,
        foreground_now_returned: 2,
        launcher_returned: 0,
        daily_rollup_returned: 0,
        evidence_claim_returned: 0,
        identity_returned: 0,
        approval_authority_returned: 0,
        approval_action_result_returned: 1,
        platform_authority_matrix_returned: 0,
        ai_classifier_result_returned: 0,
        inventory_rows: vec![inventory_row(), unavailable_inventory_row()],
        running_now_rows: vec![runtime_row(), permission_limited_runtime_row()],
        foreground_now_rows: vec![foreground_row(), permission_limited_foreground_row()],
        launcher_rows: Vec::new(),
        daily_rollups: Vec::new(),
        evidence_claim_rows: Vec::new(),
        identity_rows: Vec::new(),
        approval_authority_rows: Vec::new(),
        approval_action_result_rows: vec![enforced_action_result()],
        platform_authority_matrices: Vec::new(),
        ai_classifier_result_rows: Vec::new(),
    }
}

fn enforced_action_result() -> AppGameControlActionResult {
    let capability = enforcement_capability();
    require_ok(
        serde_json::from_value(serde_json::json!({
            "schemaVersion": APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION,
            "resultId": APP_GAME_TEST_ACTION_RESULT_ID,
            "request": approval_request(),
            "decision": approval_decision(),
            "approvalState": APP_GAME_CONTROL_APPROVAL_STATE_APPROVED,
            "capabilityState": APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED,
            "capability": enforcement_capability(),
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
        evidence_references: parent_evidence_refs(),
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
        evidence_references: parent_evidence_refs(),
        response_scope: Some(APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE.to_string()),
        decision_expires_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        audit_references: vec![APP_GAME_TEST_EVIDENCE_REF_ID.to_string()],
        persistence_state: APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE.to_string(),
        decided_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn parent_evidence_refs() -> Vec<AppGameParentEvidenceReference> {
    vec![AppGameParentEvidenceReference {
        evidence_reference_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }]
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

fn runtime_row() -> AppGameRuntimeEvidenceRow {
    AppGameRuntimeEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        runtime_evidence_id: APP_GAME_TEST_RUNTIME_EVIDENCE_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        process_identity: APP_GAME_TEST_PROCESS_IDENTITY.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        parent_process_id: Some(APP_GAME_TEST_PARENT_PROCESS_ID),
        process_name: APP_GAME_TEST_PROCESS_NAME.to_string(),
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        publisher_signature_ref: None,
        file_hash_ref: None,
        inventory_entry_id: Some(TEST_INVENTORY_ENTRY_ID.to_string()),
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        started_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        exited_at: None,
        running_duration_ms: 0,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_START.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.82,
        evidence: vec![local_db_ref(APP_GAME_TEST_EVIDENCE_REF_ID)],
    }
}

fn permission_limited_runtime_row() -> AppGameRuntimeEvidenceRow {
    AppGameRuntimeEvidenceRow {
        runtime_evidence_id: TEST_PERMISSION_LIMITED_RUNTIME_ID.to_string(),
        runtime_state: APP_GAME_RUNTIME_PERMISSION_LIMITED.to_string(),
        evidence: vec![local_db_ref(TEST_PERMISSION_LIMITED_RUNTIME_ID)],
        ..runtime_row()
    }
}

fn foreground_row() -> AppGameForegroundEvidenceRow {
    AppGameForegroundEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        foreground_evidence_id: APP_GAME_TEST_FOREGROUND_EVIDENCE_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        process_identity: APP_GAME_TEST_PROCESS_IDENTITY.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        process_name: APP_GAME_TEST_PROCESS_NAME.to_string(),
        inventory_entry_id: Some(TEST_INVENTORY_ENTRY_ID.to_string()),
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        window_ref: Some(APP_GAME_TEST_WINDOW_REF.to_string()),
        window_title_ref: Some(APP_GAME_TEST_WINDOW_TITLE_REF.to_string()),
        title_capture_state: APP_GAME_TITLE_CAPTURE_TITLE_REF.to_string(),
        foreground_started_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        foreground_ended_at: None,
        foreground_duration_ms: 0,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_FOREGROUND.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        content_knowledge_state: APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED.to_string(),
        confidence: 0.82,
        evidence: vec![local_db_ref(APP_GAME_TEST_FOREGROUND_EVIDENCE_ID)],
    }
}

fn permission_limited_foreground_row() -> AppGameForegroundEvidenceRow {
    AppGameForegroundEvidenceRow {
        foreground_evidence_id: TEST_PERMISSION_LIMITED_FOREGROUND_ID.to_string(),
        runtime_state: APP_GAME_RUNTIME_PERMISSION_LIMITED.to_string(),
        evidence: vec![local_db_ref(TEST_PERMISSION_LIMITED_FOREGROUND_ID)],
        ..foreground_row()
    }
}

fn inventory_row() -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: TEST_INVENTORY_ENTRY_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD.to_string(),
        source_ref: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_APP.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.84,
        category_candidates: Vec::new(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: vec![local_db_ref(APP_GAME_TEST_EVIDENCE_REF_ID)],
    }
}

fn unavailable_inventory_row() -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        inventory_entry_id: TEST_UNAVAILABLE_INVENTORY_ENTRY_ID.to_string(),
        inventory_state: APP_GAME_INVENTORY_STATE_UNAVAILABLE.to_string(),
        evidence: vec![local_db_ref(TEST_UNAVAILABLE_INVENTORY_ENTRY_ID)],
        ..inventory_row()
    }
}

fn local_db_ref(evidence_id: &TestStr) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}
