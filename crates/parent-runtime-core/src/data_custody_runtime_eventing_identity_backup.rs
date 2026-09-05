use super::data_custody_runtime_eventing::DataCustodyRuntimeRecord;

pub(crate) fn backup_job_event_idempotency_ref(
    job: &ocentra_schema::export_import_backup_recovery::ExportImportBackupJobRecord,
) -> String {
    format!(
        "backup-job:{}:attempt-{}:{}",
        job.job_ref,
        job.attempt,
        match job.lifecycle {
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Scheduled => "scheduled",
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Claimed => "claimed",
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Running => "running",
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Succeeded => "succeeded",
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Retryable => "retryable",
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Failed => "failed",
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::ManualRequired => "manual-required",
            ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Reconciled => "reconciled",
        }
    )
}

pub(crate) fn schedule_job_identity_is_initial(record: &DataCustodyRuntimeRecord) -> bool {
    let DataCustodyRuntimeRecord::ScheduleAndJob { schedule, job } = record else {
        return true;
    };
    let expected_job_ref = format!("backup-job:{}", schedule.schedule_ref);
    let expected_idempotency_ref = format!("backup-job:{}:attempt-0", expected_job_ref);
    job.job_ref.as_str() == expected_job_ref
        && job.schedule_ref == schedule.schedule_ref
        && job.bundle_id == schedule.bundle_id
        && job.household_id == schedule.household_id
        && job.cadence == schedule.cadence
        && job.lifecycle
            == ocentra_schema::export_import_backup_recovery::ExportImportBackupJobLifecycle::Scheduled
        && job.attempt == 0
        && job.idempotency_ref.as_str() == expected_idempotency_ref
        && job.execution_ref.is_none()
        && job.provider_operation_ref.is_none()
        && job.created_at == schedule.next_run_at
        && job.updated_at == schedule.next_run_at
        && job.manual_required_note.is_none()
}
