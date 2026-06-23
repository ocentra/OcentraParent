use ocentra_parent_agent_protocol::tracking::read_model::TrackingReadModel;
use ocentra_tracking_core::read_model::tracking_read_model_for_connection;

use crate::{ActivityStore, ActivityStoreError};

pub fn tracking_read_model_for_store(
    store: &ActivityStore,
    limit: u64,
    generated_at: &str,
) -> Result<TrackingReadModel, ActivityStoreError> {
    tracking_read_model_for_connection(&store.connection, limit, generated_at)
        .map_err(ActivityStoreError::from)
}
