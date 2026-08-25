use ocentra_schema::export_import_backup_recovery as contracts;

use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_backup_job_state::{
        advance_backup_job, BackupJobStateError, BackupJobTransition,
    };

use super::data_custody_backup_runtime_job_ledger::BackupJobLedger;

const RESTART_RECONCILIATION_NOTE: &str =
    "Backup execution was non-terminal at parent restart and requires provider reconciliation.";

pub(crate) fn reconcile_after_restart(
    ledger: &BackupJobLedger,
    now: &str,
) -> Result<Vec<contracts::ExportImportBackupJobRecord>, BackupJobStateError> {
    let mut reconciled = Vec::new();
    for job in ledger.jobs() {
        if !matches!(
            job.lifecycle,
            contracts::ExportImportBackupJobLifecycle::Claimed
                | contracts::ExportImportBackupJobLifecycle::Running
        ) {
            continue;
        }
        reconciled.push(advance_backup_job(
            job,
            BackupJobTransition {
                lifecycle: contracts::ExportImportBackupJobLifecycle::ManualRequired,
                updated_at: now.to_owned(),
                execution_ref: None,
                provider_operation_ref: None,
                manual_required_note: Some(RESTART_RECONCILIATION_NOTE.to_owned()),
            },
        )?);
    }
    Ok(reconciled)
}
