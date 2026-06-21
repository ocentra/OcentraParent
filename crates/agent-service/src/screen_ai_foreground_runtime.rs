use std::time::Duration;

use ocentra_parent_agent_core::window_capture::collect_foreground_window_observation;
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, SCREEN_SERVICE_FOREGROUND_EVENT_ID_PREFIX,
    SCREEN_SERVICE_FOREGROUND_EVIDENCE_ID_PREFIX, SCREEN_SERVICE_FOREGROUND_MODEL_ID,
    SCREEN_SERVICE_FOREGROUND_QUEUE_JOB_ID_PREFIX, SCREEN_SERVICE_FOREGROUND_RESULT_ID_PREFIX,
    SCREEN_SERVICE_FOREGROUND_SOURCE_ID, SCREEN_SERVICE_FOREGROUND_SUMMARY_CAPTURED,
    SCREEN_SERVICE_FOREGROUND_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png,
    trigger_scheduler::{
        evaluate_screen_capture_schedule, ScreenCaptureScheduleDecision,
        ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerState, ScreenCaptureTriggerInput,
    },
    CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureScope,
};

use crate::{
    activity_capture::ActivityCaptureError,
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCaptureClock,
        ScreenAiServiceCapturePaths, ScreenAiServiceCaptureRecord,
    },
    screen_ai_foreground_runtime_config::{
        foreground_key, pending_queue_record_count, ScreenAiForegroundRuntimeConfig,
    },
    screen_ai_service_event_bridge::publish_screen_capture_queue_events_for_queue_job,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScreenAiForegroundTickOutcome {
    Recorded {
        queue_job_id: String,
        foreground_key: String,
    },
    NoForegroundChange {
        foreground_key: String,
    },
    Suppressed,
    ForegroundUnavailable {
        status: ActivityCaptureCapabilityStatus,
    },
    CaptureUnavailable {
        status: ActivityCaptureCapabilityStatus,
    },
    QueueBackpressure {
        pending_count: u64,
        max_pending_queue_records: u64,
    },
}

pub(crate) type ScreenAiForegroundTickClock = ScreenAiServiceCaptureClock;

pub(crate) fn spawn_screen_ai_foreground_runtime() {
    if let Some(config) = ScreenAiForegroundRuntimeConfig::from_environment() {
        tokio::spawn(async move {
            run_screen_ai_foreground_runtime(config).await;
        });
    }
}

async fn run_screen_ai_foreground_runtime(config: ScreenAiForegroundRuntimeConfig) {
    let mut interval = tokio::time::interval(Duration::from_secs(config.poll_seconds));
    let mut state = ScreenCaptureSchedulerState {
        last_capture_at_epoch_seconds: None,
    };
    let mut last_foreground_key = None;
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
            last_foreground_key.as_deref(),
            clock,
            tick_count,
        );
        if let Ok(outcome) = outcome {
            match outcome {
                ScreenAiForegroundTickOutcome::Recorded {
                    queue_job_id,
                    foreground_key,
                } => {
                    let _ = publish_screen_capture_queue_events_for_queue_job(
                        &config.store_path,
                        &queue_job_id,
                        &observed_at,
                    )
                    .await;
                    state.last_capture_at_epoch_seconds = Some(epoch_seconds);
                    last_foreground_key = Some(foreground_key);
                    capture_count += 1;
                }
                ScreenAiForegroundTickOutcome::NoForegroundChange { foreground_key } => {
                    last_foreground_key = Some(foreground_key);
                }
                ScreenAiForegroundTickOutcome::QueueBackpressure { .. } => {}
                _ => {}
            }
        }
        if config.max_captures.is_some_and(|max| capture_count >= max) {
            break;
        }
        if config.max_ticks.is_some_and(|max| tick_count >= max) {
            break;
        }
    }
}

pub(crate) fn record_screen_ai_foreground_tick(
    config: &ScreenAiForegroundRuntimeConfig,
    state: ScreenCaptureSchedulerState,
    last_foreground_key: Option<&str>,
    clock: ScreenAiForegroundTickClock,
    tick_index: u64,
) -> Result<ScreenAiForegroundTickOutcome, ActivityCaptureError> {
    let observation = collect_foreground_window_observation();
    let Some(foreground_key) = foreground_key(&observation) else {
        return Ok(ScreenAiForegroundTickOutcome::ForegroundUnavailable {
            status: observation.status,
        });
    };
    if last_foreground_key == Some(foreground_key.as_str()) {
        return Ok(ScreenAiForegroundTickOutcome::NoForegroundChange { foreground_key });
    }
    let decision = evaluate_screen_capture_schedule(
        &config.scheduler_settings(),
        state,
        ScreenCaptureTriggerInput {
            observed_at_epoch_seconds: clock.epoch_seconds,
            trigger: ScreenCaptureScheduleTrigger::NativeAppForegroundStart,
            requested_scope: Some(ScreenCaptureScope::ActiveWindow),
        },
    );
    if !matches!(
        decision,
        ScreenCaptureScheduleDecision::EnqueueCapture { .. }
    ) {
        return Ok(ScreenAiForegroundTickOutcome::Suppressed);
    }
    let pending_count = pending_queue_record_count(&config.queue_dir)?;
    if pending_count >= config.max_pending_queue_records {
        return Ok(ScreenAiForegroundTickOutcome::QueueBackpressure {
            pending_count,
            max_pending_queue_records: config.max_pending_queue_records,
        });
    }
    match capture_active_window_png() {
        ScreenCaptureAttempt::Captured(image) => {
            let queue_job_id =
                record_screen_ai_foreground_captured_image(config, &image, clock, tick_index)?;
            Ok(ScreenAiForegroundTickOutcome::Recorded {
                queue_job_id,
                foreground_key,
            })
        }
        ScreenCaptureAttempt::Degraded(metadata) => {
            Ok(ScreenAiForegroundTickOutcome::CaptureUnavailable {
                status: metadata.status,
            })
        }
    }
}

pub(crate) fn record_screen_ai_foreground_captured_image(
    config: &ScreenAiForegroundRuntimeConfig,
    image: &CapturedScreenImage,
    clock: ScreenAiForegroundTickClock,
    sequence_index: u64,
) -> Result<String, ActivityCaptureError> {
    record_captured_screen_image_to_paths(ScreenAiServiceCaptureRecord {
        paths: ScreenAiServiceCapturePaths {
            queue_dir: &config.queue_dir,
            journal_path: &config.journal_path,
            journal_key_path: &config.journal_key_path,
            store_path: &config.store_path,
        },
        image,
        clock,
        sequence_index,
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_NATIVE_APP_FOREGROUND_START,
        source_id: SCREEN_SERVICE_FOREGROUND_SOURCE_ID,
        queue_job_id_prefix: SCREEN_SERVICE_FOREGROUND_QUEUE_JOB_ID_PREFIX,
        result_id_prefix: SCREEN_SERVICE_FOREGROUND_RESULT_ID_PREFIX,
        event_id_prefix: SCREEN_SERVICE_FOREGROUND_EVENT_ID_PREFIX,
        evidence_id_prefix: SCREEN_SERVICE_FOREGROUND_EVIDENCE_ID_PREFIX,
        summary: SCREEN_SERVICE_FOREGROUND_SUMMARY_CAPTURED,
        model_id: SCREEN_SERVICE_FOREGROUND_MODEL_ID,
        template_version: SCREEN_SERVICE_FOREGROUND_TEMPLATE_VERSION,
        temporary_image_ttl_seconds: config.temporary_image_ttl_seconds,
    })
}
