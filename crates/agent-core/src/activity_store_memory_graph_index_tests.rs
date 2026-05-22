use std::fs::remove_file;

use ocentra_parent_agent_protocol::{constants, ActivityMemoryGraphEdgeKind};

use super::{
    activity_store_memory_graph_index_persist::persist_read_model,
    activity_store_policy_preview_test_fixture::{active_window_event, browser_event},
    ActivityStore,
};

#[test]
fn durable_memory_graph_index_retrieves_relationships_after_raw_events_are_removed() {
    let store_path = temp_path(
        constants::activity_store::TEST_MEMORY_GRAPH_INDEX_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_store_path(&store_path);
    {
        let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
        store
            .ingest_events(&[browser_event(), active_window_event()])
            .expect(constants::error::ACTIVITY_STORE_INGESTS);
        let projected = store
            .activity_memory_graph_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                constants::activity_store::TEST_SECOND_OBSERVED_AT,
            )
            .expect(constants::error::ACTIVITY_STORE_QUERIES);

        assert_eq!(projected.returned_edge_count, 2);
        assert!(
            store
                .activity_memory_graph_citation_count_for_test()
                .expect(constants::error::ACTIVITY_STORE_QUERIES)
                >= 2
        );
        store
            .delete_activity_events_for_memory_graph_test()
            .expect(constants::error::ACTIVITY_STORE_QUERIES);
    }
    let reopened = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);

    let indexed = reopened
        .activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_store_path(&store_path);

    assert_eq!(indexed.returned_edge_count, 2);
    assert_eq!(indexed.returned_node_count, 3);
    assert!(indexed
        .edges
        .iter()
        .any(|edge| edge.edge_kind == ActivityMemoryGraphEdgeKind::Visited));
    assert!(indexed
        .edges
        .iter()
        .any(|edge| edge.edge_kind == ActivityMemoryGraphEdgeKind::Played));
    assert!(indexed
        .nodes
        .iter()
        .any(|node| node.label == constants::activity_store::TEST_BROWSER_URL));
    assert!(indexed
        .edges
        .iter()
        .all(|edge| !edge.trace.source_evidence_references.is_empty()));
}

#[test]
fn durable_memory_graph_index_applies_query_limit_without_dropping_stored_edges() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[browser_event(), active_window_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let limited = store
        .activity_memory_graph_read_model(1, constants::activity_store::TEST_SECOND_OBSERVED_AT)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let expanded = store
        .activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(limited.returned_edge_count, 1);
    assert_eq!(limited.omitted_edge_count, 1);
    assert_eq!(expanded.returned_edge_count, 2);
    assert_eq!(expanded.omitted_edge_count, 0);
}

#[test]
fn durable_memory_graph_index_time_range_uses_persisted_edge_observed_until() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[browser_event(), active_window_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let mut projected = store
        .activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(projected.returned_edge_count, 2);
    projected.edges.truncate(1);
    projected.edges[0].observed_until =
        Some(constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string());
    persist_read_model(&store.connection, &projected, projected.edges.len() as u64)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    store
        .delete_activity_events_for_memory_graph_test()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    let indexed = store
        .activity_memory_graph_read_model(1, constants::activity_store::TEST_THIRD_OBSERVED_AT)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(indexed.returned_edge_count, 1);
    assert_eq!(
        indexed.query.time_range.observed_from,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        indexed.query.time_range.observed_until,
        constants::activity_store::TEST_THIRD_OBSERVED_AT
    );
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

fn cleanup_store_path(store_path: &std::path::PathBuf) {
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.clone();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
    let _ = remove_file(store_path);
}
