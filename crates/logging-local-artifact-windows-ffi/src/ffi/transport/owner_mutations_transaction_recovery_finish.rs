use super::*;
use crate::constants::{TRANSACTION_INSTALLED_PHASE, TRANSACTION_OPERATION};

#[path = "owner_mutations_transaction_recovery_finish_cleanup.rs"]
mod cleanup;
#[path = "owner_mutations_transaction_recovery_finish_verify.rs"]
mod verify;

pub(super) fn complete(
    session: &mut MutationSession<'_>,
    record: &IntentRecord,
    input: &super::TransactionRecoveryInput,
    mut plans: Vec<TransactionRecoveryPlan>,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    let installing = matches!(input.phase, TransactionPhase::Quarantined);
    verify::installation(session, installing, &mut plans)?;
    write_installed_intent(session, record, input, installing)?;
    cleanup::plans(session, &mut plans)?;
    finish_receipt(session, input, plans.len())
}

fn write_installed_intent(
    session: &mut MutationSession<'_>,
    record: &IntentRecord,
    input: &super::TransactionRecoveryInput,
    installing: bool,
) -> Result<(), ArtifactError> {
    if !installing {
        return Ok(());
    }
    let progressed = record.transaction_with_phase(TransactionPhase::Installed)?;
    replace_intent(
        &session.owner.root_path,
        &input.request_id,
        &progressed,
        session.metadata.intent_directory()?,
        TRANSACTION_INSTALLED_PHASE,
    )
}

fn finish_receipt(
    session: &mut MutationSession<'_>,
    input: &super::TransactionRecoveryInput,
    count: usize,
) -> Result<Option<MutationReceipt>, ArtifactError> {
    session.verify_current()?;
    let count = u32::try_from(count).map_err(|_| ArtifactError::SizeLimit)?;
    let receipt = write_receipt(
        &session.owner.root_path,
        &input.request_id,
        TRANSACTION_OPERATION,
        TRANSACTION_OPERATION,
        &input.descriptor,
        ReceiptOutcome::TransactionCommitted { count },
        session.metadata.receipt_directory()?,
    )?;
    remove_intent(
        &session.owner.root_path,
        &input.request_id,
        session.metadata.intent_directory()?,
    )?;
    Ok(Some(receipt))
}
