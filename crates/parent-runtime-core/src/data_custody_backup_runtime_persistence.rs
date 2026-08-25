use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_backup_runtime::{
    BackupDispatchReservation, BackupRuntimeError, ParentBackupRuntime,
};
use super::data_custody_runtime_eventing::{DataCustodyRuntimeEvent, DataCustodyRuntimeEventKind};

impl ParentBackupRuntime {
    pub(crate) async fn persist_job(
        &mut self,
        job: &contracts::ExportImportBackupJobRecord,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
    ) -> Result<(), BackupRuntimeError> {
        self.persist_job_phase(job, kind, note, JournalDispatchPhase::AfterDispatch)
            .await
    }

    pub(crate) async fn persist_job_phase(
        &mut self,
        job: &contracts::ExportImportBackupJobRecord,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
        phase: JournalDispatchPhase,
    ) -> Result<(), BackupRuntimeError> {
        let event = DataCustodyRuntimeEvent::backup_job(job.clone(), kind, note);
        self.journal.append_record(event.clone(), phase).await?;
        self.ledger
            .apply_event(&event)
            .map_err(|_| BackupRuntimeError::ScheduleJob)?;
        Ok(())
    }

    pub(crate) fn reserve_backup_dispatch(
        &mut self,
        job: &contracts::ExportImportBackupJobRecord,
    ) -> Result<BackupDispatchReservation, BackupRuntimeError> {
        let execution_ref = job
            .execution_ref
            .clone()
            .ok_or(BackupRuntimeError::DispatchReservation)?;
        if !self
            .dispatch_reservations
            .insert(execution_ref.as_str().to_owned())
        {
            return Err(BackupRuntimeError::DispatchReservation);
        }
        Ok(BackupDispatchReservation::new(
            execution_ref,
            job.bundle_id.clone(),
        ))
    }
}
