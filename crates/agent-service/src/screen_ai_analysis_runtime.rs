mod adapter;
mod adapter_output_fields;
mod adapter_process;
mod adapter_redaction;
#[cfg(test)]
mod adapter_tests;
mod config;
mod event_record;
mod queue;

use std::time::Duration;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityScreenReadModelRow, LocalAiProviderSchedulerJobClass,
    SCREEN_PROVIDER_SERVICE_METADATA,
};

use crate::{
    activity_capture::{record_activity_events_to_paths, ActivityCaptureError},
    activity_surface_read_models::activity_screen_row_from_result,
    local_ai_provider_scheduler::local_ai_provider_scheduler,
    screen_ai_service_event_subscription::ScreenAiServiceEventRuntime,
};

pub(crate) use config::{
    ScreenAiAnalysisCycleClock, ScreenAiAnalysisCycleOutcome, ScreenAiAnalysisRuntimeConfig,
};
use event_record::{analysis_event_record, outcome_for_generation, screen_analysis_event};
use queue::{first_queued_screen_image, load_existing_screen_key, metadata_result_for_queue_job};

pub(crate) fn spawn_screen_ai_analysis_runtime() {
    if let Some(config) = ScreenAiAnalysisRuntimeConfig::from_environment() {
        tokio::spawn(async move {
            run_screen_ai_analysis_runtime(config).await;
        });
    }
}

async fn run_screen_ai_analysis_runtime(config: ScreenAiAnalysisRuntimeConfig) {
    let mut interval = tokio::time::interval(Duration::from_secs(config.poll_seconds));
    let event_runtime = ScreenAiServiceEventRuntime::start().await.ok();
    let mut job_count = 0;
    let mut tick_count = 0;
    loop {
        interval.tick().await;
        tick_count += 1;
        let outcome = record_screen_ai_analysis_cycle_with_events(
            &config,
            ScreenAiAnalysisCycleClock::from_system_time(),
            event_runtime.as_ref(),
        )
        .await;
        if is_counted_cycle(&outcome) {
            job_count += 1;
        }
        if config.max_jobs.is_some_and(|max| job_count >= max) {
            break;
        }
        if config.max_ticks.is_some_and(|max| tick_count >= max) {
            break;
        }
    }
}

#[cfg(test)]
pub(crate) async fn record_screen_ai_analysis_cycle(
    config: &ScreenAiAnalysisRuntimeConfig,
    clock: ScreenAiAnalysisCycleClock,
) -> Result<ScreenAiAnalysisCycleOutcome, ActivityCaptureError> {
    record_screen_ai_analysis_cycle_with_events(config, clock, None).await
}

pub(crate) async fn record_screen_ai_analysis_cycle_with_events(
    config: &ScreenAiAnalysisRuntimeConfig,
    clock: ScreenAiAnalysisCycleClock,
    event_runtime: Option<&ScreenAiServiceEventRuntime>,
) -> Result<ScreenAiAnalysisCycleOutcome, ActivityCaptureError> {
    if !config.screen_analysis_enabled {
        return Ok(ScreenAiAnalysisCycleOutcome::Suppressed);
    }
    let Some(key) = load_existing_screen_key(&config.journal_key_path)? else {
        return Ok(ScreenAiAnalysisCycleOutcome::QueueEmpty);
    };
    let queue = ocentra_parent_agent_core::ScreenEvidenceQueue::open(&config.queue_dir, key)?;
    let Some(image) = first_queued_screen_image(&queue, config.max_queue_scan)? else {
        return Ok(ScreenAiAnalysisCycleOutcome::QueueEmpty);
    };
    let metadata = metadata_result_for_queue_job(&config.store_path, &image.queue_job_id, &clock)?;
    if metadata
        .as_ref()
        .is_some_and(|result| result.provider_kind != SCREEN_PROVIDER_SERVICE_METADATA)
    {
        queue.remove_entries(std::slice::from_ref(&image.queue_job_id))?;
        return Ok(ScreenAiAnalysisCycleOutcome::AlreadyAnalyzed {
            queue_job_id: image.queue_job_id,
        });
    }

    let runtime = adapter::runtime_status(config.adapter_command.as_deref(), &clock.timestamp);
    let generation = local_ai_provider_scheduler()
        .run_generation_job(
            LocalAiProviderSchedulerJobClass::ChildSafety,
            runtime,
            || adapter::run_adapter(config, &image, metadata.as_ref()),
        )
        .await;
    let event_record = analysis_event_record(
        &image,
        metadata.as_ref(),
        &clock,
        &generation,
        &config.ocr_redaction_policy,
    );
    let outcome = outcome_for_generation(&image.queue_job_id, &generation, &event_record);
    record_activity_events_to_paths(
        &config.journal_path,
        &config.journal_key_path,
        &config.store_path,
        &[screen_analysis_event(&event_record)],
    )?;
    if let (Some(runtime), Some(row)) = (
        event_runtime,
        latest_analysis_row_for_queue_job(config, &image.queue_job_id, &clock.timestamp)?,
    ) {
        let _ = runtime
            .publish_row_ready(
                row,
                constants::screen_flow::SCREEN_ACTION_EVENT_REF,
                &clock.timestamp,
            )
            .await;
    }
    queue.remove_entries(std::slice::from_ref(&image.queue_job_id))?;
    Ok(outcome)
}

fn latest_analysis_row_for_queue_job(
    config: &ScreenAiAnalysisRuntimeConfig,
    queue_job_id: &str,
    generated_at: &str,
) -> Result<Option<ActivityScreenReadModelRow>, ActivityCaptureError> {
    let store = ActivityStore::open(&config.store_path)?;
    let summary = store.screen_evidence_recent_summary(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        generated_at,
    )?;
    Ok(summary
        .results
        .into_iter()
        .find(|result| {
            result.queue_job_id == queue_job_id
                && result.provider_kind != SCREEN_PROVIDER_SERVICE_METADATA
        })
        .map(activity_screen_row_from_result))
}

fn is_counted_cycle(outcome: &Result<ScreenAiAnalysisCycleOutcome, ActivityCaptureError>) -> bool {
    matches!(
        outcome,
        Ok(ScreenAiAnalysisCycleOutcome::AlreadyAnalyzed { .. })
            | Ok(ScreenAiAnalysisCycleOutcome::ProviderUnavailable { .. })
            | Ok(ScreenAiAnalysisCycleOutcome::InvalidOutput { .. })
            | Ok(ScreenAiAnalysisCycleOutcome::Recorded { .. })
    )
}
