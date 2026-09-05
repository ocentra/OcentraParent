use super::*;
use crate::constants::{APPEND_OPERATION, APPEND_WRITING_PHASE, APPEND_WRITTEN_PHASE};

pub(super) fn complete(
    session: &mut MutationSession<'_>,
    request_id: &str,
    relative_path: &str,
    payload: &[u8],
    mut preparation: AppendPreparation,
) -> Result<MutationReceipt, ArtifactError> {
    let mut target = preparation
        .target
        .take()
        .ok_or(ArtifactError::RecoveryRequired)?;
    let writing = preparation.intent.append_with_state(
        preparation.target_identity,
        None::<&str>,
        AppendPhase::Writing,
    )?;
    replace_intent(
        &session.owner.root_path,
        request_id,
        &writing,
        session.metadata.intent_directory()?,
        APPEND_WRITING_PHASE,
    )?;
    let offset = target.append_bounded(payload)?;
    target.sync_file()?;
    if preparation.created {
        preparation.chain.leaf()?.sync_directory()?;
    }
    let after = verify_metadata(&target, false)?;
    let expected_length = preparation
        .prior_length
        .checked_add(preparation.payload_length)
        .ok_or(ArtifactError::SizeLimit)?;
    if after.length != expected_length
        || preparation
            .target_identity
            .map(|identity| identity != identity_record(after.identity))
            .unwrap_or(false)
    {
        return Err(ArtifactError::OwnershipChanged);
    }
    session.verify_chain(&preparation.chain)?;
    let written = preparation.intent.append_with_state(
        preparation.target_identity,
        None::<&str>,
        AppendPhase::Written,
    )?;
    replace_intent(
        &session.owner.root_path,
        request_id,
        &written,
        session.metadata.intent_directory()?,
        APPEND_WRITTEN_PHASE,
    )?;
    let receipt = write_receipt(
        &session.owner.root_path,
        request_id,
        APPEND_OPERATION,
        relative_path,
        &preparation.descriptor,
        ReceiptOutcome::Appended {
            offset,
            length: preparation.payload_length,
        },
        session.metadata.receipt_directory()?,
    )?;
    let _ = remove_intent(
        &session.owner.root_path,
        request_id,
        session.metadata.intent_directory()?,
    );
    Ok(receipt)
}
