use std::collections::BTreeMap;

use ocentra_schema::export_import_backup_recovery as contracts;
use super::data_custody_backup_runtime_schedule::{job_for_schedule, BackupRuntimeScheduleError};
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_job_state::BackupJobStateError;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEvent;

#[path = "data_custody_backup_runtime_job_ledger_apply.rs"]
mod apply;
#[path = "data_custody_backup_runtime_job_ledger_event_apply.rs"]
mod event_apply;

#[derive(Debug)]
pub(crate) enum BackupJobLedgerError {
    InvalidInitialJob(BackupRuntimeScheduleError),
    InitialJobMismatch,
    ScheduleConflict,
    ScheduledManualRequired,
    ScheduleJobIdentityMismatch,
    MissingPriorJob,
    Transition(BackupJobStateError),
    TransitionMismatch,
}

#[derive(Debug, Default)]
pub(crate) struct BackupJobLedger {
    schedules: BTreeMap<String, contracts::ExportImportBackupSchedule>,
    jobs: BTreeMap<String, contracts::ExportImportBackupJobRecord>,
}

impl BackupJobLedger {
    pub(crate) fn apply_event(
        &mut self,
        event: &DataCustodyRuntimeEvent,
    ) -> Result<(), BackupJobLedgerError> {
        event_apply::apply_event(self, event)
    }

    pub(crate) fn existing_job_for_schedule(
        &self,
        schedule: &contracts::ExportImportBackupSchedule,
    ) -> Result<Option<contracts::ExportImportBackupJobRecord>, BackupJobLedgerError> {
        let Some(existing_schedule) = self.schedules.get(schedule.schedule_ref.as_str()) else {
            return Ok(None);
        };
        if existing_schedule != schedule {
            return Err(BackupJobLedgerError::ScheduleConflict);
        }
        let expected_job =
            job_for_schedule(schedule).map_err(BackupJobLedgerError::InvalidInitialJob)?;
        let Some(existing_job) = self.jobs.get(expected_job.job_ref.as_str()) else {
            return Ok(None);
        };
        apply::validate_schedule_job_identity(existing_schedule, existing_job)?;
        Ok(Some(existing_job.clone()))
    }

    pub(crate) fn claimable_job(&self) -> Option<contracts::ExportImportBackupJobRecord> {
        self.jobs
            .values()
            .find(|job| {
                job.lifecycle == contracts::ExportImportBackupJobLifecycle::Scheduled
                    && self
                        .schedules
                        .get(job.schedule_ref.as_str())
                        .is_some_and(|schedule| schedule.enabled)
            })
            .cloned()
    }

    pub(crate) fn jobs(&self) -> impl Iterator<Item = &contracts::ExportImportBackupJobRecord> {
        self.jobs.values()
    }

    pub(crate) fn schedules(&self) -> impl Iterator<Item = &contracts::ExportImportBackupSchedule> {
        self.schedules.values()
    }
}
