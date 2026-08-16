use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_SERVICE_EVENT_ID_PREFIX, SCREEN_SERVICE_EVIDENCE_ID_PREFIX, SCREEN_SERVICE_MODEL_ID,
    SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX, SCREEN_SERVICE_RESULT_ID_PREFIX, SCREEN_SERVICE_SOURCE_ID,
    SCREEN_SERVICE_SUMMARY_CAPTURED, SCREEN_SERVICE_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png,
    trigger_scheduler::{
        evaluate_screen_capture_schedule, ScreenCaptureScheduleDecision,
        ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerState, ScreenCaptureTriggerInput,
    },
    ScreenCaptureAttempt, ScreenCaptureScope,
};

use crate::{
    activity_capture::ActivityCaptureError,
    screen_ai_cadence_runtime::{
        ScreenAiCadenceRuntimeConfig, ScreenAiCadenceTickClock, ScreenAiCadenceTickOutcome,
    },
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCapturePaths,
        ScreenAiServiceCaptureRecord,
    },
};

use super::{environment, scheduler_settings};

pub(super) fn record_screen_ai_cadence_tick(
    config: &ScreenAiCadenceRuntimeConfig,
    state: ScreenCaptureSchedulerState,
    clock: ScreenAiCadenceTickClock,
    tick_index: u64,
) -> Result<ScreenAiCadenceTickOutcome, ActivityCaptureError> {
    let decision = evaluate_screen_capture_schedule(
        &scheduler_settings(config),
        state,
        ScreenCaptureTriggerInput {
            observed_at_epoch_seconds: clock.epoch_seconds,
            trigger: ScreenCaptureScheduleTrigger::TimedCadence,
            requested_scope: Some(ScreenCaptureScope::ActiveWindow),
        },
    );
    if !matches!(
        decision,
        ScreenCaptureScheduleDecision::EnqueueCapture { .. }
    ) {
        return Ok(ScreenAiCadenceTickOutcome::Suppressed);
    }
    let pending_count = environment::pending_queue_record_count(&config.queue_dir)?;
    if pending_count >= config.max_pending_queue_records {
        return Ok(ScreenAiCadenceTickOutcome::QueueBackpressure {
            pending_count,
            max_pending_queue_records: config.max_pending_queue_records,
        });
    }
    capture_tick_outcome(config, clock, tick_index)
}

fn capture_tick_outcome(
    config: &ScreenAiCadenceRuntimeConfig,
    clock: ScreenAiCadenceTickClock,
    tick_index: u64,
) -> Result<ScreenAiCadenceTickOutcome, ActivityCaptureError> {
    match capture_active_window_png() {
        ScreenCaptureAttempt::Captured(image) => {
            let queue_job_id = record_captured_cadence_image(config, &image, clock, tick_index)?;
            Ok(ScreenAiCadenceTickOutcome::Recorded {
                queue_job_id: queue_job_id.0,
            })
        }
        ScreenCaptureAttempt::Degraded(metadata) => {
            Ok(ScreenAiCadenceTickOutcome::CaptureUnavailable {
                status: metadata.status,
            })
        }
    }
}

fn record_captured_cadence_image(
    config: &ScreenAiCadenceRuntimeConfig,
    image: &ocentra_parent_screen_capture_adapter::CapturedScreenImage,
    clock: ScreenAiCadenceTickClock,
    tick_index: u64,
) -> Result<crate::screen_ai_service_capture_event_builder::ScreenText, ActivityCaptureError> {
    record_captured_screen_image_to_paths(ScreenAiServiceCaptureRecord {
        paths: ScreenAiServiceCapturePaths {
            queue_dir: &config.queue_dir,
            journal_path: &config.journal_path,
            journal_key_path: &config.journal_key_path,
            store_path: &config.store_path,
        },
        image,
        clock,
        sequence_index: tick_index,
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE,
        source_id: SCREEN_SERVICE_SOURCE_ID,
        queue_job_id_prefix: SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX,
        result_id_prefix: SCREEN_SERVICE_RESULT_ID_PREFIX,
        event_id_prefix: SCREEN_SERVICE_EVENT_ID_PREFIX,
        evidence_id_prefix: SCREEN_SERVICE_EVIDENCE_ID_PREFIX,
        summary: SCREEN_SERVICE_SUMMARY_CAPTURED,
        model_id: SCREEN_SERVICE_MODEL_ID,
        template_version: SCREEN_SERVICE_TEMPLATE_VERSION,
        temporary_image_ttl_seconds: config.temporary_image_ttl_seconds,
    })
}
