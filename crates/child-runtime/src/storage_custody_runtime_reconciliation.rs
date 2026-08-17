use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_storage_custody_core::storage_custody::StorageCustodyEffectKind;

use super::{
    storage_custody_effect_store::{StorageCustodyEffectRecord, StorageCustodyEffectStatus},
    ChildStorageCustodyRuntime, StorageCustodyTerminalEffectCapability,
};
use crate::service::ChildAgentServiceError;

static NEXT_APPLY_LEASE_OWNER: AtomicU64 = AtomicU64::new(1);

pub(crate) fn next_apply_lease_owner() -> String {
    format!(
        "child-storage-custody:{}:{}",
        std::process::id(),
        NEXT_APPLY_LEASE_OWNER.fetch_add(1, Ordering::Relaxed)
    )
}

impl ChildStorageCustodyRuntime {
    pub(crate) fn local_apply_lease_id(&self, record: &StorageCustodyEffectRecord) -> String {
        format!(
            "{}:authority-generation:{}:session-generation:{}:{}",
            self.apply_lease_owner,
            record.authority_generation,
            record.session_generation,
            record.operation_ref
        )
    }

    pub(crate) async fn acknowledge_applied_record(
        &self,
        operation_ref: &str,
    ) -> Result<(), ChildAgentServiceError> {
        let Some(record) = self
            .effects
            .records()
            .map_err(ChildAgentServiceError::Storage)?
            .into_iter()
            .find(|record| record.operation_ref == operation_ref)
        else {
            return Err(ChildAgentServiceError::Storage(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("unknown custody effect operation: {operation_ref}"),
            )));
        };
        if record.status() != StorageCustodyEffectStatus::Applied
            || record.effect_kind != StorageCustodyEffectKind::LocalDelete
        {
            return Ok(());
        }
        let terminal_effect = StorageCustodyTerminalEffectCapability::new();
        self.flow
            .acknowledge_publication(&terminal_effect, &record.action)
            .await
            .map_err(ChildAgentServiceError::Storage)
    }

    pub(crate) async fn reconcile_applied_local_deletes(
        &self,
    ) -> Result<(), ChildAgentServiceError> {
        let applied = self
            .effects
            .records()
            .map_err(ChildAgentServiceError::Storage)?
            .into_iter()
            .filter(|record| {
                record.status() == StorageCustodyEffectStatus::Applied
                    && record.effect_kind == StorageCustodyEffectKind::LocalDelete
            })
            .map(|record| record.operation_ref)
            .collect::<Vec<_>>();
        for operation_ref in applied {
            self.acknowledge_applied_record(&operation_ref).await?;
        }
        Ok(())
    }
}
