use ocentra_parent_agent_protocol::ActivityMemoryGraphReadModel;
use rusqlite::Connection;

use crate::{
    activity_store_memory_graph_builder::MemoryGraphBuilder,
    activity_store_memory_graph_rows::memory_graph_rows, ActivityStoreError,
};

pub(crate) fn activity_memory_graph_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<ActivityMemoryGraphReadModel, ActivityStoreError> {
    let rows = memory_graph_rows(connection, limit.saturating_add(1))?;
    let mut builder = MemoryGraphBuilder::new(limit, generated_at);
    for row in rows {
        builder.ingest(row);
    }
    Ok(builder.into_read_model())
}
