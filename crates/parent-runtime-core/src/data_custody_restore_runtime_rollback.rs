use super::data_custody_restore_runtime::{
    ParentRestoreRuntime, RestoreRuntimeError, RestoreRuntimeReceipts,
};
use super::data_custody_restore_runtime_executor::{
    receipts::RestoreRollbackBinding, ProviderNeutralRestorePort, RestoreExecutorError,
    RestoreExecutorMount, RestoreExecutorOperationError,
};
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::
        RestoreExecutionStage;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_migration_execution::MigrationExecutionError,
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan,
};
use super::data_custody_restore_runtime_receipts::migration_receipt_from_dispatch;

#[derive(Debug)]
pub enum RestoreRollbackError {
    CompensationNotRequired,
    Migration(MigrationExecutionError),
}

pub(crate) fn execute_migration_operation(
    plan: &RestoreExecutionPlan,
    provider: &dyn ProviderNeutralRestorePort,
    recorded_at: contracts::ExportImportTimestamp,
) -> Result<contracts::ExportImportMigrationReceipt, RestoreExecutorOperationError> {
    let reservation = plan
        .execution_binding()
        .reserve_dispatch(plan.execution_ref(), RestoreExecutionStage::Migration)
        .map_err(|_| RestoreExecutorOperationError::Executor(RestoreExecutorError::Failed))?;
    let migration = match provider.execute_migration(plan, reservation) {
        Ok(provider_receipt) => {
            if provider_receipt.execution_ref() != plan.execution_ref() {
                return Err(RestoreExecutorOperationError::Executor(
                    RestoreExecutorError::Failed,
                ));
            }
            migration_receipt_from_dispatch(
                plan,
                contracts::ExportImportMigrationOutcome::Applied,
                plan.accepted_sections().to_vec(),
                plan.rejected_sections().to_vec(),
                PartialWriteCompensation::NotRequired,
                Some(provider_receipt.provider_operation_ref()),
                None,
                recorded_at.clone(),
                None,
            )
            .map_err(|error| RestoreExecutorOperationError::Migration(error))?
        }
        Err(_error) => migration_receipt_from_dispatch(
            plan,
            contracts::ExportImportMigrationOutcome::ManualRequired,
            plan.accepted_sections().to_vec(),
            plan.rejected_sections().to_vec(),
            PartialWriteCompensation::NotRequired,
            None,
            None,
            recorded_at,
            Some("Migration executor is unavailable; apply remains manual-required.".to_owned()),
        )
        .map_err(|error| RestoreExecutorOperationError::Migration(error))?,
    };
    Ok(migration)
}

pub(crate) fn record_rollback_migration(
    plan: &RestoreExecutionPlan,
    original_provider_operation_ref: Option<contracts::ExportImportProviderOperationRef>,
    rollback_provider_operation_ref: contracts::ExportImportProviderOperationRef,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    recorded_at: contracts::ExportImportTimestamp,
    _note: Option<String>,
) -> Result<contracts::ExportImportMigrationReceipt, RestoreRollbackError> {
    if !plan.no_resurrection() {
        return Err(RestoreRollbackError::CompensationNotRequired);
    }
    migration_receipt_from_dispatch(
        plan,
        contracts::ExportImportMigrationOutcome::RolledBack,
        applied_sections,
        rejected_sections,
        PartialWriteCompensation::Applied,
        original_provider_operation_ref.as_ref(),
        Some(&rollback_provider_operation_ref),
        recorded_at,
        Some("Migration rollback completed through the mounted provider port.".to_owned()),
    )
    .map_err(RestoreRollbackError::Migration)
}

impl ParentRestoreRuntime {
    pub(crate) async fn rollback<'a>(
        &mut self,
        plan: &'a RestoreExecutionPlan,
        mount: &RestoreExecutorMount<'_>,
        provider: &dyn ProviderNeutralRestorePort,
        applied_sections: Vec<contracts::ExportImportSectionDecision>,
        rejected_sections: Vec<contracts::ExportImportSectionDecision>,
        rollback_binding: RestoreRollbackBinding<'a>,
        _note: Option<String>,
    ) -> Result<RestoreRuntimeReceipts, RestoreRuntimeError> {
        self.rollback_after_observation(
            plan,
            mount,
            provider,
            applied_sections,
            rejected_sections,
            Some(rollback_binding),
        )
        .await
    }
}
