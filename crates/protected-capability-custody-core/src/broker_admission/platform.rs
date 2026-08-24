use std::path::Path;

#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::io::Read;

use crate::platform::identity::PhysicalDatabaseIdentity;
use crate::platform::sealed::{TrustedDatabaseGuard, TrustedPlatformOwner};
use crate::platform::{
    PlatformCustodyOwner, PlatformDatabaseGuard, PlatformError, SealContext, TransitionFailure,
};

#[cfg(windows)]
use crate::platform::identity::DatabaseIdentity;
#[cfg(windows)]
use crate::platform::record::BrokerRecord;
#[cfg(windows)]
use crate::platform::request::{BrokerLookup, TransitionRequest};
#[cfg(windows)]
use sha2::{Digest, Sha256};

mod acl;
mod crypto;
mod guard;
mod record;
mod registry;
mod secret;
mod state;
mod transition;
mod writer;

pub(super) struct BrokerPlatformOwner;

impl BrokerPlatformOwner {
    pub(super) fn new() -> Self {
        Self
    }
}

impl TrustedPlatformOwner for BrokerPlatformOwner {}

impl PlatformCustodyOwner for BrokerPlatformOwner {
    fn acquire_database(
        &self,
        canonical_path: &Path,
        physical_identity: PhysicalDatabaseIdentity,
    ) -> Result<Box<dyn PlatformDatabaseGuard>, PlatformError> {
        #[cfg(windows)]
        {
            acl::validate_path(canonical_path)?;
            acl::validate_path(
                canonical_path
                    .parent()
                    .ok_or(PlatformError::InvalidAttestation)?,
            )?;
            let registry_id = registry::registry_id(canonical_path)?;
            let writer_lock = writer::open(canonical_path)?;
            let mut ledger = state::load_or_create(&registry_id, physical_identity)?;
            if ledger.physical_identity != physical_identity {
                return Err(PlatformError::Tampered);
            }
            ledger.writer_epoch = ledger
                .writer_epoch
                .checked_add(1)
                .ok_or(PlatformError::Unavailable)?;
            state::write(&registry_id, ledger)?;
            let guard = guard::BrokerPlatformGuard::new(
                canonical_path,
                registry_id,
                physical_identity,
                ledger,
                writer_lock,
            );
            guard.revalidate_live()?;
            Ok(Box::new(guard))
        }
        #[cfg(not(windows))]
        {
            let _canonical_path = canonical_path;
            let _physical_identity = physical_identity;
            Err(PlatformError::Unavailable)
        }
    }
}

#[cfg(windows)]
impl TrustedDatabaseGuard for guard::BrokerPlatformGuard {}

#[cfg(windows)]
impl PlatformDatabaseGuard for guard::BrokerPlatformGuard {
    fn attest_database(
        &self,
        canonical_path: &Path,
        identity: DatabaseIdentity,
    ) -> Result<crate::platform::PlatformAttestation, PlatformError> {
        self.attest(canonical_path, identity)
    }

    fn reserve(&self, next: TransitionRequest<'_>) -> Result<BrokerRecord, TransitionFailure> {
        transition::reserve(self, next)
    }

    fn advance(
        &self,
        prior: &BrokerRecord,
        next: TransitionRequest<'_>,
    ) -> Result<BrokerRecord, TransitionFailure> {
        transition::advance(self, prior, next)
    }

    fn current(&self, lookup: BrokerLookup<'_>) -> Result<Option<BrokerRecord>, PlatformError> {
        record::current(self, lookup)
    }

    fn open_and_verify(
        &self,
        context: SealContext<'_>,
        sealed: &[u8],
    ) -> Result<(), PlatformError> {
        record::open_and_verify(self, context, sealed)
    }
}

#[cfg(windows)]
pub(super) fn registry_id(path: &Path) -> Result<String, PlatformError> {
    registry::registry_id(path)
}

#[cfg(not(windows))]
pub(super) fn registry_id(_path: &Path) -> Result<String, PlatformError> {
    Err(PlatformError::Unavailable)
}

#[cfg(windows)]
pub(super) fn read_registry_value(
    registry_id: &str,
    name: &str,
) -> Result<Option<Vec<u8>>, PlatformError> {
    registry::read(registry_id, name)
}

#[cfg(windows)]
pub(super) fn write_registry_value(
    registry_id: &str,
    name: &str,
    value: &[u8],
) -> Result<(), PlatformError> {
    registry::write(registry_id, name, value)
}

#[cfg(windows)]
pub(super) fn delete_registry_value(registry_id: &str, name: &str) -> Result<(), PlatformError> {
    registry::delete(registry_id, name)
}

#[cfg(windows)]
pub(super) fn encrypt_dpapi(
    registry_id: &str,
    plaintext: &[u8],
    context: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    crypto::encrypt_registry_value(registry_id, context, plaintext)
}

#[cfg(windows)]
pub(super) fn decrypt_dpapi(
    registry_id: &str,
    sealed: &[u8],
    context: &[u8],
) -> Result<Vec<u8>, PlatformError> {
    crypto::decrypt_registry_value(registry_id, context, sealed)
}

#[cfg(windows)]
pub(super) fn hex(bytes: &[u8]) -> String {
    registry::hex(bytes)
}

#[cfg(windows)]
pub(super) fn validate_broker_executable(
    executable: &File,
    path: &Path,
) -> Result<(), PlatformError> {
    acl::validate_file(executable)?;
    acl::validate_path(path.parent().ok_or(PlatformError::InvalidAttestation)?)?;
    validate_pinned_hash(executable)
}

#[cfg(windows)]
fn validate_pinned_hash(executable: &File) -> Result<(), PlatformError> {
    // The release installer must replace this with the signed, deployment-owned
    // digest.  Until that provisioning step exists, admission is unavailable;
    // accepting an unsigned or caller-selected sibling would be unsafe.
    const DEPLOYED_BROKER_SHA256: Option<[u8; 32]> = None;
    let expected = DEPLOYED_BROKER_SHA256.ok_or(PlatformError::Unavailable)?;
    const MAX_EXECUTABLE_BYTES: u64 = 128 * 1024 * 1024;
    let metadata = executable
        .metadata()
        .map_err(|_| PlatformError::Unavailable)?;
    if metadata.len() == 0 || metadata.len() > MAX_EXECUTABLE_BYTES {
        return Err(PlatformError::Tampered);
    }
    let mut reader = executable
        .try_clone()
        .map_err(|_| PlatformError::Unavailable)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut total = 0_u64;
    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|_| PlatformError::Unavailable)?;
        if read == 0 {
            break;
        }
        total = total
            .checked_add(read as u64)
            .ok_or(PlatformError::Tampered)?;
        if total > MAX_EXECUTABLE_BYTES {
            return Err(PlatformError::Tampered);
        }
        hasher.update(&buffer[..read]);
    }
    if total != metadata.len() || hasher.finalize().as_slice() != expected.as_slice() {
        return Err(PlatformError::Tampered);
    }
    Ok(())
}

#[cfg(not(windows))]
pub(super) fn validate_broker_executable(
    _executable: &std::fs::File,
    _path: &Path,
) -> Result<(), PlatformError> {
    Err(PlatformError::Unavailable)
}
