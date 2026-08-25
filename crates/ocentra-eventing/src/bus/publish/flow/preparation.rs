use crate::bus::dispatch_chain::{DispatchChain, OrderedDispatchAdmission};
use crate::bus::publish::DispatchStoredError;
use crate::bus::{DispatchMode, EventBus};
use crate::StoredEventEnvelope;

pub(super) async fn prepare_ordered(
    bus: &EventBus,
    stored: &StoredEventEnvelope,
    dispatch_mode: DispatchMode,
    dispatch_chain: &DispatchChain,
) -> Result<Option<OrderedDispatchAdmission>, DispatchStoredError> {
    if dispatch_mode != DispatchMode::OrderedByAggregateKey {
        return Ok(None);
    }
    dispatch_chain
        .admit_ordered(bus, stored.aggregate_key.clone())
        .await
        .map(Some)
        .map_err(DispatchStoredError::BeforeDispatch)
}
