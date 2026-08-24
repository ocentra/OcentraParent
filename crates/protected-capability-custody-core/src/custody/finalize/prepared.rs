use super::super::reconcile;
use super::super::scope::OperationScope;
use super::super::{CustodyError, CustodyStore, PreparedCapability};
use crate::storage::Record;

pub(super) fn load(
    store: &CustodyStore,
    scope: &OperationScope<'_>,
    capability: &PreparedCapability,
) -> Result<Record, CustodyError> {
    if capability.sequence != 1 || scope.binding().locator() != &capability.locator {
        return Err(CustodyError::Conflict);
    }
    let reconciled = reconcile::current(store, scope)?;
    if reconciled.record_id != capability.record_id
        || reconciled.lookup_digest != capability.lookup_digest
    {
        return Err(CustodyError::Conflict);
    }
    Ok(reconciled)
}
