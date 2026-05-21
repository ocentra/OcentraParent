use std::fs::{read, remove_file};

use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};

use super::{
    network_observation_event, ActivityJournal, ActivityStore, JournalKey, NetworkObservation,
    JOURNAL_KEY_BYTES,
};

#[test]
fn activity_store_reports_network_flow_read_model_from_ingested_events() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = network_event();

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let row = read_model
        .rows
        .first()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 1);
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
}

#[test]
fn activity_store_replays_network_flow_read_model_from_encrypted_journal() {
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
    let mut journal = ActivityJournal::open(journal_path.clone(), key.clone())
        .expect(constants::error::JOURNAL_OPENS);
    let event = network_event();
    journal
        .append(&event)
        .expect(constants::error::JOURNAL_APPENDS);
    let journal_bytes = read(&journal_path).expect(constants::error::JOURNAL_READS);
    let reader =
        ActivityJournal::open(journal_path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);

    let status = store
        .ingest_journal(&reader)
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = store
        .network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 1);
    assert_eq!(read_model.returned, 1);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_NETWORK_DOMAIN));
}

#[test]
fn activity_store_reports_empty_network_flow_without_inventing_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    let read_model = store
        .network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
    );
}

fn network_event() -> ocentra_parent_agent_protocol::ActivityEvent {
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

fn temp_path(suffix: &str, extension: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(journal_path: &std::path::PathBuf, store_path: &std::path::PathBuf) {
    let _ = remove_file(journal_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.clone();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([11; JOURNAL_KEY_BYTES])
}
