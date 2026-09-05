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
use super::data_custody_restore_runtime_receipts::{
    migration_receipt_from_dispatch, MigrationReceiptDispatch,
};

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
        .map_err(RestoreExecutorOperationError::Reservation)?;
    let migration = match provider.execute_migration(plan, reservation) {
        Ok(provider_receipt) => {
            if provider_receipt.execution_ref() != plan.execution_ref() {
                return Err(RestoreExecutorOperationError::Executor(
                    RestoreExecutorError::Failed,
                ));
            }
            migration_receipt_from_dispatch(
                plan,
                MigrationReceiptDispatch {
                    outcome: contracts::ExportImportMigrationOutcome::Applied,
                    applied_sections: plan.accepted_sections().to_vec(),
                    rejected_sections: plan.rejected_sections().to_vec(),
                    compensation: PartialWriteCompensation::NotRequired,
                    provider_operation: Some(provider_receipt.provider_operation_ref()),
                    rollback_provider_operation: None,
                    recorded_at,
                    note: None,
                },
            )
            .map_err(RestoreExecutorOperationError::Migration)?
        }
        Err(error) => migration_receipt_from_dispatch(
            plan,
            MigrationReceiptDispatch {
                outcome: contracts::ExportImportMigrationOutcome::ManualRequired,
                applied_sections: plan.accepted_sections().to_vec(),
                rejected_sections: plan.rejected_sections().to_vec(),
                compensation: PartialWriteCompensation::NotRequired,
                provider_operation: None,
                rollback_provider_operation: None,
                recorded_at,
                note: Some(migration_provider_error_note(&error).to_owned()),
            },
        )
        .map_err(RestoreExecutorOperationError::Migration)?,
    };
    Ok(migration)
}

pub(crate) fn record_rollback_migration(
    plan: &RestoreExecutionPlan,
    original_provider_operation_ref: Option<&contracts::ExportImportProviderOperationRef>,
    rollback_provider_operation_ref: &contracts::ExportImportProviderOperationRef,
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
        MigrationReceiptDispatch {
            outcome: contracts::ExportImportMigrationOutcome::RolledBack,
            applied_sections,
            rejected_sections,
            compensation: PartialWriteCompensation::Applied,
            provider_operation: original_provider_operation_ref,
            rollback_provider_operation: Some(rollback_provider_operation_ref),
            recorded_at,
            note: Some(
                "Migration rollback completed through the mounted provider port.".to_owned(),
            ),
        },
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

fn migration_provider_error_note(error: &RestoreExecutorError) -> &'static str {
    match error {
        RestoreExecutorError::Unavailable => {
            "Migration executor is unavailable; apply remains manual-required."
        }
        RestoreExecutorError::Failed => {
            "Migration executor failed without a terminal receipt; apply remains manual-required."
        }
    }
}
