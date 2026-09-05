use std::collections::BTreeSet;

use ocentra_eventing::{error::EventingError, journal::policy::JournalDispatchPhase};
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_bundle_preflight_binding::custody_port::ImportBindingError,
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::DispatchReservationError,
    export_import_backup_recovery_migration_execution::MigrationExecutionError,
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlanError,
};

use super::data_custody_parent_runtime_clock::{clock_error, RuntimeClockError};
use super::data_custody_restore_runtime_executor::{
    RestoreAuthorityUnavailable, RestoreExecutorError, RestoreExecutorOperationError,
};
use super::data_custody_restore_runtime_ledger::{RestoreLedger, RestoreLedgerError};
use super::data_custody_restore_runtime_rollback::RestoreRollbackError;
use super::data_custody_restore_runtime_stage::stage_for_kind;
use super::data_custody_runtime_eventing::{
    DataCustodyRuntimeEvent, DataCustodyRuntimeEventJournal, DataCustodyRuntimeEventKind,
};

#[derive(Debug)]
pub enum RestoreRuntimeError {
    Eventing(EventingError),
    Authority(RestoreAuthorityUnavailable),
    Binding(ImportBindingError),
    Plan(RestoreExecutionPlanError),
    Migration(MigrationExecutionError),
    Executor(RestoreExecutorError),
    ExecutorOperation(RestoreExecutorOperationError),
    Reservation(DispatchReservationError),
    Rollback(RestoreRollbackError),
    ReplayDecode(EventingError),
    ReplaySkipped(usize),
    Ledger(RestoreLedgerFailure),
    PlanNotDurablyPending,
    RestartReconciliationRequired,
    RuntimeNotRecovered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreLedgerFailure {
    reason: String,
}

impl RestoreLedgerFailure {
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl std::fmt::Display for RestoreLedgerFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.reason)
    }
}

impl std::error::Error for RestoreLedgerFailure {}

impl From<EventingError> for RestoreRuntimeError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}

impl From<ImportBindingError> for RestoreRuntimeError {
    fn from(error: ImportBindingError) -> Self {
        Self::Binding(error)
    }
}

impl From<RestoreExecutionPlanError> for RestoreRuntimeError {
    fn from(error: RestoreExecutionPlanError) -> Self {
        Self::Plan(error)
    }
}

impl From<MigrationExecutionError> for RestoreRuntimeError {
    fn from(error: MigrationExecutionError) -> Self {
        Self::Migration(error)
    }
}

impl From<RestoreExecutorError> for RestoreRuntimeError {
    fn from(error: RestoreExecutorError) -> Self {
        Self::Executor(error)
    }
}

impl From<RestoreExecutorOperationError> for RestoreRuntimeError {
    fn from(error: RestoreExecutorOperationError) -> Self {
        Self::ExecutorOperation(error)
    }
}

impl From<RestoreRollbackError> for RestoreRuntimeError {
    fn from(error: RestoreRollbackError) -> Self {
        Self::Rollback(error)
    }
}

impl From<RestoreLedgerError> for RestoreRuntimeError {
    fn from(error: RestoreLedgerError) -> Self {
        Self::Ledger(RestoreLedgerFailure {
            reason: format!("{error:?}"),
        })
    }
}

impl From<RuntimeClockError> for RestoreRuntimeError {
    fn from(error: RuntimeClockError) -> Self {
        Self::Eventing(clock_error(&error))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreRuntimeReceipts {
    pub restore: contracts::ExportImportRestoreReceipt,
    pub migration: Option<contracts::ExportImportMigrationReceipt>,
}

/// Parent-runtime owner for restore/migration planning, durable ledgers,
/// restart reconciliation, executor mounting, rollback, and outbox events.
pub struct ParentRestoreRuntime {
    pub(crate) journal: DataCustodyRuntimeEventJournal,
    pub(crate) ledger: RestoreLedger,
    pub(crate) restart_pending_restore: BTreeSet<String>,
    pub(crate) restart_pending_migration: BTreeSet<String>,
    pub(crate) restart_pending_rollback: BTreeSet<String>,
    pub(crate) dispatch_started_restore: BTreeSet<String>,
    pub(crate) dispatch_started_migration: BTreeSet<String>,
    pub(crate) dispatch_started_rollback: BTreeSet<String>,
    pub(crate) recovered: bool,
}

impl ParentRestoreRuntime {
    pub fn new(journal: DataCustodyRuntimeEventJournal) -> Self {
        Self {
            journal,
            ledger: RestoreLedger::default(),
            restart_pending_restore: BTreeSet::new(),
            restart_pending_migration: BTreeSet::new(),
            restart_pending_rollback: BTreeSet::new(),
            dispatch_started_restore: BTreeSet::new(),
            dispatch_started_migration: BTreeSet::new(),
            dispatch_started_rollback: BTreeSet::new(),
            recovered: false,
        }
    }

    pub fn pending_operation_count(&self) -> usize {
        super::data_custody_restore_runtime_reconciliation::pending_operation_count(&self.ledger)
    }

    pub fn restore_receipt(
        &self,
        operation_ref: &contracts::ExportImportOperationRef,
    ) -> Option<&contracts::ExportImportRestoreReceipt> {
        self.ledger.restore_receipt(operation_ref)
    }

    pub(crate) async fn persist_restore(
        &mut self,
        receipt: &contracts::ExportImportRestoreReceipt,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
    ) -> Result<(), RestoreRuntimeError> {
        self.persist_restore_phase(receipt, kind, note, JournalDispatchPhase::AfterDispatch)
            .await
    }

    pub(crate) async fn persist_restore_phase(
        &mut self,
        receipt: &contracts::ExportImportRestoreReceipt,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
        phase: JournalDispatchPhase,
    ) -> Result<(), RestoreRuntimeError> {
        self.journal
            .append_record(
                DataCustodyRuntimeEvent::restore_receipt(receipt.clone(), kind.clone(), note),
                phase,
            )
            .await?;
        self.ledger.insert_restore_receipt(receipt.clone())?;
        self.ledger
            .set_restore_stage(&receipt.operation_ref, stage_for_kind(&kind));
        Ok(())
    }

    pub(crate) async fn persist_migration(
        &mut self,
        receipt: &contracts::ExportImportMigrationReceipt,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
    ) -> Result<(), RestoreRuntimeError> {
        self.persist_migration_phase(receipt, kind, note, JournalDispatchPhase::AfterDispatch)
            .await
    }

    pub(crate) async fn persist_migration_phase(
        &mut self,
        receipt: &contracts::ExportImportMigrationReceipt,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
        phase: JournalDispatchPhase,
    ) -> Result<(), RestoreRuntimeError> {
        self.journal
            .append_record(
                DataCustodyRuntimeEvent::migration_receipt(receipt.clone(), kind.clone(), note),
                phase,
            )
            .await?;
        self.ledger.insert_migration_receipt(receipt.clone())?;
        self.ledger
            .set_migration_stage(&receipt.operation_ref, stage_for_kind(&kind));
        Ok(())
    }
}
