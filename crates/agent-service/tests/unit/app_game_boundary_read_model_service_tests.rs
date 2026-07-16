use std::fs::remove_file;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_core::activity_store::ActivityStore;
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
use ocentra_parent_agent_protocol::app_game_boundary_read_model::{
    AppGameBoundaryReadModel, APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK,
    test_invariants::{
        require_json_decode, require_log_string_field, require_ok, serialize_test_json,
    },
};

const APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY: &TestStr = "inventory";
const APP_GAME_BOUNDARY_TEST_PATH_PREFIX: &TestStr = "app-game-boundary-read-model-service-tests";
const APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED: &TestStr = "catalogMatched";
const APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN: &TestStr = "inventoryScan";
const APP_GAME_TEST_EVIDENCE_CLAIM_ID: &TestStr = "claim-ocentra-inventory";
const APP_GAME_TEST_EVIDENCE_REF_ID: &TestStr = "evidence-app-game-session-1";
const APP_GAME_TEST_TIMESTAMP: &TestStr = "2026-06-03T22:15:00Z";
const APP_GAME_BOUNDARY_TEST_STORE_SUFFIX: &TestStr = "app-game-boundary-read-model-service";

#[tokio::test]
async fn app_game_boundary_command_reports_service_backed_protocol_rows() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(APP_GAME_BOUNDARY_TEST_STORE_SUFFIX);
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
    let read_model = boundary_read_model_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_BOUNDARY_READ_MODEL,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    drop(store);
    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameBoundaryReadModelReported
    );
    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.evidence_claim_row_count, 1);
    assert_eq!(read_model.rows.len(), 1);
    assert_eq!(
        read_model.rows[0].boundary_kind,
        APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM
    );
    assert_eq!(read_model.rows[0].row_count, 1);
    assert_eq!(
        read_model.rows[0].evidence_reference_ids,
        vec![
            APP_GAME_TEST_EVIDENCE_REF_ID,
            APP_GAME_TEST_EVIDENCE_CLAIM_ID
        ]
    );
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_APP_GAME_BOUNDARY_READ_MODEL_REPORTED.to_string(),
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
        command: AgentCommandName::AgentActivityAppGameBoundaryReadModelGet,
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

fn local_db_ref(evidence_id: impl std::fmt::Display) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: None,
        uri: None,
    }
}

fn boundary_read_model_payload(value: &LogFieldValue) -> AppGameBoundaryReadModel {
    let text = require_log_string_field(Some(value), constants::error::AGENT_EVENT_SERIALIZES);
    require_json_decode(text, constants::error::AGENT_EVENT_SERIALIZES)
}

fn temp_path(suffix: impl std::fmt::Display) -> TestPathBuf {
    let suffix = suffix.to_string();
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_string(),
    );
    name.push(constants::delimiter::HYPHEN);
    name.push_str(APP_GAME_BOUNDARY_TEST_PATH_PREFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &std::path::Path) {
    let _ = remove_file(path);
    let wal_path = path.with_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let shm_path = path.with_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
