#[cfg(windows)]
use crate::platform::identity::PhysicalDatabaseIdentity;
#[cfg(windows)]
use crate::platform::PlatformError;
#[cfg(windows)]
use zeroize::Zeroizing;

#[cfg(windows)]
use super::{crypto, registry};

#[cfg(windows)]
const STATE_MAGIC: [u8; 4] = *b"OCPS";
#[cfg(windows)]
const STATE_VERSION: u16 = 1;
#[cfg(windows)]
const STATE_BYTES: usize = 126;
#[cfg(windows)]
const STATE_VALUE_NAME: &str = "state";

#[cfg(windows)]
#[derive(Clone, Copy)]
pub(super) struct LedgerState {
    pub(super) physical_identity: PhysicalDatabaseIdentity,
    pub(super) key_epoch: u64,
    pub(super) writer_epoch: u64,
    pub(super) watermark: u64,
}

/// The custody watermark cannot be trusted when it is stored only in a
/// user-restorable file or registry hive.  Production deployment must provide
/// a broker/service or TPM-backed non-restorable anchor before this adapter is
/// enabled.  There is deliberately no software fallback: snapshot restore is
/// an unavailable state, never an opportunity to reset the watermark.
struct NonRestorableAntiRollbackAnchor;

impl NonRestorableAntiRollbackAnchor {
    fn acquire() -> Result<Self, PlatformError> {
        Err(PlatformError::Unavailable)
    }
}

#[cfg(windows)]
pub(super) fn load_or_create(
    registry_id: &str,
    physical_identity: PhysicalDatabaseIdentity,
) -> Result<LedgerState, PlatformError> {
    let _anti_rollback_anchor = NonRestorableAntiRollbackAnchor::acquire()?;
    match registry::read(registry_id, STATE_VALUE_NAME)? {
        Some(sealed) => {
            let plaintext = Zeroizing::new(crypto::decrypt_state(registry_id, &sealed)?);
            decode(plaintext.as_ref())
        }
        None => {
            let state = LedgerState {
                physical_identity,
                key_epoch: random_nonzero_u64()?,
                writer_epoch: random_nonzero_u64()?,
                watermark: 1,
            };
            write(registry_id, state)?;
            Ok(state)
        }
    }
}

#[cfg(windows)]
pub(super) fn write(registry_id: &str, state: LedgerState) -> Result<(), PlatformError> {
    let mut plaintext = Zeroizing::new(Vec::with_capacity(STATE_BYTES));
    plaintext.extend_from_slice(&STATE_MAGIC);
    plaintext.extend_from_slice(&STATE_VERSION.to_be_bytes());
    plaintext.extend_from_slice(state.physical_identity.as_bytes());
    plaintext.extend_from_slice(&state.key_epoch.to_be_bytes());
    plaintext.extend_from_slice(&state.writer_epoch.to_be_bytes());
    plaintext.extend_from_slice(&state.watermark.to_be_bytes());
    let sealed = crypto::encrypt_state(registry_id, plaintext.as_ref())?;
    registry::write(registry_id, STATE_VALUE_NAME, &sealed)
}

#[cfg(windows)]
fn decode(plaintext: &[u8]) -> Result<LedgerState, PlatformError> {
    if plaintext.len() != STATE_BYTES
        || plaintext.get(..4) != Some(STATE_MAGIC.as_slice())
        || plaintext.get(4..6) != Some(STATE_VERSION.to_be_bytes().as_slice())
    {
        return Err(PlatformError::Tampered);
    }
    let physical_identity = PhysicalDatabaseIdentity::from_bytes(
        plaintext.get(6..102).ok_or(PlatformError::Tampered)?,
    )?;
    let key_epoch = read_u64(plaintext, 102)?;
    let writer_epoch = read_u64(plaintext, 110)?;
    let watermark = read_u64(plaintext, 118)?;
    if key_epoch == 0 || writer_epoch == 0 || watermark == 0 {
        return Err(PlatformError::Tampered);
    }
    Ok(LedgerState {
        physical_identity,
        key_epoch,
        writer_epoch,
        watermark,
    })
}

#[cfg(windows)]
fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, PlatformError> {
    bytes
        .get(offset..offset + 8)
        .ok_or(PlatformError::Tampered)?
        .try_into()
        .map(u64::from_be_bytes)
        .map_err(map_slice_error)
}

#[cfg(windows)]
fn random_nonzero_u64() -> Result<u64, PlatformError> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes).map_err(map_random_error)?;
    let value = u64::from_be_bytes(bytes);
    if value == 0 {
        return Err(PlatformError::Unavailable);
    }
    Ok(value)
}

#[cfg(windows)]
fn map_random_error(_error: getrandom::Error) -> PlatformError {
    PlatformError::Unavailable
}

#[cfg(windows)]
fn map_slice_error(_error: std::array::TryFromSliceError) -> PlatformError {
    PlatformError::Tampered
}
