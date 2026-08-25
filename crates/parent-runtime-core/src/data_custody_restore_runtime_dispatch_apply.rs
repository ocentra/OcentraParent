use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_migration_execution::plan_migration;

use super::data_custody_restore_runtime::{ParentRestoreRuntime, RestoreRuntimeError};
use super::data_custody_restore_runtime_executor::{ProviderNeutralRestorePort, RestoreExecutorMount};
use super::data_custody_restore_runtime_ledger::RestoreLedgerError;
use super::data_custody_restore_runtime_reconciliation_validation::{
    migration_receipt_matches_plan, plan_migration_manual_required,
};
use super::data_custody_restore_runtime_rollback::execute_migration_operation;
use super::data_custody_restore_runtime_dispatch::RESTORE_MIGRATION_BEFORE_DISPATCH_NOTE;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan;

impl ParentRestoreRuntime {
    pub(crate) async fn execute_pending_migration_if_required(
        &mut self,
        plan: &RestoreExecutionPlan,
        mount: &RestoreExecutorMount<'_>,
        provider: &dyn ProviderNeutralRestorePort,
    ) -> Result<Option<contracts::ExportImportMigrationReceipt>, RestoreRuntimeError> {
        if plan.migration_ref().is_none() {
            return Ok(None);
        }
        let existing = self.ledger.migration_receipt(plan.operation_ref()).cloned();
        if let Some(receipt) = existing.as_ref() {
            validate_existing_migration(plan, receipt)?;
            if receipt.outcome != contracts::ExportImportMigrationOutcome::Planned {
                return Ok(Some(receipt.clone()));
            }
        }
        if self
            .restart_pending_migration
            .contains(plan.operation_ref().as_str())
        {
            let Some(receipt) = existing.as_ref() else {
                return Err(RestoreRuntimeError::PlanNotDurablyPending);
            };
            let manual = plan_migration_manual_required(plan, receipt, self.next_recorded_at()?)?;
            self.persist_migration(
                &manual,
                DataCustodyRuntimeEventKind::Reconciliation,
                manual.note.clone(),
            )
            .await?;
            self.restart_pending_migration
                .remove(plan.operation_ref().as_str());
            return Ok(Some(manual));
        }
        if self
            .dispatch_started_migration
            .contains(plan.operation_ref().as_str())
        {
            return Err(RestoreRuntimeError::RestartReconciliationRequired);
        }
        let mut planned = match existing {
            Some(receipt) => receipt,
            None => plan_migration(plan)?,
        };
        planned.recorded_at = self.next_recorded_at()?;
        self.persist_migration_phase(
            &planned,
            DataCustodyRuntimeEventKind::MigrationBeforeDispatch,
            Some(RESTORE_MIGRATION_BEFORE_DISPATCH_NOTE.to_owned()),
            JournalDispatchPhase::BeforeDispatch,
        )
        .await?;
        self.revalidate_authority(plan, mount)?;
        self.dispatch_started_migration
            .insert(plan.operation_ref().as_str().to_owned());
        let migration = execute_migration_operation(plan, provider, self.next_recorded_at()?)?;
        self.persist_migration(
            &migration,
            DataCustodyRuntimeEventKind::MigrationReceipt,
            migration.note.clone(),
        )
        .await?;
        self.dispatch_started_migration
            .remove(plan.operation_ref().as_str());
        Ok(Some(migration))
    }
}

fn validate_existing_migration(
    plan: &RestoreExecutionPlan,
    receipt: &contracts::ExportImportMigrationReceipt,
) -> Result<(), RestoreRuntimeError> {
    if migration_receipt_matches_plan(plan, receipt) {
        Ok(())
    } else {
        Err(RestoreRuntimeError::Ledger(
            RestoreLedgerError::IdentityMismatch,
        ))
    }
}
