use std::time::Duration;

use ocentra_parent_screen_capture_adapter::trigger_scheduler::ScreenCaptureSchedulerState;

use crate::{
    screen_ai_foreground_runtime::{ScreenAiForegroundTickClock, ScreenAiForegroundTickOutcome},
    screen_ai_foreground_runtime_config::ScreenAiForegroundRuntimeConfig,
    screen_ai_service_event_bridge::publish_screen_capture_queue_events_for_queue_job,
    screen_ai_service_event_bridge::ScreenAiQueueJobId,
    screen_ai_service_event_subscription::ObservedAtText,
};

use super::{record_screen_ai_foreground_tick, ScreenAiForegroundKey};

struct TickMutation {
    captured: bool,
    next_foreground_key: Option<ScreenAiForegroundKey>,
    last_capture_at_epoch_seconds: Option<u64>,
}

pub(super) async fn run_screen_ai_foreground_runtime(config: ScreenAiForegroundRuntimeConfig) {
    let mut interval = tokio::time::interval(Duration::from_secs(config.poll_seconds));
    let mut state = ScreenCaptureSchedulerState {
        last_capture_at_epoch_seconds: None,
    };
    let mut last_foreground_key: Option<ScreenAiForegroundKey> = None;
    let mut capture_count = 0;
    let mut tick_count = 0;
    loop {
        interval.tick().await;
        tick_count += 1;
        let clock = ScreenAiForegroundTickClock::from_system_time();
        let observed_at = clock.timestamp.clone();
        let epoch_seconds = clock.epoch_seconds;
        let outcome = record_screen_ai_foreground_tick(
            &config,
            state,
            last_foreground_key.as_ref(),
            clock,
            tick_count,
        );
        if let Ok(outcome) = outcome {
            let mutation =
                apply_tick_outcome(&config, outcome, ObservedAtText(observed_at), epoch_seconds)
                    .await;
            if mutation.captured {
                capture_count += 1;
            }
            state.last_capture_at_epoch_seconds = mutation.last_capture_at_epoch_seconds;
            last_foreground_key = mutation.next_foreground_key;
        }
        if config.max_captures.is_some_and(|max| capture_count >= max) {
            break;
        }
        if config.max_ticks.is_some_and(|max| tick_count >= max) {
            break;
        }
    }
}

async fn apply_tick_outcome(
    config: &ScreenAiForegroundRuntimeConfig,
    outcome: ScreenAiForegroundTickOutcome,
    observed_at: ObservedAtText,
    epoch_seconds: u64,
) -> TickMutation {
    match outcome {
        ScreenAiForegroundTickOutcome::Recorded {
            queue_job_id,
            foreground_key,
        } => {
            let _ = publish_screen_capture_queue_events_for_queue_job(
                &config.store_path,
                ScreenAiQueueJobId(queue_job_id),
                observed_at,
            )
            .await;
            TickMutation {
                captured: true,
                next_foreground_key: Some(ScreenAiForegroundKey(foreground_key)),
                last_capture_at_epoch_seconds: Some(epoch_seconds),
            }
        }
        ScreenAiForegroundTickOutcome::NoForegroundChange { foreground_key } => TickMutation {
            captured: false,
            next_foreground_key: Some(ScreenAiForegroundKey(foreground_key)),
            last_capture_at_epoch_seconds: None,
        },
        ScreenAiForegroundTickOutcome::QueueBackpressure { .. }
        | ScreenAiForegroundTickOutcome::Suppressed
        | ScreenAiForegroundTickOutcome::ForegroundUnavailable { .. }
        | ScreenAiForegroundTickOutcome::CaptureUnavailable { .. } => TickMutation {
            captured: false,
            next_foreground_key: None,
            last_capture_at_epoch_seconds: None,
        },
    }
}
