use std::collections::BTreeSet;

use chrono::Utc;

use ocentra_eventing::{error::EventingError, journal::policy::JournalDispatchPhase};
use ocentra_family_identity_core::household_authority_proof::CurrentVerifiedHouseholdAuthority;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_backup_job_state::BackupJobStateError,
    export_import_backup_recovery_backup_schedule::BackupScheduleError, BackupRequestInput,
};

use super::data_custody_backup_runtime_job_ledger::BackupJobLedger;
use super::data_custody_backup_runtime_reconciliation;
use super::data_custody_backup_runtime_schedule::{job_for_schedule, schedule_request_for};
use super::data_custody_runtime_eventing::{
    DataCustodyRuntimeEvent, DataCustodyRuntimeEventJournal, DataCustodyRuntimeEventKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupRuntimeScheduleInput {
    pub input: BackupRequestInput,
    pub schedule_ref: String,
    pub next_run_at: String,
    pub interval_seconds: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityUnavailable {
    Unavailable,
}

pub(crate) trait AccountCustodyAuthorityPort: Send + Sync {
    fn current_household_authority(
        &self,
        household_id: &contracts::ExportImportHouseholdId,
    ) -> Result<CurrentVerifiedHouseholdAuthority, AuthorityUnavailable>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderBackupError {
    Unavailable,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderOperationReceipt {
    execution_ref: contracts::ExportImportExecutionRef,
    provider_operation_ref: contracts::ExportImportProviderOperationRef,
}

impl ProviderOperationReceipt {
    pub(crate) fn new(
        reservation: &BackupDispatchReservation,
        provider_operation_ref: impl Into<String>,
    ) -> Option<Self> {
        Some(Self {
            execution_ref: reservation.execution_ref.clone(),
            provider_operation_ref: contracts::ExportImportProviderOperationRef::parse(
                provider_operation_ref,
            )?,
        })
    }

    pub(crate) fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub(crate) fn provider_operation_ref(&self) -> &contracts::ExportImportProviderOperationRef {
        &self.provider_operation_ref
    }
}

/// Provider-neutral port. No SDK, OAuth, filesystem, or credential adapter is
/// implemented in this crate; a parent product runtime must mount one here.
pub(crate) trait ProviderNeutralBackupPort: Send + Sync {
    fn execute_backup(
        &self,
        reservation: BackupDispatchReservation,
        job: &contracts::ExportImportBackupJobRecord,
    ) -> Result<ProviderOperationReceipt, ProviderBackupError>;
}

#[derive(Debug)]
pub(crate) struct BackupDispatchReservation {
    execution_ref: contracts::ExportImportExecutionRef,
}

impl BackupDispatchReservation {
    pub(crate) fn new(execution_ref: contracts::ExportImportExecutionRef) -> Self {
        Self { execution_ref }
    }

    pub(crate) fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
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
}

impl ParentBackupRuntime {
    pub fn new(journal: DataCustodyRuntimeEventJournal) -> Self {
        Self {
            journal,
            ledger: BackupJobLedger::default(),
            dispatch_reservations: BTreeSet::new(),
        }
    }

    pub async fn recover(&mut self) -> Result<(), BackupRuntimeError> {
        self.ledger = BackupJobLedger::default();
        self.dispatch_reservations.clear();
        self.journal.recover().await?;
        let report = self.journal.replay().await?;
        if report.skipped_count != 0 {
            return Err(BackupRuntimeError::ReplaySkipped(report.skipped_count));
        }
        for record in report.records {
            let event = DataCustodyRuntimeEventJournal::decode(&record.envelope)
                .map_err(BackupRuntimeError::ReplayDecode)?;
            self.ledger
                .apply_event(&event)
                .map_err(|_| BackupRuntimeError::ScheduleJob)?;
        }
        Ok(())
    }

    pub(crate) async fn schedule_backup(
        &mut self,
        input: BackupRuntimeScheduleInput,
        authority_port: &dyn AccountCustodyAuthorityPort,
    ) -> Result<contracts::ExportImportBackupJobRecord, BackupRuntimeError> {
        let authority = authority_port
            .current_household_authority(&input.input.household_id)
            .map_err(BackupRuntimeError::AuthorityUnavailable)?;
        let schedule = ocentra_storage_custody_core::export_import_backup_recovery::
            export_import_backup_recovery_backup_schedule::derive_backup_schedule(
                schedule_request_for(input),
                authority,
            )?;
        let job = job_for_schedule(&schedule).map_err(|_| BackupRuntimeError::ScheduleJob)?;
        if let Some(existing_job) = self
            .ledger
            .existing_job_for_schedule(&schedule)
            .map_err(|_| BackupRuntimeError::ScheduleJob)?
        {
            return Ok(existing_job);
        }

        let event = DataCustodyRuntimeEvent::schedule_and_job(
            schedule,
            job.clone(),
            format!("schedule:{}:initial-job", job.schedule_ref),
        );
        self.journal
            .append_record(event.clone(), JournalDispatchPhase::BeforeDispatch)
            .await?;
        self.ledger
            .apply_event(&event)
            .map_err(|_| BackupRuntimeError::ScheduleJob)?;
        Ok(job)
    }

    pub(crate) async fn reconcile_after_restart(&mut self) -> Result<usize, BackupRuntimeError> {
        let now = Utc::now().to_rfc3339();
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
