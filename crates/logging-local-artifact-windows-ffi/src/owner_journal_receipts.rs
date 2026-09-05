use std::fmt::Display;
use std::path::Path;

use crate::constants::TRANSACTION_MUTATION_ERROR;
use crate::error::ArtifactError;

use super::*;

#[path = "owner_journal_receipt_decode.rs"]
mod decode;
#[path = "owner_journal_receipt_window.rs"]
mod window;

impl ReceiptRecordOutcome {
    pub(crate) fn appended_bounds(&self) -> Option<(u64, u64)> {
        if let Self::Appended { offset, length } = self {
            Some((*offset, *length))
        } else {
            None
        }
    }

    pub(crate) fn transaction_count(&self) -> Option<u32> {
        if let Self::TransactionCommitted { count } = self {
            Some(*count)
        } else {
            None
        }
    }

    pub(crate) fn is_transaction_unsupported(&self) -> bool {
        matches!(
            self,
            Self::Unsupported { operation } if operation == TRANSACTION_MUTATION_ERROR
        )
    }
}

pub(super) fn read_receipt<R, O, P, D>(
    root: &Path,
    request_id: R,
    operation: O,
    relative_path: P,
    descriptor: D,
) -> Result<Option<MutationReceipt>, ArtifactError>
where
    R: Display,
    O: Display,
    P: Display,
    D: Display,
{
    let path = receipt_path(root, &request_id);
    let mut file = match OwnedFile::open_existing_file(path.as_path()) {
        Ok(file) => file,
        Err(ArtifactError::NotFound) => return Ok(None),
        Err(error) => return Err(error),
    };
    let bytes = file.read_bounded(MAX_RECEIPT_BYTES)?;
    let record: ReceiptRecord =
        serde_json::from_slice(&bytes).map_err(|_| ArtifactError::RecoveryRequired)?;
    record.validate()?;
    if !record_matches(
        &record,
        &request_id,
        &operation,
        &relative_path,
        &descriptor,
    ) {
        return Err(ArtifactError::RequestIdConflict);
    }
    Ok(Some(decode::receipt(record)))
}

fn record_matches<R, O, P, D>(
    record: &ReceiptRecord,
    request_id: &R,
    operation: &O,
    relative_path: &P,
    descriptor: &D,
) -> bool
where
    R: Display + ?Sized,
    O: Display + ?Sized,
    P: Display + ?Sized,
    D: Display + ?Sized,
{
    record.schema == JOURNAL_SCHEMA
        && record.request_id == request_id.to_string()
        && record.operation == operation.to_string()
        && record.relative_path == relative_path.to_string()
        && record.descriptor == descriptor.to_string()
}

pub(super) fn write_receipt<R, O, P, D>(
    root: &Path,
    request_id: R,
    operation: O,
    relative_path: P,
    descriptor: D,
    outcome: ReceiptOutcome,
    receipt_directory: &OwnedFile,
) -> Result<MutationReceipt, ArtifactError>
where
    R: Display + descriptors::names::TempNameInput,
    O: Display,
    P: Display,
    D: Display,
{
    window::enforce(root)?;
    let record = ReceiptRecord {
        schema: JOURNAL_SCHEMA,
        request_id: request_id.to_string(),
        operation: operation.to_string(),
        relative_path: relative_path.to_string(),
        descriptor: descriptor.to_string(),
        outcome: record_outcome(&outcome),
    };
    record.validate()?;
    let bytes = serde_json::to_vec(&record).map_err(|_| ArtifactError::RecoveryRequired)?;
    if u64::try_from(bytes.len()).map_err(|_| ArtifactError::SizeLimit)? > MAX_RECEIPT_BYTES {
        return Err(ArtifactError::SizeLimit);
    }
    let receipt_name = format!("{request_id}{JSON_SUFFIX}");
    let temp_name = intent_temp_name::<_, _, String>(&request_id, &RECEIPT_OPERATION);
    let temp_path = root
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(RECEIPTS_DIRECTORY)
        .join(&temp_name);
    let mut temp = OwnedFile::create_new_file(&temp_path)?;
    temp.write_bytes(&bytes)?;
    temp.sync_file()?;
    temp.rename_into(receipt_directory, &receipt_name)?;
    receipt_directory.sync_directory()?;
    Ok(MutationReceipt::new(
        request_id.to_string(),
        operation.to_string(),
        relative_path.to_string(),
        outcome,
        false,
    ))
}
