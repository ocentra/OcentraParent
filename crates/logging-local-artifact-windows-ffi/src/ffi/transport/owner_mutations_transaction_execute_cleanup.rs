use super::*;
use crate::constants::{REMOVE_OPERATION, REMOVE_TREE_OPERATION, REPLACE_OPERATION};

pub(super) fn remove_quarantines(
    session: &mut MutationSession<'_>,
    staged: &[StagedMutation],
    plans: &[TransactionPlan],
) -> Result<(), ArtifactError> {
    for (index, plan) in plans.iter().enumerate() {
        remove_quarantine(session, staged, index, plan)?;
    }
    Ok(())
}

fn remove_quarantine(
    session: &mut MutationSession<'_>,
    staged: &[StagedMutation],
    index: usize,
    plan: &TransactionPlan,
) -> Result<(), ArtifactError> {
    if !plan.target_existed {
        return Ok(());
    }
    let name = staged
        .get(index)
        .and_then(|item| item.quarantine_name.as_deref())
        .ok_or(ArtifactError::RecoveryRequired)?;
    let parent = plan
        .chain
        .paths
        .last()
        .ok_or(ArtifactError::RecoveryRequired)?;
    let path = parent.join(name);
    let quarantine = optional_mutation_file_for_operation(&path, plan.mutation.operation_name())?
        .ok_or(ArtifactError::RecoveryRequired)?;
    if plan.mutation.operation_name() == REMOVE_TREE_OPERATION {
        remove_tree_contents(&path, &quarantine)?;
    }
    quarantine.mark_deleted()?;
    plan.chain.leaf()?.sync_directory()?;
    session.verify_chain(&plan.chain)
}

pub(super) fn verify_absence(
    session: &MutationSession<'_>,
    plans: &[TransactionPlan],
) -> Result<(), ArtifactError> {
    for plan in plans {
        verify_target_outcome(session, plan)?;
    }
    Ok(())
}

fn verify_target_outcome(
    _session: &MutationSession<'_>,
    plan: &TransactionPlan,
) -> Result<(), ArtifactError> {
    let parent = plan
        .chain
        .paths
        .last()
        .ok_or(ArtifactError::RecoveryRequired)?;
    let path = parent.join(&plan.leaf);
    let target = optional_mutation_file_for_operation(&path, plan.mutation.operation_name())?;
    match plan.mutation.operation_name() {
        REPLACE_OPERATION if target.is_some() => Ok(()),
        REPLACE_OPERATION => Err(ArtifactError::RecoveryRequired),
        REMOVE_OPERATION | REMOVE_TREE_OPERATION if target.is_none() => Ok(()),
        REMOVE_OPERATION | REMOVE_TREE_OPERATION => Err(ArtifactError::OwnershipChanged),
        _ => Err(ArtifactError::RequestIdConflict),
    }
}
