use super::scope::OperationScope;
use super::support::sqlite::{finish_step, lock_connection, map_error as map_storage_error};
use super::support::{
    attest_path, map_platform_error, validate_attestation, validate_current, verify_broker,
};
use super::{CustodyError, CustodyStore};
use crate::binding::Binding;
use crate::platform::{record::BrokerRecord, PlatformAttestation};
use crate::storage::{self, Record};

mod persist;

pub(super) fn current(
    store: &CustodyStore,
    scope: &OperationScope<'_>,
) -> Result<Record, CustodyError> {
    let binding = scope.binding().clone();
    let attestation = attest_path(store.platform.as_ref(), &store.secured_path)?;
    let digest = binding.locator().lookup_digest();
    let broker = store
        .platform
        .current(super::support::lookup(&digest, &store.secured_path))
        .map_err(|error| map_platform_error(&error))?;
    let local = load_local(store, &digest)?;
    reconcile(store, &binding, local, broker, attestation)
}

fn reconcile(
    store: &CustodyStore,
    binding: &Binding,
    local: Option<Record>,
    broker: Option<BrokerRecord>,
    attestation: PlatformAttestation,
) -> Result<Record, CustodyError> {
    match (local, broker) {
        (None, None) => Err(CustodyError::Missing),
        (Some(_), None) => Err(CustodyError::BrokerBehind),
        (None, Some(broker)) => persist::recover_initial(store, binding, &broker, attestation),
        (Some(local), Some(broker)) => {
            reconcile_existing(store, binding, local, &broker, attestation)
        }
    }
}

fn reconcile_existing(
    store: &CustodyStore,
    binding: &Binding,
    local: Record,
    broker: &BrokerRecord,
    attestation: PlatformAttestation,
) -> Result<Record, CustodyError> {
    let local = authenticate_local(store, binding, local, attestation)?;
    let broker = verify_broker(store.platform.as_ref(), broker, &store.secured_path)?;
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

fn authenticate_local(
    store: &CustodyStore,
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

fn load_local(store: &CustodyStore, digest: &[u8; 32]) -> Result<Option<Record>, CustodyError> {
    store
        .secured_path
        .revalidate()
        .map_err(|error| super::support::map_path_error(&error))?;
    let connection = lock_connection(store)?;
    let result = storage::validate_all(&connection, store.secured_path.identity())
        .map_err(|error| map_storage_error(&error))
        .and_then(|()| {
            storage::load_by_lookup(&connection, digest).map_err(|error| map_storage_error(&error))
        });
    drop(connection);
    finish_step(store, result)
}
