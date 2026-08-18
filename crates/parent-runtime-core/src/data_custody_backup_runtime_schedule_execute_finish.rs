use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_backup_runtime::{
    BackupExecutionResult, BackupRuntimeError, ParentBackupRuntime,
};
use super::data_custody_backup_runtime_ports::{ProviderBackupError, ProviderOperationReceipt};
use super::data_custody_backup_runtime_schedule::{
    manual_required_job, succeed_job, ProviderJobOutcome,
};
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;

pub(super) async fn finish_job(
    runtime: &mut ParentBackupRuntime,
    running: &contracts::ExportImportBackupJobRecord,
    outcome: ProviderJobOutcome,
) -> Result<BackupExecutionResult, BackupRuntimeError> {
    match outcome {
        ProviderJobOutcome::ManualRequired(error) => {
            persist_manual_required(runtime, running, provider_error_note(error)).await
        }
        ProviderJobOutcome::Succeeded(receipt) => {
            let succeeded = succeed_job(running, runtime.journal.next_recorded_at()?, &receipt)?;
            runtime
                .persist_job(
                    &succeeded,
                    DataCustodyRuntimeEventKind::BackupJobTransition,
                    None,
                )
                .await?;
            Ok(BackupExecutionResult::Succeeded(succeeded))
        }
    }
}

pub(super) async fn persist_manual_required(
    runtime: &mut ParentBackupRuntime,
    job: &contracts::ExportImportBackupJobRecord,
    note: &'static str,
) -> Result<BackupExecutionResult, BackupRuntimeError> {
    let manual = manual_required_job(job, runtime.journal.next_recorded_at()?, note.to_owned())?;
    runtime
        .persist_job(
            &manual,
            DataCustodyRuntimeEventKind::Reconciliation,
            manual.manual_required_note.clone(),
        )
        .await?;
    Ok(BackupExecutionResult::ManualRequired(manual))
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
