use super::*;
use crate::constants::{REMOVE_DELETED_PHASE, REMOVE_OPERATION};

impl<'a> MutationSession<'a> {
    pub(super) fn recover_remove(
        &mut self,
        record: &IntentRecord,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        let IntentRecord::Remove {
            request_id,
            relative_path,
            descriptor,
            target_identity,
            phase,
            ..
        } = record
        else {
            return Err(ArtifactError::RecoveryRequired);
        };
        validate_request_id(request_id)?;
        validate_relative(relative_path)?;
        let (chain, target_path, _) = parent_and_leaf(&self.owner.root_path, relative_path)?;
        self.verify_chain(&chain)?;
        let target = optional_mutation_file(&target_path)?;
        let recovered = match phase {
            RemovePhase::Prepared => self.recover_prepared_remove(record, &chain, target)?,
            RemovePhase::Deleted => self.recover_deleted_remove(target_identity, &chain, target)?,
        };
        if let Some(receipt) = recovered {
            return Ok(Some(receipt));
        }
        self.finish_remove_recovery(request_id, relative_path, descriptor, true)
    }

    fn recover_prepared_remove(
        &mut self,
        record: &IntentRecord,
        chain: &DirectoryChain,
        target: Option<OwnedFile>,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        let IntentRecord::Remove {
            request_id,
            relative_path,
            descriptor,
            target_identity,
            ..
        } = record
        else {
            return Err(ArtifactError::RecoveryRequired);
        };
        let Some(file) = target else {
            if target_identity.is_some() {
                let deleted = record.remove_with_phase(RemovePhase::Deleted)?;
                replace_intent(
                    &self.owner.root_path,
                    request_id,
                    &deleted,
                    self.metadata.intent_directory()?,
                    REMOVE_DELETED_PHASE,
                )?;
                return Ok(None);
            }
            return self.finish_remove_recovery(request_id, relative_path, descriptor, false);
        };
        verify_expected_identity(&file, target_identity.as_ref())?;
        file.mark_deleted()?;
        chain.leaf()?.sync_directory()?;
        let deleted = record.remove_with_phase(RemovePhase::Deleted)?;
        replace_intent(
            &self.owner.root_path,
            request_id,
            &deleted,
            self.metadata.intent_directory()?,
            REMOVE_DELETED_PHASE,
        )?;
        Ok(None)
    }

    fn recover_deleted_remove(
        &mut self,
        target_identity: &Option<IdentityRecord>,
        chain: &DirectoryChain,
        target: Option<OwnedFile>,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        let expected = target_identity
            .as_ref()
            .ok_or(ArtifactError::RecoveryRequired)?;
        if let Some(file) = target {
            verify_expected_identity(&file, Some(expected))?;
            file.mark_deleted()?;
            chain.leaf()?.sync_directory()?;
        }
        Ok(None)
    }

    fn finish_remove_recovery(
        &mut self,
        request_id: &str,
        relative_path: &str,
        descriptor: &str,
        existed: bool,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        self.verify_current()?;
        let receipt = write_receipt(
            &self.owner.root_path,
            request_id,
            REMOVE_OPERATION,
            relative_path,
            descriptor,
            ReceiptOutcome::Removed { existed },
            self.metadata.receipt_directory()?,
        )?;
        remove_intent(
            &self.owner.root_path,
            request_id,
            self.metadata.intent_directory()?,
        )?;
        Ok(Some(receipt))
    }
}
