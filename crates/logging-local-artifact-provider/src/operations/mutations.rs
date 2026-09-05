#[path = "mutations_transaction.rs"]
mod transaction;

use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;

use super::{FailureDisposition, LeaseRequirement, LeaseState, OperationExecution, ProviderError};
use crate::protocol::{self, ProviderPayload, ValidatedOperation, ValidatedRequest};

pub(super) fn execute(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    operation: &ValidatedOperation,
) -> Result<OperationExecution, ProviderError> {
    match operation {
        ValidatedOperation::Append {
            relative_path,
            payload_base64,
        } => append(session, lease, request, relative_path, payload_base64),
        ValidatedOperation::Replace {
            relative_path,
            payload_base64,
        } => replace(session, lease, request, relative_path, payload_base64),
        ValidatedOperation::Remove { relative_path } => {
            remove(session, lease, request, relative_path)
        }
        ValidatedOperation::RemoveTree { relative_path } => {
            remove_tree(session, lease, request, relative_path)
        }
        ValidatedOperation::ApplyTransaction { mutations } => {
            transaction::apply_transaction(session, lease, request, mutations)
        }
        _ => Err(ProviderError::new(
            protocol::text::UNSUPPORTED_OPERATION,
            FailureDisposition::Continue,
        )),
    }
}

fn append(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &crate::protocol::ProviderRelativePath,
    payload_base64: &ProviderPayload,
) -> Result<OperationExecution, ProviderError> {
    super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Required)?;
    let payload = transaction::decode_payload(payload_base64, super::PayloadKind::Append)?;
    if payload.last().copied() != Some(b'\n') {
        return Err(ProviderError::new(
            protocol::text::APPEND_NOT_NEWLINE_TERMINATED,
            FailureDisposition::Continue,
        ));
    }
    let request_id = request.request_id().text();
    let path = relative_path.text();
    let receipt = session
        .append(&request_id, &path, &payload)
        .map_err(|error| super::map_owner_error(&error))?;
    super::values::append_result(
        &receipt,
        request,
        relative_path,
        super::PayloadLength(payload.len()),
    )
}

fn replace(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &crate::protocol::ProviderRelativePath,
    payload_base64: &ProviderPayload,
) -> Result<OperationExecution, ProviderError> {
    super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Required)?;
    let payload = transaction::decode_payload(payload_base64, super::PayloadKind::Replace)?;
    let request_id = request.request_id().text();
    let path = relative_path.text();
    let receipt = session
        .replace(&request_id, &path, &payload)
        .map_err(|error| super::map_owner_error(&error))?;
    super::values::replace_result(
        &receipt,
        request,
        relative_path,
        super::PayloadLength(payload.len()),
    )
}

fn remove(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &crate::protocol::ProviderRelativePath,
) -> Result<OperationExecution, ProviderError> {
    super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Required)?;
    let request_id = request.request_id().text();
    let path = relative_path.text();
    let receipt = session
        .remove(&request_id, &path)
        .map_err(|error| super::map_owner_error(&error))?;
    super::values::remove_result(
        &receipt,
        request,
        relative_path,
        super::values::ReceiptOperation::Remove,
    )
}

fn remove_tree(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &crate::protocol::ProviderRelativePath,
) -> Result<OperationExecution, ProviderError> {
    super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Required)?;
    let request_id = request.request_id().text();
    let path = relative_path.text();
    let receipt = session
        .remove_tree(&request_id, &path)
        .map_err(|error| super::map_owner_error(&error))?;
    super::values::remove_result(
        &receipt,
        request,
        relative_path,
        super::values::ReceiptOperation::RemoveTree,
    )
}
