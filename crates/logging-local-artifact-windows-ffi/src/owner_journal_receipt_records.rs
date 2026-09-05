use crate::constants::{
    APPEND_OPERATION, REMOVE_OPERATION, REMOVE_TREE_OPERATION, REPLACE_OPERATION,
    TRANSACTION_OPERATION,
};
use crate::error::ArtifactError;
use crate::owner_paths::{validate_directory_relative, validate_relative};
use crate::owner_types::{ReceiptOutcome, MAX_TRANSACTION_MUTATIONS};
use crate::platform::windows::MAX_ARTIFACT_BYTES;

use super::{ReceiptRecord, ReceiptRecordOutcome};

const SHA256_HEX_CHARS: usize = 64;

impl ReceiptRecord {
    pub(crate) fn validate(&self) -> Result<(), ArtifactError> {
        validate_common(self.schema, &self.request_id, &self.descriptor)?;
        if self.operation == APPEND_OPERATION {
            return validate_file_path(&self.relative_path).and(
                self.outcome.appended_bounds().map_or_else(
                    || Err(invalid_record()),
                    |(offset, length)| validate_append_bounds(offset, length),
                ),
            );
        }
        if self.operation == REPLACE_OPERATION {
            return validate_file_path(&self.relative_path).and(require(matches!(
                self.outcome,
                ReceiptRecordOutcome::Replaced
            )));
        }
        if self.operation == REMOVE_OPERATION {
            return validate_file_path(&self.relative_path)
                .and(validate_no_payload_descriptor(
                    &super::JournalText(REMOVE_OPERATION),
                    &self.relative_path,
                    &self.descriptor,
                ))
                .and(require(matches!(
                    self.outcome,
                    ReceiptRecordOutcome::Removed { .. }
                )));
        }
        if self.operation == REMOVE_TREE_OPERATION {
            return validate_tree_path(&self.relative_path)
                .and(validate_no_payload_descriptor(
                    &super::JournalText(REMOVE_TREE_OPERATION),
                    &self.relative_path,
                    &self.descriptor,
                ))
                .and(require(matches!(
                    self.outcome,
                    ReceiptRecordOutcome::Removed { .. }
                )));
        }
        if self.operation == TRANSACTION_OPERATION {
            return require(self.relative_path == TRANSACTION_OPERATION)
                .and(validate_transaction_outcome(&self.outcome));
        }
        Err(invalid_record())
    }
}

fn validate_common<R, D>(schema: u32, request_id: &R, descriptor: &D) -> Result<(), ArtifactError>
where
    R: std::fmt::Display + ?Sized,
    D: std::fmt::Display + ?Sized,
{
    require(
        schema == super::JOURNAL_SCHEMA
            && super::validate_request_id(&request_id.to_string()).is_ok()
            && validate_digest(descriptor).is_ok(),
    )
}

fn validate_digest<T>(value: &T) -> Result<(), ArtifactError>
where
    T: std::fmt::Display + ?Sized,
{
    let text = value.to_string();
    require(
        text.len() == SHA256_HEX_CHARS
            && text
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
    )
}

fn validate_file_path<T>(value: &T) -> Result<(), ArtifactError>
where
    T: std::fmt::Display + ?Sized,
{
    require(validate_relative(&value.to_string()).is_ok())
}

fn validate_no_payload_descriptor<P, D>(
    operation: &super::JournalText<'_>,
    relative_path: &P,
    descriptor: &D,
) -> Result<(), ArtifactError>
where
    P: std::fmt::Display + ?Sized,
    D: std::fmt::Display + ?Sized,
{
    let expected: String = super::request_descriptor(operation, relative_path, None);
    require(descriptor.to_string() == expected)
}

fn validate_tree_path<T>(value: &T) -> Result<(), ArtifactError>
where
    T: std::fmt::Display + ?Sized,
{
    let text = value.to_string();
    require(!text.is_empty() && validate_directory_relative(&text).is_ok())
}

fn validate_append_bounds(offset: u64, length: u64) -> Result<(), ArtifactError> {
    require(
        offset <= MAX_ARTIFACT_BYTES
            && length <= MAX_ARTIFACT_BYTES
            && offset
                .checked_add(length)
                .is_some_and(|total| total <= MAX_ARTIFACT_BYTES),
    )
}

fn validate_transaction_outcome(outcome: &ReceiptRecordOutcome) -> Result<(), ArtifactError> {
    let committed = outcome.transaction_count().is_some_and(|count| {
        usize::try_from(count)
            .map(|value| value > 0 && value <= MAX_TRANSACTION_MUTATIONS)
            .unwrap_or(false)
    });
    require(
        committed
            || matches!(outcome, ReceiptRecordOutcome::Removed { .. })
            || outcome.is_transaction_unsupported(),
    )
}

fn require(condition: bool) -> Result<(), ArtifactError> {
    condition.then_some(()).ok_or_else(invalid_record)
}

fn invalid_record() -> ArtifactError {
    ArtifactError::RecoveryRequired
}

pub(super) fn record_outcome(outcome: &ReceiptOutcome) -> ReceiptRecordOutcome {
    match outcome {
        ReceiptOutcome::Appended { offset, length } => ReceiptRecordOutcome::Appended {
            offset: *offset,
            length: *length,
        },
        ReceiptOutcome::Replaced => ReceiptRecordOutcome::Replaced,
        ReceiptOutcome::Removed { existed } => ReceiptRecordOutcome::Removed { existed: *existed },
        ReceiptOutcome::TransactionCommitted { count } => {
            ReceiptRecordOutcome::TransactionCommitted { count: *count }
        }
        ReceiptOutcome::Unsupported { operation } => ReceiptRecordOutcome::Unsupported {
            operation: operation.clone(),
        },
    }
}
