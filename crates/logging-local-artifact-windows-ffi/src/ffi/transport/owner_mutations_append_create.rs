use super::*;
use crate::constants::{
    APPEND_CREATED_PHASE, APPEND_OPERATION, BRIDGE_DIRECTORY, INTENTS_DIRECTORY,
    MUTATION_OWNER_DIRECTORY,
};

pub(super) fn create(
    session: &mut MutationSession<'_>,
    request_id: &str,
    preparation: &mut AppendPreparation,
) -> Result<(), ArtifactError> {
    let temp_name: String = intent_temp_name(request_id, APPEND_OPERATION);
    let temp_path = session
        .owner
        .root_path
        .join(BRIDGE_DIRECTORY)
        .join(MUTATION_OWNER_DIRECTORY)
        .join(INTENTS_DIRECTORY)
        .join(&temp_name);
    if optional_mutation_file(&temp_path)?.is_some() {
        return Err(ArtifactError::RecoveryRequired);
    }
    let temp = OwnedFile::create_new_mutation_file(&temp_path)?;
    temp.sync_file()?;
    let metadata = verify_metadata(&temp, false)?;
    let target_identity = Some(identity_record(metadata.identity));
    preparation.target_identity = target_identity;
    preparation.intent = preparation.intent.append_with_state(
        target_identity,
        Some(temp_name),
        AppendPhase::Prepared,
    )?;
    write_intent(
        &session.owner.root_path,
        request_id,
        &preparation.intent,
        session.metadata.intent_directory()?,
    )?;
    temp.rename_into(preparation.chain.leaf()?, &preparation.leaf)?;
    drop(temp);
    preparation.chain.leaf()?.sync_directory()?;
    session.verify_chain(&preparation.chain)?;
    preparation.intent = preparation.intent.append_with_state(
        target_identity,
        None::<&str>,
        AppendPhase::Created,
    )?;
    replace_intent(
        &session.owner.root_path,
        request_id,
        &preparation.intent,
        session.metadata.intent_directory()?,
        APPEND_CREATED_PHASE,
    )?;
    preparation.target = Some(OwnedFile::open_existing_mutation_file(
        &preparation.target_path,
    )?);
    let created_metadata = verify_metadata(
        preparation
            .target
            .as_ref()
            .ok_or(ArtifactError::RecoveryRequired)?,
        false,
    )?;
    if Some(identity_record(created_metadata.identity)) != target_identity
        || created_metadata.length != 0
    {
        return Err(ArtifactError::OwnershipChanged);
    }
    Ok(())
}
