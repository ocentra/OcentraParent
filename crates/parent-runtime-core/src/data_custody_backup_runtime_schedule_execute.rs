use super::data_custody_backup_runtime::{
    BackupExecutionResult, BackupRuntimeError, ParentBackupRuntime,
};
use super::data_custody_backup_runtime_ports::{
    AccountCustodyAuthorityPort, BackupCustodyArtifactPort, ProviderNeutralBackupPort,
};
use super::data_custody_backup_runtime_schedule::execute_provider;
use super::data_custody_backup_runtime_schedule_execute_artifact::{
    prepare_artifact, BackupArtifactPreparation,
};
use super::data_custody_backup_runtime_schedule_execute_authority::current_backup_authority;
use super::data_custody_backup_runtime_schedule_execute_finish::finish_job;
use super::data_custody_backup_runtime_schedule_execute_helpers::{
    claim_and_start, BackupExecutionPreparation,
};

impl ParentBackupRuntime {
    pub(crate) async fn execute_next(
        &mut self,
        authority_port: Option<&dyn AccountCustodyAuthorityPort>,
        artifact_port: Option<&dyn BackupCustodyArtifactPort>,
        provider: Option<&dyn ProviderNeutralBackupPort>,
    ) -> Result<Option<BackupExecutionResult>, BackupRuntimeError> {
        if !self.recovered {
            return Err(BackupRuntimeError::RuntimeNotRecovered);
        }
        let Some(job) = self.ledger.claimable_job() else {
            return Ok(None);
        };
        let running = match claim_and_start(self, job, authority_port).await? {
            BackupExecutionPreparation::Ready(running) => running,
            BackupExecutionPreparation::Manual(result) => return Ok(Some(result)),
        };
        let artifact = match prepare_artifact(self, &running, artifact_port, provider).await? {
            BackupArtifactPreparation::Ready(artifact) => artifact,
            BackupArtifactPreparation::Manual(result) => return Ok(Some(result)),
        };
        if current_backup_authority(authority_port, &running).is_err() {
            return Ok(Some(
                super::data_custody_backup_runtime_schedule_execute_finish::persist_manual_required(
                    self,
                    &running,
                    "Current household backup authority changed after artifact preparation.",
                )
                .await?,
            ));
        }
        let reservation = self.reserve_backup_dispatch(&running)?;
        let outcome = execute_provider(provider, &running, reservation, artifact);
        Ok(Some(finish_job(self, &running, outcome).await?))
    }
}
