use super::super::LocalArtifactMutation;

pub(crate) fn mutation_to_native(
    mutation: &LocalArtifactMutation,
) -> ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutation {
    match mutation {
        LocalArtifactMutation::Append { relative_path, payload } => {
            ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutation::Append {
                relative_path: relative_path.clone(),
                payload: payload.clone(),
            }
        }
        LocalArtifactMutation::Replace { relative_path, payload } => {
            ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutation::Replace {
                relative_path: relative_path.clone(),
                payload: payload.clone(),
            }
        }
        LocalArtifactMutation::Remove { relative_path } => {
            ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutation::Remove {
                relative_path: relative_path.clone(),
            }
        }
        LocalArtifactMutation::RemoveTree { relative_path } => {
            ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutation::RemoveTree {
                relative_path: relative_path.clone(),
            }
        }
    }
}
