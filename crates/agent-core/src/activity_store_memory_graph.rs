use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphReadModel;
use rusqlite::Connection;

use crate::{
    activity_store_memory_graph_index::refresh_activity_memory_graph_index,
    activity_store_memory_graph_index_query::indexed_activity_memory_graph_read_model,
    ActivityStoreError,
};

pub(crate) fn activity_memory_graph_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<ActivityMemoryGraphReadModel, ActivityStoreError> {
    refresh_activity_memory_graph_index(connection, generated_at)?;
    indexed_activity_memory_graph_read_model(connection, limit, generated_at)
}
