#![forbid(unsafe_code)]

#[path = "../../src/activity_capture/errors.rs"]
mod activity_capture;
#[path = "../../src/activity_capture/persistence.rs"]
mod activity_capture_persistence;
#[path = "../support/activity_store_activity_path.rs"]
mod activity_store_path;
#[path = "../../src/activity_surface_read_models/screen_row.rs"]
mod activity_surface_read_models;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "screen_ai_analysis_runtime_tests.rs"]
mod screen_ai_analysis_runtime_tests;
#[path = "../../src/screen_ai_cadence_runtime_event.rs"]
mod screen_ai_cadence_runtime_event;
#[path = "screen_ai_cadence_runtime_tests.rs"]
mod screen_ai_cadence_runtime_tests;
mod screen_ai_foreground_runtime {
    pub(crate) mod types {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_foreground_runtime/types.rs"
        ));
    }
}
#[path = "../../src/screen_ai_foreground_runtime_config.rs"]
mod screen_ai_foreground_runtime_config;
#[path = "screen_ai_foreground_runtime_tests.rs"]
mod screen_ai_foreground_runtime_tests;
#[path = "../../src/screen_ai_retention_sweeper_deletion_events.rs"]
mod screen_ai_retention_sweeper_deletion_events;
#[path = "screen_ai_retention_sweeper_deletion_events_tests.rs"]
mod screen_ai_retention_sweeper_deletion_events_tests;
#[path = "../../src/screen_ai_retention_sweeper_runtime.rs"]
mod screen_ai_retention_sweeper_runtime;
#[path = "screen_ai_retention_sweeper_runtime_startup_tests.rs"]
mod screen_ai_retention_sweeper_runtime_startup_tests;
#[path = "screen_ai_retention_sweeper_runtime_tests.rs"]
mod screen_ai_retention_sweeper_runtime_tests;
#[path = "../screen_ai_runtime/service_capture_event_builder.rs"]
mod screen_ai_service_capture_event_builder;
#[path = "../../src/screen_ai_service_event_bridge.rs"]
mod screen_ai_service_event_bridge;
#[path = "screen_ai_service_event_bridge_tests.rs"]
mod screen_ai_service_event_bridge_tests;
#[path = "../support/screen_ai_service_event_subscription_mod.rs"]
pub(crate) mod screen_ai_service_event_subscription;
#[path = "screen_ai_service_event_subscription_tests.rs"]
mod screen_ai_service_event_subscription_tests;
#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;
#[path = "../screen_ai_runtime/text.rs"]
mod test_text;
#[path = "../../src/time.rs"]
mod time;

mod screen_ai_analysis_runtime {
    pub(crate) mod adapter {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_analysis_runtime/adapter.rs"
        ));
    }
    pub(crate) mod adapter_process {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_analysis_runtime/adapter_process.rs"
        ));
    }
    pub(crate) mod adapter_redaction {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_analysis_runtime/adapter_redaction.rs"
        ));
    }
    mod adapter_tests {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/unit/screen_ai_analysis_runtime_adapter_tests.rs"
        ));
    }
    pub(crate) mod config {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_analysis_runtime/config.rs"
        ));
    }
    pub(crate) mod event_record {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_analysis_runtime/event_record.rs"
        ));
    }
    pub(crate) mod lease_heartbeat {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_analysis_runtime/lease_heartbeat.rs"
        ));
    }
    mod event_record_tests {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/unit/screen_ai_analysis_runtime_event_record_tests.rs"
        ));
    }
    pub(crate) mod queue {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/screen_ai_analysis_runtime/queue.rs"
        ));
    }

    type ScreenAiAnalysisCycleClock = config::ScreenAiAnalysisCycleClock;
    type ScreenAiAnalysisCycleOutcome = config::ScreenAiAnalysisCycleOutcome;
    type ScreenAiAnalysisRuntimeConfig = config::ScreenAiAnalysisRuntimeConfig;

    use ocentra_parent_agent_core::{
        activity_store::ActivityStore, screen_evidence_queue::ScreenEvidenceQueue,
    };
    use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_SERVICE_METADATA;

    use crate::{
        activity_capture::ActivityCaptureError,
        activity_capture_persistence::record_activity_events_to_paths,
        activity_surface_read_models::activity_screen_row_from_result,
        screen_ai_service_event_subscription,
        screen_ai_service_event_subscription::ScreenAiServiceEventRuntime,
    };

    use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;

    use self::{
        lease_heartbeat::{start_analysis_lease_heartbeat, ScreenAnalysisLeaseHeartbeatInput},
        queue::{
            first_queued_screen_image, load_existing_screen_key, metadata_result_for_queue_job,
        },
    };

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
        let queue = ScreenEvidenceQueue::open(&config.queue_dir, key.clone())?;
        let Some(image) = first_queued_screen_image(
            &queue,
            config.max_queue_scan,
            &clock,
            config.adapter_timeout_ms,
        )?
        else {
            return Ok(ScreenAiAnalysisCycleOutcome::QueueEmpty);
        };
        let _lease_heartbeat = start_analysis_lease_heartbeat(ScreenAnalysisLeaseHeartbeatInput {
            queue_dir: config.queue_dir.clone(),
            key,
            queue_job_id: image.queue_job_id.clone(),
            adapter_timeout_ms: config.adapter_timeout_ms,
        });
        let metadata = metadata_result_for_queue_job(&config.store_path, &image, &clock)?;
        if metadata
            .as_ref()
            .is_some_and(|result| result.provider_kind != SCREEN_PROVIDER_SERVICE_METADATA)
        {
            queue.complete_claimed_entry(&image.queue_job_id)?;
            return Ok(ScreenAiAnalysisCycleOutcome::AlreadyAnalyzed {
                queue_job_id: image.queue_job_id,
            });
        }

        let generation = adapter::run_adapter(config, &image, metadata.as_ref()).await;
        let event_record = event_record::analysis_event_record(
            &image,
            metadata.as_ref(),
            &clock,
            &generation,
            &config.ocr_redaction_policy,
        );
        let outcome = event_record::outcome_for_generation(&image, &generation, &event_record);
        record_activity_events_to_paths(
            &config.journal_path,
            &config.journal_key_path,
            &config.store_path,
            &[event_record::screen_analysis_event(&event_record)],
        )?;
        publish_latest_analysis_row_ready(
            event_runtime,
            config,
            &image.queue_job_id,
            &clock.timestamp,
        )
        .await?;
        queue.complete_claimed_entry(&image.queue_job_id)?;
        Ok(outcome)
    }

    pub(crate) fn adapter_runtime_status(
        command: Option<&std::path::Path>,
        timestamp: impl std::fmt::Display,
    ) -> LocalModelRuntimeStatus {
        let timestamp = timestamp.to_string();
        adapter::runtime_status(command, timestamp.as_str())
    }

    fn latest_analysis_row_for_queue_job(
        config: &ScreenAiAnalysisRuntimeConfig,
        queue_job_id: impl std::fmt::Display,
        generated_at: impl std::fmt::Display,
    ) -> Result<Option<ActivityScreenReadModelRow>, ActivityCaptureError> {
        let queue_job_id = queue_job_id.to_string();
        let generated_at = generated_at.to_string();
        let store = ActivityStore::open(&config.store_path)?;
        let summary = store.screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            generated_at.as_str(),
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

    async fn publish_latest_analysis_row_ready(
        event_runtime: Option<&ScreenAiServiceEventRuntime>,
        config: &ScreenAiAnalysisRuntimeConfig,
        queue_job_id: impl std::fmt::Display,
        timestamp: impl std::fmt::Display,
    ) -> Result<(), ActivityCaptureError> {
        let Some(runtime) = event_runtime else {
            return Ok(());
        };
        let timestamp = timestamp.to_string();
        let Some(row) =
            latest_analysis_row_for_queue_job(config, queue_job_id, timestamp.as_str())?
        else {
            return Ok(());
        };
        let _ = runtime
            .publish_row_ready(
                row,
                screen_ai_service_event_subscription::ActionRefText(
                    constants::screen_flow::SCREEN_ACTION_EVENT_REF.to_string(),
                ),
                screen_ai_service_event_subscription::ObservedAtText(timestamp.to_string()),
            )
            .await;
        Ok(())
    }
}

mod screen_ai_service_event_subscription_live_view {
    use super::screen_ai_service_event_subscription::live_view_service_runtime;
    mod live_view_service_runtime_tests {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/unit/live_view_service_runtime_tests.rs"
        ));
    }
}
