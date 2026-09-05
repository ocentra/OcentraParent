use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;

use super::{LeaseState, OperationExecution, ProviderError};
use crate::protocol::{ValidatedOperation, ValidatedRequest};

pub(super) fn execute(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    request: &ValidatedRequest,
) -> Result<OperationExecution, ProviderError> {
    match request.operation() {
        ValidatedOperation::BeginLease
        | ValidatedOperation::EndLease { .. }
        | ValidatedOperation::Shutdown => {
            super::lease::execute(session, lease, request, request.operation())
        }
        ValidatedOperation::Recover
        | ValidatedOperation::EnsureDirectory { .. }
        | ValidatedOperation::SyncDirectory { .. }
        | ValidatedOperation::Stat { .. }
        | ValidatedOperation::ReadSnapshot { .. }
        | ValidatedOperation::List { .. } => {
            super::values::execute(session, lease, request, request.operation())
        }
        ValidatedOperation::Append { .. }
        | ValidatedOperation::Replace { .. }
        | ValidatedOperation::Remove { .. }
        | ValidatedOperation::RemoveTree { .. }
        | ValidatedOperation::ApplyTransaction { .. } => {
            super::mutations::execute(session, lease, request, request.operation())
        }
    }
}
