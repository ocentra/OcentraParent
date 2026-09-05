use ocentra_parent_agent_core::window_capture::collect_foreground_window_observation;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_SERVICE_FOREGROUND_EVENT_ID_PREFIX, SCREEN_SERVICE_FOREGROUND_EVIDENCE_ID_PREFIX,
    SCREEN_SERVICE_FOREGROUND_MODEL_ID, SCREEN_SERVICE_FOREGROUND_QUEUE_JOB_ID_PREFIX,
    SCREEN_SERVICE_FOREGROUND_RESULT_ID_PREFIX, SCREEN_SERVICE_FOREGROUND_SOURCE_ID,
    SCREEN_SERVICE_FOREGROUND_SUMMARY_CAPTURED, SCREEN_SERVICE_FOREGROUND_TEMPLATE_VERSION,
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
        record_captured_screen_image_to_paths, ScreenAiServiceCapturePaths,
        ScreenAiServiceCaptureRecord,
    },
    screen_ai_foreground_runtime::{
        types::{ScreenAiForegroundKey, ScreenAiForegroundTickClock},
        ScreenAiForegroundQueueJobId, ScreenAiForegroundTickOutcome,
    },
    screen_ai_foreground_runtime_config::ScreenAiForegroundRuntimeConfig,
    screen_ai_foreground_runtime_config::{foreground_key, pending_queue_record_count},
};

pub(super) fn record_screen_ai_foreground_tick(
    config: &ScreenAiForegroundRuntimeConfig,
    state: ScreenCaptureSchedulerState,
    last_foreground_key: Option<&ScreenAiForegroundKey>,
    clock: ScreenAiForegroundTickClock,
    tick_index: u64,
) -> Result<ScreenAiForegroundTickOutcome, ActivityCaptureError> {
    let observation = collect_foreground_window_observation();
    let Some(foreground_key) = foreground_key(&observation) else {
        return Ok(ScreenAiForegroundTickOutcome::ForegroundUnavailable {
            status: observation.status,
        });
    };
    if last_foreground_key.is_some_and(|current| current.0 == foreground_key.0) {
        return Ok(ScreenAiForegroundTickOutcome::NoForegroundChange {
            foreground_key: foreground_key.0,
        });
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
                queue_job_id: queue_job_id.0,
                foreground_key: foreground_key.0,
            })
        }
        ScreenCaptureAttempt::Degraded(metadata) => {
            Ok(ScreenAiForegroundTickOutcome::CaptureUnavailable {
                status: metadata.status,
            })
        }
    }
}

pub(super) fn record_screen_ai_foreground_captured_image(
    config: &ScreenAiForegroundRuntimeConfig,
    image: &CapturedScreenImage,
    clock: ScreenAiForegroundTickClock,
    sequence_index: u64,
) -> Result<ScreenAiForegroundQueueJobId, ActivityCaptureError> {
    let queue_job_id = record_captured_screen_image_to_paths(ScreenAiServiceCaptureRecord {
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
    })?;
    Ok(ScreenAiForegroundQueueJobId(queue_job_id.0))
}
