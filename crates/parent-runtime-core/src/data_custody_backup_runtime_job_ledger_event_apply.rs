use super::super::data_custody_backup_runtime_schedule::job_for_schedule;
use super::super::data_custody_runtime_eventing::{
    DataCustodyRuntimeEvent, DataCustodyRuntimeRecord,
};
use super::apply;
use super::{BackupJobLedger, BackupJobLedgerError};

pub(super) fn apply_event(
    ledger: &mut BackupJobLedger,
    event: &DataCustodyRuntimeEvent,
) -> Result<(), BackupJobLedgerError> {
    match &event.record {
        DataCustodyRuntimeRecord::Schedule(schedule) => {
            let expected_job =
                job_for_schedule(schedule).map_err(BackupJobLedgerError::InvalidInitialJob)?;
            // Legacy schedule-only records are rehydrated into the same stable
            // initial job identity. New writes use ScheduleAndJob atomically,
            // but replay must never leave a durable schedule without work.
            apply::apply_schedule(ledger, schedule, &expected_job, true)?;
        }
        DataCustodyRuntimeRecord::ScheduleAndJob { schedule, job } => {
            let expected_job =
                job_for_schedule(schedule).map_err(BackupJobLedgerError::InvalidInitialJob)?;
            if job != &expected_job {
                return Err(BackupJobLedgerError::InitialJobMismatch);
            }
            apply::apply_schedule(ledger, schedule, job, true)?;
        }
        DataCustodyRuntimeRecord::BackupJob(job) => apply::apply_backup_job(ledger, job)?,
        DataCustodyRuntimeRecord::MigrationReceipt(_)
        | DataCustodyRuntimeRecord::RestoreReceipt(_) => {}
    }
    Ok(())
}
