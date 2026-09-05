use super::super::reconcile;
use super::super::scope::OperationScope;
use super::super::{CustodyError, CustodyStore, PreparedCapability};
use crate::storage::Record;

pub(super) fn load(
    store: &CustodyStore,
    scope: &OperationScope<'_>,
    capability: PreparedCapability,
) -> Result<Record, CustodyError> {
    let PreparedCapability {
        record_id,
        lookup_digest,
        sequence,
        locator,
    } = capability;
    if sequence != 1 || scope.binding().locator() != &locator {
        return Err(CustodyError::Conflict);
    }
    let reconciled = reconcile::current(store, scope)?;
    if reconciled.record_id != record_id || reconciled.lookup_digest != lookup_digest {
        return Err(CustodyError::Conflict);
    }
    Ok(reconciled)
}
