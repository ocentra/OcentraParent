#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::sync::Mutex;

#[cfg(windows)]
use crate::platform::identity::{DatabaseIdentity, PhysicalDatabaseIdentity};
#[cfg(windows)]
use crate::platform::{PlatformAttestation, PlatformError};

#[cfg(windows)]
use super::state::LedgerState;

#[cfg(windows)]
pub(super) struct BrokerPlatformGuard {
    pub(super) canonical_path: PathBuf,
    pub(super) registry_id: String,
    pub(super) physical_identity: PhysicalDatabaseIdentity,
    pub(super) database_identity: Mutex<Option<DatabaseIdentity>>,
    pub(super) state: Mutex<LedgerState>,
    _writer_lock: File,
}

#[cfg(windows)]
impl BrokerPlatformGuard {
    pub(super) fn new(
        canonical_path: &Path,
        registry_id: String,
        physical_identity: PhysicalDatabaseIdentity,
        state: LedgerState,
        writer_lock: File,
    ) -> Self {
        Self {
            canonical_path: canonical_path.to_path_buf(),
            registry_id,
            physical_identity,
            database_identity: Mutex::new(None),
            state: Mutex::new(state),
            _writer_lock: writer_lock,
        }
    }

    pub(super) fn attest(
        &self,
        canonical_path: &Path,
        identity: DatabaseIdentity,
    ) -> Result<PlatformAttestation, PlatformError> {
        if canonical_path != self.canonical_path
            || !identity
                .as_bytes()
                .starts_with(self.physical_identity.as_bytes())
        {
            return Err(PlatformError::InvalidAttestation);
        }
        let mut bound = self.database_identity.lock().map_err(map_poison)?;
        match *bound {
            Some(current) if current != identity => return Err(PlatformError::Tampered),
            Some(_) => {}
            None => *bound = Some(identity),
        }
        let state = *self.state.lock().map_err(map_poison)?;
        Ok(PlatformAttestation::isolated_broker(
            state.key_epoch,
            state.writer_epoch,
            state.watermark,
            identity,
        ))
    }
}

#[cfg(windows)]
pub(super) fn map_poison<T>(_error: std::sync::PoisonError<T>) -> PlatformError {
    PlatformError::Unavailable
}
