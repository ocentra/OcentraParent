use std::collections::BTreeMap;

use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_restore_runtime::{ParentRestoreRuntime, RestoreRuntimeError};
use super::data_custody_restore_runtime_executor::RestoreExecutorMount;
#[path = "data_custody_restore_runtime_ledger_event_apply.rs"]
mod event_apply;
use super::data_custody_restore_runtime_ledger_validation::{
    validate_migration_receipt, validate_restore_receipt,
};
use super::data_custody_restore_runtime_receipts::restore_receipt_from_dispatch;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEvent;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_migration_execution::{plan_migration, MigrationExecutionError},
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan,
};

#[derive(Debug)]
pub(crate) enum RestoreLedgerError {
    MissingPriorReceipt,
    IdentityMismatch,
    InvalidRestoreTransition,
    InvalidMigrationTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RestoreDispatchStage {
    Planned,
    BeforeDispatch,
    Terminal,
    RollbackBeforeDispatch,
    Rollback,
    Reconciled,
}

#[derive(Debug, Default)]
pub(crate) struct RestoreLedger {
    restore_receipts: BTreeMap<String, contracts::ExportImportRestoreReceipt>,
    migration_receipts: BTreeMap<String, contracts::ExportImportMigrationReceipt>,
    restore_stages: BTreeMap<String, RestoreDispatchStage>,
    migration_stages: BTreeMap<String, RestoreDispatchStage>,
}

impl RestoreLedger {
    pub(crate) fn apply_event(
        &mut self,
        event: &DataCustodyRuntimeEvent,
    ) -> Result<(), RestoreLedgerError> {
        event_apply::apply_event(self, event)
    }

    pub(crate) fn insert_restore_receipt(
        &mut self,
        receipt: contracts::ExportImportRestoreReceipt,
    ) -> Result<(), RestoreLedgerError> {
        if let Some(previous) = self.restore_receipts.get(receipt.operation_ref.as_str()) {
            validate_restore_receipt(previous, &receipt)?;
        }
        self.restore_receipts
            .insert(receipt.operation_ref.as_str().to_owned(), receipt);
        Ok(())
    }

    pub(crate) fn insert_migration_receipt(
        &mut self,
        receipt: contracts::ExportImportMigrationReceipt,
    ) -> Result<(), RestoreLedgerError> {
        if let Some(previous) = self.migration_receipts.get(receipt.operation_ref.as_str()) {
            validate_migration_receipt(previous, &receipt)?;
        }
        self.migration_receipts
            .insert(receipt.operation_ref.as_str().to_owned(), receipt);
        Ok(())
    }

    pub(crate) fn migration_receipt(
        &self,
        operation_ref: &contracts::ExportImportOperationRef,
    ) -> Option<&contracts::ExportImportMigrationReceipt> {
        self.migration_receipts.get(operation_ref.as_str())
    }

    pub(crate) fn restore_stage(
        &self,
        operation_ref: &contracts::ExportImportOperationRef,
    ) -> Option<RestoreDispatchStage> {
        self.restore_stages.get(operation_ref.as_str()).copied()
    }

    pub(crate) fn migration_stage(
        &self,
        operation_ref: &contracts::ExportImportOperationRef,
    ) -> Option<RestoreDispatchStage> {
        self.migration_stages.get(operation_ref.as_str()).copied()
    }

    pub(crate) fn set_restore_stage(
        &mut self,
        operation_ref: &contracts::ExportImportOperationRef,
        stage: RestoreDispatchStage,
    ) {
        self.restore_stages
            .insert(operation_ref.as_str().to_owned(), stage);
    }

    pub(crate) fn set_migration_stage(
        &mut self,
        operation_ref: &contracts::ExportImportOperationRef,
        stage: RestoreDispatchStage,
    ) {
        self.migration_stages
            .insert(operation_ref.as_str().to_owned(), stage);
    }

    pub(crate) fn restore_receipt(
        &self,
        operation_ref: &contracts::ExportImportOperationRef,
    ) -> Option<&contracts::ExportImportRestoreReceipt> {
        self.restore_receipts.get(operation_ref.as_str())
    }

    pub(crate) fn pending_migration_receipts(
        &self,
    ) -> impl Iterator<Item = &contracts::ExportImportMigrationReceipt> {
        self.migration_receipts.values().filter(|receipt| {
            matches!(
                receipt.outcome,
                contracts::ExportImportMigrationOutcome::Planned
                    | contracts::ExportImportMigrationOutcome::Applied
                    | contracts::ExportImportMigrationOutcome::Partial
            )
        })
    }

    pub(crate) fn pending_restore_receipts(
        &self,
    ) -> impl Iterator<Item = &contracts::ExportImportRestoreReceipt> {
        self.restore_receipts.values().filter(|receipt| {
            matches!(
                receipt.state,
                contracts::ExportImportRestoreApplyState::ApplyPending
                    | contracts::ExportImportRestoreApplyState::Partial
            )
        })
    }
}

impl ParentRestoreRuntime {
    pub(crate) async fn plan_restore(
        &mut self,
        bundle: &contracts::ExportImportRecoveryBundle,
        mount: &RestoreExecutorMount<'_>,
        plan_ref: impl Into<String>,
        operation_ref: impl Into<String>,
        execution_ref: impl Into<String>,
    ) -> Result<
        (
            RestoreExecutionPlan,
            Option<contracts::ExportImportMigrationReceipt>,
        ),
        RestoreRuntimeError,
    > {
        if !self.recovered {
            return Err(RestoreRuntimeError::RuntimeNotRecovered);
        }
        let plan = self.bind_plan(bundle, mount, plan_ref, operation_ref, execution_ref)?;
        let pending_restore = restore_receipt_from_dispatch(
            &plan,
            contracts::ExportImportRestoreApplyState::ApplyPending,
            Vec::new(),
            plan.rejected_sections().to_vec(),
            PartialWriteCompensation::NotRequired,
            None,
            None,
            self.next_recorded_at()?,
            Some("Restore plan is bound; apply remains parent-runtime-owned.".to_owned()),
        )?;
        self.persist_restore(
            &pending_restore,
            DataCustodyRuntimeEventKind::RestorePlanned,
            pending_restore.note.clone(),
        )
        .await?;
        let migration = match plan_migration(&plan) {
            Ok(mut receipt) => {
                receipt.recorded_at = self.next_recorded_at()?;
                self.persist_migration(
                    &receipt,
                    DataCustodyRuntimeEventKind::MigrationPlanned,
                    receipt.note.clone(),
                )
                .await?;
                Some(receipt)
            }
            Err(MigrationExecutionError::MigrationNotRequired) => None,
            Err(error) => return Err(RestoreRuntimeError::Migration(error)),
        };
        Ok((plan, migration))
    }
}
