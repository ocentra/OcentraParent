use super::*;
use crate::constants::REPLACE_OPERATION;

pub(super) fn complete(
    session: &mut MutationSession<'_>,
    mut recovery: ReplaceRecovery,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    let staged_identity = recovery
        .staged_identity
        .ok_or(ArtifactError::RecoveryRequired)?;
    let phase = recovery.phase;
    if matches!(phase, ReplacePhase::Quarantined)
        && recovery.target_identity.is_some()
        && recovery.quarantine.is_none()
    {
        return Err(ArtifactError::RecoveryRequired);
    }
    verify_old_target(&recovery)?;
    if let Some(file) = recovery.temp.as_mut() {
        verify_expected_new_file(file, &staged_identity, &recovery.expected_digest)?;
    }
    install_or_verify_target(session, &mut recovery, &staged_identity)?;
    delete_quarantine(&mut recovery)?;
    session.verify_chain(&recovery.chain)?;
    let receipt = write_receipt(
        &session.owner.root_path,
        recovery.request_id.as_str(),
        REPLACE_OPERATION,
        &recovery.relative_path,
        &recovery.descriptor,
        ReceiptOutcome::Replaced,
        session.metadata.receipt_directory()?,
    )?;
    remove_intent(
        &session.owner.root_path,
        recovery.request_id.as_str(),
        session.metadata.intent_directory()?,
    )?;
    Ok(Some(receipt))
}

fn verify_old_target(recovery: &ReplaceRecovery) -> Result<(), ArtifactError> {
    if let Some(file) = recovery.quarantine.as_ref() {
        verify_expected_identity(file, recovery.target_identity.as_ref())?;
    }
    Ok(())
}

fn install_or_verify_target(
    session: &mut MutationSession<'_>,
    recovery: &mut ReplaceRecovery,
    staged_identity: &IdentityRecord,
) -> Result<(), ArtifactError> {
    if let Some(file) = recovery.target.as_mut() {
        verify_expected_new_file(file, staged_identity, &recovery.expected_digest)?;
        let old_identity = recovery.target_identity;
        reject_old_target(old_identity.as_ref(), file)?;
        return Ok(());
    }
    let file = recovery
        .temp
        .take()
        .ok_or(ArtifactError::RecoveryRequired)?;
    file.rename_into(recovery.chain.leaf()?, &recovery.leaf)?;
    recovery.chain.leaf()?.sync_directory()?;
    recovery.target = Some(OwnedFile::open_existing_mutation_file(
        &session.owner.root_path.join(&recovery.relative_path),
    )?);
    let target = recovery
        .target
        .as_mut()
        .ok_or(ArtifactError::RecoveryRequired)?;
    verify_expected_new_file(target, staged_identity, &recovery.expected_digest)
}

fn reject_old_target(
    old_identity: Option<&IdentityRecord>,
    target: &OwnedFile,
) -> Result<(), ArtifactError> {
    let metadata = verify_metadata(target, false)?;
    if old_identity.is_some() && Some(identity_record(metadata.identity)) == old_identity.copied() {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}

fn delete_quarantine(recovery: &mut ReplaceRecovery) -> Result<(), ArtifactError> {
    if let Some(old) = recovery.quarantine.take() {
        old.mark_deleted()?;
        recovery.chain.leaf()?.sync_directory()?;
    }
    Ok(())
}
