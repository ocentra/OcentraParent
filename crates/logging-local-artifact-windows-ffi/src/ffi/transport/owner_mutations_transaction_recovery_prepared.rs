use super::*;
use crate::constants::TRANSACTION_QUARANTINED_PHASE;

pub(super) fn recover(
    session: &mut MutationSession<'_>,
    record: &IntentRecord,
    input: &super::TransactionRecoveryInput,
    mut plans: Vec<TransactionRecoveryPlan>,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    if plans.iter().all(|plan| plan.quarantine.is_none()) {
        discard(session, input, &mut plans)?;
        return Ok(None);
    }
    quarantine(session, record, input, &mut plans)?;
    super::finish::complete(session, record, input, plans)
}

fn discard(
    session: &mut MutationSession<'_>,
    input: &super::TransactionRecoveryInput,
    plans: &mut [TransactionRecoveryPlan],
) -> Result<(), ArtifactError> {
    for plan in plans {
        discard_plan(plan)?;
    }
    session.metadata.intent_directory()?.sync_directory()?;
    remove_intent(
        &session.owner.root_path,
        &input.request_id,
        session.metadata.intent_directory()?,
    )
}

fn discard_plan(plan: &mut TransactionRecoveryPlan) -> Result<(), ArtifactError> {
    verify_transaction_prestate(plan)?;
    match plan.stage.as_mut() {
        Some(stage) => {
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
            verify_expected_new_file(stage, expected, digest)?;
            stage.mark_deleted()
        }
        None if plan.item.installed_identity.is_some() => Err(ArtifactError::RecoveryRequired),
        None => Ok(()),
    }
}

fn quarantine(
    session: &mut MutationSession<'_>,
    record: &IntentRecord,
    input: &super::TransactionRecoveryInput,
    plans: &mut [TransactionRecoveryPlan],
) -> Result<(), ArtifactError> {
    for plan in plans {
        quarantine_plan(plan)?;
    }
    let progressed = record.transaction_with_phase(TransactionPhase::Quarantined)?;
    replace_intent(
        &session.owner.root_path,
        &input.request_id,
        &progressed,
        session.metadata.intent_directory()?,
        TRANSACTION_QUARANTINED_PHASE,
    )
}

fn quarantine_plan(plan: &mut TransactionRecoveryPlan) -> Result<(), ArtifactError> {
    if let Some(quarantine) = plan.quarantine.as_ref() {
        if plan.target.is_some() {
            return Err(ArtifactError::OwnershipChanged);
        }
        return verify_transaction_old(plan, quarantine);
    }
    let Some(target) = plan.target.take() else {
        return if plan.item.target_identity.is_some() {
            Err(ArtifactError::OwnershipChanged)
        } else {
            Ok(())
        };
    };
    let name = plan
        .item
        .quarantine_name
        .as_deref()
        .ok_or(ArtifactError::RecoveryRequired)?
        .to_owned();
    verify_transaction_old(plan, &target)?;
    target.rename_into(plan.chain.leaf()?, &name)?;
    plan.chain.leaf()?.sync_directory()?;
    let path = plan
        .chain
        .paths
        .last()
        .cloned()
        .ok_or(ArtifactError::RecoveryRequired)?
        .join(name);
    plan.quarantine_path = Some(path.clone());
    plan.quarantine = optional_mutation_file_for_operation(&path, &plan.item.operation)?;
    plan.quarantine
        .as_ref()
        .map(|_| ())
        .ok_or(ArtifactError::RecoveryRequired)
}
