use super::*;
use crate::constants::{REMOVE_OPERATION, REMOVE_TREE_OPERATION, REPLACE_OPERATION};

pub(super) fn plans(
    session: &mut MutationSession<'_>,
    plans: &mut [TransactionRecoveryPlan],
) -> Result<(), ArtifactError> {
    for plan in plans {
        plan_cleanup(session, plan)?;
    }
    Ok(())
}

fn plan_cleanup(
    session: &mut MutationSession<'_>,
    plan: &mut TransactionRecoveryPlan,
) -> Result<(), ArtifactError> {
    if let Some(quarantine) = plan.quarantine.take() {
        cleanup_quarantine(plan, quarantine)?;
        plan.chain.leaf()?.sync_directory()?;
    }
    if let Some(stage) = plan.stage.as_mut() {
        stage.mark_deleted()?;
        session.metadata.intent_directory()?.sync_directory()?;
    }
    verify_final_target(plan)
}

fn cleanup_quarantine(
    plan: &TransactionRecoveryPlan,
    quarantine: OwnedFile,
) -> Result<(), ArtifactError> {
    if plan.item.operation == REMOVE_TREE_OPERATION {
        let path = plan
            .quarantine_path
            .as_deref()
            .ok_or(ArtifactError::RecoveryRequired)?;
        remove_tree_contents(path, &quarantine)?;
    }
    quarantine.mark_deleted()
}

fn verify_final_target(plan: &mut TransactionRecoveryPlan) -> Result<(), ArtifactError> {
    match plan.item.operation.as_str() {
        REPLACE_OPERATION => verify_replacement_target(plan),
        REMOVE_OPERATION | REMOVE_TREE_OPERATION => verify_removed_target(plan),
        _ => Err(ArtifactError::RequestIdConflict),
    }
}

fn verify_replacement_target(plan: &mut TransactionRecoveryPlan) -> Result<(), ArtifactError> {
    let expected = plan
        .item
        .installed_identity
        .as_ref()
        .ok_or(ArtifactError::RecoveryRequired)?;
    let digest = plan
        .item
        .payload_digest
        .as_deref()
        .ok_or(ArtifactError::RecoveryRequired)?;
    let target = plan
        .target
        .as_mut()
        .ok_or(ArtifactError::RecoveryRequired)?;
    verify_expected_new_file(target, expected, digest)
}

fn verify_removed_target(plan: &TransactionRecoveryPlan) -> Result<(), ArtifactError> {
    if optional_mutation_file_for_operation(&plan.target_path, &plan.item.operation)?.is_some() {
        return Err(ArtifactError::OwnershipChanged);
    }
    Ok(())
}
