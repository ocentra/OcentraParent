use super::*;

#[path = "owner_mutations_transaction_recovery_finish.rs"]
mod finish;
#[path = "owner_mutations_transaction_recovery_plans.rs"]
mod plans;
#[path = "owner_mutations_transaction_recovery_prepared.rs"]
mod prepared;

struct TransactionRecoveryInput {
    request_id: String,
    descriptor: String,
    staged: Vec<StagedMutation>,
    phase: TransactionPhase,
}

impl TransactionRecoveryInput {
    fn read(record: &IntentRecord) -> Result<Self, ArtifactError> {
        let IntentRecord::Transaction {
            request_id,
            descriptor,
            staged,
            phase,
            ..
        } = record
        else {
            return Err(ArtifactError::RecoveryRequired);
        };
        Ok(Self {
            request_id: request_id.clone(),
            descriptor: descriptor.clone(),
            staged: staged.clone(),
            phase: *phase,
        })
    }
}

impl<'a> MutationSession<'a> {
    pub(crate) fn recover_transaction(
        &mut self,
        record: &IntentRecord,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        let input = TransactionRecoveryInput::read(record)?;
        validate_request_id(&input.request_id)?;
        if input.staged.is_empty()
            || input.staged.len() > crate::owner_types::MAX_TRANSACTION_MUTATIONS
        {
            return Err(ArtifactError::SizeLimit);
        }
        let plans = plans::open(self, &input.request_id, &input.staged)?;
        if matches!(input.phase, TransactionPhase::Prepared) {
            return prepared::recover(self, record, &input, plans);
        }
        finish::complete(self, record, &input, plans)
    }
}
