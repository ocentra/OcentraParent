use super::*;
use crate::constants::REMOVE_TREE_CHILD_REQUIRED;

pub(super) fn inputs(mutations: &[Mutation]) -> Result<(), ArtifactError> {
    if mutations.is_empty() || mutations.len() > crate::owner_types::MAX_TRANSACTION_MUTATIONS {
        return Err(ArtifactError::SizeLimit);
    }
    let mut paths = HashSet::with_capacity(mutations.len());
    for mutation in mutations {
        validate_mutation(mutation)?;
        if !paths.insert(transaction_path_key(mutation.relative_path())) {
            return Err(ArtifactError::RequestIdConflict);
        }
        bounded_payload(mutation.payload().unwrap_or(&[]))?;
    }
    reject_overlapping_paths(mutations)
}

fn validate_mutation(mutation: &Mutation) -> Result<(), ArtifactError> {
    match mutation {
        Mutation::RemoveTree { .. } => {
            validate_directory_relative(mutation.relative_path())?;
            if mutation.relative_path().is_empty() {
                return Err(ArtifactError::InvalidPath(REMOVE_TREE_CHILD_REQUIRED));
            }
        }
        Mutation::Append { .. } | Mutation::Replace { .. } | Mutation::Remove { .. } => {
            validate_relative(mutation.relative_path())?;
        }
    }
    Ok(())
}

fn reject_overlapping_paths(mutations: &[Mutation]) -> Result<(), ArtifactError> {
    for (index, mutation) in mutations.iter().enumerate() {
        if has_other_overlap(mutations, index, mutation.relative_path()) {
            return Err(ArtifactError::RequestIdConflict);
        }
    }
    Ok(())
}

fn has_other_overlap(mutations: &[Mutation], index: usize, path: &str) -> bool {
    mutations.iter().enumerate().any(|(other_index, other)| {
        other_index != index
            && (is_descendant_path(path, other.relative_path())
                || is_descendant_path(other.relative_path(), path))
    })
}

pub(super) fn single_remove_tree(mutations: &[Mutation]) -> Option<&str> {
    match mutations {
        [Mutation::RemoveTree { relative_path }] => Some(relative_path.as_str()),
        _ => None,
    }
}

pub(super) fn contains_append(mutations: &[Mutation]) -> bool {
    mutations
        .iter()
        .any(|mutation| matches!(mutation, Mutation::Append { .. }))
}
