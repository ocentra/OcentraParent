use super::*;

pub(super) fn advance(
    session: &mut MutationSession<'_>,
    recovery: &mut AppendRecovery,
) -> Result<(), ArtifactError> {
    match (recovery.target.as_ref(), recovery.temp.take()) {
        (Some(_), Some(_)) => Err(ArtifactError::OwnershipChanged),
        (Some(file), None) => verify_empty_target(file, recovery.target_identity.as_ref()),
        (None, Some(file)) => attach_created_target(session, recovery, file),
        (None, None) => Err(ArtifactError::RecoveryRequired),
    }?;
    let progressed = recovery.record.append_with_state(
        recovery.target_identity,
        None::<&str>,
        AppendPhase::Created,
    )?;
    replace_intent(
        &session.owner.root_path,
        recovery.request_id.as_str(),
        &progressed,
        session.metadata.intent_directory()?,
        APPEND_CREATED_PHASE,
    )?;
    recovery.phase = AppendPhase::Created;
    Ok(())
}

fn verify_empty_target(
    file: &OwnedFile,
    target_identity: Option<&IdentityRecord>,
) -> Result<(), ArtifactError> {
    let metadata = verify_metadata(file, false)?;
    if Some(identity_record(metadata.identity)) != target_identity.cloned() || metadata.length != 0
    {
        return Err(ArtifactError::OwnershipChanged);
    }
    Ok(())
}

fn attach_created_target(
    session: &mut MutationSession<'_>,
    recovery: &mut AppendRecovery,
    file: OwnedFile,
) -> Result<(), ArtifactError> {
    verify_empty_target(&file, recovery.target_identity.as_ref())?;
    file.rename_into(recovery.chain.leaf()?, &recovery.leaf)?;
    recovery.chain.leaf()?.sync_directory()?;
    recovery.target = Some(
        optional_mutation_file(&recovery.target_path)?.ok_or(ArtifactError::RecoveryRequired)?,
    );
    session.verify_chain(&recovery.chain)
}
