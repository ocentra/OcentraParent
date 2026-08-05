use sha2::{Digest, Sha256};
use std::{fs, path::Path};

use super::{
    platform,
    record::{hex, Record},
    Error,
};

pub(super) fn is_present(root: &Path, binding_hex: &str) -> Result<bool, Error> {
    let binding = decode_hex(binding_hex).ok_or(Error::Invalid)?;
    let record_name = hex(Sha256::digest(&binding));
    let encoded = match fs::read(root.join(format!("{record_name}.sealed"))) {
        Ok(encoded) => encoded,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(_error) => return Err(Error::Io),
    };
    let record = match serde_json::from_slice::<Record>(&encoded) {
        Ok(record) => record,
        Err(_error) => return Ok(false),
    };
    let epoch = match platform::current(&binding) {
        Ok(epoch) => epoch,
        Err(Error::Missing) | Err(Error::Unseal) => return Ok(false),
        Err(error) => return Err(error),
    };
    Ok(record.epoch_hash == hex(Sha256::digest(&epoch))
        && platform::unprotect(&record.ciphertext, &binding).is_ok())
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    value.len().is_multiple_of(2).then_some(()).and_then(|()| {
        value
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                std::str::from_utf8(pair)
                    .ok()
                    .and_then(|pair| u8::from_str_radix(pair, 16).ok())
            })
            .collect()
    })
}
