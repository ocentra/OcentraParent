use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;
use ocentra_schema::export_import_backup_recovery::ExportImportExecutionRef;

pub(crate) fn execution_idempotency_ref(
    family: &str,
    execution_ref: &ExportImportExecutionRef,
    kind: &DataCustodyRuntimeEventKind,
) -> String {
    format!(
        "{family}:{}:{}",
        execution_ref,
        match kind {
            DataCustodyRuntimeEventKind::RestorePlanned
            | DataCustodyRuntimeEventKind::MigrationPlanned => "planned",
            DataCustodyRuntimeEventKind::RestoreBeforeDispatch
            | DataCustodyRuntimeEventKind::MigrationBeforeDispatch => "before-dispatch",
            DataCustodyRuntimeEventKind::RestoreApplied
            | DataCustodyRuntimeEventKind::MigrationReceipt => "applied",
            DataCustodyRuntimeEventKind::Rollback => "rollback",
            DataCustodyRuntimeEventKind::RollbackBeforeDispatch => "rollback-before-dispatch",
            DataCustodyRuntimeEventKind::Reconciliation => "reconciliation",
            DataCustodyRuntimeEventKind::BackupScheduled
            | DataCustodyRuntimeEventKind::BackupJobTransition => "invalid",
        }
    )
}
