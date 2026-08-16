use std::path::Path;

#[cfg(windows)]
use ocentra_parent_agent_protocol::constants;
use tokio::process::Command;

pub(super) fn adapter_process_command(command: &Path) -> Command {
    #[cfg(windows)]
    {
        if is_windows_batch_adapter(command) {
            let mut process =
                Command::new(constants::local_ai_runtime::WINDOWS_COMMAND_INTERPRETER);
            process
                .arg(constants::local_ai_runtime::WINDOWS_COMMAND_RUN_ARG)
                .arg(command);
            return process;
        }
    }
    Command::new(command)
}

#[cfg(windows)]
pub(super) fn is_windows_batch_adapter(command: &Path) -> bool {
    command
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case(constants::local_ai_runtime::WINDOWS_BATCH_EXTENSION_CMD)
                || extension
                    .eq_ignore_ascii_case(constants::local_ai_runtime::WINDOWS_BATCH_EXTENSION_BAT)
        })
}
