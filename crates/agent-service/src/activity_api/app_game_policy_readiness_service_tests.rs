use std::fs::remove_file;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef,
    ActivityObserver, ActivitySource, ActivitySubject, ActivitySubjectKind, AgentCommandEnvelope,
    AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    AppGameEvidenceClaim, AppGamePolicyReadinessReadModel, LogFieldValue, LogFields,
    ACTIVITY_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_GAME, APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED,
    APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL, APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE,
    APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL, APP_GAME_JOURNAL_FIELD_REPLAY_STATE,
    APP_GAME_JOURNAL_FIELD_ROW_JSON, APP_GAME_JOURNAL_FIELD_ROW_KIND,
    APP_GAME_JOURNAL_REPLAY_STATE_STORED, APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM,
    APP_GAME_JOURNAL_SOURCE_ID, APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN,
    APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE,
    APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE, APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW,
    APP_GAME_POLICY_READINESS_STATE_MISSING, APP_GAME_POLICY_READINESS_STATE_READY,
    APP_GAME_POLICY_READINESS_STATUS_PARTIAL, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_DISPLAY_LABEL, APP_GAME_TEST_EVIDENCE_CLAIM_ID,
    APP_GAME_TEST_EVIDENCE_REF_ID, APP_GAME_TEST_TIMESTAMP,
};

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK, lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn app_game_policy_readiness_command_reports_service_backed_readiness_rows() {
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
    let read_model = policy_readiness_payload(
        &event.payload[constants::field::APP_GAME_POLICY_READINESS_READ_MODEL],
    );

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported
    );
    assert_eq!(
        read_model.capability_status,
        APP_GAME_POLICY_READINESS_STATUS_PARTIAL
    );
    assert!(!read_model.policy_evaluation_ready);
    assert!(!read_model.category_routing_ready);
    assert!(!read_model.unknown_review_required);
    assert!(read_model.manual_review_required);
    assert!(!read_model.adapter_dispatch_claimed);
    assert_eq!(read_model.evidence_claim_row_count, 1);
    assert_eq!(read_model.identity_row_count, 0);
    assert_eq!(read_model.platform_authority_row_count, 0);
    assert_eq!(read_model.category_candidate_row_count, 0);
    assert_eq!(read_model.unknown_review_row_count, 0);
    assert_eq!(read_model.rows.len(), 7);
    assert_eq!(
        read_model.rows[0].readiness_kind,
        APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE
    );
    assert_eq!(
        read_model.rows[0].readiness_state,
        APP_GAME_POLICY_READINESS_STATE_MISSING
    );
    assert_eq!(
        read_model.rows[0].evidence_reference_ids,
        vec![
            APP_GAME_TEST_EVIDENCE_REF_ID,
            APP_GAME_TEST_EVIDENCE_CLAIM_ID
        ]
    );
    assert_eq!(
        readiness_row(
            &read_model.rows,
            APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE
        )
        .readiness_state,
        APP_GAME_POLICY_READINESS_STATE_MISSING
    );
    assert_eq!(
        readiness_row(
            &read_model.rows,
            APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW
        )
        .readiness_state,
        APP_GAME_POLICY_READINESS_STATE_READY
    );
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_APP_GAME_POLICY_READINESS_READ_MODEL_REPORTED
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
        command: AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet,
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

fn policy_readiness_payload(value: &LogFieldValue) -> AppGamePolicyReadinessReadModel {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn readiness_row<'a>(
    rows: &'a [ocentra_parent_agent_protocol::AppGamePolicyReadinessRow],
    readiness_kind: &str,
) -> &'a ocentra_parent_agent_protocol::AppGamePolicyReadinessRow {
    rows.iter()
        .find(|row| row.readiness_kind == readiness_kind)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
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

fn cleanup_path(path: &std::path::PathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
