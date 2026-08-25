use ocentra_storage_custody_core::storage_custody::StorageCustodyEffectKind;

use super::{
    storage_custody_effect_store::StorageCustodyEffectRecord,
    storage_custody_runtime_validation::validate_effect_record_shape, ChildStorageCustodyRuntime,
};
use crate::{
    runtime_gate_tombstone::ChildRuntimeTombstonePublicationOutcome,
    service::ChildAgentServiceError,
};

impl ChildStorageCustodyRuntime {
    pub(crate) async fn replay_record(
        &self,
        record: &StorageCustodyEffectRecord,
    ) -> Result<bool, ChildAgentServiceError> {
        validate_effect_record_shape(record)?;
        if record.effect_kind == StorageCustodyEffectKind::LocalDelete {
            return match self
                .flow
                .publish_stored_action(&record.envelope, &record.action)
                .await
                .map_err(ChildAgentServiceError::Storage)?
            {
                ChildRuntimeTombstonePublicationOutcome::Journaled(_) => Ok(true),
                ChildRuntimeTombstonePublicationOutcome::PendingJournalRetry(_) => Ok(false),
            };
        }
        self.flow
            .publish_stored_action_to_journal(&record.envelope)
            .await
            .map_err(ChildAgentServiceError::Storage)?;
        Ok(true)
    }
}
