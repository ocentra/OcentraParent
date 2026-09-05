use ocentra_parent_screen_capture_adapter::trigger_scheduler::ScreenCaptureSchedulerState;

use crate::activity_capture::ActivityCaptureError;
use crate::screen_ai_foreground_runtime::{
    types::{ScreenAiForegroundKey, ScreenAiForegroundTickClock},
    ScreenAiForegroundTickOutcome,
};
use crate::screen_ai_foreground_runtime_config::ScreenAiForegroundRuntimeConfig;

#[path = "runtime_loop.rs"]
mod loop_runtime;
#[path = "runtime_tick.rs"]
mod tick;

pub(super) fn spawn_screen_ai_foreground_runtime() {
    if let Some(config) = ScreenAiForegroundRuntimeConfig::from_environment() {
        tokio::spawn(async move {
            run_screen_ai_foreground_runtime(config).await;
        });
    }
}

pub(super) async fn run_screen_ai_foreground_runtime(config: ScreenAiForegroundRuntimeConfig) {
    loop_runtime::run_screen_ai_foreground_runtime(config).await;
}

pub(super) fn record_screen_ai_foreground_tick(
    config: &ScreenAiForegroundRuntimeConfig,
    state: ScreenCaptureSchedulerState,
    last_foreground_key: Option<&ScreenAiForegroundKey>,
    clock: ScreenAiForegroundTickClock,
    tick_index: u64,
) -> Result<ScreenAiForegroundTickOutcome, ActivityCaptureError> {
    tick::record_screen_ai_foreground_tick(config, state, last_foreground_key, clock, tick_index)
}
