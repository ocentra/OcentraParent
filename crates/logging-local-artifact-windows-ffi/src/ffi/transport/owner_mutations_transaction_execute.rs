use super::*;
use crate::constants::{
    BRIDGE_DIRECTORY, INTENTS_DIRECTORY, MUTATION_OWNER_DIRECTORY, TRANSACTION_INSTALLED_PHASE,
    TRANSACTION_OPERATION, TRANSACTION_QUARANTINED_PHASE, TRANSACTION_STAGED_PHASE,
};

#[path = "owner_mutations_transaction_execute_cleanup.rs"]
mod cleanup;

pub(super) fn write_stages(
    session: &mut MutationSession<'_>,
    request_id: &str,
    descriptor: &str,
    staged: &mut [StagedMutation],
    plans: &mut [TransactionPlan],
) -> Result<IntentRecord, ArtifactError> {
    let intent = IntentRecord::Transaction {
        schema: 1,
        request_id: request_id.to_owned(),
        relative_paths: plans
            .iter()
            .map(|plan| plan.mutation.relative_path().to_owned())
            .collect(),
        descriptor: descriptor.to_owned(),
        staged: staged.to_vec(),
        phase: TransactionPhase::Prepared,
    };
    write_intent(
        &session.owner.root_path,
        request_id,
        &intent,
        session.metadata.intent_directory()?,
    )?;
    for plan in plans.iter_mut() {
        if let Mutation::Replace { payload, .. } = &plan.mutation {
            let stage = write_stage(session, request_id, &intent, plan, payload)?;
            let item = staged
                .iter_mut()
                .find(|item| item.relative_path == plan.mutation.relative_path())
                .ok_or(ArtifactError::RecoveryRequired)?;
            item.installed_identity =
                Some(identity_record(verify_metadata(&stage, false)?.identity));
            plan.stage = Some(stage);
        }
    }
    session.metadata.intent_directory()?.sync_directory()?;
    let staged_ready = intent.transaction_with_staged(staged.to_vec())?;
    replace_intent(
        &session.owner.root_path,
        request_id,
        &staged_ready,
        session.metadata.intent_directory()?,
        TRANSACTION_STAGED_PHASE,
    )?;
    Ok(staged_ready)
}

fn write_stage(
    session: &MutationSession<'_>,
    _request_id: &str,
    intent: &IntentRecord,
    plan: &TransactionPlan,
    payload: &[u8],
) -> Result<OwnedFile, ArtifactError> {
    let stage_name = intent_stage_name(intent, plan)?;
    let stage_path = session
        .owner
        .root_path
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(INTENTS_DIRECTORY)
        .join(stage_name);
    let mut stage = OwnedFile::create_new_mutation_file(&stage_path)?;
    stage.write_bounded(payload)?;
    stage.sync_file()?;
    verify_metadata(&stage, false)?;
    Ok(stage)
}

pub(super) fn quarantine(
    session: &mut MutationSession<'_>,
    request_id: &str,
    intent: &IntentRecord,
    plans: &mut [TransactionPlan],
) -> Result<IntentRecord, ArtifactError> {
    for (index, plan) in plans.iter_mut().enumerate() {
        if let Some(target) = plan.target.take() {
            let name = intent_quarantine_name(intent, index)?;
            target.rename_into(plan.chain.leaf()?, &name)?;
            plan.chain.leaf()?.sync_directory()?;
        } else {
            let name = format!("{request_id}.quarantine-{index}");
            reject_existing_sibling(&session.owner.root_path, &plan.chain, &name)?;
        }
    }
    let quarantined = intent.transaction_with_phase(TransactionPhase::Quarantined)?;
    replace_intent(
        &session.owner.root_path,
        request_id,
        &quarantined,
        session.metadata.intent_directory()?,
        TRANSACTION_QUARANTINED_PHASE,
    )?;
    Ok(quarantined)
}

pub(super) fn install(
    session: &mut MutationSession<'_>,
    request_id: &str,
    intent: &IntentRecord,
    plans: &mut [TransactionPlan],
) -> Result<IntentRecord, ArtifactError> {
    for plan in plans.iter_mut() {
        if let Some(stage) = plan.stage.take() {
            stage.rename_into(plan.chain.leaf()?, &plan.leaf)?;
            plan.chain.leaf()?.sync_directory()?;
        }
    }
    let installed = intent.transaction_with_phase(TransactionPhase::Installed)?;
    replace_intent(
        &session.owner.root_path,
        request_id,
        &installed,
        session.metadata.intent_directory()?,
        TRANSACTION_INSTALLED_PHASE,
    )?;
    Ok(installed)
}

pub(super) fn finalize(
    session: &mut MutationSession<'_>,
    request_id: &str,
    descriptor: &str,
    mutation_count: usize,
    staged: Vec<StagedMutation>,
    plans: Vec<TransactionPlan>,
) -> Result<MutationReceipt, ArtifactError> {
    cleanup::remove_quarantines(session, &staged, &plans)?;
    cleanup::verify_absence(session, &plans)?;
    session.verify_current()?;
    let count = u32::try_from(mutation_count).map_err(|_| ArtifactError::SizeLimit)?;
    let receipt = write_receipt(
        &session.owner.root_path,
        request_id,
        TRANSACTION_OPERATION,
        TRANSACTION_OPERATION,
        descriptor,
        ReceiptOutcome::TransactionCommitted { count },
        session.metadata.receipt_directory()?,
    )?;
    let _ = remove_intent(
        &session.owner.root_path,
        request_id,
        session.metadata.intent_directory()?,
    );
    Ok(receipt)
}
