use super::data_custody_restore_runtime::{
    ParentRestoreRuntime, RestoreRuntimeError, RestoreRuntimeReceipts,
};
use super::data_custody_restore_runtime_executor::{
    ProviderNeutralRestorePort, RestoreExecutorError, RestoreExecutorOperationError,
};
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::
        RestoreExecutionStage;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_migration_execution::{
        complete_migration, MigrationExecutionError,
    },
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan,
};

#[derive(Debug)]
pub enum RestoreRollbackError {
    CompensationNotRequired,
    Migration(MigrationExecutionError),
}

pub(crate) fn execute_migration_operation(
    plan: &RestoreExecutionPlan,
    provider: &dyn ProviderNeutralRestorePort,
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
            complete_migration(
                plan,
                contracts::ExportImportMigrationOutcome::Applied,
                plan.accepted_sections().to_vec(),
                plan.rejected_sections().to_vec(),
                PartialWriteCompensation::NotRequired,
                Some(
                    provider_receipt
                        .provider_operation_ref()
                        .as_str()
                        .to_owned(),
                ),
                None,
                None,
            )
            .map_err(|error| RestoreExecutorOperationError::Migration(error))?
        }
        Err(_error) => complete_migration(
            plan,
            contracts::ExportImportMigrationOutcome::ManualRequired,
            plan.accepted_sections().to_vec(),
            plan.rejected_sections().to_vec(),
            PartialWriteCompensation::NotRequired,
            None,
            None,
            Some("Migration executor is unavailable; apply remains manual-required.".to_owned()),
        )
        .map_err(|error| RestoreExecutorOperationError::Migration(error))?,
    };
    Ok(migration)
}

pub fn record_rollback_migration(
    plan: &RestoreExecutionPlan,
    original_provider_operation_ref: Option<String>,
    rollback_provider_operation_ref: String,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    _note: Option<String>,
) -> Result<contracts::ExportImportMigrationReceipt, RestoreRollbackError> {
    if !plan.no_resurrection() {
        return Err(RestoreRollbackError::CompensationNotRequired);
    }
    complete_migration(
        plan,
        contracts::ExportImportMigrationOutcome::RolledBack,
        applied_sections,
        rejected_sections,
        PartialWriteCompensation::Applied,
        original_provider_operation_ref,
        Some(rollback_provider_operation_ref),
        Some("Migration rollback completed through the mounted provider port.".to_owned()),
    )
    .map_err(RestoreRollbackError::Migration)
}

impl ParentRestoreRuntime {
    pub(crate) async fn rollback(
        &mut self,
        plan: &RestoreExecutionPlan,
        provider: &dyn ProviderNeutralRestorePort,
        applied_sections: Vec<contracts::ExportImportSectionDecision>,
        rejected_sections: Vec<contracts::ExportImportSectionDecision>,
        _note: Option<String>,
    ) -> Result<RestoreRuntimeReceipts, RestoreRuntimeError> {
        self.rollback_after_observation(plan, provider, applied_sections, rejected_sections, None)
            .await
    }
}
