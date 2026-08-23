use super::support::{
    attest_path, lock_connection, map_platform_error, map_storage_error, resolve_current,
    validate_attestation, validate_current, verify_broker,
};
use super::{CustodyError, CustodyStore};
use crate::authority::CurrentBindingPort;
use crate::binding::{Binding, BindingLocator};
use crate::platform::{record::BrokerRecord, PlatformAttestation, PlatformCustodyPort};
use crate::storage::{self, Record};

mod persist;

pub(super) fn current<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    locator: &BindingLocator,
) -> Result<Record, CustodyError> {
    let initial = resolve_current(store.authority.as_ref(), locator)?;
    let attestation = attest_path(store.platform.as_ref(), &store.secured_path)?;
    let digest = initial.locator().lookup_digest();
    let local = load_local(store, &digest)?;
    let broker = store
        .platform
        .current(super::support::lookup(&digest, &store.secured_path))
        .map_err(map_platform_error)?;
    let binding = resolve_current(store.authority.as_ref(), locator)?;
    reconcile(store, &binding, local, broker, attestation)
}

fn reconcile<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    binding: &Binding,
    local: Option<Record>,
    broker: Option<BrokerRecord>,
    attestation: PlatformAttestation,
) -> Result<Record, CustodyError> {
    match (local, broker) {
        (None, None) => Err(CustodyError::Missing),
        (Some(_), None) => Err(CustodyError::BrokerBehind),
        (None, Some(broker)) => persist::recover_initial(store, binding, broker, attestation),
        (Some(local), Some(broker)) => {
            reconcile_existing(store, binding, local, broker, attestation)
        }
    }
}

fn reconcile_existing<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    binding: &Binding,
    local: Record,
    broker: BrokerRecord,
    attestation: PlatformAttestation,
) -> Result<Record, CustodyError> {
    let local = authenticate_local(store, binding, local, attestation)?;
    let broker = verify_broker(store.platform.as_ref(), &broker, &store.secured_path)?;
    validate_current(&broker, binding)?;
    validate_attestation(&broker, attestation)?;
    if broker.sequence == local.sequence {
        if broker.cas_digest != local.cas_digest {
            return Err(CustodyError::Tampered);
        }
        return Ok(local);
    }
    if broker.sequence == local.sequence.saturating_add(1) {
        return persist::advance(store, binding, &local, broker);
    }
    if local.sequence > broker.sequence {
        return Err(CustodyError::BrokerBehind);
    }
    Err(CustodyError::Conflict)
}

fn authenticate_local<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    binding: &Binding,
    local: Record,
    attestation: PlatformAttestation,
) -> Result<Record, CustodyError> {
    validate_current(&local, binding)?;
    validate_attestation(&local, attestation)?;
    let broker = storage::to_broker(&local);
    let authenticated = verify_broker(store.platform.as_ref(), &broker, &store.secured_path)?;
    if authenticated.cas_digest != local.cas_digest {
        return Err(CustodyError::Tampered);
    }
    Ok(local)
}

fn load_local<P: PlatformCustodyPort, A: CurrentBindingPort>(
    store: &CustodyStore<P, A>,
    digest: &[u8; 32],
) -> Result<Option<Record>, CustodyError> {
    let connection = lock_connection(store)?;
    storage::validate_all(&connection, store.secured_path.identity()).map_err(map_storage_error)?;
    storage::load_by_lookup(&connection, digest).map_err(map_storage_error)
}
