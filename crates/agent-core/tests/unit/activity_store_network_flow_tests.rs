use std::fs::{read, remove_file};
use std::path::Path;

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::test_text::{test_ok as ok, TestResult, TestText};
use crate::{
    network_observation_event, ActivityJournal, ActivityStore, JournalKey, NetworkObservation,
    JOURNAL_KEY_BYTES,
};

#[test]
fn activity_store_reports_network_flow_read_model_from_ingested_events() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let event = network_event();

    ok(
        store.ingest_events(std::slice::from_ref(&event)),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    let read_model = ok(
        store.network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
    let row = read_model
        .rows
        .first()
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.active_rows, 1);
    assert_eq!(read_model.tombstone_rows, 0);
    assert_eq!(read_model.exportable_rows, 1);
    assert_eq!(row.event_id, event.event_id);
    assert_eq!(
        row.destination_domain,
        Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string())
    );
    assert_eq!(
        row.destination_endpoint.ip,
        Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string())
    );
    assert_eq!(
        row.destination_endpoint.port,
        Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT)
    );
    assert_eq!(row.process_id, Some(4242));
    assert_eq!(row.counters.connection_count, 1);
    assert_eq!(row.counters.bytes_sent, None);
    assert_eq!(row.counters.bytes_received, None);
    assert_eq!(row.evidence.len(), 1);
    assert_eq!(row.evidence[0].kind, ActivityEvidenceKind::LocalDbRow);
    assert!(row.evidence[0]
        .evidence_id
        .starts_with(constants::activity_capture::NETWORK_EVIDENCE_ID_PREFIX));
    assert!(row.evidence[0]
        .uri
        .as_deref()
        .unwrap_or_default()
        .starts_with(constants::activity_capture::NETWORK_EVIDENCE_URI_PREFIX));
    Ok(())
}

#[test]
fn activity_store_filters_network_retention_tombstones_from_read_model() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let event = network_event();
    let deleted_evidence_id = event.evidence[0].evidence_id.clone();

    ok(
        store.ingest_events(&[event, network_retention_deleted_event(&deleted_evidence_id)]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    let read_model = ok(
        store.network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.active_rows, 0);
    assert_eq!(read_model.tombstone_rows, 1);
    assert_eq!(read_model.exportable_rows, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.latest_tombstone_event_id.as_deref(),
        Some(constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID)
    );
    assert_eq!(
        read_model.deleted_evidence_reference_ids,
        vec![deleted_evidence_id]
    );
    Ok(())
}

#[test]
fn activity_store_replays_network_flow_read_model_from_encrypted_journal() -> TestResult {
    let journal_path = temp_path(
        constants::activity_store::TEST_NETWORK_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_NETWORK_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ok(
        ActivityJournal::open(journal_path.to_path_buf(), key.clone()),
        constants::error::JOURNAL_OPENS,
    )?;
    let event = network_event();
    ok(journal.append(&event), constants::error::JOURNAL_APPENDS)?;
    let journal_bytes = ok(read(&journal_path), constants::error::JOURNAL_READS)?;
    let reader = ok(
        ActivityJournal::open(journal_path.to_path_buf(), key),
        constants::error::JOURNAL_OPENS,
    )?;
    let store = ok(
        ActivityStore::open(&store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;

    let status = ok(
        store.ingest_journal(&reader),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    let read_model = ok(
        store.network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 1);
    assert_eq!(read_model.returned, 1);
    assert_eq!(read_model.exportable_rows, 1);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_NETWORK_DOMAIN));
    Ok(())
}

#[test]
fn activity_store_reports_empty_network_flow_without_inventing_rows() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;

    let read_model = ok(
        store.network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.active_rows, 0);
    assert_eq!(read_model.tombstone_rows, 0);
    assert_eq!(read_model.exportable_rows, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
    );
    Ok(())
}

fn network_event() -> ActivityEvent {
    network_observation_event(
        NetworkObservation {
            status: ActivityCaptureCapabilityStatus::Available,
            protocol: Some(ActivityNetworkProtocol::Tcp),
            local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
            destination_ip: Some(
                constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string(),
            ),
            destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
            destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
            tcp_state: Some(ActivityNetworkTcpState::Established),
            pid: Some(4242),
            process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
            associated_pid_count: 1,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    )
}

fn network_retention_deleted_event(deleted_event_id: impl std::fmt::Display) -> ActivityEvent {
    let deleted_event_id = deleted_event_id.to_string();
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(deleted_event_id.clone()),
    );
    fields.insert(
        constants::field::DELETED_AT.to_string(),
        LogFieldValue::String(
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT.to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::AgentService,
            source_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        },
        kind: ActivityEventKind::NetworkRetentionDeleted,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Retention,
            subject_id: deleted_event_id.clone(),
            display_name: None,
        },
        fields,
        evidence: vec![ActivityEvidenceRef {
            evidence_id: deleted_event_id,
            kind: ActivityEvidenceKind::JournalEntry,
            digest: None,
            uri: None,
        }],
    }
}

fn temp_path(suffix: impl std::fmt::Display, extension: impl std::fmt::Display) -> TestText {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&suffix.to_string());

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension.to_string());
    TestText::from_display(path.display())
}

fn cleanup_paths(journal_path: impl AsRef<Path>, store_path: impl AsRef<Path>) {
    let journal_path = journal_path.as_ref();
    let store_path = store_path.as_ref();
    let _ = remove_file(journal_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([11; JOURNAL_KEY_BYTES])
}
