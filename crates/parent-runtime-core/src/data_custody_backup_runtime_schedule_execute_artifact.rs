use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_backup_runtime::{
    BackupExecutionResult, BackupRuntimeError, ParentBackupRuntime,
};
use super::data_custody_backup_runtime_ports::{
    BackupArtifactBinding, BackupCustodyArtifactPort, ProviderNeutralBackupPort,
};
use super::data_custody_backup_runtime_schedule_execute_finish::persist_manual_required;

pub(super) enum BackupArtifactPreparation {
    Ready(BackupArtifactBinding),
    Manual(BackupExecutionResult),
}

pub(super) async fn prepare_artifact(
    runtime: &mut ParentBackupRuntime,
    job: &contracts::ExportImportBackupJobRecord,
    artifact_port: Option<&dyn BackupCustodyArtifactPort>,
    provider: Option<&dyn ProviderNeutralBackupPort>,
) -> Result<BackupArtifactPreparation, BackupRuntimeError> {
    if provider.is_none() {
        return Ok(BackupArtifactPreparation::Manual(
            persist_manual_required(
                runtime,
                job,
                "No trusted backup provider is mounted for this parent runtime.",
            )
            .await?,
        ));
    }
    let Some(artifact_port) = artifact_port else {
        return Ok(BackupArtifactPreparation::Manual(
            persist_manual_required(
                runtime,
                job,
                "Verified encrypted backup custody is not mounted for this parent runtime.",
            )
            .await?,
        ));
    };
    let artifact = match artifact_port.prepare_backup_artifact(job) {
        Ok(artifact) => artifact,
        Err(_) => {
            return Ok(BackupArtifactPreparation::Manual(
                persist_manual_required(
                    runtime,
                    job,
                    "Verified encrypted backup custody could not be prepared.",
                )
                .await?,
            ));
        }
    };
    if artifact.bundle_id() != &job.bundle_id || !artifact.is_complete() {
        return Ok(BackupArtifactPreparation::Manual(
            persist_manual_required(
                runtime,
                job,
                "Backup custody artifact does not match the durable bundle identity.",
            )
            .await?,
        ));
    }
    Ok(BackupArtifactPreparation::Ready(artifact))
}
