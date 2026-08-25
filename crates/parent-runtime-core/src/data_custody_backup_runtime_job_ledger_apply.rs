use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_job_state::{
        advance_backup_job, BackupJobStateError, BackupJobTransition,
    };

use super::{BackupJobLedger, BackupJobLedgerError};

pub(super) fn apply_schedule(
    ledger: &mut BackupJobLedger,
    schedule: &contracts::ExportImportBackupSchedule,
    job: &contracts::ExportImportBackupJobRecord,
    require_exact_job: bool,
) -> Result<(), BackupJobLedgerError> {
    if schedule.cadence == contracts::ExportImportBackupCadence::Scheduled || schedule.enabled {
        return Err(BackupJobLedgerError::ScheduledManualRequired);
    }
    if let Some(existing_schedule) = ledger.schedules.get(schedule.schedule_ref.as_str()) {
        if existing_schedule != schedule {
            return Err(BackupJobLedgerError::ScheduleConflict);
        }
    }
    if let Some(existing_job) = ledger.jobs.get(job.job_ref.as_str()) {
        if existing_job != job {
            if require_exact_job {
                return Err(BackupJobLedgerError::InitialJobMismatch);
            }
            validate_schedule_job_identity(schedule, existing_job)?;
            return Ok(());
        }
    }
    validate_schedule_job_identity(schedule, job)?;
    ledger
        .schedules
        .insert(schedule.schedule_ref.as_str().to_owned(), schedule.clone());
    ledger
        .jobs
        .entry(job.job_ref.as_str().to_owned())
        .or_insert_with(|| job.clone());
    Ok(())
}

pub(super) fn apply_backup_job(
    ledger: &mut BackupJobLedger,
    job: &contracts::ExportImportBackupJobRecord,
) -> Result<(), BackupJobLedgerError> {
    let Some(previous) = ledger.jobs.get(job.job_ref.as_str()).cloned() else {
        return Err(BackupJobLedgerError::MissingPriorJob);
    };
    if previous == *job {
        return Ok(());
    }
    if previous.lifecycle == job.lifecycle
        && previous.attempt == job.attempt
        && previous.idempotency_ref == job.idempotency_ref
    {
        return Err(BackupJobLedgerError::TransitionMismatch);
    }
    if previous.schedule_ref != job.schedule_ref
        || previous.bundle_id != job.bundle_id
        || previous.household_id != job.household_id
        || previous.cadence != job.cadence
        || previous.created_at != job.created_at
    {
        return Err(BackupJobLedgerError::TransitionMismatch);
    }
    let expected = advance_backup_job(
        &previous,
        BackupJobTransition {
            lifecycle: job.lifecycle,
            updated_at: job.updated_at.as_str().to_owned(),
            execution_ref: job
                .execution_ref
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            provider_operation_ref: job
                .provider_operation_ref
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            manual_required_note: job.manual_required_note.clone(),
        },
    )
    .map_err(BackupJobLedgerError::Transition)?;
    if expected != *job {
        return Err(BackupJobLedgerError::TransitionMismatch);
    }
    ledger
        .jobs
        .insert(job.job_ref.as_str().to_owned(), job.clone());
    Ok(())
}

pub(super) fn validate_schedule_job_identity(
    schedule: &contracts::ExportImportBackupSchedule,
    job: &contracts::ExportImportBackupJobRecord,
) -> Result<(), BackupJobLedgerError> {
    if job.schedule_ref != schedule.schedule_ref
        || job.bundle_id != schedule.bundle_id
        || job.household_id != schedule.household_id
        || job.cadence != schedule.cadence
    {
        return Err(BackupJobLedgerError::ScheduleJobIdentityMismatch);
    }
    Ok(())
}
