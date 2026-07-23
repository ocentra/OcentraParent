use std::path::Path;

use ocentra_parent_agent_core::{
    activity_store::ActivityStore, screen_evidence_queue::ScreenEvidenceExpiredQueueEntry,
};

use crate::activity_surface_read_models::activity_screen_row_from_result;
use crate::screen_ai_service_event_subscription::{ObservedAtText, ScreenAiServiceEventRuntime};

#[path = "screen_ai_retention_sweeper_deletion_events/conversions.rs"]
mod conversions;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiRetentionSweeperDeletionEventOutcome {
    pub(crate) queue_job_id: String,
    pub(crate) downstream_event_count: usize,
    pub(crate) raw_image_escaped: bool,
}

pub(crate) async fn publish_screen_retention_deletion_events(
    runtime: &ScreenAiServiceEventRuntime,
    store_path: &Path,
    expired_entries: &[ScreenEvidenceExpiredQueueEntry],
    observed_at: impl Into<ScreenRetentionObservedAt>,
) -> Vec<ScreenAiRetentionSweeperDeletionEventOutcome> {
    let observed_at = observed_at.into();
    let mut outcomes = Vec::new();
    let Ok(store) = ActivityStore::open(store_path) else {
        return outcomes;
    };
    for entry in expired_entries {
        let Ok(Some(result)) = store.screen_evidence_result_for_queue_job(&entry.queue_job_id)
        else {
            continue;
        };
        if let Ok(report) = runtime
            .publish_deletion_row(
                activity_screen_row_from_result(result),
                ObservedAtText(observed_at.0.clone()),
            )
            .await
        {
            outcomes.push(ScreenAiRetentionSweeperDeletionEventOutcome {
                queue_job_id: entry.queue_job_id.clone(),
                downstream_event_count: report.publish_reports.len(),
                raw_image_escaped: report.raw_image_escaped(),
            });
        }
    }
    outcomes
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenRetentionObservedAt(String);
