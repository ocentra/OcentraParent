use super::super::data_custody_restore_runtime::RestoreRuntimeError;
use super::super::data_custody_restore_runtime_executor::{
    receipts::RestoreRollbackBinding, RestoreExecutorError,
};
use super::super::data_custody_restore_runtime_ledger::RestoreLedgerError;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan;

pub(super) fn validate_rollback_authority<'a>(
    plan: &'a RestoreExecutionPlan,
    existing_restore: &contracts::ExportImportRestoreReceipt,
    observed_rollback_binding: Option<&RestoreRollbackBinding<'a>>,
) -> Result<Option<contracts::ExportImportProviderOperationRef>, RestoreRuntimeError> {
    if !existing_restore.compensation_applied
        && existing_restore.rollback_provider_operation_ref.is_some()
    {
        return Err(RestoreRuntimeError::Ledger(
            RestoreLedgerError::IdentityMismatch,
        ));
    }
    if existing_restore.compensation_applied {
        return Err(RestoreRuntimeError::Ledger(
            RestoreLedgerError::IdentityMismatch,
        ));
    }
    let observed_provider_operation_ref =
        observed_rollback_binding.map(|binding| binding.provider_operation_ref().clone());
    if let (Some(existing), Some(observed)) = (
        existing_restore.provider_operation_ref.as_ref(),
        observed_provider_operation_ref.as_ref(),
    ) {
        if existing != observed {
            return Err(RestoreRuntimeError::Ledger(
                RestoreLedgerError::IdentityMismatch,
            ));
        }
    }
    if existing_restore.provider_operation_ref.is_none()
        && observed_provider_operation_ref.is_none()
    {
        return Err(RestoreRuntimeError::Executor(RestoreExecutorError::Failed));
    }
    let Some(rollback_binding) = observed_rollback_binding else {
        // A persisted ref is journal evidence, not restartable provider
        // authority. The sealed provider binding must be present in memory.
        return Err(RestoreRuntimeError::Executor(RestoreExecutorError::Failed));
    };
    if !rollback_binding.is_bound_to(plan.execution_binding(), plan.execution_ref()) {
        return Err(RestoreRuntimeError::Executor(RestoreExecutorError::Failed));
    }
    observed_provider_operation_ref
        .or_else(|| existing_restore.provider_operation_ref.clone())
        .map(Some)
        .ok_or(RestoreRuntimeError::Executor(RestoreExecutorError::Failed))
}
