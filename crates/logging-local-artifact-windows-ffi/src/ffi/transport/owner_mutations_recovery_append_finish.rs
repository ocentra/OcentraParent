use super::*;
use crate::platform::windows::Metadata;

pub(super) fn complete(
    session: &mut MutationSession<'_>,
    recovery: AppendRecovery,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    let mut recovery = recovery;
    let mut target = recovery
        .target
        .take()
        .ok_or(ArtifactError::OwnershipChanged)?;
    verify_expected_identity(&target, recovery.target_identity.as_ref())?;
    let metadata = verify_metadata(&target, false)?;
    if matches!(recovery.phase, AppendPhase::Created) {
        return remove_created_target(session, &recovery, target, metadata.length);
    }
    if matches!(recovery.phase, AppendPhase::Prepared) {
        remove_intent(
            &session.owner.root_path,
            recovery.request_id.as_str(),
            session.metadata.intent_directory()?,
        )?;
        return Ok(None);
    }
    finish_written_append(session, &recovery, &mut target, metadata)
}

fn remove_created_target(
    session: &mut MutationSession<'_>,
    recovery: &AppendRecovery,
    target: OwnedFile,
    length: u64,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    if length != 0 {
        return Err(ArtifactError::RecoveryRequired);
    }
    target.mark_deleted()?;
    recovery.chain.leaf()?.sync_directory()?;
    remove_intent(
        &session.owner.root_path,
        recovery.request_id.as_str(),
        session.metadata.intent_directory()?,
    )?;
    Ok(None)
}

fn finish_written_append(
    session: &mut MutationSession<'_>,
    recovery: &AppendRecovery,
    target: &mut OwnedFile,
    metadata: Metadata,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    let expected_end = recovery
        .prior_length
        .checked_add(recovery.payload_length)
        .ok_or(ArtifactError::SizeLimit)?;
    let bytes = target.read_bounded(MAX_ARTIFACT_BYTES)?;
    let prior_offset =
        usize::try_from(recovery.prior_length).map_err(|_| ArtifactError::SizeLimit)?;
    if metadata.length == expected_end
        && bytes.len() >= prior_offset
        && payload_digest::<_, String>(&bytes[prior_offset..]) == recovery.expected_digest
    {
        return append_receipt(session, recovery);
    }
    if metadata.length == recovery.prior_length {
        return finish_unwritten_append(session, recovery, target);
    }
    Err(ArtifactError::RecoveryRequired)
}

fn append_receipt(
    session: &mut MutationSession<'_>,
    recovery: &AppendRecovery,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    let receipt = write_receipt(
        &session.owner.root_path,
        recovery.request_id.as_str(),
        APPEND_OPERATION,
        &recovery.relative_path,
        &recovery.descriptor,
        ReceiptOutcome::Appended {
            offset: recovery.prior_length,
            length: recovery.payload_length,
        },
        session.metadata.receipt_directory()?,
    )?;
    remove_intent(
        &session.owner.root_path,
        recovery.request_id.as_str(),
        session.metadata.intent_directory()?,
    )?;
    Ok(Some(receipt))
}

fn finish_unwritten_append(
    session: &mut MutationSession<'_>,
    recovery: &AppendRecovery,
    target: &mut OwnedFile,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    if recovery.payload_length == 0 {
        return append_receipt(session, recovery);
    }
    if recovery.created {
        target.mark_deleted()?;
        recovery.chain.leaf()?.sync_directory()?;
    }
    remove_intent(
        &session.owner.root_path,
        recovery.request_id.as_str(),
        session.metadata.intent_directory()?,
    )?;
    Ok(None)
}
