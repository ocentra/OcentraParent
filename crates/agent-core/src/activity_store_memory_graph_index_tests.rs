use std::fmt::Debug;
use std::fs::remove_file;
use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdgeKind;
use ocentra_parent_agent_protocol::constants;

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
        let store = open_store(&store_path);
        ingest_memory_graph_events(&store);
        let projected = activity_store_query(store.activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ));

        assert_eq!(projected.returned_edge_count, 2);
        assert!(activity_store_query(store.activity_memory_graph_citation_count_for_test()) >= 2);
        activity_store_query(store.delete_activity_events_for_memory_graph_test());
    }
    let reopened = open_store(&store_path);

    let indexed = activity_store_query(reopened.activity_memory_graph_read_model(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    ));
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
    let store = open_in_memory_store();
    ingest_memory_graph_events(&store);

    let limited =
        activity_store_query(store.activity_memory_graph_read_model(
            1,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ));
    let expanded = activity_store_query(store.activity_memory_graph_read_model(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    ));

    assert_eq!(limited.returned_edge_count, 1);
    assert_eq!(limited.omitted_edge_count, 1);
    assert_eq!(expanded.returned_edge_count, 2);
    assert_eq!(expanded.omitted_edge_count, 0);
}

#[test]
fn durable_memory_graph_index_time_range_uses_persisted_edge_observed_until() {
    let store = open_in_memory_store();
    ingest_memory_graph_events(&store);
    let mut projected = activity_store_query(store.activity_memory_graph_read_model(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    ));

    assert_eq!(projected.returned_edge_count, 2);
    projected.edges.truncate(1);
    projected.edges[0].observed_until =
        Some(constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string());
    activity_store_query(persist_read_model(
        &store.connection,
        &projected,
        projected.edges.len() as u64,
    ));
    activity_store_query(store.delete_activity_events_for_memory_graph_test());

    let indexed = activity_store_query(
        store
            .activity_memory_graph_read_model(1, constants::activity_store::TEST_THIRD_OBSERVED_AT),
    );

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

fn open_store(store_path: &Path) -> ActivityStore {
    activity_store_open(ActivityStore::open(store_path))
}

fn open_in_memory_store() -> ActivityStore {
    activity_store_open(ActivityStore::open_in_memory())
}

fn ingest_memory_graph_events(store: &ActivityStore) {
    activity_store_ingest(store.ingest_events(&[browser_event(), active_window_event()]));
}

fn activity_store_open<T, E>(result: Result<T, E>) -> T
where
    E: Debug,
{
    result.unwrap_or_else(|error| {
        unreachable!("{}: {error:?}", constants::error::ACTIVITY_STORE_OPENS)
    })
}

fn activity_store_ingest<T, E>(result: Result<T, E>) -> T
where
    E: Debug,
{
    result.unwrap_or_else(|error| {
        unreachable!("{}: {error:?}", constants::error::ACTIVITY_STORE_INGESTS)
    })
}

fn activity_store_query<T, E>(result: Result<T, E>) -> T
where
    E: Debug,
{
    result.unwrap_or_else(|error| {
        unreachable!("{}: {error:?}", constants::error::ACTIVITY_STORE_QUERIES)
    })
}

fn cleanup_store_path(store_path: &Path) {
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
    let _ = remove_file(store_path);
}
