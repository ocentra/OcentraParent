use ocentra_schema::export_import_backup_recovery as contracts;

use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_job_state::{
        advance_backup_job, BackupJobStateError, BackupJobTransition,
    };
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_schedule::BackupScheduleRequest;
use super::data_custody_backup_runtime::BackupDispatchReservation;
use super::data_custody_backup_runtime_ports::{
    BackupArtifactBinding, ProviderBackupError, ProviderNeutralBackupPort,
    ProviderOperationReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupRuntimeScheduleError {
    InvalidJobRef,
    InvalidIdempotencyRef,
}

#[derive(Debug)]
pub(crate) enum ProviderJobOutcome {
    Succeeded(ProviderOperationReceipt),
    ManualRequired(ProviderBackupError),
}

/// Creates the first durable job record for a storage-owned schedule. The
/// scheduler does not select family authority or a provider; those are
/// supplied through their owning runtime ports when the job is executed.
pub fn job_for_schedule(
    schedule: &contracts::ExportImportBackupSchedule,
) -> Result<contracts::ExportImportBackupJobRecord, BackupRuntimeScheduleError> {
    let job_ref = contracts::ExportImportJobRef::parse(format!(
        "backup-job:{}",
        schedule.schedule_ref.as_str()
    ))
    .ok_or(BackupRuntimeScheduleError::InvalidJobRef)?;
    let idempotency_ref = contracts::ExportImportIdempotencyRef::parse(format!(
        "backup-job:{}:attempt-0",
        job_ref.as_str()
    ))
    .ok_or(BackupRuntimeScheduleError::InvalidIdempotencyRef)?;

    Ok(contracts::ExportImportBackupJobRecord {
        job_ref,
        schedule_ref: schedule.schedule_ref.clone(),
        bundle_id: schedule.bundle_id.clone(),
        household_id: schedule.household_id.clone(),
        cadence: schedule.cadence,
        lifecycle: contracts::ExportImportBackupJobLifecycle::Scheduled,
        attempt: 0,
        idempotency_ref,
        execution_ref: None,
        provider_operation_ref: None,
        created_at: schedule.next_run_at.clone(),
        updated_at: schedule.next_run_at.clone(),
        manual_required_note: None,
    })
}

pub(crate) fn claim_job(
    job: &contracts::ExportImportBackupJobRecord,
    now: String,
) -> Result<contracts::ExportImportBackupJobRecord, BackupJobStateError> {
    advance_backup_job(
        job,
        BackupJobTransition {
            lifecycle: contracts::ExportImportBackupJobLifecycle::Claimed,
            updated_at: now,
            execution_ref: None,
            provider_operation_ref: None,
            manual_required_note: None,
        },
    )
}

pub(crate) fn start_job(
    job: &contracts::ExportImportBackupJobRecord,
    now: String,
) -> Result<contracts::ExportImportBackupJobRecord, BackupJobStateError> {
    let execution_ref = format!("execution:{}:attempt-{}", job.job_ref, job.attempt);
    advance_backup_job(
        job,
        BackupJobTransition {
            lifecycle: contracts::ExportImportBackupJobLifecycle::Running,
            updated_at: now,
            execution_ref: Some(execution_ref),
            provider_operation_ref: None,
            manual_required_note: None,
        },
    )
}

pub(crate) fn execute_provider(
    provider: Option<&dyn ProviderNeutralBackupPort>,
    job: &contracts::ExportImportBackupJobRecord,
    reservation: BackupDispatchReservation,
    artifact: BackupArtifactBinding,
) -> ProviderJobOutcome {
    let Some(provider) = provider else {
        return ProviderJobOutcome::ManualRequired(ProviderBackupError::Unavailable);
    };
    match provider.execute_backup(reservation, artifact, job) {
        Ok(receipt) => ProviderJobOutcome::Succeeded(receipt),
        // The provider contract does not yet expose a trusted status query.
        // Treat every error as outcome-unknown and require reconciliation;
        // retrying could duplicate an already-dispatched backup.
        Err(error) => ProviderJobOutcome::ManualRequired(error),
    }
}

pub(crate) fn manual_required_job(
    job: &contracts::ExportImportBackupJobRecord,
    now: String,
    note: String,
) -> Result<contracts::ExportImportBackupJobRecord, BackupJobStateError> {
    advance_backup_job(
        job,
        BackupJobTransition {
            lifecycle: contracts::ExportImportBackupJobLifecycle::ManualRequired,
            updated_at: now,
            execution_ref: None,
            provider_operation_ref: None,
            manual_required_note: Some(note),
        },
    )
}

pub(crate) fn succeed_job(
    job: &contracts::ExportImportBackupJobRecord,
    now: String,
    receipt: &ProviderOperationReceipt,
) -> Result<contracts::ExportImportBackupJobRecord, BackupJobStateError> {
    if job
        .execution_ref
        .as_ref()
        .is_none_or(|execution_ref| execution_ref != receipt.execution_ref())
    {
        return Err(BackupJobStateError::ExecutionRefRequired);
    }
    advance_backup_job(
        job,
        BackupJobTransition {
            lifecycle: contracts::ExportImportBackupJobLifecycle::Succeeded,
            updated_at: now,
            execution_ref: Some(receipt.execution_ref().as_str().to_owned()),
            provider_operation_ref: Some(receipt.provider_operation_ref().as_str().to_owned()),
            manual_required_note: None,
        },
    )
}

pub fn schedule_request_for(
    input: super::data_custody_backup_runtime::BackupRuntimeScheduleInput,
) -> BackupScheduleRequest {
    BackupScheduleRequest {
        input: input.input,
        schedule_ref: input.schedule_ref,
        next_run_at: input.next_run_at,
        interval_seconds: input.interval_seconds,
    }
}
