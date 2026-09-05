use super::*;
use crate::constants::{
    REMOVE_TREE_CHILD_REQUIRED, REMOVE_TREE_OPERATION, REMOVE_TREE_RECOVERY_QUARANTINED_PHASE,
    TRANSACTION_OPERATION,
};

struct TreeRecovery {
    request_id: String,
    relative_path: String,
    descriptor: String,
    target_identity: IdentityRecord,
    receipt_operation: String,
    receipt_relative_path: String,
    phase: RemoveTreePhase,
    parent_chain: DirectoryChain,
    quarantine_path: PathBuf,
    target: Option<OwnedFile>,
    quarantine: Option<OwnedFile>,
}

impl TreeRecovery {
    fn load(session: &MutationSession<'_>, record: &IntentRecord) -> Result<Self, ArtifactError> {
        let input = TreeRecoveryInput::read(record)?;
        let (parent_chain, target_path, _) =
            parent_and_leaf(&session.owner.root_path, &input.relative_path)?;
        session.verify_chain(&parent_chain)?;
        let quarantine_path = parent_chain
            .paths
            .last()
            .cloned()
            .ok_or(ArtifactError::RecoveryRequired)?
            .join(&input.quarantine_name);
        let target = optional_directory(&target_path)?;
        let quarantine = optional_directory(&quarantine_path)?;
        Ok(Self {
            request_id: input.request_id,
            relative_path: input.relative_path,
            descriptor: input.descriptor,
            target_identity: input.target_identity,
            receipt_operation: input.receipt_operation,
            receipt_relative_path: input.receipt_relative_path,
            phase: input.phase,
            parent_chain,
            quarantine_path,
            target,
            quarantine,
        })
    }
}

struct TreeRecoveryInput {
    request_id: String,
    relative_path: String,
    descriptor: String,
    target_identity: IdentityRecord,
    quarantine_name: String,
    receipt_operation: String,
    receipt_relative_path: String,
    phase: RemoveTreePhase,
}

impl TreeRecoveryInput {
    fn read(record: &IntentRecord) -> Result<Self, ArtifactError> {
        let IntentRecord::RemoveTree {
            request_id,
            relative_path,
            descriptor,
            target_identity,
            quarantine_name,
            receipt_operation,
            receipt_relative_path,
            phase,
            ..
        } = record
        else {
            return Err(ArtifactError::RecoveryRequired);
        };
        validate_request_id(request_id)?;
        validate_directory_relative(relative_path)?;
        if relative_path.is_empty() {
            return Err(ArtifactError::InvalidPath(REMOVE_TREE_CHILD_REQUIRED));
        }
        validate_receipt_scope(receipt_operation, receipt_relative_path, relative_path)?;
        let expected = format!("{request_id}.tree-quarantine");
        if quarantine_name.as_deref() != Some(expected.as_str()) {
            return Err(ArtifactError::RecoveryRequired);
        }
        let quarantine_name = quarantine_name
            .clone()
            .ok_or(ArtifactError::RecoveryRequired)?;
        crate::platform::windows::validate_leaf(&quarantine_name)?;
        Ok(Self {
            request_id: request_id.clone(),
            relative_path: relative_path.clone(),
            descriptor: descriptor.clone(),
            target_identity: *target_identity,
            quarantine_name,
            receipt_operation: receipt_operation.clone(),
            receipt_relative_path: receipt_relative_path.clone(),
            phase: *phase,
        })
    }
}

fn validate_receipt_scope(
    operation: &str,
    receipt_path: &str,
    relative_path: &str,
) -> Result<(), ArtifactError> {
    if !matches!(operation, REMOVE_TREE_OPERATION | TRANSACTION_OPERATION) {
        return Err(ArtifactError::RequestIdConflict);
    }
    let valid = (operation == REMOVE_TREE_OPERATION && receipt_path == relative_path)
        || (operation == TRANSACTION_OPERATION && receipt_path == TRANSACTION_OPERATION);
    if valid {
        Ok(())
    } else {
        Err(ArtifactError::RequestIdConflict)
    }
}

impl<'a> MutationSession<'a> {
    pub(super) fn recover_remove_tree(
        &mut self,
        record: &IntentRecord,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        let mut recovery = TreeRecovery::load(self, record)?;
        if matches!(recovery.phase, RemoveTreePhase::Prepared) {
            return self.recover_prepared_tree(&mut recovery);
        }
        if recovery.target.is_some() {
            return Err(ArtifactError::OwnershipChanged);
        }
        let active = recovery
            .quarantine
            .take()
            .ok_or(ArtifactError::RecoveryRequired)?;
        verify_expected_directory_identity(&active, &recovery.target_identity)?;
        self.finish_remove_tree_recovery(&mut recovery, active)
    }

    fn recover_prepared_tree(
        &mut self,
        recovery: &mut TreeRecovery,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        if let Some(quarantine) = recovery.quarantine.take() {
            if recovery.target.is_some() {
                return Err(ArtifactError::OwnershipChanged);
            }
            verify_expected_directory_identity(&quarantine, &recovery.target_identity)?;
            let progressed = IntentRecord::RemoveTree {
                schema: 1,
                request_id: recovery.request_id.clone(),
                relative_path: recovery.relative_path.clone(),
                descriptor: recovery.descriptor.clone(),
                target_identity: recovery.target_identity,
                quarantine_name: Some(
                    recovery
                        .quarantine_path
                        .file_name()
                        .and_then(|value| value.to_str())
                        .ok_or(ArtifactError::RecoveryRequired)?
                        .to_owned(),
                ),
                receipt_operation: recovery.receipt_operation.clone(),
                receipt_relative_path: recovery.receipt_relative_path.clone(),
                phase: RemoveTreePhase::Quarantined,
            };
            replace_intent(
                &self.owner.root_path,
                recovery.request_id.as_str(),
                &progressed,
                self.metadata.intent_directory()?,
                REMOVE_TREE_RECOVERY_QUARANTINED_PHASE,
            )?;
            return self.finish_remove_tree_recovery(recovery, quarantine);
        }
        let target = recovery
            .target
            .take()
            .ok_or(ArtifactError::OwnershipChanged)?;
        verify_expected_directory_identity(&target, &recovery.target_identity)?;
        remove_intent(
            &self.owner.root_path,
            recovery.request_id.as_str(),
            self.metadata.intent_directory()?,
        )?;
        Ok(None)
    }

    fn finish_remove_tree_recovery(
        &mut self,
        recovery: &mut TreeRecovery,
        active: OwnedFile,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        verify_expected_directory_identity(&active, &recovery.target_identity)?;
        remove_tree_contents(&recovery.quarantine_path, &active)?;
        active.mark_deleted()?;
        recovery.parent_chain.leaf()?.sync_directory()?;
        self.verify_current()?;
        let receipt = write_receipt(
            &self.owner.root_path,
            recovery.request_id.as_str(),
            &recovery.receipt_operation,
            &recovery.receipt_relative_path,
            &recovery.descriptor,
            ReceiptOutcome::Removed { existed: true },
            self.metadata.receipt_directory()?,
        )?;
        remove_intent(
            &self.owner.root_path,
            recovery.request_id.as_str(),
            self.metadata.intent_directory()?,
        )?;
        Ok(Some(receipt))
    }
}
