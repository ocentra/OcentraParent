use super::*;
use crate::constants::{BRIDGE_DIRECTORY, INTENTS_DIRECTORY, MUTATION_OWNER_DIRECTORY};

#[path = "owner_mutations_recovery_replace_finish.rs"]
mod finish;

pub(super) struct ReplaceRecovery {
    pub(super) request_id: String,
    pub(super) relative_path: String,
    pub(super) descriptor: String,
    pub(super) expected_digest: String,
    pub(super) target_identity: Option<IdentityRecord>,
    pub(super) staged_identity: Option<IdentityRecord>,
    pub(super) phase: ReplacePhase,
    pub(super) chain: DirectoryChain,
    pub(super) leaf: String,
    pub(super) temp: Option<OwnedFile>,
    pub(super) quarantine: Option<OwnedFile>,
    pub(super) target: Option<OwnedFile>,
}

impl ReplaceRecovery {
    fn load(session: &MutationSession<'_>, record: &IntentRecord) -> Result<Self, ArtifactError> {
        let IntentRecord::Replace {
            request_id,
            relative_path,
            descriptor,
            payload_digest,
            temp_name,
            quarantine_name,
            target_identity,
            staged_identity,
            phase,
            ..
        } = record
        else {
            return Err(ArtifactError::RecoveryRequired);
        };
        validate_names(request_id, temp_name, quarantine_name)?;
        let (chain, target_path, leaf) = parent_and_leaf(&session.owner.root_path, relative_path)?;
        session.verify_chain(&chain)?;
        let temp_path = session
            .owner
            .root_path
            .join(BRIDGE_DIRECTORY)
            .join(MUTATION_OWNER_DIRECTORY)
            .join(INTENTS_DIRECTORY)
            .join(temp_name);
        let temp = optional_mutation_file(&temp_path)?;
        let quarantine_path = chain
            .paths
            .last()
            .cloned()
            .ok_or(ArtifactError::RecoveryRequired)?
            .join(quarantine_name);
        let quarantine = optional_mutation_file(&quarantine_path)?;
        let target = optional_mutation_file(&target_path)?;
        Ok(Self {
            request_id: request_id.clone(),
            relative_path: relative_path.clone(),
            descriptor: descriptor.clone(),
            expected_digest: payload_digest.clone(),
            target_identity: *target_identity,
            staged_identity: *staged_identity,
            phase: *phase,
            chain,
            leaf,
            temp,
            quarantine,
            target,
        })
    }
}

impl<'a> MutationSession<'a> {
    pub(super) fn recover_replace(
        &mut self,
        record: &IntentRecord,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        let mut recovery = ReplaceRecovery::load(self, record)?;
        if matches!(recovery.phase, ReplacePhase::Prepared) {
            if recovery.quarantine.is_some() {
                recovery.phase = ReplacePhase::Quarantined;
                return finish::complete(self, recovery);
            }
            verify_prepared_replace(&recovery)?;
            if let Some(file) = recovery.temp.take() {
                file.mark_deleted()?;
                self.metadata.intent_directory()?.sync_directory()?;
            }
            remove_intent(
                &self.owner.root_path,
                recovery.request_id.as_str(),
                self.metadata.intent_directory()?,
            )?;
            return Ok(None);
        }
        finish::complete(self, recovery)
    }
}

fn validate_names(
    request_id: &str,
    temp_name: &str,
    quarantine_name: &str,
) -> Result<(), ArtifactError> {
    if temp_name != format!("{request_id}.replace.tmp")
        || quarantine_name != format!("{request_id}.replace.quarantine")
    {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}

fn verify_prepared_replace(recovery: &ReplaceRecovery) -> Result<(), ArtifactError> {
    if let Some(file) = recovery.target.as_ref() {
        verify_expected_identity(file, recovery.target_identity.as_ref())?;
    } else if recovery.target_identity.is_some() {
        return Err(ArtifactError::OwnershipChanged);
    }
    Ok(())
}
