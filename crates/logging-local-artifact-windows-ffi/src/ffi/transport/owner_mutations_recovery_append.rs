use super::*;
use crate::constants::{
    APPEND_CREATED_PHASE, APPEND_OPERATION, BRIDGE_DIRECTORY, INTENTS_DIRECTORY,
    MUTATION_OWNER_DIRECTORY,
};

#[path = "owner_mutations_recovery_append_finish.rs"]
mod finish;
#[path = "owner_mutations_recovery_append_prepare.rs"]
mod prepare;

pub(super) struct AppendRecovery {
    pub(super) record: IntentRecord,
    pub(super) request_id: String,
    pub(super) relative_path: String,
    pub(super) descriptor: String,
    pub(super) expected_digest: String,
    pub(super) payload_length: u64,
    pub(super) prior_length: u64,
    pub(super) created: bool,
    pub(super) target_identity: Option<IdentityRecord>,
    pub(super) phase: AppendPhase,
    pub(super) chain: DirectoryChain,
    pub(super) target_path: PathBuf,
    pub(super) leaf: String,
    pub(super) temp: Option<OwnedFile>,
    pub(super) target: Option<OwnedFile>,
}

impl AppendRecovery {
    fn load(session: &MutationSession<'_>, record: &IntentRecord) -> Result<Self, ArtifactError> {
        let IntentRecord::Append {
            request_id,
            relative_path,
            descriptor,
            payload_digest,
            payload_length,
            prior_length,
            created,
            target_identity,
            temp_name,
            phase,
            ..
        } = record
        else {
            return Err(ArtifactError::RecoveryRequired);
        };
        validate_request_id(request_id)?;
        validate_temp_name(request_id, temp_name.as_deref())?;
        let (chain, target_path, leaf) = parent_and_leaf(&session.owner.root_path, relative_path)?;
        session.verify_chain(&chain)?;
        let temp = open_append_temp(session, temp_name.as_deref())?;
        let target = optional_mutation_file(&target_path)?;
        if *created && target_identity.is_none() {
            return Err(ArtifactError::RecoveryRequired);
        }
        Ok(Self {
            record: record.clone(),
            request_id: request_id.clone(),
            relative_path: relative_path.clone(),
            descriptor: descriptor.clone(),
            expected_digest: payload_digest.clone(),
            payload_length: *payload_length,
            prior_length: *prior_length,
            created: *created,
            target_identity: *target_identity,
            phase: *phase,
            chain,
            target_path,
            leaf,
            temp,
            target,
        })
    }
}

impl<'a> MutationSession<'a> {
    pub(super) fn recover_append(
        &mut self,
        record: &IntentRecord,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        let mut recovery = AppendRecovery::load(self, record)?;
        if recovery.created && matches!(recovery.phase, AppendPhase::Prepared) {
            prepare::advance(self, &mut recovery)?;
        }
        finish::complete(self, recovery)
    }
}

fn validate_temp_name(request_id: &str, temp_name: Option<&str>) -> Result<(), ArtifactError> {
    if temp_name.is_some_and(|name| name != format!("{request_id}.append.tmp")) {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}

fn open_append_temp(
    session: &MutationSession<'_>,
    temp_name: Option<&str>,
) -> Result<Option<OwnedFile>, ArtifactError> {
    let Some(name) = temp_name else {
        return Ok(None);
    };
    let path = session
        .owner
        .root_path
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(INTENTS_DIRECTORY)
        .join(name);
    optional_mutation_file(&path)
}
