#[path = "value_dispatch.rs"]
mod dispatch;
#[path = "value_encoding.rs"]
mod encoding;
#[path = "value_read.rs"]
mod read;
#[path = "receipt_results.rs"]
mod receipts;
#[path = "value_results.rs"]
mod results;

use ocentra_parent_logging_core::local_artifact_mutation::{
    LocalArtifactMutationReceipt, LocalArtifactMutationSession,
};

use super::{LeaseState, MutationCount, OperationExecution, PayloadLength, ProviderError};
use crate::protocol::{ProviderRelativePath, ValidatedOperation, ValidatedRequest, WireIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReceiptOperation {
    Append,
    Replace,
    Remove,
    RemoveTree,
    Transaction,
}

pub(super) fn wire_identity(
    identity: ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactIdentity,
) -> WireIdentity {
    encoding::wire_identity(identity)
}

pub(super) fn execute(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    operation: &ValidatedOperation,
) -> Result<OperationExecution, ProviderError> {
    dispatch::execute(session, lease, request, operation)
}

pub(super) fn append_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
    payload_length: PayloadLength,
) -> Result<OperationExecution, ProviderError> {
    receipts::append_result(receipt, request, relative_path, payload_length)
}

pub(super) fn replace_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
    payload_length: PayloadLength,
) -> Result<OperationExecution, ProviderError> {
    receipts::replace_result(receipt, request, relative_path, payload_length)
}

pub(super) fn remove_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
    operation: ReceiptOperation,
) -> Result<OperationExecution, ProviderError> {
    receipts::remove_result(receipt, request, relative_path, operation)
}

pub(super) fn transaction_result(
    receipt: &LocalArtifactMutationReceipt,
    request: &ValidatedRequest,
    mutation_count: MutationCount,
) -> Result<OperationExecution, ProviderError> {
    receipts::transaction_result(receipt, request, mutation_count)
}
