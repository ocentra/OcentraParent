use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

pub fn enforcement_timer_state_path() -> PathBuf {
    env::var(constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = env::temp_dir();
            path.push(constants::enforcement::TIMER_STATE_FILE_NAME);
            path
        })
}
