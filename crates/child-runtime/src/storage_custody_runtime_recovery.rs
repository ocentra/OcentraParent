use ocentra_storage_custody_core::{
    storage_custody::StorageCustodyEffectKind,
    storage_custody_effect_store::{StorageCustodyEffectRecord, StorageCustodyEffectStatus},
};

use super::{
    storage_custody_runtime_authority::record_still_matches_authority,
    storage_custody_runtime_validation::coherent_local_delete, ChildStorageCustodyOutcome,
    ChildStorageCustodyRuntime,
};
use crate::service::ChildAgentServiceError;

impl ChildStorageCustodyRuntime {
    pub(crate) async fn resume_pending_record(
        &self,
        record: &StorageCustodyEffectRecord,
    ) -> Result<ChildStorageCustodyOutcome, ChildAgentServiceError> {
        if record.status == StorageCustodyEffectStatus::Applying {
            return Ok(ChildStorageCustodyOutcome::PendingRecovery {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
            });
        }
        if let Some(outcome) = recovery_manual_outcome(self, record)? {
            return Ok(outcome);
        }
        if record.status == StorageCustodyEffectStatus::Prepared
            && !self.replay_record(record).await?
        {
            return Ok(ChildStorageCustodyOutcome::PendingJournalRetry {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
            });
        }
        if record.status == StorageCustodyEffectStatus::Prepared {
            self.effects
                .mark_journaled(&record.operation_ref)
                .map_err(ChildAgentServiceError::Storage)?;
        }
        self.finish_record(record).await
    }
}

fn recovery_manual_outcome(
    runtime: &ChildStorageCustodyRuntime,
    record: &StorageCustodyEffectRecord,
) -> Result<Option<ChildStorageCustodyOutcome>, ChildAgentServiceError> {
    if !record_still_matches_authority(&runtime.authority, record) {
        return Ok(Some(mark_manual(
            runtime,
            record,
            "current custody authority changed or was revoked during recovery",
        )?));
    }
    if record.effect_kind == StorageCustodyEffectKind::LocalDelete && !coherent_local_delete(record)
    {
        return Ok(Some(mark_manual(
            runtime,
            record,
            "local delete record is not a coherent expired custody action",
        )?));
    }
    Ok(None)
}

fn mark_manual(
    runtime: &ChildStorageCustodyRuntime,
    record: &StorageCustodyEffectRecord,
    reason: &str,
) -> Result<ChildStorageCustodyOutcome, ChildAgentServiceError> {
    runtime
        .effects
        .mark_manual_required(&record.operation_ref, reason)
        .map_err(ChildAgentServiceError::Storage)?;
    Ok(ChildStorageCustodyOutcome::ManualRequired {
        operation_ref: record.operation_ref.clone(),
        effect: record.effect_kind,
        reason: reason.to_owned(),
    })
}
