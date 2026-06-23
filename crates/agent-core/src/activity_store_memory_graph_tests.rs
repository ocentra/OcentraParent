use std::fs::{read, remove_file};
use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdgeKind;
use ocentra_parent_agent_protocol::constants;

use super::{
    activity_store_policy_preview_test_fixture::{active_window_event, browser_event},
    ActivityJournal, ActivityStore, JournalKey, JOURNAL_KEY_BYTES,
};

#[test]
fn activity_memory_graph_reads_visited_and_played_edges_from_sqlite() {
    let store = ActivityStore::open_in_memory()
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_OPENS));
    let browser = browser_event();
    let game = active_window_event();
    store
        .ingest_events(&[browser, game])
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_INGESTS));

    let read_model = store
        .activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_QUERIES));

    assert_eq!(read_model.returned_edge_count, 2);
    assert_eq!(read_model.returned_node_count, 3);
    assert_eq!(
        read_model.capability_status,
        ocentra_parent_agent_protocol::ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY
    );
    assert!(read_model
        .edges
        .iter()
        .any(|edge| edge.edge_kind == ActivityMemoryGraphEdgeKind::Visited));
    assert!(read_model
        .edges
        .iter()
        .any(|edge| edge.edge_kind == ActivityMemoryGraphEdgeKind::Played));
    assert!(read_model.edges.iter().all(|edge| {
        edge.trace
            .source_evidence_references
            .iter()
            .any(|reference| reference.kind == ParentEvidenceReferenceKind::ActivityEvent)
    }));
    assert!(read_model
        .nodes
        .iter()
        .any(|node| node.label == constants::activity_store::TEST_BROWSER_URL));
    assert!(read_model
        .nodes
        .iter()
        .any(|node| node.label == constants::activity_store::TEST_APP_GAME_WINDOW_TITLE));
}

#[test]
fn activity_memory_graph_replays_from_encrypted_journal_without_plaintext_leak() {
    let journal_path = temp_path(
        constants::activity_store::TEST_MEMORY_GRAPH_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_MEMORY_GRAPH_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ActivityJournal::open(journal_path.clone(), key.clone())
        .unwrap_or_else(|_| unreachable!("{}", constants::error::JOURNAL_OPENS));
    journal
        .append(&browser_event())
        .unwrap_or_else(|_| unreachable!("{}", constants::error::JOURNAL_APPENDS));
    let journal_bytes =
        read(&journal_path).unwrap_or_else(|_| unreachable!("{}", constants::error::JOURNAL_READS));
    let reader = ActivityJournal::open(journal_path.clone(), key)
        .unwrap_or_else(|_| unreachable!("{}", constants::error::JOURNAL_OPENS));
    let store = ActivityStore::open(&store_path)
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_OPENS));

    store
        .ingest_journal(&reader)
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_INGESTS));
    let read_model = store
        .activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_QUERIES));
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(read_model.returned_edge_count, 1);
    assert!(!String::from_utf8_lossy(&journal_bytes)
        .contains(constants::activity_store::TEST_BROWSER_URL));
}

#[test]
fn activity_memory_graph_reports_empty_store_without_inventing_nodes() {
    let store = ActivityStore::open_in_memory()
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_OPENS));

    let read_model = store
        .activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_QUERIES));

    assert_eq!(read_model.returned_edge_count, 0);
    assert_eq!(read_model.returned_node_count, 0);
    assert_eq!(read_model.nodes.len(), 0);
    assert_eq!(read_model.edges.len(), 0);
    assert_eq!(
        read_model.capability_status,
        ocentra_parent_agent_protocol::ACTIVITY_MEMORY_GRAPH_CAPABILITY_NO_EVIDENCE
    );
}

#[test]
fn activity_memory_graph_reports_omitted_edges_when_limit_is_reached() {
    let store = ActivityStore::open_in_memory()
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_OPENS));
    store
        .ingest_events(&[browser_event(), active_window_event()])
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_INGESTS));

    let read_model = store
        .activity_memory_graph_read_model(1, constants::activity_store::TEST_SECOND_OBSERVED_AT)
        .unwrap_or_else(|_| unreachable!("{}", constants::error::ACTIVITY_STORE_QUERIES));

    assert_eq!(read_model.returned_edge_count, 1);
    assert_eq!(read_model.omitted_edge_count, 1);
    assert_eq!(
        read_model.degraded_reasons,
        vec![ocentra_parent_agent_protocol::ACTIVITY_MEMORY_GRAPH_REASON_EDGE_LIMIT]
    );
}

fn temp_path(suffix: &str, extension: &str) -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(journal_path: &Path, store_path: &Path) {
    let _ = remove_file(journal_path);
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
    let _ = remove_file(store_path);
    for index in 1..=3 {
        let mut rotated_path = journal_path.to_path_buf();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([15; JOURNAL_KEY_BYTES])
}
