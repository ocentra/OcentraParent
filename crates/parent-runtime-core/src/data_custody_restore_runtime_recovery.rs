use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_restore_runtime::{ParentRestoreRuntime, RestoreRuntimeError};
use super::data_custody_restore_runtime_ledger::{RestoreDispatchStage, RestoreLedger};
use super::data_custody_runtime_eventing::{
    DataCustodyRuntimeEvent, DataCustodyRuntimeEventJournal, DataCustodyRuntimeEventKind,
};

impl ParentRestoreRuntime {
    pub async fn recover(&mut self) -> Result<(), RestoreRuntimeError> {
        self.recovered = false;
        self.ledger = RestoreLedger::default();
        self.restart_pending_restore.clear();
        self.restart_pending_migration.clear();
        self.restart_pending_rollback.clear();
        self.journal.recover().await?;
        let report = self.journal.replay().await?;
        if report.skipped_count != 0 {
            return Err(RestoreRuntimeError::ReplaySkipped(report.skipped_count));
        }
        for record in report.records {
            let event = DataCustodyRuntimeEventJournal::decode(&record.envelope)
                .map_err(RestoreRuntimeError::ReplayDecode)?;
            apply_replayed_event(self, event)?;
        }
        self.restart_pending_restore = self
            .ledger
            .pending_restore_receipts()
            .filter(|receipt| restore_requires_restart(receipt, &self.ledger))
            .map(|receipt| receipt.operation_ref.as_str().to_owned())
            .collect();
        self.restart_pending_migration = self
            .ledger
            .pending_migration_receipts()
            .filter(|receipt| migration_requires_restart(receipt, &self.ledger))
            .map(|receipt| receipt.operation_ref.as_str().to_owned())
            .collect();
        self.dispatch_started_restore.clear();
        self.dispatch_started_migration.clear();
        self.dispatch_started_rollback.clear();
        self.recovered = true;
        Ok(())
    }
}

fn apply_replayed_event(
    runtime: &mut ParentRestoreRuntime,
    event: DataCustodyRuntimeEvent,
) -> Result<(), RestoreRuntimeError> {
    match event.kind {
        DataCustodyRuntimeEventKind::RollbackBeforeDispatch => {
            runtime
                .restart_pending_rollback
                .insert(event.operation_ref.clone());
        }
        DataCustodyRuntimeEventKind::Rollback | DataCustodyRuntimeEventKind::Reconciliation => {
            runtime
                .restart_pending_rollback
                .remove(&event.operation_ref);
        }
        _ => {}
    }
    runtime.ledger.apply_event(&event)?;
    Ok(())
}

fn restore_requires_restart(
    receipt: &contracts::ExportImportRestoreReceipt,
    ledger: &RestoreLedger,
) -> bool {
    receipt.state == contracts::ExportImportRestoreApplyState::ApplyPending
        && matches!(
            ledger.restore_stage(&receipt.operation_ref),
            Some(RestoreDispatchStage::Planned | RestoreDispatchStage::BeforeDispatch)
        )
}

fn migration_requires_restart(
    receipt: &contracts::ExportImportMigrationReceipt,
    ledger: &RestoreLedger,
) -> bool {
    receipt.outcome == contracts::ExportImportMigrationOutcome::Planned
        && matches!(
            ledger.migration_stage(&receipt.operation_ref),
            Some(RestoreDispatchStage::Planned | RestoreDispatchStage::BeforeDispatch)
        )
}
