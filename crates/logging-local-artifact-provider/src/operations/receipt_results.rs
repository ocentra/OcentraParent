use ocentra_parent_logging_core::local_artifact_mutation::{
    LocalArtifactMutationOutcome, LocalArtifactMutationReceipt,
};
use serde_json::json;

use super::super::{MutationCount, OperationExecution, PayloadLength, ProviderError};
use super::ReceiptOperation;
use crate::protocol::types::ResponseResult;
use crate::protocol::{ProviderRelativePath, ValidatedRequest};

pub(super) fn append_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
    payload_length: PayloadLength,
) -> Result<OperationExecution, ProviderError> {
    super::results::verify_receipt(receipt, request, ReceiptOperation::Append, relative_path)?;
    if let LocalArtifactMutationOutcome::Appended { length, .. } = receipt.outcome() {
        if *length != payload_length.0 as u64 {
            return Err(super::results::unexpected_outcome());
        }
        return Ok(success(crate::protocol::text::object(vec![
            (
                crate::protocol::text::TextId::WrittenKey,
                json!(payload_length.0),
            ),
            (
                crate::protocol::text::TextId::ReplayedKey,
                json!(receipt.replayed()),
            ),
        ])));
    }
    if matches!(
        receipt.outcome(),
        LocalArtifactMutationOutcome::Unsupported { .. }
    ) {
        return Err(super::results::unsupported_failure());
    }
    Err(super::results::unexpected_outcome())
}

pub(super) fn replace_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
    payload_length: PayloadLength,
) -> Result<OperationExecution, ProviderError> {
    super::results::verify_receipt(receipt, request, ReceiptOperation::Replace, relative_path)?;
    if matches!(receipt.outcome(), LocalArtifactMutationOutcome::Replaced) {
        return Ok(success(crate::protocol::text::object(vec![
            (
                crate::protocol::text::TextId::WrittenKey,
                json!(payload_length.0),
            ),
            (
                crate::protocol::text::TextId::ReplayedKey,
                json!(receipt.replayed()),
            ),
        ])));
    }
    if matches!(
        receipt.outcome(),
        LocalArtifactMutationOutcome::Unsupported { .. }
    ) {
        return Err(super::results::unsupported_failure());
    }
    Err(super::results::unexpected_outcome())
}

pub(super) fn remove_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
    operation: ReceiptOperation,
) -> Result<OperationExecution, ProviderError> {
    super::results::verify_receipt(receipt, request, operation, relative_path)?;
    if let LocalArtifactMutationOutcome::Removed { existed } = receipt.outcome() {
        return Ok(success(crate::protocol::text::object(vec![
            (crate::protocol::text::TextId::RemovedKey, json!(existed)),
            (
                crate::protocol::text::TextId::ReplayedKey,
                json!(receipt.replayed()),
            ),
        ])));
    }
    if matches!(
        receipt.outcome(),
        LocalArtifactMutationOutcome::Unsupported { .. }
    ) {
        return Err(super::results::unsupported_failure());
    }
    Err(super::results::unexpected_outcome())
}

pub(super) fn transaction_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    mutation_count: MutationCount,
) -> Result<OperationExecution, ProviderError> {
    let marker = crate::protocol::ProviderRelativePath::transaction_marker();
    super::results::verify_receipt(receipt, request, ReceiptOperation::Transaction, &marker)?;
    if let LocalArtifactMutationOutcome::TransactionCommitted { count } = receipt.outcome() {
        // CAST-JUSTIFICATION: protocol validation bounds this count to 256.
        if *count != mutation_count.0 as u32 {
            return Err(super::results::unexpected_outcome());
        }
        return Ok(success(crate::protocol::text::object(vec![
            (
                crate::protocol::text::TextId::AppliedKey,
                json!(mutation_count.0),
            ),
            (
                crate::protocol::text::TextId::ReplayedKey,
                json!(receipt.replayed()),
            ),
        ])));
    }
    if matches!(
        receipt.outcome(),
        LocalArtifactMutationOutcome::Unsupported { .. }
    ) {
        return Err(super::results::unsupported_failure());
    }
    Err(super::results::unexpected_outcome())
}

fn success(result: ResponseResult) -> OperationExecution {
    OperationExecution {
        result,
        disposition: super::super::ExecutionDisposition::Continue,
    }
}
