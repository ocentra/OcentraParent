use super::*;

pub(super) fn path(
    chain: &DirectoryChain,
    request_id: &str,
    index: usize,
    item: &StagedMutation,
) -> Result<Option<PathBuf>, ArtifactError> {
    let expected = item
        .target_identity
        .as_ref()
        .map(|_| format!("{request_id}.quarantine-{index}"));
    if item.quarantine_name != expected {
        return Err(ArtifactError::RecoveryRequired);
    }
    let parent = chain.paths.last().ok_or(ArtifactError::RecoveryRequired)?;
    match expected {
        Some(name) => Ok(Some(parent.join(name))),
        None => reject_candidate(parent, request_id, index, &item.operation),
    }
}

fn reject_candidate(
    parent: &Path,
    request_id: &str,
    index: usize,
    operation: &str,
) -> Result<Option<PathBuf>, ArtifactError> {
    let candidate = parent.join(format!("{request_id}.quarantine-{index}"));
    if optional_mutation_file_for_operation(&candidate, operation)?.is_some() {
        return Err(ArtifactError::OwnershipChanged);
    }
    Ok(None)
}

pub(super) fn open(
    path: &Option<PathBuf>,
    operation: &str,
) -> Result<Option<OwnedFile>, ArtifactError> {
    match path {
        Some(path) => optional_mutation_file_for_operation(path, operation),
        None => Ok(None),
    }
}
