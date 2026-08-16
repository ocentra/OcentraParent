use std::time::Duration;

use ocentra_parent_screen_capture_adapter::trigger_scheduler::{
    ScreenCaptureSchedulerSettings, ScreenCaptureSchedulerState,
};

use crate::{
    screen_ai_cadence_runtime::{
        ScreenAiCadenceRuntimeConfig, ScreenAiCadenceTickClock, ScreenAiCadenceTickOutcome,
    },
    screen_ai_service_event_bridge::publish_screen_capture_queue_events_for_queue_job,
    screen_ai_service_event_bridge::ScreenAiQueueJobId,
    screen_ai_service_event_subscription::ObservedAtText,
};

#[path = "runtime_environment.rs"]
mod environment;
#[path = "runtime_tick.rs"]
mod tick;

pub(super) fn spawn_screen_ai_cadence_runtime() {
    if let Some(config) = from_environment() {
        tokio::spawn(async move {
            run_screen_ai_cadence_runtime(config).await;
        });
    }
}

pub(super) async fn run_screen_ai_cadence_runtime(config: ScreenAiCadenceRuntimeConfig) {
    let mut interval = tokio::time::interval(Duration::from_secs(config.cadence_seconds));
    let mut state = ScreenCaptureSchedulerState {
        last_capture_at_epoch_seconds: None,
    };
    let mut capture_count = 0;
    let mut tick_count = 0;
    loop {
        interval.tick().await;
        tick_count += 1;
        let clock = ScreenAiCadenceTickClock::from_system_time();
        let observed_at = clock.timestamp.clone();
        let epoch_seconds = clock.epoch_seconds;
        let outcome = tick::record_screen_ai_cadence_tick(&config, state, clock, tick_count);
        if let Ok(ScreenAiCadenceTickOutcome::Recorded { queue_job_id }) = outcome {
            let _ = publish_screen_capture_queue_events_for_queue_job(
                &config.store_path,
                ScreenAiQueueJobId(queue_job_id),
                ObservedAtText(observed_at),
            )
            .await;
            state.last_capture_at_epoch_seconds = Some(epoch_seconds);
            capture_count += 1;
        }
        if config.max_captures.is_some_and(|max| capture_count >= max) {
            break;
        }
        if config.max_ticks.is_some_and(|max| tick_count >= max) {
            break;
        }
    }
}

pub(super) fn from_environment() -> Option<ScreenAiCadenceRuntimeConfig> {
    environment::from_environment()
}

pub(super) fn scheduler_settings(
    config: &ScreenAiCadenceRuntimeConfig,
) -> ScreenCaptureSchedulerSettings {
    environment::scheduler_settings(config)
}
