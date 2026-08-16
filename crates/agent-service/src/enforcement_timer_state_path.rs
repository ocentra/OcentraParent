use std::{env, fs, io, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementTimerStatePath(pub(crate) PathBuf);

impl AsRef<std::path::Path> for EnforcementTimerStatePath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementTimerStateDir(PathBuf);

impl EnforcementTimerStatePath {
    pub(crate) fn parent_dir(&self) -> Option<EnforcementTimerStateDir> {
        self.0
            .parent()
            .map(|path| EnforcementTimerStateDir(path.to_path_buf()))
    }
}

impl EnforcementTimerStateDir {
    pub(crate) fn create_all(&self) -> io::Result<()> {
        fs::create_dir_all(&self.0)
    }
}

pub(crate) fn enforcement_timer_state_path() -> EnforcementTimerStatePath {
    configured_timer_state_path().unwrap_or_else(default_timer_state_path)
}

fn configured_timer_state_path() -> Option<EnforcementTimerStatePath> {
    env::var(constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH)
        .ok()
        .map(|value| EnforcementTimerStatePath(PathBuf::from(value)))
}

fn default_timer_state_path() -> EnforcementTimerStatePath {
    let mut path = env::temp_dir();
    path.push(constants::enforcement::TIMER_STATE_FILE_NAME);
    EnforcementTimerStatePath(path)
}
