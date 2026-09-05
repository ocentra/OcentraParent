use super::*;
use crate::constants::REMOVE_TREE_OPERATION;

#[path = "owner_mutations_transaction_recovery_plans_quarantine.rs"]
mod quarantine;
#[path = "owner_mutations_transaction_recovery_plans_stage.rs"]
mod stage;
#[path = "owner_mutations_transaction_recovery_plans_validate.rs"]
mod validate;

pub(super) fn open(
    session: &MutationSession<'_>,
    request_id: &str,
    staged: &[StagedMutation],
) -> Result<Vec<TransactionRecoveryPlan>, ArtifactError> {
    staged
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, item)| open_one(session, request_id, index, item))
        .collect()
}

fn open_one(
    session: &MutationSession<'_>,
    request_id: &str,
    index: usize,
    item: StagedMutation,
) -> Result<TransactionRecoveryPlan, ArtifactError> {
    validate::item(&item)?;
    let (chain, target_path, leaf) =
        parent_and_leaf(&session.owner.root_path, &item.relative_path)?;
    session.verify_chain(&chain)?;
    let is_tree = item.operation == REMOVE_TREE_OPERATION;
    let target = optional_mutation_file_for_operation(&target_path, &item.operation)?;
    validate::target(target.as_ref(), is_tree)?;
    let quarantine_path = quarantine::path(&chain, request_id, index, &item)?;
    let quarantine = quarantine::open(&quarantine_path, &item.operation)?;
    let stage_path = stage::path(session, request_id, index, &item)?;
    let stage = stage::open(stage_path.as_deref())?;
    Ok(TransactionRecoveryPlan {
        item,
        chain,
        target_path,
        leaf,
        target,
        stage,
        quarantine_path,
        quarantine,
    })
}
