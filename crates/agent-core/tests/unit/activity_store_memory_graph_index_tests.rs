use std::fs::remove_file;
use std::path::Path;

use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdgeKind;
use ocentra_parent_agent_protocol::constants;

use crate::test_text::{test_ok as ok, TestResult, TestText};
use crate::{
    activity_store_memory_graph_index_persist::persist_read_model,
    activity_store_policy_preview_support::{active_window_event, browser_event},
    ActivityStore,
};

#[test]
fn durable_memory_graph_index_retrieves_relationships_after_raw_events_are_removed() -> TestResult {
    let store_path = temp_path(
        constants::activity_store::TEST_MEMORY_GRAPH_INDEX_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_store_path(&store_path);
    {
        let store = ok(
            ActivityStore::open(&store_path),
            constants::error::ACTIVITY_STORE_OPENS,
        )?;
        ingest_memory_graph_events(&store)?;
        let projected = ok(
            store.activity_memory_graph_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                constants::activity_store::TEST_SECOND_OBSERVED_AT,
            ),
            constants::error::ACTIVITY_STORE_QUERIES,
        )?;

        assert_eq!(projected.returned_edge_count, 2);
        assert!(
            ok(
                store.activity_memory_graph_indexed_citation_count(),
                constants::error::ACTIVITY_STORE_QUERIES,
            )? >= 2
        );
        ok(
            store.delete_activity_events_for_memory_graph_reindex(),
            constants::error::ACTIVITY_STORE_INGESTS,
        )?;
    }
    let reopened = ok(
        ActivityStore::open(&store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;

    let indexed = ok(
        reopened.activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
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
    Ok(())
}

#[test]
fn durable_memory_graph_index_applies_query_limit_without_dropping_stored_edges() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    ingest_memory_graph_events(&store)?;

    let limited = ok(
        store.activity_memory_graph_read_model(
            1,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
    let expanded = ok(
        store.activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(limited.returned_edge_count, 1);
    assert_eq!(limited.omitted_edge_count, 1);
    assert_eq!(expanded.returned_edge_count, 2);
    assert_eq!(expanded.omitted_edge_count, 0);
    Ok(())
}

#[test]
fn durable_memory_graph_index_time_range_uses_persisted_edge_observed_until() -> TestResult {
    let store = ok(
        ActivityStore::open_in_memory(),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    ingest_memory_graph_events(&store)?;
    let mut projected = ok(
        store.activity_memory_graph_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(projected.returned_edge_count, 2);
    projected.edges.truncate(1);
    projected.edges[0].observed_until =
        Some(constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string());
    ok(
        persist_read_model(
            store.connection_for_test(),
            &projected,
            projected.edges.len() as u64,
        ),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    ok(
        store.delete_activity_events_for_memory_graph_reindex(),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;

    let indexed = ok(
        store
            .activity_memory_graph_read_model(1, constants::activity_store::TEST_THIRD_OBSERVED_AT),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(indexed.returned_edge_count, 1);
    assert_eq!(
        indexed.query.time_range.observed_from,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        indexed.query.time_range.observed_until,
        constants::activity_store::TEST_THIRD_OBSERVED_AT
    );
    Ok(())
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

fn ingest_memory_graph_events(store: &ActivityStore) -> TestResult {
    ok(
        store.ingest_events(&[browser_event(), active_window_event()]),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    Ok(())
}

fn cleanup_store_path(store_path: impl AsRef<Path>) {
    let store_path = store_path.as_ref();
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
    let _ = remove_file(store_path);
}
