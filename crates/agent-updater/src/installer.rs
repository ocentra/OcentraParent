use std::path::Path;
use std::process::Stdio;

use tokio::process::Command;

use crate::error::UpdaterError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsiUpgradeOutcome {
    Completed,
    CompletedRebootRequired,
}

pub async fn start_msi_upgrade(msi_path: &Path) -> Result<MsiUpgradeOutcome, UpdaterError> {
    let mut command = Command::new("msiexec.exe");
    command.arg("/i").arg(msi_path).arg("/qn").arg("/norestart");
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let status = command.status().await.map_err(|error| {
        UpdaterError::Process(format!("failed to start MSI installer: {error}"))
    })?;
    if status.code() == Some(3010) {
        return Ok(MsiUpgradeOutcome::CompletedRebootRequired);
    }
    if !status.success() {
        return Err(UpdaterError::Process(format!(
            "MSI installer exited with {}",
            status
                .code()
                .map_or_else(|| "unknown status".to_owned(), |code| code.to_string())
        )));
    }
    Ok(MsiUpgradeOutcome::Completed)
}
