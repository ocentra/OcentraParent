use super::*;
use crate::constants::{
    BRIDGE_DIRECTORY, INTENTS_DIRECTORY, MUTATION_OWNER_DIRECTORY, REPLACE_INSTALLED_PHASE,
    REPLACE_OPERATION, REPLACE_QUARANTINED_PHASE, REPLACE_STAGED_PHASE,
};

struct ReplacePreparation {
    chain: DirectoryChain,
    leaf: String,
    target: Option<OwnedFile>,
    temp: OwnedFile,
    intent: IntentRecord,
    descriptor: String,
}

impl<'a> MutationSession<'a> {
    pub fn replace(
        &mut self,
        request_id: &str,
        relative_path: &str,
        payload: &[u8],
    ) -> Result<MutationReceipt, ArtifactError> {
        validate_request_id(request_id)?;
        validate_relative(relative_path)?;
        bounded_payload(payload)?;
        let descriptor = request_descriptor(REPLACE_OPERATION, relative_path, Some(payload));
        if let Some(receipt) = read_receipt(
            &self.owner.root_path,
            request_id,
            REPLACE_OPERATION,
            relative_path,
            &descriptor,
        )? {
            return Ok(receipt);
        }
        let preparation = self.prepare_replace(request_id, relative_path, payload, descriptor)?;
        self.commit_replace(request_id, relative_path, preparation)
    }

    fn prepare_replace(
        &mut self,
        request_id: &str,
        relative_path: &str,
        payload: &[u8],
        descriptor: String,
    ) -> Result<ReplacePreparation, ArtifactError> {
        let (chain, target_path, leaf) = parent_and_leaf(&self.owner.root_path, relative_path)?;
        self.verify_chain(&chain)?;
        let target = open_replace_target(&target_path)?;
        let target_identity = target
            .as_ref()
            .map(|file| {
                verify_metadata(file, false).map(|metadata| identity_record(metadata.identity))
            })
            .transpose()?;
        let temp_name = format!("{request_id}.replace.tmp");
        let quarantine_name = format!("{request_id}.replace.quarantine");
        let temp_path = self
            .owner
            .root_path
            .join(BRIDGE_DIRECTORY)
            .join(MUTATION_OWNER_DIRECTORY)
            .join(INTENTS_DIRECTORY)
            .join(&temp_name);
        if optional_mutation_file(&temp_path)?.is_some() {
            return Err(ArtifactError::RecoveryRequired);
        }
        reject_existing_sibling(&self.owner.root_path, &chain, &quarantine_name)?;
        let mut intent = IntentRecord::Replace {
            schema: 1,
            request_id: request_id.to_owned(),
            relative_path: relative_path.to_owned(),
            descriptor: descriptor.clone(),
            payload_digest: payload_digest(payload),
            temp_name,
            quarantine_name,
            target_identity,
            staged_identity: None,
            phase: ReplacePhase::Prepared,
        };
        write_intent(
            &self.owner.root_path,
            request_id,
            &intent,
            self.metadata.intent_directory()?,
        )?;
        let mut temp = OwnedFile::create_new_mutation_file(&temp_path)?;
        temp.write_bounded(payload)?;
        temp.sync_file()?;
        let staged_metadata = verify_metadata(&temp, false)?;
        intent = intent.replace_with_state(
            Some(identity_record(staged_metadata.identity)),
            ReplacePhase::Prepared,
        )?;
        replace_intent(
            &self.owner.root_path,
            request_id,
            &intent,
            self.metadata.intent_directory()?,
            REPLACE_STAGED_PHASE,
        )?;
        Ok(ReplacePreparation {
            chain,
            leaf,
            target,
            temp,
            intent,
            descriptor,
        })
    }

    fn commit_replace(
        &mut self,
        request_id: &str,
        relative_path: &str,
        mut preparation: ReplacePreparation,
    ) -> Result<MutationReceipt, ArtifactError> {
        let quarantine_name = quarantine_name(&preparation.intent)?;
        if let Some(existing) = preparation.target.as_ref() {
            existing.rename_into(preparation.chain.leaf()?, &quarantine_name)?;
            preparation.chain.leaf()?.sync_directory()?;
            preparation.intent = preparation
                .intent
                .replace_with_phase(ReplacePhase::Quarantined)?;
            replace_intent(
                &self.owner.root_path,
                request_id,
                &preparation.intent,
                self.metadata.intent_directory()?,
                REPLACE_QUARANTINED_PHASE,
            )?;
        }
        preparation
            .temp
            .rename_into(preparation.chain.leaf()?, &preparation.leaf)?;
        preparation.chain.leaf()?.sync_directory()?;
        preparation.intent = preparation
            .intent
            .replace_with_phase(ReplacePhase::Installed)?;
        replace_intent(
            &self.owner.root_path,
            request_id,
            &preparation.intent,
            self.metadata.intent_directory()?,
            REPLACE_INSTALLED_PHASE,
        )?;
        delete_replaced_target(&mut preparation, self)?;
        self.verify_chain(&preparation.chain)?;
        let receipt = write_receipt(
            &self.owner.root_path,
            request_id,
            REPLACE_OPERATION,
            relative_path,
            &preparation.descriptor,
            ReceiptOutcome::Replaced,
            self.metadata.receipt_directory()?,
        )?;
        let _ = remove_intent(
            &self.owner.root_path,
            request_id,
            self.metadata.intent_directory()?,
        );
        Ok(receipt)
    }
}

fn open_replace_target(path: &Path) -> Result<Option<OwnedFile>, ArtifactError> {
    match OwnedFile::open_existing_mutation_file(path) {
        Ok(file) => Ok(Some(file)),
        Err(ArtifactError::NotFound) => Ok(None),
        Err(error) => Err(error),
    }
}

fn quarantine_name(intent: &IntentRecord) -> Result<String, ArtifactError> {
    let IntentRecord::Replace {
        quarantine_name, ..
    } = intent
    else {
        return Err(ArtifactError::RecoveryRequired);
    };
    Ok(quarantine_name.clone())
}

fn delete_replaced_target(
    preparation: &mut ReplacePreparation,
    session: &MutationSession<'_>,
) -> Result<(), ArtifactError> {
    let Some(old) = preparation.target.take() else {
        return Ok(());
    };
    old.mark_deleted()?;
    preparation.chain.leaf()?.sync_directory()?;
    session.verify_current()
}
