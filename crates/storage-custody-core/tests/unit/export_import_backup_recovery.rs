use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_job_state::{
        advance_backup_job, BackupJobStateError, BackupJobTransition,
    };

macro_rules! parsed {
    ($type:ty, $value:expr) => {
        <$type>::parse($value).expect("typed export/import reference")
    };
}

#[test]
fn export_import_backup_recovery_backup_job_lifecycle_preserves_typed_execution_and_attempts() {
    let scheduled = sample_backup_job();

    let claimed = advance_backup_job(
        &scheduled,
        transition(
            contracts::ExportImportBackupJobLifecycle::Claimed,
            "2026-08-28T18:31:00.000Z",
            None,
            None,
            None,
        ),
    )
    .expect("scheduled job can be claimed");
    assert_eq!(
        claimed.lifecycle,
        contracts::ExportImportBackupJobLifecycle::Claimed
    );
    assert_eq!(claimed.attempt, 1);
    assert_eq!(
        claimed.idempotency_ref.as_str(),
        "backup-job-wp05-runtime:attempt-1"
    );

    let running = advance_backup_job(
        &claimed,
        transition(
            contracts::ExportImportBackupJobLifecycle::Running,
            "2026-08-28T18:32:00.000Z",
            Some("execution-wp05-runtime-1"),
            None,
            None,
        ),
    )
    .expect("claimed job can enter running state with execution reference");
    assert_eq!(
        running.execution_ref.as_ref().map(|value| value.as_str()),
        Some("execution-wp05-runtime-1")
    );

    let retryable = advance_backup_job(
        &running,
        transition(
            contracts::ExportImportBackupJobLifecycle::Retryable,
            "2026-08-28T18:33:00.000Z",
            None,
            None,
            None,
        ),
    )
    .expect("running job can be retried without replacing its execution reference");
    assert_eq!(
        retryable.lifecycle,
        contracts::ExportImportBackupJobLifecycle::Retryable
    );
    assert_eq!(retryable.attempt, 1);
    assert_eq!(retryable.execution_ref, running.execution_ref);

    let reclaimed = advance_backup_job(
        &retryable,
        transition(
            contracts::ExportImportBackupJobLifecycle::Claimed,
            "2026-08-28T18:34:00.000Z",
            None,
            None,
            None,
        ),
    )
    .expect("retryable job can be claimed again");
    assert_eq!(reclaimed.attempt, 2);
    assert_eq!(
        reclaimed.idempotency_ref.as_str(),
        "backup-job-wp05-runtime:attempt-2"
    );
    assert_eq!(reclaimed.execution_ref, retryable.execution_ref);

    let succeeded = advance_backup_job(
        &reclaimed,
        transition(
            contracts::ExportImportBackupJobLifecycle::Running,
            "2026-08-28T18:35:00.000Z",
            None,
            None,
            None,
        ),
    )
    .expect("reclaimed job can resume running");
    let succeeded = advance_backup_job(
        &succeeded,
        transition(
            contracts::ExportImportBackupJobLifecycle::Succeeded,
            "2026-08-28T18:36:00.000Z",
            None,
            Some("provider-operation-wp05-runtime-1"),
            None,
        ),
    )
    .expect("running job can succeed only with provider operation reference");
    assert_eq!(
        succeeded.lifecycle,
        contracts::ExportImportBackupJobLifecycle::Succeeded
    );
    assert_eq!(
        succeeded
            .provider_operation_ref
            .as_ref()
            .map(|value| value.as_str()),
        Some("provider-operation-wp05-runtime-1")
    );
}

#[test]
fn export_import_backup_recovery_backup_job_rejects_unbound_execution_and_provider_success() {
    let scheduled = sample_backup_job();

    assert!(matches!(
        advance_backup_job(
            &scheduled,
            transition(
                contracts::ExportImportBackupJobLifecycle::Running,
                "2026-08-28T18:40:00.000Z",
                None,
                None,
                None,
            ),
        ),
        Err(BackupJobStateError::InvalidTransition { .. })
    ));

    let claimed = advance_backup_job(
        &scheduled,
        transition(
            contracts::ExportImportBackupJobLifecycle::Claimed,
            "2026-08-28T18:41:00.000Z",
            None,
            None,
            None,
        ),
    )
    .expect("scheduled job can be claimed");
    assert_eq!(
        advance_backup_job(
            &claimed,
            transition(
                contracts::ExportImportBackupJobLifecycle::Running,
                "2026-08-28T18:42:00.000Z",
                None,
                None,
                None,
            ),
        ),
        Err(BackupJobStateError::ExecutionRefRequired)
    );

    let running = advance_backup_job(
        &claimed,
        transition(
            contracts::ExportImportBackupJobLifecycle::Running,
            "2026-08-28T18:43:00.000Z",
            Some("execution-wp05-runtime-2"),
            None,
            None,
        ),
    )
    .expect("execution reference binds the running state");
    assert_eq!(
        advance_backup_job(
            &running,
            transition(
                contracts::ExportImportBackupJobLifecycle::Succeeded,
                "2026-08-28T18:44:00.000Z",
                None,
                None,
                None,
            ),
        ),
        Err(BackupJobStateError::ProviderOperationRefRequired)
    );
}

#[test]
fn export_import_backup_recovery_backup_job_manual_required_requires_a_reason() {
    let scheduled = sample_backup_job();

    assert_eq!(
        advance_backup_job(
            &scheduled,
            transition(
                contracts::ExportImportBackupJobLifecycle::ManualRequired,
                "2026-08-28T18:50:00.000Z",
                None,
                None,
                None,
            ),
        ),
        Err(BackupJobStateError::ManualRequiredNoteMissing)
    );

    let manual = advance_backup_job(
        &scheduled,
        transition(
            contracts::ExportImportBackupJobLifecycle::ManualRequired,
            "2026-08-28T18:51:00.000Z",
            None,
            None,
            Some("provider runtime is not mounted by the owning service"),
        ),
    )
    .expect("manual-required transition carries an explicit reason");
    assert_eq!(
        manual.lifecycle,
        contracts::ExportImportBackupJobLifecycle::ManualRequired
    );
    assert_eq!(
        manual.manual_required_note.as_deref(),
        Some("provider runtime is not mounted by the owning service")
    );
}

fn sample_backup_job() -> contracts::ExportImportBackupJobRecord {
    contracts::ExportImportBackupJobRecord {
        job_ref: parsed!(contracts::ExportImportJobRef, "backup-job-wp05-runtime"),
        schedule_ref: parsed!(
            contracts::ExportImportScheduleRef,
            "backup-schedule-wp05-runtime"
        ),
        bundle_id: parsed!(
            contracts::ExportImportBundleId,
            "backup-bundle-wp05-runtime"
        ),
        household_id: parsed!(
            contracts::ExportImportHouseholdId,
            "backup-household-wp05-runtime"
        ),
        cadence: contracts::ExportImportBackupCadence::Manual,
        lifecycle: contracts::ExportImportBackupJobLifecycle::Scheduled,
        attempt: 0,
        idempotency_ref: parsed!(
            contracts::ExportImportIdempotencyRef,
            "backup-job-wp05-runtime:initial"
        ),
        execution_ref: None,
        provider_operation_ref: None,
        created_at: parsed!(contracts::ExportImportTimestamp, "2026-08-28T18:30:00.000Z"),
        updated_at: parsed!(contracts::ExportImportTimestamp, "2026-08-28T18:30:00.000Z"),
        manual_required_note: None,
    }
}

fn transition(
    lifecycle: contracts::ExportImportBackupJobLifecycle,
    updated_at: &str,
    execution_ref: Option<&str>,
    provider_operation_ref: Option<&str>,
    manual_required_note: Option<&str>,
) -> BackupJobTransition {
    BackupJobTransition {
        lifecycle,
        updated_at: updated_at.to_owned(),
        execution_ref: execution_ref.map(str::to_owned),
        provider_operation_ref: provider_operation_ref.map(str::to_owned),
        manual_required_note: manual_required_note.map(str::to_owned),
    }
}
