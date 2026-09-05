use super::*;

#[path = "owner_mutations_recovery_append.rs"]
mod append;
#[path = "owner_mutations_recovery_remove.rs"]
mod remove;
#[path = "owner_mutations_recovery_replace.rs"]
mod replace;
#[path = "owner_mutations_recovery_tree.rs"]
mod tree;

impl<'a> MutationSession<'a> {
    pub fn recover(&mut self) -> Result<Vec<MutationReceipt>, ArtifactError> {
        self.verify_current()?;
        reconcile_receipt_temps(&self.owner.root_path, self.metadata.receipt_directory()?)?;
        let records = read_intents(&self.owner.root_path, self.metadata.intent_directory()?)?;
        let mut recovered = Vec::new();
        for record in records {
            if let Some(receipt) = self.recover_record(&record)? {
                recovered.push(receipt);
            }
        }
        self.verify_current()?;
        Ok(recovered)
    }

    fn recover_record(
        &mut self,
        record: &IntentRecord,
    ) -> Result<Option<MutationReceipt>, ArtifactError> {
        if let Some(receipt) = read_receipt(
            &self.owner.root_path,
            record.request_id(),
            record.operation(),
            record.relative_path(),
            record.descriptor(),
        )? {
            remove_intent(
                &self.owner.root_path,
                record.request_id(),
                self.metadata.intent_directory()?,
            )?;
            return Ok(Some(receipt));
        }
        match record {
            IntentRecord::Append { .. } => self.recover_append(record),
            IntentRecord::Replace { .. } => self.recover_replace(record),
            IntentRecord::Remove { .. } => self.recover_remove(record),
            IntentRecord::Transaction { .. } => self.recover_transaction(record),
            IntentRecord::RemoveTree { .. } => self.recover_remove_tree(record),
        }
    }
}
