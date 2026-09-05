use super::*;

#[path = "owner_mutations_transaction_plan_names.rs"]
mod names;
#[path = "owner_mutations_transaction_plan_target.rs"]
mod target;

pub(super) fn build(
    session: &MutationSession<'_>,
    request_id: &str,
    mutations: &[Mutation],
    staged: &mut [StagedMutation],
) -> Result<Vec<TransactionPlan>, ArtifactError> {
    let mut plans = Vec::with_capacity(mutations.len());
    for (index, mutation) in mutations.iter().enumerate() {
        let prepared = prepare(session, request_id, index, mutation)?;
        staged[index].staged_name = prepared.staged_name;
        staged[index].quarantine_name = prepared.quarantine_name;
        staged[index].target_identity = prepared.target_identity;
        plans.push(TransactionPlan {
            mutation: mutation.clone(),
            chain: prepared.chain,
            leaf: prepared.leaf,
            target_existed: prepared.target.is_some(),
            target: prepared.target,
            stage: None,
        });
    }
    Ok(plans)
}

struct PreparedPlan {
    chain: DirectoryChain,
    leaf: String,
    target: Option<OwnedFile>,
    target_identity: Option<IdentityRecord>,
    staged_name: Option<String>,
    quarantine_name: Option<String>,
}

fn prepare(
    session: &MutationSession<'_>,
    request_id: &str,
    index: usize,
    mutation: &Mutation,
) -> Result<PreparedPlan, ArtifactError> {
    let (chain, target_path, leaf) =
        parent_and_leaf(&session.owner.root_path, mutation.relative_path())?;
    session.verify_chain(&chain)?;
    let is_tree = matches!(mutation, Mutation::RemoveTree { .. });
    let target = target::open(&target_path, is_tree)?;
    let target_identity = target
        .as_ref()
        .map(|file| {
            verify_metadata(file, is_tree).map(|metadata| identity_record(metadata.identity))
        })
        .transpose()?;
    let staged_name = names::staged_name(request_id, index, mutation);
    let quarantine_candidate = names::quarantine_name(request_id, index);
    if target.is_none() {
        reject_existing_sibling(&session.owner.root_path, &chain, &quarantine_candidate)?;
    }
    let quarantine_name = target.as_ref().map(|_| quarantine_candidate);
    names::reject_stage_conflict(session, staged_name.as_deref())?;
    if let Some(name) = quarantine_name.as_deref() {
        reject_existing_sibling(&session.owner.root_path, &chain, name)?;
    }
    Ok(PreparedPlan {
        chain,
        leaf,
        target,
        target_identity,
        staged_name,
        quarantine_name,
    })
}
