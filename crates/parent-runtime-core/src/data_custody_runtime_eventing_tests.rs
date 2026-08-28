use super::{
    DataCustodyRuntimeEvent, DataCustodyRuntimeEventJournal, DataCustodyRuntimeEventKind,
    DataCustodyRuntimeRecord,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_schema::export_import_backup_recovery as contracts;

macro_rules! parsed {
    ($type:ty, $value:expr) => {
        <$type>::parse($value).expect("typed data-custody reference")
    };
}

#[tokio::test]
async fn data_custody_journal_appends_replays_decodes_and_reopens_the_same_typed_event(
) -> Result<(), EventingError> {
    let path = journal_path("append-restart");
    let journal = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-eventing-first-{}", std::process::id()),
    )?;
    journal.recover().await?;

    let recorded_at = journal
        .next_recorded_at()
        .expect("recovered journal issues a monotonic timestamp");
    let event = DataCustodyRuntimeEvent::backup_job(
        sample_backup_job(&recorded_at),
        DataCustodyRuntimeEventKind::BackupJobTransition,
        Some("typed WP05 journal event".to_owned()),
    );
    let first_append = journal
        .append_record(event.clone(), JournalDispatchPhase::BeforeDispatch)
        .await?;
    let duplicate_append = journal
        .append_record(event.clone(), JournalDispatchPhase::BeforeDispatch)
        .await?;

    assert_eq!(duplicate_append.sequence, first_append.sequence);
    assert_eq!(duplicate_append.current_hash, first_append.current_hash);

    let report = journal.replay().await?;
    assert_eq!(report.skipped_count, 0);
    assert_eq!(report.records.len(), 1);
    let decoded = DataCustodyRuntimeEventJournal::decode(&report.records[0].envelope)?;
    assert_eq!(decoded, event);
    assert!(matches!(
        &decoded.record,
        DataCustodyRuntimeRecord::BackupJob(job)
            if job.lifecycle == contracts::ExportImportBackupJobLifecycle::Scheduled
    ));
    drop(journal);

    let reopened = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-eventing-restart-{}", std::process::id()),
    )?;
    reopened.recover().await?;
    let restart_report = reopened.replay().await?;
    assert_eq!(restart_report.skipped_count, 0);
    assert_eq!(restart_report.records.len(), 1);
    let restarted_event =
        DataCustodyRuntimeEventJournal::decode(&restart_report.records[0].envelope)?;
    assert_eq!(restarted_event, event);

    let _ = std::fs::remove_file(path);
    Ok(())
}

#[tokio::test]
async fn data_custody_journal_rejects_a_malformed_durable_row_during_recovery() {
    let path = journal_path("malformed");
    std::fs::write(&path, b"{malformed durable row}\n").expect("malformed journal fixture writes");
    let journal = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-eventing-malformed-{}", std::process::id()),
    )
    .expect("data-custody journal identity is valid");

    assert!(matches!(
        journal.recover().await,
        Err(EventingError::JournalCorruptLine { line: 1, .. })
    ));
    let _ = std::fs::remove_file(path);
}

fn sample_backup_job(recorded_at: &str) -> contracts::ExportImportBackupJobRecord {
    contracts::ExportImportBackupJobRecord {
        job_ref: parsed!(contracts::ExportImportJobRef, "journal-job-wp05"),
        schedule_ref: parsed!(contracts::ExportImportScheduleRef, "journal-schedule-wp05"),
        bundle_id: parsed!(contracts::ExportImportBundleId, "journal-bundle-wp05"),
        household_id: parsed!(contracts::ExportImportHouseholdId, "journal-household-wp05"),
        cadence: contracts::ExportImportBackupCadence::Manual,
        lifecycle: contracts::ExportImportBackupJobLifecycle::Scheduled,
        attempt: 0,
        idempotency_ref: parsed!(
            contracts::ExportImportIdempotencyRef,
            "journal-job-wp05:initial"
        ),
        execution_ref: None,
        provider_operation_ref: None,
        created_at: parsed!(contracts::ExportImportTimestamp, recorded_at),
        updated_at: parsed!(contracts::ExportImportTimestamp, recorded_at),
        manual_required_note: None,
    }
}

fn journal_path(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-data-custody-wp05-eventing-{label}-{}.ndjson",
        std::process::id()
    ))
}
