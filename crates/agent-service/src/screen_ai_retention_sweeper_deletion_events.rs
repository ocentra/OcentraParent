use std::path::Path;

use ocentra_parent_agent_core::screen_evidence_queue::ScreenEvidenceExpiredQueueEntry;

use crate::screen_ai_service_event_bridge::{
    publish_screen_deletion_event_for_queue_job, ScreenAiQueueJobId,
};
use crate::screen_ai_service_event_subscription::ObservedAtText;

#[path = "screen_ai_retention_sweeper_deletion_events/conversions.rs"]
mod conversions;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiRetentionSweeperDeletionEventOutcome {
    pub(crate) queue_job_id: String,
    pub(crate) downstream_event_count: usize,
    pub(crate) raw_image_escaped: bool,
}

pub(crate) async fn publish_screen_retention_deletion_events(
    store_path: &Path,
    expired_entries: &[ScreenEvidenceExpiredQueueEntry],
    observed_at: impl Into<ScreenRetentionObservedAt>,
) -> Vec<ScreenAiRetentionSweeperDeletionEventOutcome> {
    let observed_at = observed_at.into();
    let mut outcomes = Vec::new();
    for entry in expired_entries {
        if let Ok(Some(report)) = publish_screen_deletion_event_for_queue_job(
            store_path,
            ScreenAiQueueJobId(entry.queue_job_id.clone()),
            ObservedAtText(observed_at.0.clone()),
        )
        .await
        {
            outcomes.push(ScreenAiRetentionSweeperDeletionEventOutcome {
                queue_job_id: entry.queue_job_id.clone(),
                downstream_event_count: report.stored_events.len(),
                raw_image_escaped: report.raw_image_escaped(),
            });
        }
    }
    outcomes
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenRetentionObservedAt(String);

impl ScreenRetentionObservedAt {
    pub(crate) fn from_display(value: impl std::fmt::Display) -> Self {
        Self(value.to_string())
    }
}
