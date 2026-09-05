use std::io::{stderr, stdout, Write};

use crate::error::UpdaterError;
use crate::update::UpdateOutcome;

pub(super) fn print_outcome(
    result: Result<UpdateOutcome, UpdaterError>,
) -> Result<(), UpdaterError> {
    match result {
        Ok(UpdateOutcome::Current { version }) => {
            write_stdout_line(&format!("updater-current:{version}"))
        }
        Ok(UpdateOutcome::WouldInstall { current, latest }) => {
            write_stdout_line(&format!("updater-would-install:{current}->{latest}"))
        }
        Ok(UpdateOutcome::InstallerCompleted { current, latest }) => {
            write_stdout_line(&format!("updater-installer-completed:{current}->{latest}"))
        }
        Ok(UpdateOutcome::InstallerCompletedRebootRequired { current, latest }) => {
            write_stdout_line(&format!(
                "updater-installer-completed-reboot-required:{current}->{latest}"
            ))
        }
        Err(error) => {
            write_stderr_line(&error.to_string())?;
            std::process::exit(1);
        }
    }
}

fn write_stdout_line(message: &str) -> Result<(), UpdaterError> {
    let mut output = stdout().lock();
    writeln!(output, "{message}")?;
    Ok(())
}

fn write_stderr_line(message: &str) -> Result<(), UpdaterError> {
    let mut output = stderr().lock();
    writeln!(output, "{message}")?;
    Ok(())
}
