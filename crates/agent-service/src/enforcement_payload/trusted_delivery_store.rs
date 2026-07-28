use std::fs::{read, OpenOptions};
use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::constants;
use sha2::{Digest, Sha256};

use super::trusted_delivery::TrustedDeliveryDirectory;
use super::EnforcementText;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TrustedDeliveryRecordPath(PathBuf);

impl TrustedDeliveryRecordPath {
    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TrustedDeliveryReceiptPath(PathBuf);

impl TrustedDeliveryReceiptPath {
    fn path(&self) -> &Path {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TrustedDeliveryKey(EnforcementText);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TrustedDeliveryStoreError {
    Missing,
    Replay,
    Other,
}

pub(crate) fn read_record(
    directory: &TrustedDeliveryDirectory,
    delivery_id: &EnforcementText,
) -> Result<Vec<u8>, TrustedDeliveryStoreError> {
    read(record_path(directory, delivery_id).path()).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            TrustedDeliveryStoreError::Missing
        } else {
            TrustedDeliveryStoreError::Other
        }
    })
}

pub(crate) fn create_receipt(
    directory: &TrustedDeliveryDirectory,
    delivery_id: &EnforcementText,
) -> Result<(), TrustedDeliveryStoreError> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(receipt_path(directory, delivery_id).path())
    {
        Ok(receipt) => {
            drop(receipt);
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            Err(TrustedDeliveryStoreError::Replay)
        }
        Err(_) => Err(TrustedDeliveryStoreError::Other),
    }
}

pub(crate) fn record_path(
    directory: &TrustedDeliveryDirectory,
    delivery_id: &EnforcementText,
) -> TrustedDeliveryRecordPath {
    let TrustedDeliveryKey(EnforcementText(key)) = delivery_key(delivery_id);
    let mut path = directory.path().join(key);
    path.set_extension(constants::enforcement::TRUSTED_DELIVERY_RECORD_EXTENSION);
    TrustedDeliveryRecordPath(path)
}

fn receipt_path(
    directory: &TrustedDeliveryDirectory,
    delivery_id: &EnforcementText,
) -> TrustedDeliveryReceiptPath {
    let TrustedDeliveryKey(EnforcementText(key)) = delivery_key(delivery_id);
    let mut path = directory.path().join(key);
    path.set_extension(constants::enforcement::TRUSTED_DELIVERY_RECEIPT_EXTENSION);
    TrustedDeliveryReceiptPath(path)
}

fn delivery_key(delivery_id: &EnforcementText) -> TrustedDeliveryKey {
    let mut hasher = Sha256::new();
    hasher.update(delivery_id.0.as_bytes());
    TrustedDeliveryKey(EnforcementText(format!("{:x}", hasher.finalize())))
}
