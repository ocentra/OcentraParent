use super::data_custody_restore_runtime::{ParentRestoreRuntime, RestoreRuntimeError};
use super::data_custody_restore_runtime_ledger::RestoreLedger;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_migration_execution::complete_migration,
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan,
};

pub(crate) fn pending_operation_count(ledger: &RestoreLedger) -> usize {
    ledger.pending_migration_receipts().count() + ledger.pending_restore_receipts().count()
}

impl ParentRestoreRuntime {
    pub(crate) async fn reconcile_after_restart(
        &mut self,
        plan: &RestoreExecutionPlan,
    ) -> Result<usize, RestoreRuntimeError> {
        if pending_operation_count(&self.ledger) == 0 {
            return Ok(0);
        }
        let Some(receipt) = self.ledger.migration_receipt(plan.operation_ref()).cloned() else {
            return Ok(0);
        };
        if !matches!(
            receipt.outcome,
            contracts::ExportImportMigrationOutcome::Planned
                | contracts::ExportImportMigrationOutcome::Applied
                | contracts::ExportImportMigrationOutcome::Partial
        ) {
            return Ok(0);
        }
        let reconciled = complete_migration(
            plan,
            contracts::ExportImportMigrationOutcome::Reconciled,
            receipt.applied_sections,
            receipt.rejected_sections,
            PartialWriteCompensation::NotRequired,
            receipt
                .provider_operation_ref
                .map(|reference| reference.as_str().to_owned()),
            receipt
                .rollback_provider_operation_ref
                .map(|reference| reference.as_str().to_owned()),
            Some(
                "Restore or migration receipt requires parent-runtime restart reconciliation."
                    .to_owned(),
            ),
        )?;
        self.persist_migration(
            &reconciled,
            DataCustodyRuntimeEventKind::Reconciliation,
            reconciled.note.clone(),
        )
        .await?;
        Ok(1)
    }
}
