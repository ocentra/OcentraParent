use super::*;
use crate::constants::{REMOVE_OPERATION, REMOVE_TREE_OPERATION, REPLACE_OPERATION};

pub(super) fn installation(
    session: &mut MutationSession<'_>,
    installing: bool,
    plans: &mut [TransactionRecoveryPlan],
) -> Result<(), ArtifactError> {
    for plan in plans {
        plan_installation(session, installing, plan)?;
    }
    Ok(())
}

fn plan_installation(
    session: &mut MutationSession<'_>,
    installing: bool,
    plan: &mut TransactionRecoveryPlan,
) -> Result<(), ArtifactError> {
    if installing && plan.item.target_identity.is_some() && plan.quarantine.is_none() {
        return Err(ArtifactError::RecoveryRequired);
    }
    if let Some(quarantine) = plan.quarantine.as_ref() {
        verify_transaction_old(plan, quarantine)?;
    }
    match plan.item.operation.as_str() {
        REPLACE_OPERATION => replace(session, installing, plan),
        REMOVE_OPERATION | REMOVE_TREE_OPERATION => remove(plan, installing),
        _ => Err(ArtifactError::RequestIdConflict),
    }
}

fn replace(
    session: &mut MutationSession<'_>,
    installing: bool,
    plan: &mut TransactionRecoveryPlan,
) -> Result<(), ArtifactError> {
    let expected = plan
        .item
        .installed_identity
        .as_ref()
        .copied()
        .ok_or(ArtifactError::RecoveryRequired)?;
    let digest = plan
        .item
        .payload_digest
        .clone()
        .ok_or(ArtifactError::RecoveryRequired)?;
    if plan.target.is_some() {
        verify_existing(session, plan, &expected, &digest)
    } else if installing {
        install(plan, &expected, &digest)
    } else {
        Err(ArtifactError::RecoveryRequired)
    }
}

fn verify_existing(
    session: &mut MutationSession<'_>,
    plan: &mut TransactionRecoveryPlan,
    expected: &IdentityRecord,
    digest: &str,
) -> Result<(), ArtifactError> {
    let target = plan
        .target
        .as_mut()
        .ok_or(ArtifactError::RecoveryRequired)?;
    verify_expected_new_file(target, expected, digest)?;
    if let Some(stage) = plan.stage.as_mut() {
        verify_expected_new_file(stage, expected, digest)?;
        stage.mark_deleted()?;
        session.metadata.intent_directory()?.sync_directory()?;
    }
    Ok(())
}

fn install(
    plan: &mut TransactionRecoveryPlan,
    expected: &IdentityRecord,
    digest: &str,
) -> Result<(), ArtifactError> {
    let mut stage = plan.stage.take().ok_or(ArtifactError::RecoveryRequired)?;
    verify_expected_new_file(&mut stage, expected, digest)?;
    stage.rename_into(plan.chain.leaf()?, &plan.leaf)?;
    drop(stage);
    plan.chain.leaf()?.sync_directory()?;
    let mut target =
        optional_mutation_file(&plan.target_path)?.ok_or(ArtifactError::RecoveryRequired)?;
    verify_expected_new_file(&mut target, expected, digest)?;
    plan.target = Some(target);
    Ok(())
}

fn remove(plan: &TransactionRecoveryPlan, installing: bool) -> Result<(), ArtifactError> {
    if plan.target.is_some() {
        return Err(ArtifactError::OwnershipChanged);
    }
    if plan.quarantine.is_none() && plan.item.target_identity.is_some() && !installing {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}
