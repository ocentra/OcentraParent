use super::*;
use crate::constants::{REMOVE_DELETED_PHASE, REMOVE_OPERATION};

impl<'a> MutationSession<'a> {
    pub fn remove(
        &mut self,
        request_id: &str,
        relative_path: &str,
    ) -> Result<MutationReceipt, ArtifactError> {
        validate_request_id(request_id)?;
        validate_relative(relative_path)?;
        let descriptor: String = request_descriptor(REMOVE_OPERATION, relative_path, None);
        if let Some(receipt) = read_receipt(
            &self.owner.root_path,
            request_id,
            REMOVE_OPERATION,
            relative_path,
            &descriptor,
        )? {
            return Ok(receipt);
        }
        let (chain, target_path, _) = parent_and_leaf(&self.owner.root_path, relative_path)?;
        self.verify_chain(&chain)?;
        let target = match OwnedFile::open_existing_mutation_file(&target_path) {
            Ok(file) => Some(file),
            Err(ArtifactError::NotFound) => None,
            Err(error) => return Err(error),
        };
        let target_identity = target
            .as_ref()
            .map(|file| {
                verify_metadata(file, false).map(|metadata| identity_record(metadata.identity))
            })
            .transpose()?;
        let intent = IntentRecord::Remove {
            schema: 1,
            request_id: request_id.to_owned(),
            relative_path: relative_path.to_owned(),
            descriptor: descriptor.clone(),
            target_identity,
            phase: RemovePhase::Prepared,
        };
        write_intent(
            &self.owner.root_path,
            request_id,
            &intent,
            self.metadata.intent_directory()?,
        )?;
        if let Some(file) = target.as_ref() {
            file.mark_deleted()?;
            chain.leaf()?.sync_directory()?;
            self.verify_chain(&chain)?;
            let deleted = intent.remove_with_phase(RemovePhase::Deleted)?;
            replace_intent(
                &self.owner.root_path,
                request_id,
                &deleted,
                self.metadata.intent_directory()?,
                REMOVE_DELETED_PHASE,
            )?;
        }
        let receipt = write_receipt(
            &self.owner.root_path,
            request_id,
            REMOVE_OPERATION,
            relative_path,
            &descriptor,
            ReceiptOutcome::Removed {
                existed: target.is_some(),
            },
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
