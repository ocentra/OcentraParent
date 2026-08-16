use ocentra_parent_agent_protocol::constants;
use rusqlite::Connection;

use crate::{
    activity_store_memory_graph_builder::MemoryGraphBuilder,
    activity_store_memory_graph_index_persist::persist_read_model,
    activity_store_memory_graph_rows::memory_graph_index_rows, ActivityStoreError,
};

pub(crate) fn refresh_activity_memory_graph_index(
    connection: &Connection,
    generated_at: &str,
) -> Result<(), ActivityStoreError> {
    let rows = memory_graph_index_rows(connection)?;
    let source_event_count = rows.len() as u64;
    let mut builder = MemoryGraphBuilder::new(
        constants::activity_store::MEMORY_GRAPH_INDEX_REFRESH_LIMIT,
        generated_at,
    );
    for row in rows {
        builder.ingest(&row);
    }
    let read_model = builder.into_read_model();
    persist_read_model(connection, &read_model, source_event_count)
}
