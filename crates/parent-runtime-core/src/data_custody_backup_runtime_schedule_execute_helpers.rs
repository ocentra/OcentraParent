use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_backup_runtime::{BackupRuntimeError, ParentBackupRuntime};
use super::data_custody_backup_runtime_ports::AccountCustodyAuthorityPort;
use super::data_custody_backup_runtime_schedule::{claim_job, start_job};
use super::data_custody_backup_runtime_schedule_execute_authority::current_backup_authority;
use super::data_custody_backup_runtime_schedule_execute_finish::persist_manual_required;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;

pub(super) enum BackupExecutionPreparation {
    Ready(contracts::ExportImportBackupJobRecord),
    Manual(super::data_custody_backup_runtime::BackupExecutionResult),
}

pub(super) async fn claim_and_start(
    runtime: &mut ParentBackupRuntime,
    job: contracts::ExportImportBackupJobRecord,
    authority_port: Option<&dyn AccountCustodyAuthorityPort>,
) -> Result<BackupExecutionPreparation, BackupRuntimeError> {
    if current_backup_authority(authority_port, &job).is_err() {
        return Ok(BackupExecutionPreparation::Manual(
            persist_manual_required(
                runtime,
                &job,
                "Current household backup authority is unavailable or no longer valid.",
            )
            .await?,
        ));
    }
    let claimed = claim_job(&job, runtime.journal.next_recorded_at()?)?;
    runtime
        .persist_job(
            &claimed,
            DataCustodyRuntimeEventKind::BackupJobTransition,
            None,
        )
        .await?;
    if current_backup_authority(authority_port, &claimed).is_err() {
        return Ok(BackupExecutionPreparation::Manual(
            persist_manual_required(
                runtime,
                &claimed,
                "Current household backup authority changed before execution.",
            )
            .await?,
        ));
    }
    let running = start_job(&claimed, runtime.journal.next_recorded_at()?)?;
    runtime
        .persist_job_phase(
            &running,
            DataCustodyRuntimeEventKind::BackupJobTransition,
            None,
            JournalDispatchPhase::BeforeDispatch,
        )
        .await?;
    if current_backup_authority(authority_port, &running).is_err() {
        return Ok(BackupExecutionPreparation::Manual(
            persist_manual_required(
                runtime,
                &running,
                "Current household backup authority changed before provider dispatch.",
            )
            .await?,
        ));
    }
    Ok(BackupExecutionPreparation::Ready(running))
}
