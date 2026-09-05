use super::*;
use crate::constants::{
    REMOVE_TREE_OPERATION, ROOT_SEPARATOR, TRANSACTION_MUTATION_ERROR, TRANSACTION_OPERATION,
};

#[path = "owner_mutations_transaction_execute.rs"]
mod execute;
#[path = "owner_mutations_transaction_plan.rs"]
mod plan;
#[path = "owner_mutations_transaction_recovery.rs"]
mod recovery;
#[path = "owner_mutations_transaction_validate.rs"]
mod validate;

pub(super) struct TransactionPlan {
    pub(super) mutation: Mutation,
    pub(super) chain: DirectoryChain,
    pub(super) leaf: String,
    pub(super) target_existed: bool,
    pub(super) target: Option<OwnedFile>,
    pub(super) stage: Option<OwnedFile>,
}

pub(super) struct TransactionRecoveryPlan {
    pub(super) item: StagedMutation,
    pub(super) chain: DirectoryChain,
    pub(super) target_path: PathBuf,
    pub(super) leaf: String,
    pub(super) target: Option<OwnedFile>,
    pub(super) stage: Option<OwnedFile>,
    pub(super) quarantine_path: Option<PathBuf>,
    pub(super) quarantine: Option<OwnedFile>,
}

impl<'a> MutationSession<'a> {
    pub fn apply_transaction(
        &mut self,
        request_id: &str,
        mutations: &[Mutation],
    ) -> Result<MutationReceipt, ArtifactError> {
        validate_request_id(request_id)?;
        let descriptor: String = transaction_descriptor(mutations);
        if let Some(receipt) = read_receipt(
            &self.owner.root_path,
            request_id,
            TRANSACTION_OPERATION,
            TRANSACTION_OPERATION,
            &descriptor,
        )? {
            return Ok(receipt);
        }
        validate::inputs(mutations)?;
        if let Some(relative_path) = validate::single_remove_tree(mutations) {
            return self.remove_tree_internal(
                request_id,
                relative_path,
                &descriptor,
                TRANSACTION_OPERATION,
                TRANSACTION_OPERATION,
            );
        }
        if validate::contains_append(mutations) {
            return self.unsupported_transaction(request_id, &descriptor);
        }
        let mut staged: Vec<StagedMutation> = mutation_records(mutations)?;
        let mut plans = plan::build(self, request_id, mutations, &mut staged)?;
        let prepared =
            execute::write_stages(self, request_id, &descriptor, &mut staged, &mut plans)?;
        let quarantined = execute::quarantine(self, request_id, &prepared, &mut plans)?;
        execute::install(self, request_id, &quarantined, &mut plans)?;
        execute::finalize(
            self,
            request_id,
            &descriptor,
            mutations.len(),
            staged,
            plans,
        )
    }

    fn unsupported_transaction(
        &mut self,
        request_id: &str,
        descriptor: &str,
    ) -> Result<MutationReceipt, ArtifactError> {
        self.verify_current()?;
        write_receipt(
            &self.owner.root_path,
            request_id,
            TRANSACTION_OPERATION,
            TRANSACTION_OPERATION,
            descriptor,
            ReceiptOutcome::Unsupported {
                operation: TRANSACTION_MUTATION_ERROR.to_owned(),
            },
            self.metadata.receipt_directory()?,
        )
    }
}

pub(super) fn verify_transaction_prestate(
    plan: &TransactionRecoveryPlan,
) -> Result<(), ArtifactError> {
    if let Some(target) = plan.target.as_ref() {
        verify_transaction_old(plan, target)?;
    } else if plan.item.target_identity.is_some() {
        return Err(ArtifactError::OwnershipChanged);
    }
    Ok(())
}

pub(super) fn verify_transaction_old(
    plan: &TransactionRecoveryPlan,
    file: &OwnedFile,
) -> Result<(), ArtifactError> {
    let Some(expected) = plan.item.target_identity.as_ref() else {
        return Err(ArtifactError::OwnershipChanged);
    };
    if plan.item.operation == REMOVE_TREE_OPERATION {
        verify_expected_directory_identity(file, expected)
    } else {
        verify_expected_identity(file, Some(expected))
    }
}

pub(super) fn verify_transaction_kind(
    file: &OwnedFile,
    directory: bool,
) -> Result<(), ArtifactError> {
    verify_metadata(file, directory).map(|_| ())
}

pub(super) fn optional_mutation_file_for_operation(
    path: &Path,
    operation: &str,
) -> Result<Option<OwnedFile>, ArtifactError> {
    if operation == REMOVE_TREE_OPERATION {
        optional_directory(path)
    } else {
        optional_mutation_file(path)
    }
}

pub(super) fn is_descendant_path(ancestor: &str, candidate: &str) -> bool {
    let ancestor = transaction_path_key(ancestor);
    let candidate = transaction_path_key(candidate);
    candidate.len() > ancestor.len()
        && candidate.starts_with(&ancestor)
        && candidate.as_bytes().get(ancestor.len()) == Some(&b'\\')
}

pub(super) fn transaction_path_key(path: &str) -> String {
    path.replace('/', ROOT_SEPARATOR).to_lowercase()
}

pub(super) fn intent_stage_name(
    intent: &IntentRecord,
    plan: &TransactionPlan,
) -> Result<String, ArtifactError> {
    let IntentRecord::Transaction { staged, .. } = intent else {
        return Err(ArtifactError::RecoveryRequired);
    };
    staged
        .iter()
        .find(|item| item.relative_path == plan.mutation.relative_path())
        .and_then(|item| item.staged_name.clone())
        .ok_or(ArtifactError::RecoveryRequired)
}

pub(super) fn intent_quarantine_name(
    intent: &IntentRecord,
    index: usize,
) -> Result<String, ArtifactError> {
    let IntentRecord::Transaction { staged, .. } = intent else {
        return Err(ArtifactError::RecoveryRequired);
    };
    staged
        .get(index)
        .and_then(|item| item.quarantine_name.clone())
        .ok_or(ArtifactError::RecoveryRequired)
}
