use std::sync::Mutex;

use ocentra_parent_agent_protocol::{constants, screen_evidence};

use crate::{
    screen_ai_retention_sweeper_runtime::{
        spawn_screen_ai_retention_sweeper_runtime, ScreenAiRetentionSweeperRuntimeConfig,
    },
    test_require_some::require_some,
};

static RETENTION_RUNTIME_ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn screen_retention_sweeper_startup_respects_explicit_enablement() {
    let _guard = RETENTION_RUNTIME_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let env_name = screen_evidence::SCREEN_SERVICE_RETENTION_SWEEPER_RUNTIME_ENABLED_ENV;
    let previous = std::env::var_os(env_name);
    std::env::remove_var(env_name);

    let config = require_some(
        ScreenAiRetentionSweeperRuntimeConfig::from_environment(),
        constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES,
    );
    assert_eq!(config.poll_seconds, 5);
    assert_eq!(config.max_sweeps, None);
    assert_eq!(config.max_ticks, None);

    std::env::set_var(env_name, "false");
    assert_eq!(
        ScreenAiRetentionSweeperRuntimeConfig::from_environment(),
        None
    );
    spawn_screen_ai_retention_sweeper_runtime();

    match previous {
        Some(value) => std::env::set_var(env_name, value),
        None => std::env::remove_var(env_name),
    }
}
