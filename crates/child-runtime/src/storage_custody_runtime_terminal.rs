use std::io;

use ocentra_storage_custody_core::storage_custody::StorageCustodyEffectKind;

use super::{
    storage_custody_effect_store::StorageCustodyEffectRecord,
    storage_custody_runtime_authority::record_still_matches_authority,
    storage_custody_runtime_delete::delete_local_file,
    storage_custody_runtime_reasons::manual_required_reason,
    storage_custody_runtime_validation::invalid_custody, ChildStorageCustodyOutcome,
    ChildStorageCustodyRuntime,
};
use crate::service::ChildAgentServiceError;

impl ChildStorageCustodyRuntime {
    pub(crate) async fn finish_record_by_ref(
        &self,
        operation_ref: &str,
    ) -> Result<ChildStorageCustodyOutcome, ChildAgentServiceError> {
        let record = self
            .effects
            .records()
            .map_err(ChildAgentServiceError::Storage)?
            .into_iter()
            .find(|record| record.operation_ref == operation_ref)
            .ok_or_else(|| invalid_custody("custody effect disappeared after journaling"))?;
        self.finish_record(&record).await
    }

    pub(crate) async fn finish_record(
        &self,
        record: &StorageCustodyEffectRecord,
    ) -> Result<ChildStorageCustodyOutcome, ChildAgentServiceError> {
        if !record_still_matches_authority(&self.authority, record) {
            self.effects
                .mark_manual_required(
                    &record.operation_ref,
                    "current custody authority changed or was revoked before local effect",
                )
                .map_err(ChildAgentServiceError::Storage)?;
            return Ok(ChildStorageCustodyOutcome::ManualRequired {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
                reason: "current custody authority changed or was revoked before local effect"
                    .to_owned(),
            });
        }
        if record.effect_kind != StorageCustodyEffectKind::LocalDelete {
            let reason = manual_required_reason(record.effect_kind);
            self.effects
                .mark_manual_required(&record.operation_ref, reason)
                .map_err(ChildAgentServiceError::Storage)?;
            return Ok(ChildStorageCustodyOutcome::ManualRequired {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
                reason: reason.to_owned(),
            });
        }
        let Some(relative_path) = record.relative_path.as_deref() else {
            return Err(invalid_custody("local delete record omitted relative path"));
        };
        finish_local_delete(self, record, relative_path).await
    }
}

async fn finish_local_delete(
    runtime: &ChildStorageCustodyRuntime,
    record: &StorageCustodyEffectRecord,
    relative_path: &str,
) -> Result<ChildStorageCustodyOutcome, ChildAgentServiceError> {
    let lease_id = runtime.local_apply_lease_id(record);
    runtime
        .effects
        .begin_local_apply(&record.operation_ref, &lease_id)
        .map_err(ChildAgentServiceError::Storage)?;
    if !record_still_matches_authority(&runtime.authority, record) {
        let reason = "current custody authority changed or was revoked during local effect claim";
        runtime
            .effects
            .mark_manual_required_with_lease(&record.operation_ref, &lease_id, reason)
            .map_err(ChildAgentServiceError::Storage)?;
        return Ok(ChildStorageCustodyOutcome::ManualRequired {
            operation_ref: record.operation_ref.clone(),
            effect: record.effect_kind,
            reason: reason.to_owned(),
        });
    }
    match delete_local_file(&runtime.root, relative_path) {
        Ok(()) => {
            runtime
                .effects
                .mark_applied(&record.operation_ref, &lease_id)
                .map_err(ChildAgentServiceError::Storage)?;
            let terminal_effect = super::StorageCustodyTerminalEffectCapability::new();
            runtime
                .flow
                .acknowledge_publication(&terminal_effect, &record.action)
                .await
                .map_err(ChildAgentServiceError::Storage)?;
            Ok(ChildStorageCustodyOutcome::Applied {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
            })
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let reason =
                "local payload is absent after a pending delete; manual reconciliation is required";
            runtime
                .effects
                .mark_manual_required_with_lease(&record.operation_ref, &lease_id, reason)
                .map_err(ChildAgentServiceError::Storage)?;
            Ok(ChildStorageCustodyOutcome::ManualRequired {
                operation_ref: record.operation_ref.clone(),
                effect: record.effect_kind,
                reason: reason.to_owned(),
            })
        }
        Err(error) => {
            runtime
                .effects
                .mark_manual_required_with_lease(
                    &record.operation_ref,
                    &lease_id,
                    "local delete failed; manual reconciliation is required",
                )
                .map_err(ChildAgentServiceError::Storage)?;
            Err(ChildAgentServiceError::Storage(error))
        }
    }
}
