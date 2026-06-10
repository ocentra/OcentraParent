use std::path::Path;

use ocentra_parent_agent_core::ScreenEvidenceExpiredQueueEntry;

use crate::screen_ai_service_event_bridge::publish_screen_deletion_event_for_queue_job;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiRetentionSweeperDeletionEventOutcome {
    pub(crate) queue_job_id: String,
    pub(crate) downstream_event_count: usize,
    pub(crate) raw_image_escaped: bool,
}

pub(crate) async fn publish_screen_retention_deletion_events(
    store_path: &Path,
    expired_entries: &[ScreenEvidenceExpiredQueueEntry],
    observed_at: &str,
) -> Vec<ScreenAiRetentionSweeperDeletionEventOutcome> {
    let mut outcomes = Vec::new();
    for entry in expired_entries {
        if let Ok(Some(report)) = publish_screen_deletion_event_for_queue_job(
            store_path,
            &entry.queue_job_id,
            observed_at,
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
