use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_schedule::derive_backup_schedule;

use super::data_custody_backup_runtime::{
    BackupRuntimeError, BackupRuntimeScheduleInput, ParentBackupRuntime,
};
use super::data_custody_backup_runtime_job_ledger::BackupJobLedger;
use super::data_custody_backup_runtime_ports::AccountCustodyAuthorityPort;
use super::data_custody_backup_runtime_schedule::{job_for_schedule, schedule_request_for};
use super::data_custody_runtime_eventing::{
    DataCustodyRuntimeEvent, DataCustodyRuntimeEventJournal,
};

impl ParentBackupRuntime {
    pub async fn recover(&mut self) -> Result<(), BackupRuntimeError> {
        self.recovered = false;
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
                .map_err(BackupRuntimeError::schedule_job)?;
        }
        self.recovered = true;
        Ok(())
    }

    pub(crate) async fn schedule_backup(
        &mut self,
        input: BackupRuntimeScheduleInput,
        authority_port: &dyn AccountCustodyAuthorityPort,
    ) -> Result<contracts::ExportImportBackupJobRecord, BackupRuntimeError> {
        if !self.recovered {
            return Err(BackupRuntimeError::RuntimeNotRecovered);
        }
        let authority = authority_port
            .current_household_authority(&input.input.household_id)
            .map_err(BackupRuntimeError::AuthorityUnavailable)?;
        let schedule = derive_backup_schedule(schedule_request_for(input), authority)?;
        let job = job_for_schedule(&schedule).map_err(BackupRuntimeError::schedule_job)?;
        if let Some(existing_job) = self
            .ledger
            .existing_job_for_schedule(&schedule)
            .map_err(BackupRuntimeError::schedule_job)?
        {
            return Ok(existing_job);
        }

        let event = DataCustodyRuntimeEvent::schedule_and_job(
            schedule,
            job.clone(),
            format!("schedule:{}:initial-job", job.schedule_ref),
            self.journal
                .next_recorded_at()
                .map_err(BackupRuntimeError::from)?,
        );
        self.journal
            .append_record(event.clone(), JournalDispatchPhase::BeforeDispatch)
            .await?;
        self.ledger
            .apply_event(&event)
            .map_err(BackupRuntimeError::schedule_job)?;
        Ok(job)
    }
}
