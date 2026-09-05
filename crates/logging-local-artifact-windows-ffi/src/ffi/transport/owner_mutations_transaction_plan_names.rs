use super::*;
use crate::constants::{BRIDGE_DIRECTORY, INTENTS_DIRECTORY, MUTATION_OWNER_DIRECTORY};

pub(super) fn staged_name(request_id: &str, index: usize, mutation: &Mutation) -> Option<String> {
    match mutation {
        Mutation::Replace { .. } => Some(format!("{request_id}.stage-{index}")),
        Mutation::Remove { .. } => None,
        Mutation::Append { .. } | Mutation::RemoveTree { .. } => None,
    }
}

pub(super) fn quarantine_name(request_id: &str, index: usize) -> String {
    format!("{request_id}.quarantine-{index}")
}

pub(super) fn reject_stage_conflict(
    session: &MutationSession<'_>,
    name: Option<&str>,
) -> Result<(), ArtifactError> {
    let Some(name) = name else {
        return Ok(());
    };
    let path = session
        .owner
        .root_path
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(INTENTS_DIRECTORY)
        .join(name);
    if optional_mutation_file(&path)?.is_some() {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}
