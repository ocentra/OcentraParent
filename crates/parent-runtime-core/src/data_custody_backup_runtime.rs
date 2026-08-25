use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_backup_job_state::BackupJobStateError,
    export_import_backup_recovery_backup_schedule::BackupScheduleError, BackupRequestInput,
};

use super::data_custody_backup_runtime_job_ledger::BackupJobLedger;
use super::data_custody_backup_runtime_ports::{AuthorityUnavailable, ProviderBackupError};
use super::data_custody_backup_runtime_reconciliation;
use super::data_custody_parent_runtime_clock::{clock_error, RuntimeClockError};
use super::data_custody_runtime_eventing::{
    DataCustodyRuntimeEventJournal, DataCustodyRuntimeEventKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupRuntimeScheduleInput {
    pub input: BackupRequestInput,
    pub schedule_ref: String,
    pub next_run_at: String,
    pub interval_seconds: Option<u64>,
}

#[derive(Debug)]
pub(crate) struct BackupDispatchReservation {
    execution_ref: contracts::ExportImportExecutionRef,
    bundle_id: contracts::ExportImportBundleId,
}

impl BackupDispatchReservation {
    pub(crate) fn new(
        execution_ref: contracts::ExportImportExecutionRef,
        bundle_id: contracts::ExportImportBundleId,
    ) -> Self {
        Self {
            execution_ref,
            bundle_id,
        }
    }

    pub(crate) fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub(crate) fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }
}

#[derive(Debug)]
pub enum BackupRuntimeError {
    Eventing(EventingError),
    AuthorityUnavailable(AuthorityUnavailable),
    Schedule(BackupScheduleError),
    JobState(BackupJobStateError),
    ScheduleJob,
    ReplayDecode(EventingError),
    Provider(ProviderBackupError),
    ReplaySkipped(usize),
    DispatchReservation,
    RuntimeNotRecovered,
}

impl From<EventingError> for BackupRuntimeError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}

impl From<BackupJobStateError> for BackupRuntimeError {
    fn from(error: BackupJobStateError) -> Self {
        Self::JobState(error)
    }
}

impl From<BackupScheduleError> for BackupRuntimeError {
    fn from(error: BackupScheduleError) -> Self {
        Self::Schedule(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupExecutionResult {
    Succeeded(contracts::ExportImportBackupJobRecord),
    Retryable(contracts::ExportImportBackupJobRecord),
    ManualRequired(contracts::ExportImportBackupJobRecord),
}

/// Parent-runtime owner for durable backup scheduling, job persistence,
/// restart reconciliation, and provider-neutral executor mounting.
pub struct ParentBackupRuntime {
    pub(crate) journal: DataCustodyRuntimeEventJournal,
    pub(crate) ledger: BackupJobLedger,
    pub(crate) dispatch_reservations: BTreeSet<String>,
    pub(crate) recovered: bool,
}

impl ParentBackupRuntime {
    pub fn new(journal: DataCustodyRuntimeEventJournal) -> Self {
        Self {
            journal,
            ledger: BackupJobLedger::default(),
            dispatch_reservations: BTreeSet::new(),
            recovered: false,
        }
    }

    pub(crate) async fn reconcile_after_restart(&mut self) -> Result<usize, BackupRuntimeError> {
        if !self.recovered {
            return Err(BackupRuntimeError::RuntimeNotRecovered);
        }
        let now = self
            .journal
            .next_recorded_at()
            .map_err(BackupRuntimeError::from)?;
        let reconciled = data_custody_backup_runtime_reconciliation::reconcile_after_restart(
            &self.ledger,
            &now,
        )?;
        for job in &reconciled {
            self.persist_job(
                job,
                DataCustodyRuntimeEventKind::Reconciliation,
                job.manual_required_note.clone(),
            )
            .await?;
        }
        Ok(reconciled.len())
    }

    pub fn jobs(&self) -> impl Iterator<Item = &contracts::ExportImportBackupJobRecord> {
        self.ledger.jobs()
    }

    pub fn schedules(&self) -> impl Iterator<Item = &contracts::ExportImportBackupSchedule> {
        self.ledger.schedules()
    }
}

impl From<AuthorityUnavailable> for BackupRuntimeError {
    fn from(error: AuthorityUnavailable) -> Self {
        Self::AuthorityUnavailable(error)
    }
}

impl From<ProviderBackupError> for BackupRuntimeError {
    fn from(error: ProviderBackupError) -> Self {
        Self::Provider(error)
    }
}

impl From<RuntimeClockError> for BackupRuntimeError {
    fn from(error: RuntimeClockError) -> Self {
        Self::Eventing(clock_error(error))
    }
}
