use super::*;
use crate::constants::{
    REMOVE_OPERATION, REMOVE_TREE_CHILD_REQUIRED, REMOVE_TREE_OPERATION, REPLACE_OPERATION,
};

pub(super) fn item(item: &StagedMutation) -> Result<(), ArtifactError> {
    match item.operation.as_str() {
        REPLACE_OPERATION | REMOVE_OPERATION => {
            validate_relative(&item.relative_path)?;
        }
        REMOVE_TREE_OPERATION => validate_tree(item)?,
        _ => return Err(ArtifactError::RequestIdConflict),
    }
    Ok(())
}

fn validate_tree(item: &StagedMutation) -> Result<(), ArtifactError> {
    validate_directory_relative(&item.relative_path)?;
    if item.relative_path.is_empty() {
        return Err(ArtifactError::InvalidPath(REMOVE_TREE_CHILD_REQUIRED));
    }
    Ok(())
}

pub(super) fn target(target: Option<&OwnedFile>, is_tree: bool) -> Result<(), ArtifactError> {
    if let Some(target) = target {
        verify_transaction_kind(target, is_tree)?;
    }
    Ok(())
}
