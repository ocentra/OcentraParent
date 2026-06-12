use ocentra_parent_agent_protocol::TrackingReadModel;

use crate::{ActivityStore, ActivityStoreError};

pub fn tracking_read_model_for_store(
    store: &ActivityStore,
    limit: u64,
    generated_at: &str,
) -> Result<TrackingReadModel, ActivityStoreError> {
    ocentra_tracking_core::tracking_read_model_for_connection(
        &store.connection,
        limit,
        generated_at,
    )
    .map_err(ActivityStoreError::from)
}
