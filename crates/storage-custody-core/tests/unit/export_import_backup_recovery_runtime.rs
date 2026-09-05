use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_job_state::{
        advance_backup_job, BackupJobStateError, BackupJobTransition,
    };

use super::support::StorageCustodyTestValueExt;

#[test]
fn backup_job_transition_round_trips_durable_refs_without_minting_provider_success() {
    let scheduled = sample_backup_job();
    let claimed = advance_backup_job(
        &scheduled,
        BackupJobTransition {
            lifecycle: contracts::ExportImportBackupJobLifecycle::Claimed,
            updated_at: "2026-08-28T19:00:00.000Z".to_owned(),
            execution_ref: None,
            provider_operation_ref: None,
            manual_required_note: None,
        },
    )
    .assume_ok();
    let running = advance_backup_job(
        &claimed,
        BackupJobTransition {
            lifecycle: contracts::ExportImportBackupJobLifecycle::Running,
            updated_at: "2026-08-28T19:01:00.000Z".to_owned(),
            execution_ref: Some("execution-wp05-runtime-round-trip".to_owned()),
            provider_operation_ref: None,
            manual_required_note: None,
        },
    )
    .assume_ok();

    let encoded = serde_json::to_vec(&running).assume_ok();
    let decoded: contracts::ExportImportBackupJobRecord =
        serde_json::from_slice(&encoded).assume_ok();

    assert_eq!(decoded, running);
    assert_eq!(decoded.attempt, 1);
    assert_eq!(
        decoded.lifecycle,
        contracts::ExportImportBackupJobLifecycle::Running
    );
    assert_eq!(
        decoded.execution_ref.as_ref().map(|value| value.as_str()),
        Some("execution-wp05-runtime-round-trip")
    );
    assert!(decoded.provider_operation_ref.is_none());
}

#[test]
fn backup_job_transition_rejects_invalid_timestamp_and_provider_reference_shapes() {
    let scheduled = sample_backup_job();
    assert_eq!(
        advance_backup_job(
            &scheduled,
            BackupJobTransition {
                lifecycle: contracts::ExportImportBackupJobLifecycle::Claimed,
                updated_at: " ".to_owned(),
                execution_ref: None,
                provider_operation_ref: None,
                manual_required_note: None,
            },
        ),
        Err(BackupJobStateError::InvalidTimestamp)
    );

    let claimed = advance_backup_job(
        &scheduled,
        BackupJobTransition {
            lifecycle: contracts::ExportImportBackupJobLifecycle::Claimed,
            updated_at: "2026-08-28T19:10:00.000Z".to_owned(),
            execution_ref: None,
            provider_operation_ref: None,
            manual_required_note: None,
        },
    )
    .assume_ok();
    assert_eq!(
        advance_backup_job(
            &claimed,
            BackupJobTransition {
                lifecycle: contracts::ExportImportBackupJobLifecycle::Running,
                updated_at: "2026-08-28T19:11:00.000Z".to_owned(),
                execution_ref: Some(" ".to_owned()),
                provider_operation_ref: None,
                manual_required_note: None,
            },
        ),
        Err(BackupJobStateError::InvalidExecutionRef)
    );
}

fn sample_backup_job() -> contracts::ExportImportBackupJobRecord {
    let parsed = |value: &str| contracts::ExportImportJobRef::parse(value).assume_ok();
    contracts::ExportImportBackupJobRecord {
        job_ref: parsed("backup-job-wp05-runtime-round-trip"),
        schedule_ref: contracts::ExportImportScheduleRef::parse(
            "backup-schedule-wp05-runtime-round-trip",
        )
        .assume_ok(),
        bundle_id: contracts::ExportImportBundleId::parse("backup-bundle-wp05-runtime-round-trip")
            .assume_ok(),
        household_id: contracts::ExportImportHouseholdId::parse(
            "backup-household-wp05-runtime-round-trip",
        )
        .assume_ok(),
        cadence: contracts::ExportImportBackupCadence::Manual,
        lifecycle: contracts::ExportImportBackupJobLifecycle::Scheduled,
        attempt: 0,
        idempotency_ref: contracts::ExportImportIdempotencyRef::parse(
            "backup-job-wp05-runtime-round-trip:initial",
        )
        .assume_ok(),
        execution_ref: None,
        provider_operation_ref: None,
        created_at: contracts::ExportImportTimestamp::parse("2026-08-28T19:00:00.000Z").assume_ok(),
        updated_at: contracts::ExportImportTimestamp::parse("2026-08-28T19:00:00.000Z").assume_ok(),
        manual_required_note: None,
    }
}
