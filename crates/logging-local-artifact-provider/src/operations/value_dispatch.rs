use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;

use super::super::{FailureDisposition, LeaseState, OperationExecution, ProviderError};
use crate::protocol::{self, ValidatedOperation, ValidatedRequest};

pub(super) fn execute(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    operation: &ValidatedOperation,
) -> Result<OperationExecution, ProviderError> {
    match operation {
        ValidatedOperation::Recover => super::read::recover(session, lease, request),
        ValidatedOperation::EnsureDirectory { relative_path } => {
            super::read::ensure_directory(session, lease, request, relative_path)
        }
        ValidatedOperation::SyncDirectory { relative_path } => {
            super::read::sync_directory(session, lease, request, relative_path)
        }
        ValidatedOperation::Stat { relative_path } => {
            super::read::stat(session, lease, request, relative_path)
        }
        ValidatedOperation::ReadSnapshot {
            relative_path,
            maximum_bytes,
        } => super::read::read_snapshot(session, lease, request, relative_path, *maximum_bytes),
        ValidatedOperation::List { relative_path } => {
            super::read::list(session, lease, request, relative_path)
        }
        _ => Err(ProviderError::new(
            protocol::text::UNSUPPORTED_OPERATION,
            FailureDisposition::Continue,
        )),
    }
}
