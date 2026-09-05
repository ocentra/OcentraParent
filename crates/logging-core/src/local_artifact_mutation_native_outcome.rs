use super::super::LocalArtifactMutationOutcome;

pub(crate) fn outcome_from_native(
    outcome: &ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome,
) -> LocalArtifactMutationOutcome {
    match outcome {
        ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Appended {
            offset,
            length,
        } => LocalArtifactMutationOutcome::Appended {
            offset: *offset,
            length: *length,
        },
        ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Replaced => {
            LocalArtifactMutationOutcome::Replaced
        }
        ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Removed { existed } => {
            LocalArtifactMutationOutcome::Removed { existed: *existed }
        }
        ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::TransactionCommitted { count } => {
            LocalArtifactMutationOutcome::TransactionCommitted { count: *count }
        }
        ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Unsupported { operation } => {
            LocalArtifactMutationOutcome::Unsupported {
                operation: operation.clone(),
            }
        }
    }
}
