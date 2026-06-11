use std::fs::remove_file;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef,
    ActivityObserver, ActivitySource, ActivitySubject, ActivitySubjectKind, AgentCommandEnvelope,
    AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    AppGameChildRuntimeTransportReceiptReadModel, AppGameRuntimeEvidenceRow, LogFieldValue,
    LogFields, ACTIVITY_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CATALOG_READY,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL,
    APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE, APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL,
    APP_GAME_JOURNAL_FIELD_REPLAY_STATE, APP_GAME_JOURNAL_FIELD_ROW_JSON,
    APP_GAME_JOURNAL_FIELD_ROW_KIND, APP_GAME_JOURNAL_REPLAY_STATE_STORED,
    APP_GAME_JOURNAL_ROW_KIND_RUNTIME, APP_GAME_JOURNAL_SOURCE_ID,
    APP_GAME_OBSERVATION_MODE_PROCESS_START, APP_GAME_RUNTIME_RUNNING, APP_GAME_SCHEMA_VERSION,
    APP_GAME_TEST_CATALOG_REF, APP_GAME_TEST_EVIDENCE_REF_ID, APP_GAME_TEST_EXECUTABLE_PATH_REF,
    APP_GAME_TEST_PARENT_PROCESS_ID, APP_GAME_TEST_PROCESS_ID, APP_GAME_TEST_PROCESS_IDENTITY,
    APP_GAME_TEST_PROCESS_NAME, APP_GAME_TEST_RUNTIME_EVIDENCE_ID, APP_GAME_TEST_TIMESTAMP,
};

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK, lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn child_runtime_transport_receipt_command_reports_live_read_model() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[runtime_activity_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model = child_runtime_transport_receipt_payload(
        &event.payload[constants::field::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL],
    );

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported
    );
    assert_eq!(
        read_model.read_model_id,
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID
    );
    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.transport_required_count, 1);
    assert_eq!(
        read_model.rows[0].source_runtime_writer_row_id,
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID
    );
    assert!(read_model.rows[0]
        .required_transport_refs
        .contains(&APP_GAME_TEST_EVIDENCE_REF_ID.to_string()));
    assert!(!read_model.runtime_transport_executed);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id:
            constants::event_id::ACTIVITY_APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_REPORTED
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
        command: AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet,
        payload: LogFields::new(),
    }
}

fn child_runtime_transport_receipt_payload(
    value: &LogFieldValue,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn runtime_activity_event() -> ActivityEvent {
    let row = runtime_row();
    let mut fields = LogFields::new();
    fields.insert(
        APP_GAME_JOURNAL_FIELD_ROW_KIND.to_string(),
        LogFieldValue::String(APP_GAME_JOURNAL_ROW_KIND_RUNTIME.to_string()),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_ROW_JSON.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&row).expect(constants::error::AGENT_EVENT_SERIALIZES),
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
        LogFieldValue::String(APP_GAME_CLASSIFICATION_KNOWN_APP.to_string()),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: APP_GAME_TEST_RUNTIME_EVIDENCE_ID.to_string(),
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
            subject_id: APP_GAME_TEST_PROCESS_IDENTITY.to_string(),
            display_name: Some(APP_GAME_TEST_PROCESS_NAME.to_string()),
        },
        fields,
        evidence: vec![local_db_ref(APP_GAME_TEST_EVIDENCE_REF_ID)],
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
        inventory_entry_id: None,
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

fn local_db_ref(evidence_id: &str) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn temp_path(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::value::APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_TEMP_SUFFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &std::path::Path) {
    let _ = remove_file(path);
}
