use ocentra_storage_custody_core::{
    storage_custody::StorageCustodyEffectKind,
    storage_custody_effect_store::{StorageCustodyEffectRecord, StorageCustodyEffectStore},
};

use super::{
    storage_custody_runtime_authority::record_still_matches_authority,
    storage_custody_runtime_validation::{coherent_local_delete, validate_effect_record_shape},
    ChildStorageCustodyOutcome, ChildStorageCustodyReadiness, ChildStorageCustodyRuntime,
};
use crate::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
    runtime_gate_tombstone::ChildRuntimeTombstonePublicationOutcome,
    service::ChildAgentServiceError,
};

impl ChildStorageCustodyRuntime {
    pub(crate) fn open(
        root: &Path,
        flow: ChildRuntimeTombstoneEventFlow,
        authority: super::ChildStorageCustodyAuthorityHandle,
    ) -> Result<Self, ChildAgentServiceError> {
        let root = root
            .canonicalize()
            .map_err(ChildAgentServiceError::Storage)?;
        let effects = StorageCustodyEffectStore::open(root.join("custody-effects"))
            .map_err(ChildAgentServiceError::Storage)?;
        let existing_records = effects.records().map_err(ChildAgentServiceError::Storage)?;
        for record in &existing_records {
            validate_effect_record_shape(record)?;
        }
        Ok(Self {
            root,
            flow,
            effects,
            authority,
        })
    }

    pub(crate) fn readiness(&self) -> ChildStorageCustodyReadiness {
        if self.authority.has_current_binding() {
            ChildStorageCustodyReadiness::CurrentAuthority
        } else {
            ChildStorageCustodyReadiness::ManualRequired
        }
    }

    pub(crate) async fn recover_pending(&self) -> Result<(), ChildAgentServiceError> {
        let records = self
            .effects
            .pending_records()
            .map_err(ChildAgentServiceError::Storage)?;
        for record in records {
            let _ = self.resume_pending_record(&record).await?;
        }
        Ok(())
    }

    pub(crate) async fn resume_pending_record(
        &self,
        record: &StorageCustodyEffectRecord,
    ) -> Result<ChildStorageCustodyOutcome, ChildAgentServiceError> {
        if !record_still_matches_authority(&self.authority, record) {
            let reason = "current custody authority changed or was revoked during recovery";
            self.effects
                .mark_manual_required(&record.operation_ref, reason)
                .map_err(ChildAgentServiceError::Storage)?;
            return Ok(ChildStorageCustodyOutcome::ManualRequired {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
                reason: reason.to_owned(),
            });
        }
        if record.effect_kind == StorageCustodyEffectKind::LocalDelete
            && !coherent_local_delete(record)
        {
            let reason = "local delete record is not a coherent expired custody action";
            self.effects
                .mark_manual_required(&record.operation_ref, reason)
                .map_err(ChildAgentServiceError::Storage)?;
            return Ok(ChildStorageCustodyOutcome::ManualRequired {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
                reason: reason.to_owned(),
            });
        }
        if !self.replay_record(record).await? {
            return Ok(ChildStorageCustodyOutcome::PendingJournalRetry {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
            });
        }
        self.effects
            .mark_journaled(&record.operation_ref)
            .map_err(ChildAgentServiceError::Storage)?;
        self.finish_record(record).await
    }

    pub(crate) async fn replay_record(
        &self,
        record: &StorageCustodyEffectRecord,
    ) -> Result<bool, ChildAgentServiceError> {
        validate_effect_record_shape(record)?;
        if record.effect_kind == StorageCustodyEffectKind::LocalDelete {
            match self
                .flow
                .publish_stored_action(&record.envelope, &record.action)
                .await
                .map_err(ChildAgentServiceError::Storage)?
            {
                ChildRuntimeTombstonePublicationOutcome::Journaled(_) => Ok(true),
                ChildRuntimeTombstonePublicationOutcome::PendingJournalRetry(_) => Ok(false),
            }
        } else {
            self.flow
                .publish_stored_action_to_journal(&record.envelope)
                .await
                .map_err(ChildAgentServiceError::Storage)?;
            Ok(true)
        }
    }
}
