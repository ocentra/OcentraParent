use sha2::{Digest, Sha256};
use std::{fs, path::Path};

use super::{hex, platform, record::Record, Error};

pub(super) struct PreviousActiveRecord {
    pub(super) record: Record,
    pub(super) epoch: Vec<u8>,
}

pub(super) fn preserve_active(
    binding: &[u8],
    record_path: &Path,
) -> Result<Option<PreviousActiveRecord>, Error> {
    let encoded = match fs::read(record_path) {
        Ok(encoded) => encoded,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_error) => return Err(Error::Io),
    };
    let record = match serde_json::from_slice::<Record>(&encoded) {
        Ok(record) => record,
        Err(_error) => return Ok(None),
    };
    let epoch = match platform::current(binding) {
        Ok(epoch) => epoch,
        Err(Error::Missing) | Err(Error::Unseal) => return Ok(None),
        Err(error) => return Err(error),
    };
    let valid = record.epoch_hash == hex(Sha256::digest(&epoch))
        && platform::unprotect(&record.ciphertext, binding).is_ok();
    Ok(valid.then_some(PreviousActiveRecord { record, epoch }))
}
