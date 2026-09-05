use super::super::{FailureDisposition, ProviderError};
use super::ReceiptOperation;
use crate::protocol::{self, OperationName, ProviderRelativePath, ValidatedRequest};

pub(super) fn verify_receipt(
    receipt: &ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    operation: ReceiptOperation,
    relative_path: &ProviderRelativePath,
) -> Result<(), ProviderError> {
    let request_id = request.request_id().text();
    let path = relative_path.text();
    let expected_operation = match operation {
        ReceiptOperation::Append => OperationName::Append.text(),
        ReceiptOperation::Replace => OperationName::Replace.text(),
        ReceiptOperation::Remove => OperationName::Remove.text(),
        ReceiptOperation::RemoveTree => protocol::text::RECEIPT_REMOVE_TREE.text(),
        ReceiptOperation::Transaction => protocol::text::RECEIPT_TRANSACTION.text(),
    };
    if receipt.request_id() != request_id
        || receipt.operation() != expected_operation
        || receipt.relative_path() != path
    {
        return Err(ProviderError::new(
            protocol::text::UNEXPECTED_RECEIPT,
            FailureDisposition::Terminate,
        ));
    }
    Ok(())
}

pub(super) fn unsupported_failure() -> ProviderError {
    ProviderError::new(
        protocol::text::UNSUPPORTED_OUTCOME,
        FailureDisposition::Continue,
    )
}

pub(super) fn unexpected_outcome() -> ProviderError {
    ProviderError::new(
        protocol::text::UNEXPECTED_OUTCOME,
        FailureDisposition::Terminate,
    )
}
