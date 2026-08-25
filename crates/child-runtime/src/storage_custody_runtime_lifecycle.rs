use std::path::Path;

use super::{
    storage_custody_effect_store::{
        StorageCustodyEffectRecord, StorageCustodyEffectStatus, StorageCustodyEffectStore,
    },
    storage_custody_runtime_validation::validate_effect_record_shape,
    ChildStorageCustodyRuntime,
};
use crate::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
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
            apply_lease_owner:
                super::storage_custody_runtime_reconciliation::next_apply_lease_owner(),
        })
    }

    pub(crate) async fn recover_pending(&self) -> Result<(), ChildAgentServiceError> {
        self.reconcile_applied_local_deletes().await?;
        let records = self
            .effects
            .pending_records()
            .map_err(ChildAgentServiceError::Storage)?;
        for record in records {
            if record.status() == StorageCustodyEffectStatus::Applying {
                // The effect-store process lock is held by this service
                // instance.  An Applying row therefore belongs to a prior
                // instance; crash timing cannot prove whether the filesystem
                // mutation happened, so reconcile it as manual rather than
                // retrying or claiming terminal success.
                self.effects
                    .mark_orphaned_apply_manual_required(&record.operation_ref)
                    .map_err(ChildAgentServiceError::Storage)?;
                continue;
            }
            let _ = self.resume_pending_record(&record).await?;
        }
        Ok(())
    }
}
