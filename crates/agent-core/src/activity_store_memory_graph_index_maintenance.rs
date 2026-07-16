use crate::{activity_store::ActivityStore, ActivityStoreError};

impl ActivityStore {
    pub fn activity_memory_graph_indexed_citation_count(&self) -> Result<u64, ActivityStoreError> {
        crate::activity_store_memory_graph_index_query::indexed_citation_count(&self.connection)
    }

    pub fn delete_activity_events_for_memory_graph_reindex(
        &self,
    ) -> Result<(), ActivityStoreError> {
        crate::activity_store_memory_graph_index_query::delete_activity_events_for_memory_graph_test(
            &self.connection,
        )
    }
}
