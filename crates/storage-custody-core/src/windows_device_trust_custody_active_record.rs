use sha2::{Digest, Sha256};

use super::{
    platform,
    record::{hex, Record},
    Error,
};

pub(super) fn record_is_active(record: &Record, binding: &[u8]) -> Result<bool, Error> {
    let epoch = match platform::current(binding) {
        Ok(epoch) => epoch,
        Err(Error::Missing) | Err(Error::Unseal) => return Ok(false),
        Err(error) => return Err(error),
    };
    Ok(record.epoch_hash == hex(Sha256::digest(&epoch))
        && platform::unprotect(&record.ciphertext, binding).is_ok())
}
