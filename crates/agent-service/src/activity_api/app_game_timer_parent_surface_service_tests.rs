use std::fs::remove_file;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, policy_constants, ActivityEvent, ActivityEventKind, ActivityEvidenceKind,
    ActivityEvidenceRef, ActivityObserver, ActivitySource, ActivitySubject, ActivitySubjectKind,
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute, AppGameEvidenceClaim, AppGameTimerParentSurfaceReadModel,
    LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION,
    APP_GAME_CATALOG_READY, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED, APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL,
    APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE, APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL,
    APP_GAME_JOURNAL_FIELD_REPLAY_STATE, APP_GAME_JOURNAL_FIELD_ROW_JSON,
    APP_GAME_JOURNAL_FIELD_ROW_KIND, APP_GAME_JOURNAL_REPLAY_STATE_STORED,
    APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM, APP_GAME_JOURNAL_SOURCE_ID,
    APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_DISPLAY_LABEL, APP_GAME_TEST_EVIDENCE_CLAIM_ID,
    APP_GAME_TEST_EVIDENCE_REF_ID, APP_GAME_TEST_TIMESTAMP,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL, APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
};

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK,
    enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths},
    lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn app_game_timer_parent_surface_command_reports_service_backed_rows() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[evidence_claim_activity_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model = timer_parent_surface_payload(
        &event.payload[constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL],
    );

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

    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[evidence_claim_activity_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let enforcement_paths = enforcement_paths(&store_path, &timer_state_path);
    cleanup_path(&enforcement_paths.journal_path);
    cleanup_path(&enforcement_paths.key_path);
    let enforcement_event = build_enforcement_audit_report_with_paths(
        enforcement_execute_command(),
        enforcement_paths.clone(),
    )
    .await;

    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model = timer_parent_surface_payload(
        &event.payload[constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL],
    );

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    std::env::remove_var(constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH);
    cleanup_path(&store_path);
    cleanup_path(&timer_state_path);
    cleanup_path(&enforcement_paths.journal_path);
    cleanup_path(&enforcement_paths.key_path);

    assert_eq!(
        enforcement_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    assert!(read_model.timer_runtime_claimed);
    assert!(read_model.scheduler_persistence_claimed);
    assert!(read_model.durable_scheduler_storage_claimed);
    assert!(read_model.audit_runtime_claimed);
    assert!(read_model.rollback_runtime_claimed);
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
    assert!(!read_model.adapter_dispatch_claimed);
    assert!(!read_model.child_delivery_claimed);
    assert!(!read_model.platform_enforcement_claimed);
    assert!(!read_model.raw_private_source_rows_included);
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

fn enforcement_execute_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentEnforcementExecute,
        payload: enforcement_execute_payload(),
    }
}

fn enforcement_execute_payload() -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_DECISION_ID.to_string(),
        LogFieldValue::String(policy_constants::TEST_DECISION_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_VERSION.to_string(),
        LogFieldValue::String(policy_constants::TEST_POLICY_VERSION.to_string()),
    );
    fields.insert(
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
    );
    fields.insert(
        constants::field::POLICY_ACTION.to_string(),
        LogFieldValue::String(policy_constants::ACTION_ASK_PARENT.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_DEVICE.to_string()),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_DRY_RUN.to_string(),
        LogFieldValue::Boolean(false),
    );
    fields.insert(
        constants::field::POLICY_REASON_CODES.to_string(),
        LogFieldValue::String(policy_constants::TEST_REASON_PARENT_BLOCK.to_string()),
    );
    fields.insert(
        constants::field::POLICY_RULE_IDS.to_string(),
        LogFieldValue::String(policy_constants::TEST_BLOCK_RULE_ID.to_string()),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVIDENCE_ID.to_string()),
    );
    fields.insert(
        constants::field::EXPIRES_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EXPIRES_AT.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ACTION_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_ACTION_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_TIMER_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_TIMER_EVENT_ID.to_string()),
    );
    fields
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
        LogFieldValue::String(
            serde_json::to_string(&claim).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
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
            subject_id: ocentra_parent_agent_protocol::APP_GAME_JOURNAL_EVIDENCE_CLAIM_SUBJECT_ID
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

fn local_db_ref(evidence_id: &str) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn timer_parent_surface_payload(value: &LogFieldValue) -> AppGameTimerParentSurfaceReadModel {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn temp_path(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn enforcement_paths(
    store_path: &std::path::Path,
    timer_state_path: &std::path::Path,
) -> EnforcementJournalPaths {
    EnforcementJournalPaths {
        journal_path: temp_path(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        key_path: temp_path(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: store_path.to_path_buf(),
        timer_state_path: timer_state_path.to_path_buf(),
    }
}

fn cleanup_path(path: &std::path::PathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
