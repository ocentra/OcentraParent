use chrono::Utc;
use ocentra_eventing::journal::policy::JournalDispatchPhase;

use super::data_custody_backup_runtime::{
    BackupExecutionResult, BackupRuntimeError, ParentBackupRuntime, ProviderBackupError,
    ProviderNeutralBackupPort,
};
use super::data_custody_backup_runtime_schedule::{
    claim_job, execute_provider, manual_required_job, start_job, succeed_job, ProviderJobOutcome,
};
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;
use ocentra_schema::export_import_backup_recovery as contracts;

impl ParentBackupRuntime {
    pub(crate) async fn execute_next(
        &mut self,
        provider: Option<&dyn ProviderNeutralBackupPort>,
    ) -> Result<Option<BackupExecutionResult>, BackupRuntimeError> {
        let job = match self.ledger.claimable_job() {
            Some(job) => job,
            None => return Ok(None),
        };
        let claimed = claim_job(&job, Utc::now().to_rfc3339())?;
        self.persist_job(
            &claimed,
            DataCustodyRuntimeEventKind::BackupJobTransition,
            None,
        )
        .await?;
        let running = start_job(&claimed, Utc::now().to_rfc3339())?;
        self.persist_job_phase(
            &running,
            DataCustodyRuntimeEventKind::BackupJobTransition,
            None,
            JournalDispatchPhase::BeforeDispatch,
        )
        .await?;

        let reservation = self.reserve_backup_dispatch(&running)?;
        finish_job(
            self,
            &running,
            execute_provider(provider, &running, reservation),
        )
        .await
    }
}

async fn finish_job(
    runtime: &mut ParentBackupRuntime,
    running: &contracts::ExportImportBackupJobRecord,
    outcome: ProviderJobOutcome,
) -> Result<Option<BackupExecutionResult>, BackupRuntimeError> {
    match outcome {
        ProviderJobOutcome::ManualRequired(error) => {
            finish_manual_job(runtime, running, error).await
        }
        ProviderJobOutcome::Succeeded(receipt) => {
            finish_succeeded_job(runtime, running, &receipt).await
        }
    }
}

async fn finish_manual_job(
    runtime: &mut ParentBackupRuntime,
    running: &contracts::ExportImportBackupJobRecord,
    error: ProviderBackupError,
) -> Result<Option<BackupExecutionResult>, BackupRuntimeError> {
    let manual = manual_required_job(
        running,
        Utc::now().to_rfc3339(),
        provider_error_note(error).to_owned(),
    )?;
    runtime
        .persist_job(
            &manual,
            DataCustodyRuntimeEventKind::Reconciliation,
            manual.manual_required_note.clone(),
        )
        .await?;
    Ok(Some(BackupExecutionResult::ManualRequired(manual)))
}

async fn finish_succeeded_job(
    runtime: &mut ParentBackupRuntime,
    running: &contracts::ExportImportBackupJobRecord,
    receipt: &super::data_custody_backup_runtime::ProviderOperationReceipt,
) -> Result<Option<BackupExecutionResult>, BackupRuntimeError> {
    let succeeded = succeed_job(running, Utc::now().to_rfc3339(), receipt)?;
    runtime
        .persist_job(
            &succeeded,
            DataCustodyRuntimeEventKind::BackupJobTransition,
            None,
        )
        .await?;
    Ok(Some(BackupExecutionResult::Succeeded(succeeded)))
}

fn provider_error_note(error: ProviderBackupError) -> &'static str {
    match error {
        ProviderBackupError::Unavailable => {
            "No trusted provider adapter is mounted for this parent runtime."
        }
        ProviderBackupError::Failed => {
            "Provider backup outcome is unknown; status reconciliation is required before retry."
        }
    }
}
