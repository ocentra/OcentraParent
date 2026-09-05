use super::*;
use crate::constants::{
    REMOVE_TREE_CHILD_REQUIRED, REMOVE_TREE_OPERATION, REMOVE_TREE_QUARANTINED_PHASE,
    TRANSACTION_OPERATION,
};

struct TreeQuarantine<'a> {
    request_id: &'a str,
    relative_path: &'a str,
    descriptor: &'a str,
    receipt_operation: &'a str,
    receipt_relative_path: &'a str,
    parent_chain: DirectoryChain,
    target: OwnedFile,
    target_identity: IdentityRecord,
    quarantine_name: String,
}

impl<'a> MutationSession<'a> {
    pub fn remove_tree(
        &mut self,
        request_id: &str,
        relative_path: &str,
    ) -> Result<MutationReceipt, ArtifactError> {
        validate_request_id(request_id)?;
        validate_directory_relative(relative_path)?;
        if relative_path.is_empty() {
            return Err(ArtifactError::InvalidPath(REMOVE_TREE_CHILD_REQUIRED));
        }
        let descriptor: String = request_descriptor(REMOVE_TREE_OPERATION, relative_path, None);
        if let Some(receipt) = read_receipt(
            &self.owner.root_path,
            request_id,
            REMOVE_TREE_OPERATION,
            relative_path,
            &descriptor,
        )? {
            return Ok(receipt);
        }
        self.remove_tree_internal(
            request_id,
            relative_path,
            &descriptor,
            REMOVE_TREE_OPERATION,
            relative_path,
        )
    }

    pub(super) fn remove_tree_internal(
        &mut self,
        request_id: &str,
        relative_path: &str,
        descriptor: &str,
        receipt_operation: &str,
        receipt_relative_path: &str,
    ) -> Result<MutationReceipt, ArtifactError> {
        validate_tree_scope(
            request_id,
            relative_path,
            receipt_operation,
            receipt_relative_path,
        )?;
        self.verify_current()?;

        let (parent_chain, target_path, _) = parent_and_leaf(&self.owner.root_path, relative_path)?;
        self.verify_chain(&parent_chain)?;
        let target = match OwnedFile::open_mutation_directory(&target_path) {
            Ok(file) => file,
            Err(ArtifactError::NotFound) => {
                return missing_tree_receipt(
                    self,
                    request_id,
                    receipt_operation,
                    receipt_relative_path,
                    descriptor,
                )
            }
            Err(error) => return Err(error),
        };
        let target_metadata = verify_metadata(&target, true)?;
        let target_identity = identity_record(target_metadata.identity);
        let quarantine_name = format!("{request_id}.tree-quarantine");
        reject_existing_sibling(&self.owner.root_path, &parent_chain, &quarantine_name)?;
        self.quarantine_tree(TreeQuarantine {
            request_id,
            relative_path,
            descriptor,
            receipt_operation,
            receipt_relative_path,
            parent_chain,
            target,
            target_identity,
            quarantine_name,
        })
    }

    fn quarantine_tree(
        &mut self,
        quarantine: TreeQuarantine<'_>,
    ) -> Result<MutationReceipt, ArtifactError> {
        let TreeQuarantine {
            request_id,
            relative_path,
            descriptor,
            receipt_operation,
            receipt_relative_path,
            parent_chain,
            target,
            target_identity,
            quarantine_name,
        } = quarantine;
        let intent = IntentRecord::RemoveTree {
            schema: 1,
            request_id: request_id.to_owned(),
            relative_path: relative_path.to_owned(),
            descriptor: descriptor.to_owned(),
            target_identity,
            quarantine_name: Some(quarantine_name.clone()),
            receipt_operation: receipt_operation.to_owned(),
            receipt_relative_path: receipt_relative_path.to_owned(),
            phase: RemoveTreePhase::Prepared,
        };
        write_intent(
            &self.owner.root_path,
            request_id,
            &intent,
            self.metadata.intent_directory()?,
        )?;

        target.rename_into(parent_chain.leaf()?, &quarantine_name)?;
        parent_chain.leaf()?.sync_directory()?;
        let quarantined = intent.remove_tree_with_phase(RemoveTreePhase::Quarantined)?;
        replace_intent(
            &self.owner.root_path,
            request_id,
            &quarantined,
            self.metadata.intent_directory()?,
            REMOVE_TREE_QUARANTINED_PHASE,
        )?;
        let quarantine_path = parent_chain
            .paths
            .last()
            .cloned()
            .ok_or(ArtifactError::RecoveryRequired)?
            .join(&quarantine_name);
        verify_metadata(&target, true)?;
        remove_tree_contents(&quarantine_path, &target)?;
        target.mark_deleted()?;
        parent_chain.leaf()?.sync_directory()?;
        self.verify_current()?;
        let receipt = write_receipt(
            &self.owner.root_path,
            request_id,
            receipt_operation,
            receipt_relative_path,
            descriptor,
            ReceiptOutcome::Removed { existed: true },
            self.metadata.receipt_directory()?,
        )?;
        remove_intent(
            &self.owner.root_path,
            request_id,
            self.metadata.intent_directory()?,
        )?;
        Ok(receipt)
    }
}

fn validate_tree_scope(
    request_id: &str,
    relative_path: &str,
    receipt_operation: &str,
    receipt_relative_path: &str,
) -> Result<(), ArtifactError> {
    validate_request_id(request_id)?;
    let relative = validate_directory_relative(relative_path)?;
    if relative.as_os_str().is_empty() {
        return Err(ArtifactError::InvalidPath(REMOVE_TREE_CHILD_REQUIRED));
    }
    if !matches!(
        receipt_operation,
        REMOVE_TREE_OPERATION | TRANSACTION_OPERATION
    ) {
        return Err(ArtifactError::RequestIdConflict);
    }
    let valid_receipt = (receipt_operation == REMOVE_TREE_OPERATION
        && receipt_relative_path == relative_path)
        || (receipt_operation == TRANSACTION_OPERATION
            && receipt_relative_path == TRANSACTION_OPERATION);
    if valid_receipt {
        Ok(())
    } else {
        Err(ArtifactError::RequestIdConflict)
    }
}

fn missing_tree_receipt(
    session: &MutationSession<'_>,
    request_id: &str,
    receipt_operation: &str,
    receipt_relative_path: &str,
    descriptor: &str,
) -> Result<MutationReceipt, ArtifactError> {
    write_receipt(
        &session.owner.root_path,
        request_id,
        receipt_operation,
        receipt_relative_path,
        descriptor,
        ReceiptOutcome::Removed { existed: false },
        session.metadata.receipt_directory()?,
    )
}
