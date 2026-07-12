use std::time::Duration;

use ocentra_parent_screen_capture_adapter::trigger_scheduler::ScreenCaptureSchedulerState;
use ocentra_parent_screen_capture_adapter::CapturedScreenImage;

use crate::{
    activity_capture::ActivityCaptureError,
    screen_ai_foreground_runtime::{
        ScreenAiForegroundKey, ScreenAiForegroundQueueJobId, ScreenAiForegroundRuntimeConfig,
        ScreenAiForegroundTickClock, ScreenAiForegroundTickOutcome,
    },
    screen_ai_service_event_bridge::publish_screen_capture_queue_events_for_queue_job,
    screen_ai_service_event_bridge::ScreenAiQueueJobId,
    screen_ai_service_event_subscription::ObservedAtText,
};

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

pub(super) fn record_screen_ai_foreground_captured_image(
    config: &ScreenAiForegroundRuntimeConfig,
    image: &CapturedScreenImage,
    clock: ScreenAiForegroundTickClock,
    sequence_index: u64,
) -> Result<ScreenAiForegroundQueueJobId, ActivityCaptureError> {
    tick::record_screen_ai_foreground_captured_image(config, image, clock, sequence_index)
}
