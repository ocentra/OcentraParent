use crate::enforcement_timer_report::TimerReportError;
use crate::enforcement_timer_state_file::EnforcementTimerStateFileError;

use super::command::EnforcementTimerCommandError;

pub(super) fn timer_state_file_error(
    error: EnforcementTimerStateFileError,
) -> EnforcementTimerCommandError {
    match error {
        EnforcementTimerStateFileError::ActiveTimerStateRequired => {
            EnforcementTimerCommandError::ActiveTimerStateRequired
        }
        EnforcementTimerStateFileError::Store => {
            EnforcementTimerCommandError::Report(TimerReportError::Store)
        }
        EnforcementTimerStateFileError::Serialize => {
            EnforcementTimerCommandError::Report(TimerReportError::Serialize)
        }
    }
}
