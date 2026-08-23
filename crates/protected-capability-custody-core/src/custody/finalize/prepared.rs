use super::super::reconcile;
use super::super::support::{lock_connection, map_storage_error};
use super::super::{CustodyError, CustodyStore, PreparedCapability};
use crate::authority::CurrentBindingPort;
use crate::binding::Binding;
use crate::platform::PlatformCustodyPort;
use crate::storage::{self, Record};

pub(super) fn load<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    capability: &PreparedCapability,
) -> Result<Record, CustodyError> {
    let stored = load_stored(store, &capability.record_id)?;
    if stored.lookup_digest != capability.lookup_digest || stored.sequence != capability.sequence {
        return Err(CustodyError::Conflict);
    }
    let binding = Binding::decode(&stored.canonical_binding).map_err(|_| CustodyError::Tampered)?;
    let reconciled = reconcile::current(store, binding.locator())?;
    if reconciled.record_id != capability.record_id
        || reconciled.lookup_digest != capability.lookup_digest
    {
        return Err(CustodyError::Conflict);
    }
    Ok(reconciled)
}

fn load_stored<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    record_id: &[u8; 32],
) -> Result<Record, CustodyError> {
    let connection = lock_connection(store)?;
    storage::validate_all(&connection, store.secured_path.identity()).map_err(map_storage_error)?;
    storage::load_by_id(&connection, record_id)
        .map_err(map_storage_error)?
        .ok_or(CustodyError::Missing)
}
