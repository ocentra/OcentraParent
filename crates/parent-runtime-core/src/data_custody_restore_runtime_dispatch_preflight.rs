use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan,
};

use super::data_custody_restore_runtime::{
    ParentRestoreRuntime, RestoreRuntimeError, RestoreRuntimeReceipts,
};
use super::data_custody_restore_runtime_executor::{
    ProviderNeutralRestorePort, RestoreExecutorMount,
};
use super::data_custody_restore_runtime_ledger::RestoreLedgerError;
use super::data_custody_restore_runtime_receipts::restore_receipt_from_dispatch;
use super::data_custody_restore_runtime_reconciliation_validation::restore_receipt_matches_plan;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;

pub(crate) enum RestorePreparation<'a> {
    Complete(RestoreRuntimeReceipts),
    Ready {
        existing_restore: contracts::ExportImportRestoreReceipt,
        provider: &'a dyn ProviderNeutralRestorePort,
        migration: Option<contracts::ExportImportMigrationReceipt>,
    },
}

impl ParentRestoreRuntime {
    pub(crate) async fn prepare_restore_dispatch<'a>(
        &mut self,
        plan: &RestoreExecutionPlan,
        mount: &'a RestoreExecutorMount<'a>,
    ) -> Result<RestorePreparation<'a>, RestoreRuntimeError> {
        let existing_restore = self.durable_restore(plan)?;
        self.revalidate_authority(plan, mount)?;
        if self
            .restart_pending_restore
            .contains(plan.operation_ref().as_str())
        {
            return self.reconcile_restore_after_restart(plan).await;
        }
        if self
            .restart_pending_rollback
            .contains(plan.operation_ref().as_str())
        {
            return Err(RestoreRuntimeError::RestartReconciliationRequired);
        }
        if let Some(completed) = self.completed_restore(plan, existing_restore.clone())? {
            return Ok(completed);
        }
        if self
            .dispatch_started_restore
            .contains(plan.operation_ref().as_str())
        {
            return Err(RestoreRuntimeError::RestartReconciliationRequired);
        }
        let Some(provider) = mount.provider() else {
            return self.block_restore_without_provider(plan).await;
        };
        let migration = self
            .execute_pending_migration_if_required(plan, mount, provider)
            .await?;
        if migration.as_ref().is_some_and(|receipt| {
            receipt.outcome != contracts::ExportImportMigrationOutcome::Applied
        }) {
            return Ok(RestorePreparation::Complete(RestoreRuntimeReceipts {
                restore: existing_restore,
                migration,
            }));
        }
        Ok(RestorePreparation::Ready {
            existing_restore,
            provider,
            migration,
        })
    }

    fn durable_restore(
        &self,
        plan: &RestoreExecutionPlan,
    ) -> Result<contracts::ExportImportRestoreReceipt, RestoreRuntimeError> {
        let existing_restore = self
            .ledger
            .restore_receipt(plan.operation_ref())
            .cloned()
            .ok_or(RestoreRuntimeError::PlanNotDurablyPending)?;
        if !restore_receipt_matches_plan(plan, &existing_restore) {
            return Err(RestoreRuntimeError::Ledger(
                RestoreLedgerError::IdentityMismatch,
            ));
        }
        Ok(existing_restore)
    }

    fn completed_restore(
        &self,
        plan: &RestoreExecutionPlan,
        existing_restore: contracts::ExportImportRestoreReceipt,
    ) -> Result<Option<RestorePreparation<'static>>, RestoreRuntimeError> {
        if existing_restore.state == contracts::ExportImportRestoreApplyState::ApplyPending {
            return Ok(None);
        }
        if existing_restore.state != contracts::ExportImportRestoreApplyState::Applied {
            return Ok(Some(RestorePreparation::Complete(RestoreRuntimeReceipts {
                restore: existing_restore,
                migration: self.ledger.migration_receipt(plan.operation_ref()).cloned(),
            })));
        }
        if self
            .ledger
            .migration_receipt(plan.operation_ref())
            .is_some_and(|receipt| {
                receipt.outcome == contracts::ExportImportMigrationOutcome::Planned
            })
        {
            return Err(RestoreRuntimeError::RestartReconciliationRequired);
        }
        Ok(Some(RestorePreparation::Complete(RestoreRuntimeReceipts {
            restore: existing_restore,
            migration: self.ledger.migration_receipt(plan.operation_ref()).cloned(),
        })))
    }

    async fn reconcile_restore_after_restart(
        &mut self,
        plan: &RestoreExecutionPlan,
    ) -> Result<RestorePreparation<'static>, RestoreRuntimeError> {
        let blocked = blocked_restore_after_restart(self, plan)?;
        self.persist_restore(
            &blocked,
            DataCustodyRuntimeEventKind::Reconciliation,
            blocked.note.clone(),
        )
        .await?;
        self.restart_pending_restore
            .remove(plan.operation_ref().as_str());
        Ok(RestorePreparation::Complete(RestoreRuntimeReceipts {
            restore: blocked,
            migration: None,
        }))
    }

    async fn block_restore_without_provider(
        &mut self,
        plan: &RestoreExecutionPlan,
    ) -> Result<RestorePreparation<'static>, RestoreRuntimeError> {
        let blocked = blocked_restore_without_provider(self, plan)?;
        self.persist_restore(
            &blocked,
            DataCustodyRuntimeEventKind::Reconciliation,
            blocked.note.clone(),
        )
        .await?;
        Ok(RestorePreparation::Complete(RestoreRuntimeReceipts {
            restore: blocked,
            migration: None,
        }))
    }
}

fn blocked_restore_after_restart(
    runtime: &ParentRestoreRuntime,
    plan: &RestoreExecutionPlan,
) -> Result<contracts::ExportImportRestoreReceipt, RestoreRuntimeError> {
    restore_receipt_from_dispatch(
        plan,
        contracts::ExportImportRestoreApplyState::Blocked,
        Vec::new(),
        plan.rejected_sections().to_vec(),
        PartialWriteCompensation::NotRequired,
        None,
        None,
        runtime.next_recorded_at()?,
        Some(
            "Restore was pending before restart; provider status reconciliation is required."
                .to_owned(),
        ),
    )
    .map_err(RestoreRuntimeError::Plan)
}

fn blocked_restore_without_provider(
    runtime: &ParentRestoreRuntime,
    plan: &RestoreExecutionPlan,
) -> Result<contracts::ExportImportRestoreReceipt, RestoreRuntimeError> {
    restore_receipt_from_dispatch(
        plan,
        contracts::ExportImportRestoreApplyState::Blocked,
        Vec::new(),
        plan.rejected_sections().to_vec(),
        PartialWriteCompensation::NotRequired,
        None,
        None,
        runtime.next_recorded_at()?,
        Some("Restore executor is not mounted; local truth remains unchanged.".to_owned()),
    )
    .map_err(RestoreRuntimeError::Plan)
}
