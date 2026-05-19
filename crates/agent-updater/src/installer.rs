use std::path::Path;
use std::process::Stdio;

use tokio::process::Command;

use crate::error::UpdaterError;

pub async fn start_msi_upgrade(msi_path: &Path) -> Result<(), UpdaterError> {
    let mut command = Command::new("msiexec.exe");
    command.arg("/i").arg(msi_path).arg("/qn").arg("/norestart");
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command.spawn().map_err(|error| {
        UpdaterError::Process(format!("failed to start MSI installer: {error}"))
    })?;
    Ok(())
}
