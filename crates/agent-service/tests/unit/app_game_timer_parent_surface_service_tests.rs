#[path = "../support/test_invariants.rs"]
mod test_invariants;

use std::fs::{remove_file, write};
use std::path::{Path as TestPath, PathBuf as TestPathBuf};
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_protocol::activity::policy::{
    ParentActorReference, ParentEvidenceReference, ParentEvidenceReferenceKind, PolicyAction,
    PolicyTarget, PolicyTargetType,
};
use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameEvidenceClaim, APP_GAME_CATALOG_READY, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL,
    APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE, APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL,
    APP_GAME_JOURNAL_FIELD_REPLAY_STATE, APP_GAME_JOURNAL_FIELD_ROW_JSON,
    APP_GAME_JOURNAL_FIELD_ROW_KIND, APP_GAME_JOURNAL_REPLAY_STATE_STORED,
    APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM, APP_GAME_JOURNAL_SOURCE_ID,
    APP_GAME_RUNTIME_NOT_CLAIMED, APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_DISPLAY_LABEL,
};
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::{
    AppGameTimerParentSurfaceReadModel,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL, APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementActiveTimerState, EnforcementAdapterKind,
    EnforcementAdapterResultCode, EnforcementAuditEvent, EnforcementAuditEventKind,
    EnforcementCapabilityState, EnforcementCapabilityStatus, EnforcementDependencyState,
    EnforcementIntent, EnforcementIntentSource, EnforcementMode, EnforcementPermissionState,
    EnforcementResult, EnforcementResultStatus, EnforcementRollbackState, EnforcementTimerEvent,
    EnforcementTimerEventKind, ParentActionReference, ParentPlatform,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::ParentActorRole;
use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK,
    test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    },
    test_text::TestText,
};

const APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY: &TestStr = "inventory";
const APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED: &TestStr = "catalogMatched";
const APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN: &TestStr = "inventoryScan";
const APP_GAME_TEST_EVIDENCE_CLAIM_ID: &TestStr = "claim-ocentra-inventory";
const APP_GAME_TEST_EVIDENCE_REF_ID: &TestStr = "evidence-app-game-session-1";
const APP_GAME_TEST_TIMESTAMP: &TestStr = "2026-06-03T22:15:00Z";

#[tokio::test]
async fn app_game_timer_parent_surface_command_reports_service_backed_rows() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = require_ok(
        ActivityStore::open(&store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    require_ok(
        store.ingest_events(&[evidence_claim_activity_event()]),
        constants::error::ACTIVITY_STORE_INGESTS,
    );

    let body = serialize_test_json(&command_envelope());
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let read_model = timer_parent_surface_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
    );
    assert_eq!(
        read_model.capability_status,
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL
    );
    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.ready_for_parent_surface_count, 0);
    assert_eq!(read_model.blocked_by_source_freshness_count, 1);
    assert_eq!(read_model.blocked_by_compiler_decision_count, 0);
    assert_eq!(read_model.runtime_manual_required_count, 0);
    assert_empty_control_child_ux_rows(&read_model);
    assert!(!read_model.timer_runtime_claimed);
    assert!(!read_model.scheduler_persistence_claimed);
    assert!(!read_model.durable_scheduler_storage_claimed);
    assert!(!read_model.audit_runtime_claimed);
    assert!(!read_model.rollback_runtime_claimed);
    assert!(!read_model.adapter_dispatch_claimed);
    assert!(!read_model.child_delivery_claimed);
    assert!(!read_model.platform_enforcement_claimed);
    assert!(!read_model.raw_private_source_rows_included);
    assert_eq!(
        read_model.rows[0].target_domain,
        APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP
    );
    assert_eq!(
        read_model.rows[0].timer_surface_state,
        APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS
    );
    assert_eq!(
        read_model.rows[0].evidence_reference_ids,
        vec![
            APP_GAME_TEST_EVIDENCE_REF_ID,
            APP_GAME_TEST_EVIDENCE_CLAIM_ID
        ]
    );
}

#[tokio::test]
async fn app_game_timer_parent_surface_reports_existing_active_timer_state_store() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    let timer_state_path = temp_path(constants::enforcement::TIMER_STATE_ID_PREFIX);
    cleanup_path(&store_path);
    cleanup_path(&timer_state_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);
    std::env::set_var(
        constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH,
        &timer_state_path,
    );

    let store = require_ok(
        ActivityStore::open(&store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    require_ok(
        store.ingest_events(&[evidence_claim_activity_event()]),
        constants::error::ACTIVITY_STORE_INGESTS,
    );
    drop(store);
    write_active_timer_state_fixture(&timer_state_path);

    let body = serialize_test_json(&command_envelope());
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let read_model = timer_parent_surface_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    std::env::remove_var(constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH);
    cleanup_path(&store_path);
    cleanup_path(&timer_state_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
    );
    assert!(read_model.timer_runtime_claimed);
    assert!(read_model.scheduler_persistence_claimed);
    assert!(read_model.durable_scheduler_storage_claimed);
    assert!(read_model.audit_runtime_claimed);
    assert!(read_model.rollback_runtime_claimed);
    assert_empty_control_child_ux_rows(&read_model);
    assert!(!read_model.adapter_dispatch_claimed);
    assert!(!read_model.child_delivery_claimed);
    assert!(!read_model.platform_enforcement_claimed);
    assert!(!read_model.raw_private_source_rows_included);
}

#[tokio::test]
async fn app_game_timer_parent_surface_timer_state_helpers_are_linked() -> Result<(), TestString> {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let timer_state_path = temp_path(constants::enforcement::TIMER_STATE_ID_PREFIX);
    cleanup_path(&timer_state_path);

    let state = active_timer_state_fixture();
    let outcome = EnforcementBoundaryOutcome {
        action: state.action.clone(),
        result: state.result.clone(),
        audit_event: state.audit_event.clone(),
        timer_event: Some(state.timer_event.clone()),
        adapter_request: None,
    };

    let stored = crate::enforcement_timer_state_file::store_active_timer_state_for_outcome_with_app_game_session(
        &outcome,
        &timer_state_path,
        policy_constants::TEST_EVALUATED_AT,
        None,
    )
    .await
    .map_err(|error| format!("{error:?}"))?;
    let stored = require_some(stored, constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(stored.state_id, constants::enforcement::TEST_TIMER_STATE_ID);

    crate::enforcement_timer_state_file::remove_active_timer_state(&timer_state_path)
        .await
        .map_err(|error| format!("{error:?}"))?;

    cleanup_path(&timer_state_path);
    Ok(())
}

fn assert_empty_control_child_ux_rows(read_model: &AppGameTimerParentSurfaceReadModel) {
    assert_eq!(read_model.control_action_result_count, 0);
    assert!(read_model.control_action_result_reference_ids.is_empty());
    assert!(read_model.control_action_result_statuses.is_empty());
    assert!(read_model
        .control_action_result_capability_states
        .is_empty());
    assert!(read_model
        .control_action_result_enforcement_statuses
        .is_empty());
    assert!(read_model.child_facing_reason_reference_ids.is_empty());
    assert!(read_model.child_facing_status_reference_ids.is_empty());
    assert_eq!(read_model.child_ux_handoff_ready_count, 0);
    assert_eq!(read_model.child_ux_handoff_blocked_count, 0);
    assert!(read_model.child_ux_handoff_reference_ids.is_empty());
    assert_eq!(read_model.child_ux_local_handoff_artifact_record_count, 0);
    assert_eq!(read_model.child_ux_local_handoff_artifact_skipped_count, 0);
    assert!(read_model
        .child_ux_local_handoff_artifact_reference_ids
        .is_empty());
    assert_eq!(
        read_model.child_ux_parent_preference_setup_draft_ready_count,
        0
    );
    assert_eq!(
        read_model.child_ux_parent_preference_setup_unavailable_visible_count,
        0
    );
    assert!(read_model
        .child_ux_parent_preference_setup_reference_ids
        .is_empty());
    assert!(read_model
        .child_ux_parent_preference_setup_records
        .is_empty());
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED
            .to_string(),
        sent_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
        payload: LogFields::new(),
    }
}

fn evidence_claim_activity_event() -> ActivityEvent {
    let claim = evidence_claim();
    let mut fields = LogFields::new();
    fields.insert(
        APP_GAME_JOURNAL_FIELD_ROW_KIND.to_string(),
        LogFieldValue::String(APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM.to_string()),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_ROW_JSON.to_string(),
        LogFieldValue::String(serialize_test_json(&claim)),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL.to_string(),
        LogFieldValue::String(APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL.to_string()),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_REPLAY_STATE.to_string(),
        LogFieldValue::String(APP_GAME_JOURNAL_REPLAY_STATE_STORED.to_string()),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE.to_string(),
        LogFieldValue::String(APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string()),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: APP_GAME_TEST_EVIDENCE_CLAIM_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            observer: ActivityObserver::AgentService,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::DeviceIdleStateObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id:
                ocentra_parent_agent_protocol::app_game::APP_GAME_JOURNAL_EVIDENCE_CLAIM_SUBJECT_ID
                    .to_string(),
            display_name: Some(APP_GAME_TEST_DISPLAY_LABEL.to_string()),
        },
        fields,
        evidence: vec![local_db_ref(APP_GAME_TEST_EVIDENCE_REF_ID)],
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

fn local_db_ref(evidence_id: TestText) -> ActivityEvidenceRef {
    let evidence_id = evidence_id;
    ActivityEvidenceRef {
        evidence_id: evidence_id.as_ref().to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn timer_parent_surface_payload(value: &LogFieldValue) -> AppGameTimerParentSurfaceReadModel {
    let text = require_log_string_field(Some(value), constants::error::AGENT_EVENT_SERIALIZES);
    require_json_decode(text, constants::error::AGENT_EVENT_SERIALIZES)
}

fn write_active_timer_state_fixture(path: &std::path::Path) {
    let state = active_timer_state_fixture();
    let text = require_ok(
        serde_json::to_string_pretty(&state),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    require_ok(
        write(path, text),
        constants::value::ACTIVITY_CAPTURE_STORE_ERROR,
    );
}

fn active_timer_state_fixture() -> EnforcementActiveTimerState {
    let action = enforcement_action_fixture();
    let capability = enforcement_capability_fixture();
    let result = enforcement_result_fixture(&action, capability.clone());
    EnforcementActiveTimerState {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        state_id: constants::enforcement::TEST_TIMER_STATE_ID.to_string(),
        action: action.clone(),
        result: result.clone(),
        audit_event: EnforcementAuditEvent {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            audit_event_id: constants::enforcement::TEST_AUDIT_EVENT_ID.to_string(),
            audit_event_kind: EnforcementAuditEventKind::Succeeded,
            action: action.clone(),
            result,
            capability,
            unavailable_status: None,
            policy_version: policy_constants::TEST_POLICY_VERSION.to_string(),
            evidence_references: vec![parent_evidence_fixture()],
            actor: Some(parent_actor_fixture()),
            parent_override: action.parent_approval.clone(),
            journal_sequence: Some(constants::enforcement::TEST_JOURNAL_SEQUENCE.to_string()),
            observed_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        },
        timer_event: EnforcementTimerEvent {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            timer_event_id: constants::enforcement::TEST_TIMER_EVENT_ID.to_string(),
            timer_event_kind: EnforcementTimerEventKind::RestartRecovered,
            action_id: action.action_id,
            policy_decision_id: policy_constants::TEST_DECISION_ID.to_string(),
            evidence_references: vec![parent_evidence_fixture()],
            scheduled_at: policy_constants::TEST_EVALUATED_AT.to_string(),
            effective_at: Some(policy_constants::TEST_EXPIRES_AT.to_string()),
            rollback_token: Some(constants::enforcement::TEST_ROLLBACK_TOKEN.to_string()),
            recovered_after_restart: true,
            unavailable_reason: None,
        },
        stored_at: policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}

fn enforcement_action_fixture() -> EnforcementAction {
    let intent = enforcement_intent_fixture();
    EnforcementAction {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        action_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        intent_id: intent.intent_id.clone(),
        policy_decision_id: intent.policy_decision_id.clone(),
        policy_action: intent.requested_action,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        platform: ParentPlatform::Windows,
        target: intent.target.clone(),
        mode: EnforcementMode::TerminateProcess,
        capability: enforcement_capability_fixture(),
        reason_codes: vec![policy_constants::TEST_REASON_PARENT_BLOCK.to_string()],
        evidence_references: vec![parent_evidence_fixture()],
        local_ai_result_id: None,
        parent_approval: intent.parent_approval,
        dry_run: false,
        requested_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        expires_at: Some(policy_constants::TEST_EXPIRES_AT.to_string()),
        rollback_token: Some(constants::enforcement::TEST_ROLLBACK_TOKEN.to_string()),
    }
}

fn enforcement_result_fixture(
    action: &EnforcementAction,
    capability: EnforcementCapabilityStatus,
) -> EnforcementResult {
    EnforcementResult {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        result_id: constants::enforcement::TEST_RESULT_ID.to_string(),
        action_id: action.action_id.clone(),
        status: EnforcementResultStatus::ActuallyEnforced,
        adapter_result_code: EnforcementAdapterResultCode::ProcessTerminated,
        started_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        completed_at: Some(policy_constants::TEST_EVALUATED_AT.to_string()),
        rollback_token: action.rollback_token.clone(),
        rollback_state: EnforcementRollbackState::Available,
        unavailable_reason: None,
        unavailable_status: None,
        failed_reason: None,
        next_check_at: None,
        capability,
    }
}

fn enforcement_capability_fixture() -> EnforcementCapabilityStatus {
    EnforcementCapabilityStatus {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Supported,
        permission_state: EnforcementPermissionState::NotRequired,
        dependency_state: EnforcementDependencyState::Installed,
        supported_actions: vec![
            EnforcementMode::TerminateProcess,
            EnforcementMode::TemporaryBlock,
        ],
        degraded_reason: None,
        last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}

fn enforcement_intent_fixture() -> EnforcementIntent {
    EnforcementIntent {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        intent_id: constants::enforcement::TEST_INTENT_ID.to_string(),
        source: EnforcementIntentSource::LocalPolicyEvaluator,
        actor: Some(parent_actor_fixture()),
        device: parent_device_fixture(),
        policy_decision_id: policy_constants::TEST_DECISION_ID.to_string(),
        target: policy_target_fixture(),
        requested_action: PolicyAction::Block,
        evidence_references: vec![parent_evidence_fixture()],
        parent_approval: Some(parent_action_reference_fixture()),
        idempotency_key: constants::enforcement::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

fn parent_actor_fixture() -> ParentActorReference {
    ParentActorReference {
        actor_id: policy_constants::TEST_PARENT_ACTOR_ID.to_string(),
        role: ParentActorRole::Parent,
    }
}

fn parent_action_reference_fixture() -> ParentActionReference {
    ParentActionReference {
        action_reference_id: constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID.to_string(),
        actor: parent_actor_fixture(),
        policy_version: policy_constants::TEST_POLICY_VERSION.to_string(),
        created_at: policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}

fn parent_device_fixture() -> ParentDeviceReference {
    ParentDeviceReference {
        device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
        child_profile_id: Some(policy_constants::TEST_CHILD_PROFILE_ID.to_string()),
        label: constants::enforcement::TEST_CHILD_DEVICE_LABEL.to_string(),
        platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
    }
}

fn policy_target_fixture() -> PolicyTarget {
    PolicyTarget {
        target_id: constants::enforcement::TEST_PROCESS_TARGET_ID.to_string(),
        target_type: PolicyTargetType::Process,
        target_value: constants::enforcement::TEST_PROCESS_TARGET_VALUE.to_string(),
    }
}

fn parent_evidence_fixture() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: policy_constants::TEST_EVIDENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}

fn temp_path(suffix: TestText) -> TestPathBuf {
    let suffix = suffix;
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.as_ref());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&unique_suffix());

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn unique_suffix() -> TestString {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| TestString::from("0"))
}

fn cleanup_path(path: impl AsRef<TestPath>) {
    let path = path.as_ref();
    let _ = remove_file(path);
    let mut wal_path = path.to_path_buf();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.to_path_buf();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
